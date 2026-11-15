"""T84: authenticate historical runtime locks with one bounded graph extension.

Current desktop/raster tests still run --locked. Old captures retain their
producing lock; no package version/checksum change is accepted as equivalent.
"""
import hashlib
import re
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
CURRENT=ROOT/'desktop/faithful/Cargo.lock'

def compatible(current,historical):
    if current==historical:return True
    match=re.search(rb'(?ms)^\[\[package\]\]\nname = "onenil-engine"\n.*?(?=^\[\[package\]\]|\Z)',current)
    if match is None:return False
    block=match.group()
    if block.count(b' "sha2",\n')!=1:return False
    previous=current[:match.start()]+block.replace(b' "sha2",\n',b'',1)+current[match.end():]
    return previous==historical

def capture_lock_digest(record):
    producer=record['producer']
    assert re.fullmatch('[0-9a-f]{40}',producer),'invalid historical producer'
    historical=subprocess.check_output(['git','-C',str(ROOT),'show',producer+':desktop/faithful/Cargo.lock'])
    digest=hashlib.sha256(historical).hexdigest()
    assert record['runtime']['cargo_lock_sha256']==digest,'producing runtime lock differs'
    assert compatible(CURRENT.read_bytes(),historical),'unclassified desktop dependency change'
    return digest
