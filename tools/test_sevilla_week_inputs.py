"""[designed:T93] The sealed receipt refuses altered week facts and provenance."""
import copy
import json
import tempfile
import unittest
from pathlib import Path
import sevilla_week_inputs as s

class ReceiptTests(unittest.TestCase):
    def test_pinned_receipt_authenticates(self):
        s.verify_receipt(json.loads(s.OUTPUT.read_text()))

    def test_mutated_fields_refuse(self):
        original = json.loads(s.OUTPUT.read_text())
        edits = [
            lambda x: x['presentation']['cash'].__setitem__('total_in', 38761),
            lambda x: x['presentation']['transfer_rows'][0].__setitem__('player', 'foreign'),
            lambda x: x['following_fixture'].__setitem__('opponent_flags', 0),
            lambda x: x['conditions'].__setitem__('2581', 14),
            lambda x: x.__setitem__('research_commit', '0' * 40),
            lambda x: x['sources'].__setitem__(next(iter(x['sources'])), '0' * 64),
            lambda x: x['original_save'].__setitem__('sha256', '0' * 64),
        ]
        for edit in edits:
            probe = copy.deepcopy(original)
            edit(probe)
            with self.assertRaises(ValueError):
                s.verify_receipt(probe)

    def test_changed_source_refuses_before_decode(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            name = next(iter(s.SOURCES))
            path = root / name
            path.parent.mkdir(parents=True)
            path.write_text('{}')
            with self.assertRaisesRegex(ValueError, 'source identity'):
                s.read_record(root, name)

if __name__ == '__main__':
    unittest.main()
