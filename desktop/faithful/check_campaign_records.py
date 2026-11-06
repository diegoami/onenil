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
    assert len(set(record['desktop_process_ids']))==2 and len(record['native_edit_probe'])==2
    blocked=steps['unsupported-campaign-match-is-not-exhibition'];assert not blocked['worker_pid'] and not blocked['exhibition'] and blocked['campaign']==restored
    assert steps['separate-exhibition']['exhibition']
    assert steps['full-time']['score']==[2,1] and steps['full-time']['frames']==4199 and steps['full-time']['events']==832
    start=next(i for i,s in enumerate(record['steps']) if s['name']=='fresh-desktop-reload')
    assert all(s['state']['campaign']==restored for s in record['steps'][start:])
    assert steps['completed-no-campaign-post']['campaign']['next_fixture']['opponent']==17
    assert len(record['worker_pids'])==2

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
