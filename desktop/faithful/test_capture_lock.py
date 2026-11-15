"""T84 historical evidence accepts only the exact producing graph + sha2 edge."""
import copy
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch
import capture_lock as c
class LockTests(unittest.TestCase):
    def setUp(self):
        self.record=json.loads((c.ROOT/'records/T65/window-r0001.json').read_text())
        self.old=subprocess.check_output(['git','-C',str(c.ROOT),'show',self.record['producer']+':desktop/faithful/Cargo.lock'])
    def test_actual_capture_producer_and_only_added_edge(self):
        self.assertEqual(c.capture_lock_digest(self.record),self.record['runtime']['cargo_lock_sha256'])
        self.assertTrue(c.compatible(self.old,self.old))
        self.assertTrue(c.compatible(c.CURRENT.read_bytes(),self.old))
    def test_other_version_checksum_or_edge_refuses(self):
        current=c.CURRENT.read_bytes()
        for altered in [current.replace(b'checksum = "',b'checksum = "0',1),
                        current.replace(b'version = "',b'version = "99',1),
                        current.replace(b' "sha2",\n',b' "sha2",\n "invented",\n',1)]:
            self.assertFalse(c.compatible(altered,self.old))
    def test_wrong_recorded_lock_refuses(self):
        bad=copy.deepcopy(self.record);bad['runtime']['cargo_lock_sha256']='0'*64
        with self.assertRaisesRegex(AssertionError,'producing runtime lock'):c.capture_lock_digest(bad)
    def test_wrong_current_graph_refuses_even_with_valid_record(self):
        with tempfile.TemporaryDirectory()as temp:
            p=Path(temp)/'Cargo.lock';p.write_bytes(c.CURRENT.read_bytes()+b'\n')
            with patch.object(c,'CURRENT',p),self.assertRaisesRegex(AssertionError,'unclassified desktop'):c.capture_lock_digest(self.record)
    def test_invalid_producer_refuses_before_git(self):
        bad=copy.deepcopy(self.record);bad['producer']='HEAD'
        with self.assertRaisesRegex(AssertionError,'invalid historical producer'):c.capture_lock_digest(bad)
if __name__=='__main__':unittest.main()
