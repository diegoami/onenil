#!/usr/bin/env python3
"""[designed:T69] Authenticate computed fixture ownership and fresh-process saved result."""
import argparse
import copy
import json
import struct
from functools import lru_cache
from pathlib import Path
from check_match_records import ROOT, digest
from check_refusal_records import native_rgb
RECORD_SHA = 'dc7238c5f11edba68be573327b887118d79cd9d77a8c7fafb6ceeacea07d730c'
PRODUCER = '969f499076fd97fdb735249c4b8accb8746077ce'
BINARY_SHA = '2103092e5925ee25fb51d95ef3d610baedef862e65396b61634050a0f2c468bf'
REFUSAL = 'This fixture already has a result. Week progression is not available yet.'



@lru_cache(maxsize=1)
def original_ui():
    raw = (ROOT/'records/T66/source-r0001/admission-r0002.json').read_bytes()
    assert digest(raw)=='45ca77eda64349d90700eb3983f6c725b5308e0dcf87c8a788bb081fde3ab627'
    return json.loads(raw)['payload']['ui']


def check(record, initial_raw, edited_raw, completed_raw):
    assert record['producer'] == PRODUCER
    assert record['binary_sha256'] == BINARY_SHA
    assert record['schema'] == 'native-edited-campaign-result-window/1'
    assert record['status'] == 'passed' and not record['error'] and not record['x11_errors']
    assert record['fresh_process_count'] == 3
    assert record['cancelled_and_closed_workers_reaped'] is True
    assert record['font_sha256'] == 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
    assert record['names_sha256'] == digest((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes())
    assert record['runtime']['window_backend'] == 'software/minifb'
    assert record['runtime']['cargo_lock_sha256'] == digest((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
    initial, edited, completed = map(json.loads, (initial_raw, edited_raw, completed_raw))
    source = json.loads((ROOT/'campaign/fixtures/research-seed.json').read_text())['initial']
    projected = copy.deepcopy(initial)
    for name in ('origin', 'version', 'revision'): projected.pop(name)
    projected['schema'] = 'original-club-seed/1'
    for name in ('first_team', 'reserves'):
        projected['club'][name] = [p or 0 for p in projected['club'][name]]
    assert projected == source and initial['revision'] == 0 and 'played_fixture' not in initial
    expected_edit = copy.deepcopy(initial)
    expected_edit['club']['first_team'][1:3] = [2583,2582]
    expected_edit['revision'] = 1
    assert edited == expected_edit
    assert record['source_verification_sha256'] == '8c6cea86100e1062c0478a3f437cf4185e25209a900ef55755e4c16788caaea4'
    assert record['source_verification_sha256'] == digest((ROOT/'records/T68/verification-r0001/manifest.json').read_bytes())
    steps = {s['name']:s['state'] for s in record['steps']}
    before = ['initial-campaign','initial-save','campaign-preparation','campaign-worker','cancel-preserves-campaign','fresh-match-pitch','native-player-actions','detailed-commentary','bench','half-time','half-time-bench','full-time']
    after = ['result-recorded','completed-save','completed-fixture-refuses','fresh-process-reload','restored-fixture-refuses']
    original_steps = ['initial-campaign','initial-save','original-players','select-pearce','carry-pearce','select-hendry']
    edit_steps = ['exchange-players','players-return','play-next-selected','edited-save','edited-process-reload']
    before = before[2:]
    assert list(steps) == original_steps+edit_steps+before+after and len(record['steps']) == 26
    for name in original_steps: assert steps[name]['campaign'] == initial, name
    for name in edit_steps: assert steps[name]['campaign'] == edited, name
    assert steps['original-players']['screen']=='Players' and steps['original-players']['selected']=='First(0)'
    for name,selected,carried in [('select-pearce','First(1)',None),('carry-pearce','First(1)',2582),('select-hendry','First(2)',2582),('exchange-players','First(2)',None)]:
        assert steps[name]['selected']==selected and steps[name]['carried']==carried
    assert steps['players-return']['menus'][0]==1 and steps['play-next-selected']['menus'][0]==0
    assert steps['edited-process-reload']['worker_absent'] is True and steps['edited-process-reload']['frames'] is None
    ui_steps = ['initial-campaign','original-players','select-pearce','carry-pearce','select-hendry','exchange-players','players-return','play-next-selected']
    for source, name in zip(original_ui(), ui_steps, strict=True):
        state = steps[name]
        assert [p or 0 for p in state['first']] == source['roster']
        assert (state['carried'] is not None) == source['put_here']
        if source['menu'] is not None:
            assert state['screen']=='Main' and state['menus'][0]=={'PLAY NEXT GAME':0,'Players':1}[source['menu']]
        else:
            assert state['screen']=='Players' and state['card']==source['selected_player']
    final = steps['full-time']
    assert (final['screen'],final['phase'],final['frames'],final['events'],final['score']) == ('FullTime','FullTime',3970,870,[0,2])
    expected = copy.deepcopy(edited)
    expected['revision'] = 2
    expected['played_fixture'] = dict(fixture=initial['next_fixture'],season=initial['season'],week_index=initial['week_index'],request_revision=1,request_sha256=digest(edited_raw),score=final['score'],native_return='0AB9:EBE2')
    assert completed == expected
    for name in before: assert steps[name]['campaign'] == edited, name
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
    raw=path.read_bytes();assert digest(raw)==RECORD_SHA,'immutable T69 capture differs'
    record=json.loads(raw)
    saves={s['name']:s for s in record['save_proofs']};assert set(saves)=={'initial-save','edited-save','completed-save'}
    contents={}
    for name,proof in saves.items():
        assert Path(proof['path']).name==proof['path']
        data=(path.parent/proof['path']).read_bytes();assert digest(data)==proof['sha256'];contents[name]=data
    steps=check(record,contents['initial-save'],contents['edited-save'],contents['completed-save'])
    seen=set()
    for shot in record['screenshots']:
        assert Path(shot['path']).name==shot['path'] and shot['name'] not in seen
        seen.add(shot['name']);data=(path.parent/shot['path']).read_bytes()
        assert digest(data)==shot['sha256'] and data[:8]==b'\x89PNG\r\n\x1a\n'
        assert list(struct.unpack('>II',data[16:24]))==shot['size']==[1440,900]
        stable={'recorded-result':'result-recorded','completed-refusal':'completed-fixture-refuses','reloaded-result':'fresh-process-reload','edited-players':'exchange-players','reloaded-edit':'edited-process-reload'}
        if shot['name'] in stable: assert digest(native_rgb(data))==steps[stable[shot['name']]]['raster_sha256']
    assert seen=={'main','edited-players','reloaded-edit','preparation','pitch-actions','commentary','bench','half-time','full-time','recorded-result','completed-refusal','reloaded-result'}
    for damage in ('score','request','fixture','revision','worker','week','roster','source','producer'):
        bad=copy.deepcopy(record);saved=json.loads(contents['completed-save'])
        if damage=='worker':
            next(s['state'] for s in bad['steps'] if s['name']=='restored-fixture-refuses')['worker_absent']=False
        elif damage=='score':saved['played_fixture']['score']=[2,0]
        elif damage=='request':saved['played_fixture']['request_sha256']='0'*64
        elif damage=='fixture':saved['played_fixture']['fixture']['opponent']=134
        elif damage=='revision':saved['revision']=1
        elif damage=='roster':saved['club']['first_team'][0:2]=[2583,2581]
        elif damage=='source':bad['source_verification_sha256']='0'*64
        elif damage=='producer':bad['producer']='foreign'
        else:saved['week_index']=1
        try:check(bad,contents['initial-save'],contents['edited-save'],json.dumps(saved).encode())
        except AssertionError:pass
        else:raise AssertionError('corrupt saved result accepted: '+damage)
    print('PASS edited original keys,26 steps,12 screenshots,3 saves,3 processes,5 raster bindings,9 corruptions')


if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('record',type=Path);verify(p.parse_args().record)
