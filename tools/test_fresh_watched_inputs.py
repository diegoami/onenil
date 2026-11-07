"""Public provenance and admission corruption checks for T61 incoming facts."""
import copy
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import fresh_watched_inputs as subject


class FreshIncomingTests(unittest.TestCase):
    def test_independent_pairs_project_only_unchanged_inputs(self):
        value = subject.admission()
        self.assertEqual(len(value['support']), 2032)
        self.assertTrue(all(f['area'] == 'ds' for f in value['support']))
        self.assertTrue(any(f['offset'] == 0x949F for f in value['support']))

    def test_changed_original_leaf_refuses_even_with_valid_json(self):
        with tempfile.TemporaryDirectory() as directory:
            directory = Path(directory)
            for name in subject.PINS:
                (directory / name).write_bytes((subject.DIRECTORY / name).read_bytes())
            name = next(iter(subject.PINS))
            value = subject.read(directory / name)
            value['payload']['fields'][0]['entry'] ^= 1
            (directory / name).write_text(json.dumps(value))
            with patch.object(subject, 'DIRECTORY', directory):
                with self.assertRaisesRegex(ValueError, 'source digest differs'):
                    subject.projection()

    def test_changed_or_output_seeded_admission_refuses(self):
        original = subject.admission()
        with tempfile.TemporaryDirectory() as directory:
            fixture = Path(directory) / 'incoming.json'
            for damage in ('value', 'output', 'source'):
                value = copy.deepcopy(original)
                if damage == 'value':
                    value['support'][0]['value'] ^= 1
                elif damage == 'output':
                    value['ticks'] = []
                else:
                    value['sources'].pop(next(iter(value['sources'])))
                fixture.write_text(json.dumps(value))
                with patch.object(subject, 'FIXTURE', fixture):
                    with self.assertRaisesRegex(ValueError, 'admission differs'):
                        subject.admission()


if __name__ == '__main__':
    unittest.main()
