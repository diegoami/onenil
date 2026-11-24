#!/usr/bin/env python3
"""Authenticate actual native campaign UI/save/restart and separate-exhibition evidence."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
def projection(c):
    value=copy.deepcopy(c)
    for field in ['version','origin','revision']:value.pop(field)
    value['schema']='original-club-seed/1'
    for field in ['first_team','reserves']:value['club'][field]=[p or 0 for p in value['club'][field]]
    value['players'].sort(key=lambda p:p['id']);return value

def state_check(record):
    assert record['schema']=='native-campaign-window-check/1' and record['status']=='passed' and not record['error'] and not record['x11_errors']
    steps={r['name']:r['state'] for r in record['steps']};assert len(steps)==len(record['steps'])
    source=json.loads((ROOT/'campaign/fixtures/research-seed.json').read_text())
    assert projection(steps['main']['campaign'])==source['initial']
    assert steps['carried-source-destination-card']['carried']==2581 and steps['carried-source-destination-card']['card']==2582
    assert steps['cancel-without-swap']['campaign']==steps['main']['campaign']
    assert steps['confirmed-swap']['campaign']['club']['first_team'][0]==2598
    assert steps['confirmed-swap']['campaign']['club']['first_team'][17]==2581
    for kind in ['malformed','version','namespace','legacy-experiment']:assert steps[kind+'-load-preserves-campaign']['campaign']==steps['saved']['campaign']
    restored=steps['fresh-desktop-reload']['campaign'];assert projection(restored)==source['changed'] and restored['revision']==3
    assert steps['restored-keeper-instructions']['card']==2598 and steps['restored-current-club-position']['card']==2582
    # T56/T57 records have two launches; T114's startup scenarios and T115's
    # campaign-match route add two each.
    launches=2+2*int('startup' in steps)+2*int('campaign-match-start' in steps)
    assert len(set(record['desktop_process_ids']))==launches and len(record['native_edit_probe'])==2
    if 'startup' in steps:
        assert {'startup','new-game-not-implemented','startup-invalid-save','invalid-save-refused-at-startup','startup-valid-save','startup-loads-saved-campaign'}<=set(steps)
    blocked=steps['unsupported-campaign-match-is-not-exhibition'];assert not blocked['worker_pid'] and not blocked['exhibition'] and blocked['campaign']==restored
    assert steps['separate-exhibition']['exhibition']
    assert steps['full-time']['score']==[2,1] and steps['full-time']['frames']==4199 and steps['full-time']['events']==832
    start=next(i for i,s in enumerate(record['steps']) if s['name']=='fresh-desktop-reload')
    end=next((i for i,s in enumerate(record['steps']) if s['name']=='campaign-match-start'),len(record['steps']))
    assert all(s['state']['campaign']==restored for s in record['steps'][start:end])
    assert steps['completed-no-campaign-post']['campaign']['next_fixture']['opponent']==17
    assert len(record['worker_pids'])==2+2*int('campaign-match-start' in steps)
    if 'campaign-match-start' in steps:
        # [confirmed:T69/T80] The one supported edited XI, its computed result,
        # exactly one posting, the W030 tail and the fresh-process reload.
        assert steps['campaign-match-edited']['campaign']['club']['first_team'][1:3]==[2583,2582]
        assert steps['campaign-match-abandoned-preparation']['campaign']==steps['campaign-match-edited']['campaign']
        assert not steps['campaign-match-preparation']['exhibition']
        assert steps['campaign-match-full-time']['score']==[0,2]
        recorded=steps['campaign-match-recorded']['campaign']
        assert recorded['played_fixture']['score']==[0,2] and recorded['revision']==2
        assert steps['campaign-match-double-post-refused']['campaign']==recorded
        advanced=steps['campaign-match-week-one']['campaign']
        assert advanced['week_index']==1 and advanced['cash']==421491 and advanced['next_fixture']['opponent']==176
        assert steps['campaign-match-reloaded']['screen']=='Main' and steps['campaign-match-reloaded']['campaign']==advanced

def verify(path):
    r=json.loads(path.read_text());state_check(r)
    subprocess.check_output(['git','cat-file','-e',r['producer']+'^{commit}'],cwd=ROOT)
    assert r['font_sha256']=='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
    seen=set()
    for shot in r['screenshots']:
        assert shot['path'] not in seen and Path(shot['path']).name==shot['path'];seen.add(shot['path']);image=path.parent/shot['path']
        assert hashlib.sha256(image.read_bytes()).hexdigest()==shot['sha256'];assert list(Image.open(image).size)==shot['size']
    assert {tuple(s['size']) for s in r['screenshots']}=={(1440,900),(960,640),(720,450)}
    for kind in ['initial-cash','carried-id','restored-position','campaign-post']:
        bad=copy.deepcopy(r);steps={s['name']:s['state'] for s in bad['steps']}
        if kind=='initial-cash':steps['main']['campaign']['cash']+=1
        elif kind=='carried-id':steps['carried-source-destination-card']['carried']=0
        elif kind=='restored-position':steps['fresh-desktop-reload']['campaign']['players'][1]['current_position']=2
        else:steps['completed-no-campaign-post']['campaign']['week_index']=1
        try:state_check(bad)
        except AssertionError:pass
        else:raise AssertionError('corruption accepted: '+kind)
    print('PASS',len(r['steps']),'native steps,',len(seen),'images,4 negative controls')
p=argparse.ArgumentParser();p.add_argument('record',type=Path);args=p.parse_args();verify(args.record)
