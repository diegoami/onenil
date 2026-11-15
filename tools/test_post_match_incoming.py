"""T84 sealed early leaves reject changed numeric values and corpus drift."""
import json
import tempfile
import unittest
from pathlib import Path
import post_match_incoming as p
class EarlySourceTests(unittest.TestCase):
    def test_accepted_original_leaves(self):p.verify_local()
    def copy(self,directory):
        for name in p.PINS:(directory/name).write_bytes((p.DIRECTORY/name).read_bytes())
    def test_rehashed_numeric_late_seed_is_still_foreign(self):
        with tempfile.TemporaryDirectory()as temp:
            root=Path(temp);self.copy(root);path=root/'incoming-r0001.json'
            r=json.loads(path.read_text());r['payload']['boundary']['seed']=15361
            next(f for f in r['payload']['fields']if f['field']=='seed+0')['value']=15361
            import hashlib
            r['payload_sha256']=hashlib.sha256(json.dumps(r['payload'],sort_keys=True,separators=(',',':')).encode()).hexdigest()
            path.write_text(json.dumps(r))
            with self.assertRaisesRegex(ValueError,'digest differs'):p.verify_local(root)
    def test_missing_or_extra_leaf_refuses(self):
        for extra in [False,True]:
            with tempfile.TemporaryDirectory()as temp:
                root=Path(temp);self.copy(root)
                if extra:(root/'invented.json').write_text('{}')
                else:(root/'incoming-r0002.json').unlink()
                with self.assertRaisesRegex(ValueError,'census differs'):p.verify_local(root)
if __name__=='__main__':unittest.main()
