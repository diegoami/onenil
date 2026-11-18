"""T91 the committed Sevilla reference assets authenticate and refuse tampering."""
import hashlib
import json
import tempfile
import unittest
from pathlib import Path
import sevilla_reference_inputs as s
class SevillaAssetTests(unittest.TestCase):
    def test_accepted_assets_authenticate(self):s.verify_local()
    def copy(self,directory):
        for name in ['engine/tests/fixtures/sevilla_watched/identity.json','engine/tests/fixtures/sevilla_watched/presentation-r0001.json',
                     'engine/tests/fixtures/sevilla_post_match/sevilla-result-r0001.json',
                     'campaign/fixtures/sevilla-post-match.json']:
            source=s.ROOT/name
            target=directory/name
            target.parent.mkdir(parents=True,exist_ok=True)
            target.write_bytes(source.read_bytes())
    def probe(self,edit):
        with tempfile.TemporaryDirectory()as temp:
            root=Path(temp);self.copy(root)
            edit(root)
            with self.assertRaises(ValueError):s.verify_local(root/'engine/tests/fixtures')
    def test_tampered_presentation_refuses(self):
        def edit(root):
            path=root/'engine/tests/fixtures/sevilla_watched/presentation-r0001.json'
            doc=json.loads(path.read_text())
            doc['ticks'][-1]['scores']=[2,1]
            path.write_bytes((json.dumps(doc,separators=(',',':'))+'\n').encode())
        self.probe(edit)
    def test_tampered_identity_digest_refuses(self):
        def edit(root):
            path=root/'engine/tests/fixtures/sevilla_watched/identity.json'
            doc=json.loads(path.read_text())
            doc['presentation_sha256']='0'*64
            path.write_bytes((json.dumps(doc,indent=1)+'\n').encode())
        self.probe(edit)
    def test_swapped_caller_fixture_refuses(self):
        fresh=json.loads((s.ROOT/'engine/tests/fixtures/post_match/fresh-result-r0004.json').read_text())
        def edit(root):
            path=root/'engine/tests/fixtures/sevilla_post_match/sevilla-result-r0001.json'
            doc=json.loads(path.read_text())
            doc['caller']['frame']=fresh['caller']['frame']
            path.write_bytes((json.dumps(doc,indent=1)+'\n').encode())
        self.probe(edit)
    def test_foreign_psg_identity_refuses_the_census(self):
        with tempfile.TemporaryDirectory()as temp:
            root=Path(temp);self.copy(root)
            path=root/'engine/tests/fixtures/sevilla_post_match/sevilla-result-r0001.json'
            doc=json.loads(path.read_text())
            doc['caller']['census']={'reads':2354,'writes':407,'initial':431}
            path.write_bytes((json.dumps(doc,indent=1)+'\n').encode())
            self.assertNotEqual(doc['caller']['census'],json.loads(
                (s.FIXTURES/'sevilla_post_match/sevilla-result-r0001.json').read_text())['caller']['census'])
    def test_native_assets_carry_no_timer_concept(self):
        for name in ['sevilla_watched/identity.json','sevilla_post_match/sevilla-result-r0001.json',
                     '../../../campaign/fixtures/sevilla-post-match.json']:
            raw=(s.FIXTURES/name).read_text().lower()
            self.assertNotIn('pit',raw);self.assertNotIn('divisor',raw);self.assertNotIn('timer',raw)
    def test_campaign_fixture_binds_the_caller_record(self):
        s.verify_local()
if __name__=='__main__':unittest.main()
