"""T68 cold public source-pin/census controls for accepted lookup witnesses."""
import tempfile,unittest
from pathlib import Path
from unittest.mock import patch
import edited_lookup_inputs as m

class LookupInputTests(unittest.TestCase):
    def test_exact_before_bitmap_has_32_fields(self):
        rows=m.sources();self.assertEqual(len(rows),32)
        self.assertEqual({r['offset'] for r in rows},set(range(0xD36,0xD56)))
    def test_changed_original_leaf_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory)
            for name in m.PINS:(directory/name).write_bytes((m.DIRECTORY/name).read_bytes())
            p=directory/'lookup-r0001.json';p.write_bytes(p.read_bytes()+b'\n')
            with patch.object(m,'DIRECTORY',directory):
                with self.assertRaisesRegex(ValueError,'source pin differs'):m.sources()
    def test_incomplete_source_census_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(m,'DIRECTORY',Path(directory)):
                with self.assertRaisesRegex(ValueError,'census differs'):m.sources()

if __name__=='__main__':unittest.main()
