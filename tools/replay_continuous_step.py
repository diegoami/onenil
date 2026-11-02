#!/usr/bin/env python3
"""One original initial state enters one native session;all later observations are assertions."""
import argparse,copy,json
from pathlib import Path
import replay_step_tail as common
import replay_ratings as ratings
import replay_placement as placement
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/continuous'

def load():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology':raise ValueError('continuity repository differs')
    for name,sha in pin['sources'].items():
        if common.sha((FIXTURES/name).read_bytes())!=sha:raise ValueError('continuity source digest differs')
    for name,sha in pin['prior_native_pins'].items():
        if common.sha((FIXTURES.parent/name/'provenance.json').read_bytes())!=sha:raise ValueError('prior native source pin differs')
    rows=[json.loads(l) for l in (FIXTURES/'T36-r0001.jsonl').read_text().splitlines()]
    if rows[0]['schema']!='match-admission/1' or len(rows)!=96 or any(r['schema']!='match-boundary/1' or r['step']!=i for i,r in enumerate(rows[1:])):raise ValueError('one admission and95 ordered assertion boundaries required')
    a=rows[0]['state'];first=rows[1]['input'];week=rows[0]['week']
    rs,_,hs,_,_=ratings.load_cases();ps,_,_,_=placement.load_cases();pc=next(c for c in ps if c['week']==week and c['step']==0)
    handoff=next(h for h in hs if h['week']==week and h['call']==0)
    rm={(r['week'],r['call']):r for r in rs};sides=[copy.deepcopy(rm[week,n]['input']) for n in handoff['sides']]
    for r in sides:r['seed']=first['seed'];r['ratings']=first['ratings']
    if [r['club'] for r in sides]!=first['clubs'] or [r['side'] for r in sides]!=[0,1]:raise ValueError('initial rating side identities differ')
    outer=a['positions']['context']['context']['rebuild']['outer']
    master=[{'id':p['id'],'p8':[p[f'p8_{i}'] for i in range(8)],'p15':[p['p15_word0']&255,p['p15_word0']>>8]+[p[f'p15_{i}'] for i in range(2,15)]} for p in outer['players']]
    admission={'contest':{'k':pc['contest']['input']['k'],'seed':first['seed'],'ratings':first['ratings'],'goals':first['goal_bytes']},
               'ratings':sides,'players':master,'scenes':outer['scenes'],'goal':a['goals'],'extra':{'state':a['extra'],'opponent_d0b':a['threshold']},
               'event':a['events'],'reset':a['reset'],'substitution':{'context':outer['selection'],'replacement':None},
               'neutral':a['neutral'],'clock':a['clock'],'history':a['history'],'config':a['config'],
               'selectors':[r['value'] for r in a['selectors']],'scorer_slots':a['scorer_slots'],'minute_limits':a['minute_limits'],
               'replacement_word':outer['globals']['position_default']}
    return admission,rows[1:],pin

def normalized(s):
    s=copy.deepcopy(s);s.pop('save_block_digests',None);return s

def check(admission,expected):
    import onenil_engine as native
    session=native.NativeQuickSession(common.encoded(admission))
    if not common.same(json.loads(session.snapshot()),normalized(expected[0]['input'])):raise ValueError('initial native admission projection differs')
    count=0
    while True:
        got=json.loads(session.step()) # Deliberately no per-step inputs.
        want=expected[count]
        for k in ('input','output'):
            if not common.same(got[k],normalized(want[k])):raise ValueError(f'continuous step{count} {k} differs')
        if not common.same(got['clock'],want['clock']):raise ValueError(f'continuous step{count} caller clock differs')
        count+=1
        if got['clock']['result']:break
        if count>=len(expected):raise ValueError('native sequence failed to terminate')
    if count!=46:raise ValueError('observed first-half boundary count differs')
    print('One admitted native image/RNG matches46 complete DD02/clock transitions through actual halftime,including three goals/resets;not setup/full-match/playable acceptance.')

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true',required=True);ap.parse_args();a,c,_=load();check(a,c)
if __name__=='__main__':main()
