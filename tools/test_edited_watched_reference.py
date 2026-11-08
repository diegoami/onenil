"""T68 full reference identity, independent receipt and acceptance-gate controls."""
import json,os,shutil,tempfile,unittest
from pathlib import Path
from unittest.mock import patch
import edited_watched_reference as reference
import edited_lookup_inputs as lookup
import replay_edited_watched as replay

class EditedReferenceTests(unittest.TestCase):
    def source(self):return Path(os.environ.get('ONENIL_REFERENCE_TEST_DIR',reference.DIRECTORY))
    def test_complete_independent_source(self):
        index,finish=reference.sources(self.source())
        self.assertEqual((len(index['chunks']),finish['ticks'],finish['draws'],finish['actions']),(63,3970,170212,869))
        self.assertEqual(finish['boundary'],'0AB9:EBE2')
    def test_source_acceptance_markers_are_required(self):
        for original,bitmap in [(None,'1'*40),('1'*40,None)]:
            with patch.object(reference,'RESEARCH_ACCEPTANCE',original),patch.object(lookup,'RESEARCH_ACCEPTANCE',bitmap):
                with self.assertRaisesRegex(ValueError,'accepted T71 and T73'):replay.require_acceptance()
    def test_import_refuses_before_original_research_acceptance(self):
        with patch.object(reference,'RESEARCH_ACCEPTANCE',None):
            with self.assertRaisesRegex(ValueError,'accepted T71 research merge'):reference.import_sources(self.source())
    def test_corrupt_repeat_receipt_chunk_and_census_refuse(self):
        for damage in ('repeat','receipt','chunk','foreign'):
            with tempfile.TemporaryDirectory() as directory:
                root=Path(directory)/'source';shutil.copytree(self.source(),root)
                if damage=='repeat':
                    p=root/'repeat-0004.json';v=json.loads(p.read_text());v['result']['ticks']-=1;p.write_text(json.dumps(v))
                elif damage=='receipt':
                    p=root/'attempt-0004-verified-000000.json';v=json.loads(p.read_text());v['producer']='0'*40;p.write_text(json.dumps(v))
                elif damage=='chunk':
                    p=root/'chunk-000000.json';p.write_bytes(p.read_bytes()+b'\n')
                else:(root/'foreign.json').write_text('{}')
                with self.assertRaises(ValueError):reference.sources(root)

if __name__=='__main__':unittest.main()
