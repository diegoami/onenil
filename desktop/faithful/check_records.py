#!/usr/bin/env python3
"""[designed] Verify retained native graphical evidence, images and lifecycle assertions."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def verify(path):
    r=json.loads(path.read_text());assert r['schema']=='native-window-check/1' and r['status']=='passed' and not r['error'] and not r.get('x11_errors')
    subprocess.check_output(['git','cat-file','-e',r['producer']+'^{commit}'],cwd=ROOT)
    assert len(r['binary_sha256'])==len(r['font_sha256'])==64
    assert r['font_sha256']=='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
    steps={s['name']:s['state'] for s in r['steps']};assert len(steps)==len(r['steps'])
    assert steps['carried-source-destination-card']['carried']==0 and steps['carried-source-destination-card']['card']==1
    assert steps['cancel-without-swap']['first'][:2]==[0,1] and steps['cancel-without-swap']['carried'] is None
    if r.get('scope')!='native focus/menu/move':
        assert steps['confirmed-swap']['first'][:2]==[1,0]
        assert steps['reloaded']['first'][:2]==steps['malformed-load-preserves-editor']['first'][:2]==[1,0]
        assert steps['full-time']['score']==[2,1] and steps['full-time']['frames']==4199 and steps['full-time']['events']==832
        assert steps['pitch']['players']==22 and steps['caller-return']['screen']=='Preparation'
        assert steps['cancel-preparation']['worker_pid'] is None and len(r['worker_pids'])==2
    if 'focus-return-keeps-pending-move' in steps:
        assert steps['focus-return-keeps-pending-move']['carried']==0
        assert steps['native-actions-progress']['frames']>=500 and steps['native-actions-progress']['events']>20
    seen=set()
    from PIL import Image
    for shot in r['screenshots']:
        image=path.parent/shot['path'];assert image.parent==path.parent and shot['path'] not in seen;seen.add(shot['path'])
        assert hashlib.sha256(image.read_bytes()).hexdigest()==shot['sha256']
        assert list(Image.open(image).size)==shot['size']
    assert {tuple(s['size']) for s in r['screenshots']}=={(1440,900),(960,640),(720,450)}
    return len(steps),len(seen)
p=argparse.ArgumentParser();p.add_argument('records',type=Path,nargs='+');args=p.parse_args()
for record in args.records:print(record,verify(record))
