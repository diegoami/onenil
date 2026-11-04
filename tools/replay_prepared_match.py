#!/usr/bin/env python3
"""Original-reference checks for one owned selected-squad watched lifecycle."""
import argparse
import hashlib
import copy
from functools import cache
import json
import subprocess
from pathlib import Path
import replay_watched_match as reference
import replay_prepared_watched as prefix
import watched_oracle as oracle
ROOT=Path(__file__).resolve().parents[1]
FIXTURE=ROOT/'engine/tests/fixtures/prepared_match/admission.json'
FIXTURE_SHA='842f1453cae4ba9ab626d7e57316494cfe7e70f64ea80e746c3834de0e12989f'
# Filled only after the immutable original full-lifecycle observation completes.
FIELD_MANIFEST_SHA=None
FIELD_DIRECTORY=ROOT/'records/T45/original-r0001'
def read(path):return json.loads(path.read_text())
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def canonical(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()
# Source-indexed immutable numeric tables,plus named incoming state whose first
# current-match stores are documented in the native prepared constructor.
STATIC_TABLES=[(0x097A,0x09BE),(0x09FB,0x0B5A),(0x0BF6,0x0C68),
    (0x0CD7,0x0CE0),(0x0D34,0x0D56),(0x0D5C,0x0D5E),(0x0D60,0x0D91),
    (0x0D92,0x0E6A),(0x0F20,0x0F30),(0x1274,0x1288),(0x129E,0x12CE),
    (0x8708,0x8716),(0x935B,0x9A2A),(0xA23C,0xA244)]
def owns_incoming(area,offset):
    if type(offset)is not int or offset<0:return False
    if area=='ds':
        return any(lo<=offset<hi for lo,hi in STATIC_TABLES) or 0xD0A6<=offset<0xD0AC or 0xD144<=offset<0xD148 or 0xD1D4<=offset<0xD1DC or offset in(0xD084,0xD09E,0xD0B6,0xD0BD,0xD1C9,0xCF5A,0xCF5B,0xE5DD)
    if area=='config':return offset<256 and offset not in(0,1,2,0xF2,0xF6)
    return offset<dict(counts=4004,results=58*176,fixtures=174*176).get(area,0)
@cache
def incoming_source():
    observation,_=prefix.sources()
    return {(row['area'],row['offset']):row for row in observation['fields']}
def validate_input(value):
    source=incoming_source()
    if set(value)!={'preparation','support','no_pending_key'}or value['no_pending_key']is not True:raise ValueError('owned prepared input shape differs')
    original=read(prefix.FIXTURE)['input']
    if canonical(value['preparation'])!=canonical(original['preparation']):raise ValueError('owned selected initial preparation differs')
    seen=set()
    if type(value['support'])is not list:raise ValueError('incoming support must be a list')
    for fact in value['support']:
        if type(fact)is not dict or type(fact.get('area'))is not str or type(fact.get('offset'))is not int:raise ValueError('incoming fact location differs')
        if set(fact)!={'area','offset','value'}or type(fact['value'])is not int or not 0<=fact['value']<=255:raise ValueError('incoming fact shape differs')
        key=fact['area'],fact['offset'];row=source.get(key)
        if key in seen or not owns_incoming(*key)or row is None or row['before']!=row['entry']or fact['value']!=row['before']:
            raise ValueError('incoming fact ownership/provenance differs:'+str(key))
        seen.add(key)
    return value

def admission(fixture=None):
    frozen=read(FIXTURE)
    if sha(FIXTURE)!=FIXTURE_SHA or (fixture is not None and canonical(fixture)!=canonical(frozen)):
        raise ValueError('frozen prepared match admission differs')
    if set(frozen)!={'schema','initial_source_manifest_sha256','reference_index_sha256','input'} or frozen['schema']!='native-prepared-match/1':
        raise ValueError('prepared match admission shape differs')
    if frozen['initial_source_manifest_sha256']!=prefix.MANIFESTS['original-r0002'] or frozen['reference_index_sha256']!=reference.INDEX_SHA:
        raise ValueError('prepared match provenance roots differ')
    return validate_input(frozen['input'])

def selftest():
    value=admission();rejected=0
    def reject(candidate,check=validate_input):
        nonlocal rejected
        try:check(candidate)
        except ValueError:rejected+=1
        else:raise AssertionError('invalid prepared match admission accepted')
    for area,offset in [('ds',0xD142),('ds',0xD148),('ds',0xD2A),('ds',0xA51C),('ds',0xDE89),('ds',0x380),('p8',0),('frame',0),('main',0)]:
        changed=copy.deepcopy(value);changed['support'].append(dict(area=area,offset=offset,value=0));reject(changed)
    for mutation in ('duplicate','value','boolean','pending','setup'):
        changed=copy.deepcopy(value)
        if mutation=='duplicate':changed['support'].append(changed['support'][0].copy())
        elif mutation=='value':changed['support'][0]['value']^=1
        elif mutation=='boolean':changed['support'][0]['offset']=True
        elif mutation=='pending':changed['no_pending_key']=False
        else:changed['preparation']['setup']['managed_selection_mode']^=1
        reject(changed)
    changed=read(FIXTURE);changed['input']['support'][0]['value']^=1
    # Updating an untrusted advertised digest cannot change the pinned root.
    changed['reference_index_sha256']=hashlib.sha256(canonical(changed)).hexdigest();reject(changed,admission)
    _,_,events,_=reference.sources()
    expected=reference_goal_trace(events,4199)
    for index,goals in [(1211,[1,0]),(1212,[0,0]),(1537,[1,1])]:
        changed=copy.deepcopy(expected);changed[index]=goals
        try:validate_goal_trace(changed,events,4199)
        except ValueError:rejected+=1
        else:raise AssertionError('changed live score boundary accepted')
    from unittest.mock import patch
    with patch.object(subprocess,'check_output',side_effect=AssertionError('producer commit must not be needed')):
        for number in (1,2):
            start=read(ROOT/f'records/T45/original-r{number:04d}/start.json')
            validate_observer_identity(start)
            changed=copy.deepcopy(start);changed['observer_sha256']='0'*64;reject(changed,validate_observer_identity)
            changed=copy.deepcopy(start);changed['schema']='unknown';reject(changed,validate_observer_identity)
    print('prepared match input/provenance/outcome/live-score/observer guards:',rejected,'rejected')

def replay_probe(probe,value,prefix_steps=0):
    result=subprocess.run([str(probe)],input=json.dumps(dict(input=value,prefix_steps=prefix_steps))+'\n',capture_output=True,text=True,check=True)
    packet=json.loads(result.stdout)
    if set(packet)!={'ok'}:raise ValueError(packet.get('error',{}).get('message','owned native completion failed'))
    return packet['ok']

def replay_python(value,require_field_proof=True):
    import onenil_engine as api
    request=json.dumps(value,sort_keys=True,separators=(',',':'))
    for prefix_steps in (0,171):
        session=api.NativePreparedWatchedSession(request)
        for _ in range(prefix_steps):session.step()
        packet=json.loads(session.finish())
        if set(packet)!={'ok'}:raise ValueError('Python prepared completion failed:'+str(packet.get('error',{}).get('message')))
        compare(packet['ok'],require_field_proof)
        if set(json.loads(session.finish()))!={'error'}:raise ValueError('Python completed session accepted a second finish')
        try:session.step()
        except ValueError:pass
        else:raise ValueError('Python completed session accepted another step')
        del packet,session
    print('Python fresh and171-retained prepared full lifecycle parity:ok')

def validate_observer_identity(start):
    paths={'original-prepared-full-lifecycle/1':'preservation-observer-r0001.py',
           'original-prepared-full-lifecycle/2':'preservation-observer-r0002.py'}
    if start['schema']not in paths:raise ValueError('unknown original observer protocol')
    observer=(ROOT/'records/T45'/paths[start['schema']]).read_bytes()
    if start['observer_sha256']!=hashlib.sha256(observer).hexdigest():
        raise ValueError('original full-preservation frozen observer identity differs')

def field_preservation():
    if FIELD_MANIFEST_SHA is None:raise ValueError('full original field-preservation observation is not yet pinned')
    manifest=read(FIELD_DIRECTORY/'manifest.json')
    if sha(FIELD_DIRECTORY/'manifest.json')!=FIELD_MANIFEST_SHA:raise ValueError('full original field-preservation root differs')
    for name,digest in manifest['files'].items():
        if sha(FIELD_DIRECTORY/name)!=digest:raise ValueError('full original preservation leaf differs')
    start=read(FIELD_DIRECTORY/'start.json')
    validate_observer_identity(start)
    if start['reference_index_sha256']!=reference.INDEX_SHA or start['prepared_admission_sha256']!=sha(prefix.FIXTURE):
        raise ValueError('original full-preservation input/reference identity differs')
    result=read(FIELD_DIRECTORY/'result.json')
    projection=dict(site='0AB9:E303',field='background gameplay rectangle',cells=1275,
        encoding='space/attribute7;centre marker9',outside_background_unchanged=True)
    if result['projection']!=projection:raise ValueError('original field-initializer projection differs')
    if result['status']!='completed'or(result['ticks'],result['draws'],result['actions'])!=(4199,182811,831)or result['boundary']!='0AB9:EBE2':
        raise ValueError('incomplete original full-lifecycle preservation')
    ticks,draws,events,finish=reference.sources()
    digest=lambda values:hashlib.sha256(b''.join(canonical(v)+b'\n'for v in values)).hexdigest()
    if result['ticks_sha256']!=digest(ticks)or result['actions_sha256']!=digest(events):raise ValueError('original preservation trace digest differs')
    if result['draws_sha256']!=hashlib.sha256(canonical([oracle.normalized(d)for d in draws])).hexdigest()or result['final']!=finish['final']:
        raise ValueError('original preservation draws/final differ')
    return result

def reference_goal_trace(events,tick_count):
    # [derived] Pinned reference has three ordinary goals and no own-goal event.
    # Source5424/542A increments D0C4+side inside the watched call;its accepted
    # event38 is emitted before7B79. Do not generalize this to other sessions.
    goals=[e for e in events if e['code'] in (38,39)]
    if [(e['tick'],e['code'],e['side'])for e in goals]!=[(1213,38,0),(1538,38,0),(3747,38,1)]:
        raise ValueError('pinned reference scoring-event identity differs')
    score=[0,0];cursor=0;trace=[]
    for tick in range(1,tick_count+1):
        while cursor<len(goals)and goals[cursor]['tick']==tick:
            score[goals[cursor]['side']]+=1;cursor+=1
        trace.append(score.copy())
    return trace

def validate_goal_trace(actual,events,tick_count):
    if actual!=reference_goal_trace(events,tick_count):raise ValueError('owned live goal score boundaries differ')

def compare(out,require_field_proof=True):
    ticks,draws,events,finish=reference.sources()
    if out['return']!='0AB9:EBE2'or out['ticks']!=ticks:raise ValueError('owned prepared full tick lifecycle/return differs')
    if out['draws']!=[oracle.normalized(d)for d in draws]:raise ValueError('owned computed setup-and-match draw stream differs')
    expected=[{k:v for k,v in event.items()if k!='event'}for event in events]
    if out['events']!=expected:raise ValueError('owned full watched actions differ')
    if out['final']!=finish['final']:raise ValueError('owned prepared final state differs')
    # Source E069/E06E stores C5/C4 into final D086/D087.
    if out['live_goals']!=list(reversed(finish['final']['scores'])):raise ValueError('owned live/final score mapping differs')
    validate_goal_trace(out['tick_live_goals'],events,len(ticks))
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
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--python',action='store_true');p.add_argument('--check',action='store_true');p.add_argument('--selftest',action='store_true');p.add_argument('--compare-output',type=Path);p.add_argument('--development-reference-only',action='store_true');p.add_argument('--input',type=Path);p.add_argument('--probe',type=Path,default=ROOT/'engine/target/debug/examples/prepared_watched_match_probe');p.add_argument('--prefix-steps',type=int,default=0);a=p.parse_args()
    if a.python:replay_python(admission(),not a.development_reference_only)
    elif a.selftest:selftest()
    elif a.check:
        admission();field_preservation();print('prepared match provenance and full original field/clock proof:ok')
    elif a.compare_output:
        packet=read(a.compare_output)
        if set(packet)!={'ok'}:raise ValueError('prepared match did not complete successfully')
        compare(packet['ok'],not a.development_reference_only)
        print('owned prepared full original reference parity:ok'+(';full original field/clock proof remains pending'if a.development_reference_only else''))
    elif a.input:
        value=validate_input(read(a.input)['input'])
        out=replay_probe(a.probe,value,a.prefix_steps)
        compare(out,not a.development_reference_only)
        print('owned initial-input replay matches original full lifecycle')
    else:p.error('choose --check, --selftest, --compare-output or --input')
if __name__=='__main__':main()
