#!/usr/bin/env python3
"""Original-reference checks for one owned selected-squad watched lifecycle."""
import argparse
import hashlib
import json
from pathlib import Path
import replay_watched_match as reference
import replay_prepared_watched as prefix
import watched_oracle as oracle
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/'engine/tests/fixtures/prepared_match/admission.json'
# Filled only after the immutable original full-lifecycle observation completes.
FIELD_MANIFEST_SHA=None
FIELD_DIRECTORY=ROOT/'records/T45/original-r0001'
def read(path):return json.loads(path.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()
def field_preservation():
    if FIELD_MANIFEST_SHA is None:raise ValueError('full original field-preservation observation is not yet pinned')
    manifest=read(FIELD_DIRECTORY/'manifest.json')
    if sha(FIELD_DIRECTORY/'manifest.json')!=FIELD_MANIFEST_SHA:raise ValueError('full original field-preservation root differs')
    for name,digest in manifest['files'].items():
        if sha(FIELD_DIRECTORY/name)!=digest:raise ValueError('full original preservation leaf differs')
    result=read(FIELD_DIRECTORY/'result.json')
    if result['status']!='completed'or(result['ticks'],result['draws'],result['actions'])!=(4199,182811,831)or result['boundary']!='0AB9:EBE2':
        raise ValueError('incomplete original full-lifecycle preservation')
    ticks,draws,events,finish=reference.sources()
    digest=lambda values:hashlib.sha256(b''.join(canonical(v)+b'\n'for v in values)).hexdigest()
    if result['ticks_sha256']!=digest(ticks)or result['actions_sha256']!=digest(events):raise ValueError('original preservation trace digest differs')
    if result['draws_sha256']!=hashlib.sha256(canonical([oracle.normalized(d)for d in draws])).hexdigest()or result['final']!=finish['final']:
        raise ValueError('original preservation draws/final differ')
    return result

def compare(out,require_field_proof=True):
    ticks,draws,events,finish=reference.sources()
    if out['return']!='0AB9:EBE2'or out['ticks']!=ticks:raise ValueError('owned prepared full tick lifecycle/return differs')
    if out['draws']!=[oracle.normalized(d)for d in draws]:raise ValueError('owned computed setup-and-match draw stream differs')
    expected=[{k:v for k,v in event.items()if k!='event'}for event in events]
    if out['events']!=expected:raise ValueError('owned full watched actions differ')
    if out['final']!=finish['final']:raise ValueError('owned prepared final state differs')
    # Source E069/E06E stores C5/C4 into final D086/D087.
    if out['live_goals']!=list(reversed(finish['final']['scores'])):raise ValueError('owned live/final score mapping differs')
    if len(out['tick_live_goals'])!=len(ticks):raise ValueError('owned live goal counter trace is incomplete')
    if len(out['setup_events'])!=1 or out['setup_events'][0]['code']!=59 or out['setup_events'][0]['tick']!=0:
        raise ValueError('owned setup coin commentary differs')
    if require_field_proof:
        original=field_preservation()
        if out['caller_clocks']!=original['caller_clocks']:raise ValueError('owned complete caller clocks differ')
    else:
        import replay_watched_caller as caller
        (actual,_),_=caller.sources()
        expected_clocks=[dict(tick=i+1,clock=row['output']['clock'])for i,row in enumerate(actual)]
        if out['caller_clocks'][:171]!=expected_clocks:raise ValueError('owned known caller clock prefix differs')
    return out

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--compare-output',type=Path);p.add_argument('--development-reference-only',action='store_true');a=p.parse_args()
    if a.compare_output:
        packet=read(a.compare_output)
        if set(packet)!={'ok'}:raise ValueError('prepared match did not complete successfully')
        compare(packet['ok'],not a.development_reference_only)
        print('owned prepared full original reference parity:ok'+(';full original field/clock proof remains pending'if a.development_reference_only else''))
    else:p.error('choose --compare-output')
if __name__=='__main__':main()
