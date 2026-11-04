#!/usr/bin/env python3
"""Verify initial support provenance and replay one owned prepared watched image."""
import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path
import replay_watched_setup as setup
import replay_watched_caller as caller
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/'engine/tests/fixtures/prepared_watched/admission.json'
FIXTURE_SHA='0d7f13e5824ac2907c114cd453c34085ae5ea0b20a894862880727aa4c1dccc6'
MANIFESTS={'original-r0002':'fe7abcc3c374071703816ee38df14806665ba73742581e2a7d0fcad55ad3fb92',
           'headless-original-r0001':'9520e0b788490d490500d005a5811de4f9d1667fd0f05155ed6bd6542d0c0bcc'}
def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def sources():
    result={}
    for name,digest in MANIFESTS.items():
        directory=ROOT/'records/T44'/name
        if sha(directory/'manifest.json')!=digest:raise ValueError('prepared source manifest differs')
        manifest=read(directory/'manifest.json')
        if set(manifest['files'])!={'start.json','result.json'}:raise ValueError('prepared source census differs')
        for leaf,expected in manifest['files'].items():
            if sha(directory/leaf)!=expected:raise ValueError('prepared source leaf differs')
        start=read(directory/'start.json')
        if start['observer_sha256']!=sha(ROOT/'tools/prepared_watched_oracle.py'):raise ValueError('prepared observer differs')
        if start['preparation_admission_sha256']!=sha(setup.FIXTURE):raise ValueError('prepared setup source differs')
        result[name]=read(directory/'result.json')
        if result[name]['status']!='completed':raise ValueError('incomplete original observation')
    setup.admission()
    return result['original-r0002'],result['headless-original-r0001']

def support_location(address):
    for area,base in [('fixtures',0x80000),('results',0x60000),('counts',0x50000),('config',0x49000),('ds',0)]:
        if 0<=address-base<65536:return area,address-base
    raise ValueError('missing input is outside initial support domains')

def select(probe):
    if FIXTURE.exists():raise ValueError('refusing to overwrite a frozen admission')
    original,_=sources();lookup={(f['area'],f['offset']):f for f in original['fields']}
    request=dict(input=dict(preparation=read(setup.FIXTURE)['input'],support=[],no_pending_key=True),steps=171)
    with subprocess.Popen([str(probe)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True) as child:
        for _ in range(1000):
            child.stdin.write(json.dumps(request)+'\n');child.stdin.flush()
            out=json.loads(child.stdout.readline())
            if 'ok' in out:break
            error=out['error'];print(error,flush=True)
            missing=re.search(r'Missing \{ scratch: false, address: (\d+), writing: (?:false|true)',error)
            if not missing:raise ValueError('native bridge failed outside an initial-support miss')
            key=support_location(int(missing[1]));fact=lookup.get(key)
            if fact is None or fact['before']!=fact['entry']:raise ValueError(f'missing initial ownership/computation: {key}')
            value=dict(area=key[0],offset=key[1],value=fact['before'])
            if value in request['input']['support']:raise ValueError('repeated initial-support miss')
            # The native constructor also independently rejects owned/output areas.
            request['input']['support'].append(value)
        else:raise ValueError('initial support selection bound exceeded')
        child.stdin.close()
    fixture=dict(schema='native-prepared-watched/1',sources=MANIFESTS,input=request['input'])
    if FIXTURE.exists():raise ValueError('refusing to overwrite a frozen admission')
    verify(fixture)
    compare(out['ok'])
    FIXTURE.parent.mkdir(parents=True,exist_ok=True);FIXTURE.write_text(json.dumps(fixture,indent=2)+'\n')
    print('prepared initial support selected:',len(request['input']['support']))

def allowed(area, offset):
    # Numeric read-only ranges from the reached original helper graph. Mutable
    # queue/scene/ball/seed/ABI/page state is excluded independently of its values.
    ranges=[(0x0D5C,0x0D5E),(0xD0A6,0xD0AC),(0x09FB,0x0B5A),(0x097A,0x09BE),(0x0D36,0x0D56),(0x0BF6,0x0C68),
            (0x0CD7,0x0CE0),(0x0D60,0x0D91),(0x0D92,0x0E6A),
            (0x0F20,0x0F30),(0x1274,0x1288),(0x129E,0x12CE),
            (0x8708,0x8716),(0x935B,0xA51C),(0xD886,0xD936)]
    if area=='ds':return any(lo<=offset<hi for lo,hi in ranges) or offset in (0xD084,0xD09E,0xD0B6,0xD0BD,0xE5DD,0xCF5A,0xCF5B)
    if area=='config':return 0<=offset<256 and offset not in (0,1,2,0xF2,0xF6)
    return 0<=offset<dict(counts=4004,results=58*176,fixtures=174*176).get(area,0)

def verify(fixture):
    if FIXTURE.exists() and (sha(FIXTURE)!=FIXTURE_SHA or fixture!=read(FIXTURE)):
        raise ValueError('frozen prepared admission differs')
    original,_=sources()
    if set(fixture)!={'schema','sources','input'} or fixture['schema']!='native-prepared-watched/1' or fixture['sources']!=MANIFESTS:raise ValueError('prepared fixture provenance differs')
    value=fixture['input']
    if set(value)!={'preparation','support','no_pending_key'} or value['no_pending_key'] is not True:raise ValueError('prepared input shape differs')
    if value['preparation']!=read(setup.FIXTURE)['input']:raise ValueError('selected setup input differs')
    lookup={(f['area'],f['offset']):f for f in original['fields']};seen=set()
    for fact in value['support']:
        if set(fact)!={'area','offset','value'} or type(fact['offset']) is not int or type(fact['value']) is not int or not 0<=fact['value']<=255:raise ValueError('initial support shape differs')
        key=fact['area'],fact['offset'];source=lookup.get(key)
        if not allowed(*key) or key in seen or source is None or source['before']!=source['entry'] or fact['value']!=source['before']:raise ValueError('initial support source differs')
        seen.add(key)
    return value

def compare(out):
    original,prefix=sources()
    if out['first_entry']!=original['first_entry']:raise ValueError('computed first entry differs')
    observation=out['observation']
    if observation['ticks']!=prefix['ticks']:raise ValueError('computed original prefix ticks differ')
    if observation['draws']!=prefix['draws']:raise ValueError('computed setup/caller draws differ')
    events=[e for e in observation['events'] if e['tick']>0]
    expected_events=[{key:value for key,value in event.items() if key!='event'} for event in prefix['events']]
    if events!=expected_events:raise ValueError('computed watched actions differ')
    (actual,_),_=caller.sources()
    if [c['clock'] for c in out['clocks']] != [c['output']['clock'] for c in actual]:raise ValueError('computed per-iteration caller clocks differ')
    if out['clocks'][-1]['state']!=prefix['clock_end']:raise ValueError('computed caller clock end differs')

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--probe',type=Path,default=ROOT/'engine/target/debug/examples/prepared_watched_probe');p.add_argument('--select',action='store_true');p.add_argument('--check',action='store_true');p.add_argument('--replay',action='store_true');p.add_argument('--python',action='store_true',help='Replay through the installed PyO3 API');a=p.parse_args()
    if a.select:select(a.probe)
    if a.check or a.replay:
        value=verify(read(FIXTURE))
        if a.replay:
            if a.python:
                import onenil_engine as native
                session=native.NativePreparedWatchedSession(json.dumps(value))
                first=json.loads(session.snapshot())
                clocks=[json.loads(session.step()) for _ in range(171)]
                out=dict(first_entry=first,clocks=clocks,observation=json.loads(session.observation()))
                compare(out)
            else:
                result=subprocess.run([str(a.probe)],input=json.dumps(dict(input=value,steps=171))+'\n',capture_output=True,text=True,check=True)
                out=json.loads(result.stdout)
                if 'ok' not in out:raise ValueError(out)
                compare(out['ok'])
        print('prepared watched provenance/replay: ok')
if __name__=='__main__':main()
