"""T68 public ownership corruption controls; no original emulator required."""
import copy,json,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
import edited_watched_inputs as subject

class EditedWatchedInputTests(unittest.TestCase):
    def test_own_case_has_complete_before_support_and_preparation(self):
        value=subject.request();self.assertEqual(len(value['support']),2059)
        self.assertEqual(value['preparation']['setup']['legacy_income_total'],0)
        self.assertTrue(value['no_pending_key'])
    def test_changed_or_output_injected_fixture_refuses(self):
        original=subject.admission()
        with tempfile.TemporaryDirectory() as directory:
            path=Path(directory)/'incoming.json'
            for damage in ('byte','output','identity'):
                value=copy.deepcopy(original)
                if damage=='byte':value['support'][0]['value']^=1
                elif damage=='output':value['ticks']=[]
                else:value['sources'].pop(next(iter(value['sources'])))
                path.write_text(json.dumps(value))
                with patch.object(subject,'FIXTURE',path):
                    with self.assertRaisesRegex(ValueError,'original before values'):subject.admission()

if __name__=='__main__':unittest.main()
