"""T66 public source authentication and input/output ownership controls."""
import copy
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
import replay_edited_preparation as subject

class EditedPreparationTests(unittest.TestCase):
    def test_complete_source_projection_uses_before_values(self):
        value,original=subject.admission()
        self.assertEqual(len(original['points']),11)
        self.assertEqual(len(original['draws']),258)
        self.assertEqual(value['input']['setup']['legacy_income_total'],0)
        self.assertEqual(original['points'][7]['income_total'],25841)
        self.assertNotIn('first_entry',value['input'])
        self.assertEqual(set(value['input']),{'setup','incoming'})

    def test_changed_source_leaf_refuses(self):
        with tempfile.TemporaryDirectory() as directory:
            directory=Path(directory)
            for name in subject.PINS:
                (directory/name).write_bytes((subject.DIRECTORY/name).read_bytes())
            path=directory/'support-r0001.json';path.write_bytes(path.read_bytes()+b'\n')
            with patch.object(subject,'DIRECTORY',directory):
                with self.assertRaisesRegex(ValueError,'source digest differs'):subject.sources()

    def test_changed_fixture_or_computed_output_refuses(self):
        original,_=subject.admission()
        with tempfile.TemporaryDirectory() as directory:
            fixture=Path(directory)/'admission.json'
            for damage in ('fixture','income','output','identity'):
                value=copy.deepcopy(original)
                if damage=='fixture':value['input']['setup']['legacy']['config'][0]^=1
                elif damage=='income':value['input']['setup']['legacy_income_total']=25841
                elif damage=='output':value['input']['first_entry']={}
                else:value['repeat_sha256']='0'*64
                fixture.write_text(json.dumps(value))
                with patch.object(subject,'FIXTURE',fixture):
                    with self.assertRaisesRegex(ValueError,'incoming source projection'):subject.admission()

    def test_source_directory_requires_exact_census(self):
        with tempfile.TemporaryDirectory() as directory:
            with patch.object(subject,'DIRECTORY',Path(directory)):
                with self.assertRaisesRegex(ValueError,'census differs'):subject.sources()

if __name__=='__main__':unittest.main()
