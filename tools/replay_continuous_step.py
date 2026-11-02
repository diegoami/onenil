#!/usr/bin/env python3
"""One original initial state enters one native session;all later observations are assertions."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
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
    projected=(FIXTURES/'admission.json').read_text().strip()
    if not common.same(json.loads(projected),admission):raise ValueError('initial admission projection differs')
    dp=json.loads((FIXTURES/'draws-provenance.json').read_text())
    if common.sha((FIXTURES/'draws.jsonl').read_bytes())!=dp['projection_sha256'] or dp['trace_sha256']!='66fe8ba4c5cbc6af163ebcbe1e5ba815a89abfd5df755818cabe5a8968ef68ef':raise ValueError('fixed original draw provenance differs')
    pin['draws_provenance']=dp
    return admission,rows[1:],pin

def normalized(s):
    s=copy.deepcopy(s);s.pop('save_block_digests',None);return s

def check(admission,expected):
    import onenil_engine as native
    session=native.NativeQuickSession(common.encoded(admission))
    if not common.same(json.loads(session.snapshot()),normalized(expected[0]['input'])):raise ValueError('initial native admission projection differs')
    count=0
    draws=[json.loads(l) for l in (FIXTURES/'draws.jsonl').read_text().splitlines()]
    while True:
        got=json.loads(session.step()) # Deliberately no per-step inputs.
        want=expected[count]
        for k in ('input','output'):
            if not common.same(got[k],normalized(want[k])):raise ValueError(f'continuous step{count} {k} differs')
        if not common.same(got['clock'],{k:want['clock'][k] for k in ('output','result','event')}) or not common.same(got['clock_input'],want['clock']['input']):raise ValueError(f'continuous step{count} caller clock differs')
        expected_draws=[{k:v for k,v in d.items() if k!='i'} for d in draws[want['draw_start']:want['clock']['draw_end']]]
        if not common.same(got['draws'],expected_draws):raise ValueError(f'continuous step{count} RNG stream differs')
        count+=1
        if got['clock']['result']:break
        if count>=len(expected):raise ValueError('native sequence failed to terminate')
    if count!=46:raise ValueError('observed first-half boundary count differs')
    print('One admitted native image/RNG matches46 complete DD02/clock transitions through actual halftime,including three goals/resets;not setup/full-match/playable acceptance.')

def benchmark(admission,pin,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_continuous_step.py','pyproject.toml'):raise ValueError('commit source before measuring')
    if native.__debug_assertions__:raise ValueError('release extension required')
    value=common.encoded(admission);started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();steps=0;size=0
    for _ in range(repetitions):
        session=native.NativeQuickSession(value)
        while True:
            result=session.step();size+=len(result);steps+=1
            if json.loads(result)['clock']['result']:break
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T25-r*.json')),default=0)+1;path=out/f'T25-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
         'source_pins':pin,'routine':'one admitted continuing DD02/7B7B first-half session','repetitions':repetitions,'steps':steps,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized initial admission;release parse/setup/native successive steps/serialization and output decoding for termination included',
         'limits':'Fixed reference first half,after setup;not native setup,second half,full match,playable shell or emulator/full-game speed comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    ap=argparse.ArgumentParser(description=__doc__);g=ap.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true')
    ap.add_argument('--out',default='records/T25');ap.add_argument('--repetitions',type=int,default=10);args=ap.parse_args()
    if args.repetitions<1:raise ValueError('positive repetitions required')
    a,c,pin=load();check(a,c)
    if args.benchmark:benchmark(a,pin,args.repetitions,args.out)
if __name__=='__main__':main()
