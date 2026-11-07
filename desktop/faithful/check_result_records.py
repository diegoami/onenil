#!/usr/bin/env python3
"""[designed:T65] Authenticate computed fixture ownership and fresh-process saved result."""
import argparse
import copy
import json
import struct
from pathlib import Path
from check_match_records import ROOT, digest
from check_refusal_records import native_rgb
RECORD_SHA = '25c5ecdc7e116e84d888cb985045ffddbc2ab37304b45598dfa24f6fb7f886ab'
REFUSAL = 'This fixture already has a result. Week progression is not available yet.'


def check(record, initial_raw, completed_raw):
    assert record['schema'] == 'native-campaign-result-window/1'
    assert record['status'] == 'passed' and not record['error'] and not record['x11_errors']
    assert record['fresh_process_count'] == 2
    assert record['cancelled_and_closed_workers_reaped'] is True
    assert record['font_sha256'] == 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
    assert record['names_sha256'] == digest((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes())
    assert record['runtime']['window_backend'] == 'software/minifb'
    assert record['runtime']['cargo_lock_sha256'] == digest((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
    initial, completed = json.loads(initial_raw), json.loads(completed_raw)
    source = json.loads((ROOT/'campaign/fixtures/research-seed.json').read_text())['initial']
    projected = copy.deepcopy(initial)
    for name in ('origin', 'version', 'revision'): projected.pop(name)
    projected['schema'] = 'original-club-seed/1'
    for name in ('first_team', 'reserves'):
        projected['club'][name] = [p or 0 for p in projected['club'][name]]
    assert projected == source and initial['revision'] == 0 and 'played_fixture' not in initial
    steps = {s['name']:s['state'] for s in record['steps']}
    before = ['initial-campaign','initial-save','campaign-preparation','campaign-worker','cancel-preserves-campaign','fresh-match-pitch','native-player-actions','detailed-commentary','bench','half-time','half-time-bench','full-time']
    after = ['result-recorded','completed-save','completed-fixture-refuses','fresh-process-reload','restored-fixture-refuses']
    assert list(steps) == before+after and len(record['steps']) == 17
    final = steps['full-time']
    assert (final['screen'],final['phase'],final['frames'],final['events'],final['score']) == ('FullTime','FullTime',3825,749,[1,4])
    expected = copy.deepcopy(initial)
    expected['revision'] = 1
    expected['played_fixture'] = dict(fixture=initial['next_fixture'],season=initial['season'],week_index=initial['week_index'],request_revision=0,request_sha256=digest(initial_raw),score=final['score'],native_return='0AB9:EBE2')
    assert completed == expected
    for name in before: assert steps[name]['campaign'] == initial, name
    for name in after: assert steps[name]['campaign'] == completed, name
    for name in ('completed-fixture-refuses','restored-fixture-refuses'):
        s = steps[name]
        assert s['screen'] == 'Preparation' and s['worker_absent'] is True
        assert s['message'] == REFUSAL and len(s['message']) <= 80
    restored = steps['fresh-process-reload']
    assert restored['screen'] == 'Main' and restored['frames'] is None and restored['worker_absent'] is True
    assert steps['fresh-match-pitch']['players'] == 22
    assert steps['fresh-match-pitch']['match_clubs'] == ['Blackburn R','P St Germain']
    assert 'recorded' in steps['result-recorded']['message'] and len(steps['result-recorded']['message']) <= 80
    names=json.loads((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_text())['players']
    messages=json.loads((ROOT/'desktop/faithful/messages.json').read_text())
    actions=steps['detailed-commentary']['recent_actions'];assert len(actions)==8
    for action in actions:
        text=messages[action['native']['code']][1]
        for marker,field in (('{p}','player'),('{s}','secondary')):
            if marker in text:text=text.replace(marker,names[str(action['native'][field])])
        assert action['text']==text and 'Play continues' not in text
    return steps


def verify(path):
    raw=path.read_bytes();assert digest(raw)==RECORD_SHA,'immutable T65 capture differs'
    record=json.loads(raw)
    saves={s['name']:s for s in record['save_proofs']};assert set(saves)=={'initial-save','completed-save'}
    contents={}
    for name,proof in saves.items():
        assert Path(proof['path']).name==proof['path']
        data=(path.parent/proof['path']).read_bytes();assert digest(data)==proof['sha256'];contents[name]=data
    steps=check(record,contents['initial-save'],contents['completed-save'])
    seen=set()
    for shot in record['screenshots']:
        assert Path(shot['path']).name==shot['path'] and shot['name'] not in seen
        seen.add(shot['name']);data=(path.parent/shot['path']).read_bytes()
        assert digest(data)==shot['sha256'] and data[:8]==b'\x89PNG\r\n\x1a\n'
        assert list(struct.unpack('>II',data[16:24]))==shot['size']==[1440,900]
        stable={'recorded-result':'result-recorded','completed-refusal':'completed-fixture-refuses','reloaded-result':'fresh-process-reload'}
        if shot['name'] in stable: assert digest(native_rgb(data))==steps[stable[shot['name']]]['raster_sha256']
    assert seen=={'main','preparation','pitch-actions','commentary','bench','half-time','full-time','recorded-result','completed-refusal','reloaded-result'}
    for damage in ('score','request','fixture','revision','worker','week'):
        bad=copy.deepcopy(record);saved=json.loads(contents['completed-save'])
        if damage=='worker':
            next(s['state'] for s in bad['steps'] if s['name']=='restored-fixture-refuses')['worker_absent']=False
        elif damage=='score':saved['played_fixture']['score']=[4,1]
        elif damage=='request':saved['played_fixture']['request_sha256']='0'*64
        elif damage=='fixture':saved['played_fixture']['fixture']['opponent']=134
        elif damage=='revision':saved['revision']=2
        else:saved['week_index']=1
        try:check(bad,contents['initial-save'],json.dumps(saved).encode())
        except AssertionError:pass
        else:raise AssertionError('corrupt saved result accepted: '+damage)
    print('PASS computed result once,17 keys,10 screenshots,2 saves,fresh process,3 raster bindings,6 corruptions')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('record',type=Path);verify(p.parse_args().record)
