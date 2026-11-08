#!/usr/bin/env python3
"""[designed:T70] Authenticate the actual readable desktop and resize evidence."""
import argparse
import hashlib
import json
import struct
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
RECORD_SHA = 'f4cc1385c57f7529c12f56fddb0021374734d69614089b045b9e2c3a02d60182'
PRODUCER = '5a7091c97a550d49167143b0a5a42a9cc75c0596'
BINARY_SHA = 'c7d95eee06f939dc4bb294fd26a2a472946fe7f799c283d9a8c3658060c6e431'
FONT_SHA = 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
def digest(raw):
    return hashlib.sha256(raw).hexdigest()
def verify(path):
    raw=path.read_bytes();assert digest(raw)==RECORD_SHA
    r=json.loads(raw)
    assert r['schema']=='native-desktop-readability-window/1'
    assert r['status']=='passed' and r['error'] is None and not r['x11_errors']
    assert r['producer']==PRODUCER and r['binary_sha256']==BINARY_SHA
    assert r['font_sha256']==FONT_SHA
    assert r['runtime']['window_backend']=='software/minifb'
    assert r['runtime']['cargo_lock_sha256']==digest((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
    assert r['fresh_process_count']==1 and r['cancelled_and_closed_workers_reaped'] is True
    steps={s['name']:s['state'] for s in r['steps']}
    assert list(steps)==['main','players','preparation','worker','pitch','commentary']
    assert [s['screen'] for s in steps.values()]==['Main','Players','Preparation','Preparing','Pitch','History']
    initial=steps['main']['campaign']
    assert all(s['campaign']==initial and s['exhibition'] is False for s in steps.values())
    assert steps['main']['menus'][0]==0 and steps['preparation']['menus'][1]==2
    assert steps['pitch']['players']==22 and steps['commentary']['recent_actions']
    assert all('Play continues' not in a['text'] for a in steps['commentary']['recent_actions'])
    seen=set()
    for shot in r['screenshots']:
        name=shot['name'];assert name not in seen;seen.add(name)
        assert Path(shot['path']).name==shot['path']
        data=(path.parent/shot['path']).read_bytes()
        assert digest(data)==shot['sha256'] and data[:8]==b'\x89PNG\r\n\x1a\n'
        size=list(struct.unpack('>II',data[16:24]))
        assert size==shot['size']==([1440,900] if name.endswith('-default') else [1280,800])
    assert seen=={screen+'-'+size for screen in ('main','players','preparation','commentary') for size in ('default','resized')}
    print('PASS readable desktop: 6 stages,8 actual screenshots,2 sizes,owned native worker,no campaign mutation')
if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('record',type=Path);verify(parser.parse_args().record)
