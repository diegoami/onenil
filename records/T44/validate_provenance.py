#!/usr/bin/env python3
"""Reject changed/rehashed prepared admissions and original numeric source leaves."""
import copy
import hashlib
import json
import shutil
import sys
import tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tools'))
import replay_prepared_watched as replay
fixture=replay.read(replay.FIXTURE)
replay.verify(fixture)

def rejects(value):
    try:replay.verify(value)
    except (ValueError,KeyError,TypeError):return
    raise AssertionError('tampered prepared admission accepted')
count=0
for mutate in [
    lambda v:v['input']['support'][0].update(value=v['input']['support'][0]['value']^1),
    lambda v:v['input']['support'].pop(),
    lambda v:v['input']['support'].append(copy.deepcopy(v['input']['support'][0])),
    lambda v:v['input'].update(expected_first_entry={}),
    lambda v:v['input']['preparation'].update(expected_draws=[]),
    lambda v:v['input']['support'].append(dict(area='ds',offset=0xA51C,value=0)),
    lambda v:v['input']['support'].append(dict(area='ds',offset=0xCF32,value=156)),
    lambda v:v['input']['support'].append(dict(area='ds',offset=0xD7BA,value=0)),
    lambda v:v['input']['support'].append(dict(area='frame',offset=0,value=0)),
    lambda v:v['input']['support'].append(dict(area='background',offset=0,value=32)),
    lambda v:v['input'].update(no_pending_key=False),
    lambda v:v['input']['support'][0].update(value=True),
    lambda v:v.update(sources={}),
]:
    bad=copy.deepcopy(fixture);mutate(bad);rejects(bad);count+=1
# Rewriting the fixture and keeping its own source declarations is insufficient:
# its independently pinned root must also remain exact.
original_fixture=replay.FIXTURE
with tempfile.TemporaryDirectory() as directory:
    replay.FIXTURE=Path(directory)/'admission.json'
    bad=copy.deepcopy(fixture);bad['input']['support'][0]['value']^=1
    replay.FIXTURE.write_text(json.dumps(bad,indent=2)+'\n');rejects(bad);count+=1
replay.FIXTURE=original_fixture
original_root=replay.ROOT
with tempfile.TemporaryDirectory() as directory:
    temporary=Path(directory);target=temporary/'records/T44/original-r0002'
    shutil.copytree(ROOT/'records/T44/original-r0002',target)
    replay.ROOT=temporary
    result=target/'result.json';result.write_text(result.read_text()+'\n')
    rejects(fixture);count+=1
    manifest=replay.read(target/'manifest.json')
    manifest['files']['result.json']=hashlib.sha256(result.read_bytes()).hexdigest()
    (target/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    rejects(fixture);count+=1
replay.ROOT=original_root
print(f'prepared provenance:original valid;{count} tamper/outcome/rehash cases rejected')
