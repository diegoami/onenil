"""T82 static translation and corruption controls; no execution parity claim."""
import argparse
import copy
import json
import re
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import compile_watched as compiler
import generate_post_match as graph


class PostMatchGraphTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = graph.GRAPH.read_bytes()
        cls.metadata = json.loads(graph.MANIFEST.read_text())

    def test_exact_regeneration_from_pinned_listings(self):
        graph.verify(self.source, self.metadata, RESEARCH_ROOT)

    def test_root_and_complete_direct_call_closure(self):
        text = self.source.decode()
        definitions = set(re.findall(r'pub\(super\) fn (p_[0-9a-f_]+)', text))
        self.assertIn('p_0ab9_e679', definitions)
        self.assertIn('p_1998_1897', definitions)
        self.assertIn('p_1998_14a5', definitions)
        self.assertIn('p_1b55_2301', definitions)
        calls = set(re.findall(r'\b(p_[0-9a-f_]+)\(', text))
        reused = set()
        for module in graph.REUSED:
            reused.update(re.findall(r'pub\(super\) fn (p_[0-9a-f_]+)',
                                     (graph.BASE / (module + '.rs')).read_text()))
        self.assertFalse(definitions & reused)
        self.assertFalse(calls - definitions - reused)
        self.assertEqual(re.findall(r'Err\("(unported[^"]*)"', text),
                         ['unported original 1998:14A5'])
        self.assertEqual(text.count('invalid compiled label'), len(definitions) - 1)
        self.assertIn('Err("unported original 1998:14A5".into())', text)
        # The scalar condition kernel and its descendants are not retranslated.
        self.assertNotIn('p_1998_13f9', definitions)

    def test_every_translated_instruction_has_original_address(self):
        text = self.source.decode()
        code, _ = compiler.listing(RESEARCH_ROOT)
        external = set()
        for module in graph.REUSED:
            external.update((int(a, 16), int(b, 16)) for a, b in re.findall(
                r'pub\(super\) fn p_([0-9a-f]{4})_([0-9a-f]{4})',
                (graph.BASE / (module + '.rs')).read_text()))
        closure = compiler.graph(code, [(0x0AB9, 0xE679)], external, {(0x1998, 0x14A5)})
        expected = {(segment, ip) for (segment, entry), rows in closure.items()
                    if (segment, entry) != (0x1998, 0x14A5) for ip in rows}
        actual = {(int(a), int(b)) for a, b in re.findall(r'c.pc = \((\d+), (\d+)\);', text)}
        self.assertEqual(actual, expected)
        self.assertEqual(self.metadata['procedures'], len(closure))
        self.assertEqual(text.count('c.step()?;'), sum(len(rows) for key, rows in closure.items()
                                                    if key != (0x1998, 0x14A5)))

    def test_changed_graph_refuses(self):
        with self.assertRaisesRegex(ValueError, 'graph digest differs'):
            graph.verify(self.source + b'\n', self.metadata, RESEARCH_ROOT)

    def test_changed_listing_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name in self.metadata['listing_digests']:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes((RESEARCH_ROOT / name).read_bytes())
            first = root / next(iter(self.metadata['listing_digests']))
            first.write_bytes(first.read_bytes() + b'\n')
            with self.assertRaisesRegex(ValueError, 'listing digest differs'):
                graph.verify(self.source, self.metadata, root)

    def test_changed_reused_graph_refuses(self):
        metadata = copy.deepcopy(self.metadata)
        metadata['reused'][0]['sha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'reused module digest differs'):
            graph.verify(self.source, metadata, RESEARCH_ROOT)

    def test_reblessed_wrong_metadata_refuses_regeneration(self):
        metadata = copy.deepcopy(self.metadata)
        metadata['instructions'] += 1
        with self.assertRaisesRegex(ValueError, 'regeneration differs'):
            graph.verify(self.source, metadata, RESEARCH_ROOT)

    def test_boundary_cannot_be_silently_enabled_or_extended(self):
        for field, value in [('execution_validated', True), ('boundaries', {}),
                             ('roots', ['0AB9:E532']), ('source_addresses', False)]:
            metadata = copy.deepcopy(self.metadata)
            metadata[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                graph.verify(self.source, metadata, RESEARCH_ROOT)

    def test_unexpected_instruction_fails_instead_of_deferring(self):
        code = {(0x0AB9, 0xE679): {0xE679: ('unported_opcode', '', 0xE67A, 'fixture')}}
        with self.assertRaises(ValueError):
            compiler.compile_graph(code)

    def test_generator_cannot_overwrite_record(self):
        with patch('sys.argv', ['generate_post_match.py', '--research-root', str(RESEARCH_ROOT)]):
            with self.assertRaises(FileExistsError):
                graph.main()


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--research-root', required=True, type=Path)
    args, remaining = parser.parse_known_args()
    RESEARCH_ROOT = args.research_root.resolve()
    unittest.main(argv=[__file__] + remaining)
