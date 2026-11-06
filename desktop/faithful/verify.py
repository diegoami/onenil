#!/usr/bin/env python3
"""[designed] Retain actual Rust core checks and compare every frame/event to accepted adapter."""
import argparse, hashlib, importlib.util, json, os, subprocess, time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--record',type=Path,required=True);args=p.parse_args()
assert not args.record.exists()
producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert not subprocess.check_output(['git','status','--porcelain','--','desktop/faithful'],cwd=ROOT,text=True)
started=time.monotonic()
checks=subprocess.run(['cargo','test','--manifest-path','desktop/faithful/Cargo.toml'],cwd=ROOT,text=True,capture_output=True)
assert checks.returncode == 0, checks.stdout+checks.stderr
binary=ROOT/'desktop/faithful/target/debug/onenil-desktop'
raw=subprocess.check_output([str(binary),'--worker',str(ROOT),'52'],cwd=ROOT)
packet=json.loads(raw);assert packet['id']==52 and set(packet['result'])=={'Ok'}
timeline=packet['result']['Ok']
spec=importlib.util.spec_from_file_location('accepted',ROOT/'tools/play_watched.py');mod=importlib.util.module_from_spec(spec);spec.loader.exec_module(mod)
accepted=mod.Match()._data

def normalize(view):
    return dict(clock=view['clock'],score=view['score'],ball=view['ball'],players=[dict(scene=p['scene'],player=p['player'],club=p['club'],focus=p['focus'],position=dict(x=p['x'],y=p['y'])) for p in view['players']])
def event(e):
    return dict(id=e['id'],native={k:v for k,v in e.items() if k not in ('id','kind','text')},kind=e['kind'],text=e['text'])
assert timeline['initial']==normalize(accepted['initial'])
assert timeline['setup_events']==[event(e) for e in accepted['setup_events']]
assert len(timeline['frames'])==len(accepted['frames'])==4199
for i,(actual,expected) in enumerate(zip(timeline['frames'],accepted['frames'])):
    assert actual == dict(view=normalize(expected['view']),events=[event(e) for e in expected['events']]),i
assert timeline['native_return']==accepted['native_return']=='0AB9:EBE2'
record=dict(schema='desktop-core-check/1',producer=producer,elapsed_seconds=time.monotonic()-started,tests=dict(exit=checks.returncode,stdout=checks.stdout,stderr=checks.stderr),native_worker=dict(packet_sha256=hashlib.sha256(raw).hexdigest(),frames=len(timeline['frames']),draws=timeline['draws'],clock_rows=timeline['clock_rows'],score=timeline['frames'][-1]['view']['score']),comparison='Every initial/frame player, coordinate, focus, ball, clock, score and setup/action event equals accepted tools/play_watched.py presentation of independently computed unchanged Rust engine output.',boundaries='Fixed reference64 exhibition. Captured squad editor is separate. No campaign posting or live managed substitutions.')
with args.record.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
print('PASS',record['native_worker'])
