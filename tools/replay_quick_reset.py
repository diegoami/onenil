#!/usr/bin/env python3
"""Replay named quick52B6 reset inputs through Rust; Python joins evidence only."""
import argparse,copy,json,platform,subprocess,sys,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_post_score as strategy
import replay_extra_event as common
import replay_step_tail as tail
ROOT=tail.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/quick_reset'
sha,same,encoded=tail.sha,tail.same,tail.encoded


def load_cases():
    old,_=strategy.load_cases();prior={(c['week'],c['step']):c for c in old}
    index,pin=common.pinned_index(FIXTURES,'quick-reset-corpus/1')
    if tuple(index[k] for k in ('weeks','calls','draws','copies','motions','helpers'))!=(6,17,12,374,374,747):raise ValueError('reset coverage differs')
    if index['t20_index_sha256']!=sha((FIXTURES.parent/'post_score/index.json').read_bytes()):raise ValueError('retained strategy index differs')
    meta=json.loads((FIXTURES.parent/'post_score/index.json').read_text());meta={r['week']:r for r in meta['records']}
    cases=[];seen=set();files=set();weeks=set()
    for r in index['records']:
        if r['cases'] in files or r['week'] in weeks:raise ValueError('duplicate reset file/week')
        rows=common.read_rows(FIXTURES,r['cases'],r['cases_sha256'])
        if len(rows)!=r['calls']:raise ValueError('incomplete reset week')
        for c in rows:
            key=c['week'],c['step']
            if c['schema']!='quick-reset-case/1' or c['week']!=r['week'] or key in seen or key not in prior or not same(c['post_score'],prior[key]):raise ValueError('reset identity/literal strategy differs')
            if any(c['source'][a]!=meta[c['week']][b] for a,b in [('t20_record','record'),('t20_record_sha256','record_sha256'),('t20_cases','cases'),('t20_cases_sha256','cases_sha256')]):raise ValueError('reset retained source differs')
            cases.append(c);seen.add(key)
        files.add(r['cases']);weeks.add(r['week'])
    if seen!=set(prior) or len(cases)!=17 or len(files)!=6:raise ValueError('reset calls/source coverage incomplete')
    return cases,pin


def expected(c):
    out={k:copy.deepcopy(c[k]) for k in ('output','copies','motions','keeper','strategy_input','strategy_output','helpers')}
    if out['keeper']:
        for k in ('draw_start','draw_end'):out['keeper'][k]-=c['draw_start']
    out['strategy']=strategy.expected(c['post_score'])
    draws=[]
    for d in c['draws']:
        sites=({'0AB9:4FDF'}|strategy.SITES).intersection(d['callers'])
        if len(sites)!=1:raise ValueError('unattributed reset draw')
        draws.append({'site':sites.pop(),**{k:d[k] for k in ('n','value','seed_before','seed_after')}})
    out['draws']=draws;return out


def check(cases):
    import onenil_engine as native
    counts=[0,0,0,0]
    for c in cases:
        got=json.loads(native.quick_goal_reset(encoded(c['input'])))
        if not same(got,expected(c)):raise ValueError(f"{c['week']} step {c['step']}: reset/helper/intermediate fields differ")
        p=c['post_score']['prefix'];composed=json.loads(native.placement_prefix_quick_reset(encoded(p['placement']['input']),encoded(p['prefix']['input']),encoded(c['input'])))
        if not same(composed['reset'],got) or not same(composed['predecessor']['prefix'],tail.expected_prefix(p)):raise ValueError('actual prefix→reset→strategy handoff differs')
        for i,k in enumerate(('draws','copies','motions','helpers')):counts[i]+=len(got[k])
    if counts!=[12,374,374,747]:raise ValueError('native reset coverage differs')
    c=cases[0];p=c['post_score']['prefix']
    for field in ('seed','actor','remaining'):
        bad=copy.deepcopy(c['input'])
        if field=='seed':bad['strategy']['seed']^=1
        else:bad[field]^=1
        try:native.placement_prefix_quick_reset(encoded(p['placement']['input']),encoded(p['prefix']['input']),encoded(bad))
        except ValueError:pass
        else:raise ValueError('bridge accepted wrong computed reset handoff')
    bad=dict(c['input'],strategy_input=c['strategy_input'])
    try:native.quick_goal_reset(encoded(bad))
    except ValueError:pass
    else:raise ValueError('bridge accepted expected intermediate strategy as input')
    print('Rust quick reset: 17 resets / 12 draws / 374 copies / 374 motions / 747 helpers; computed prefix and strategy boundaries preserved')


def benchmark(cases,pin,repetitions,output):
    from onenil_engine import onenil_engine as native
    if tail.git('status','--porcelain','--','engine','tools/replay_quick_reset.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if native.__debug_assertions__:raise ValueError('release extension required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();serialized=0
    for _ in range(repetitions):
        for value in prepared:serialized+=len(native.quick_goal_reset(value))
    elapsed=time.perf_counter_ns()-begin;directory=ROOT/output;directory.mkdir(parents=True,exist_ok=True)
    n=1+max((int(p.stem.split('-r')[-1]) for p in directory.glob('T09-r*.json')),default=0);path=directory/f'T09-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':tail.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,
         'maturin':version('maturin'),'extension_sha256':sha(Path(native.__file__).read_bytes())},'archaeology':pin,'routine':'named quick52B6 reset with actual-seed strategy',
         'distinct_resets':len(cases),'source_weeks':6,'repetitions':repetitions,'calls':len(cases)*repetitions,'rng_draws':12*repetitions,
         'copies':374*repetitions,'motion_visits':374*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':serialized,
         'method':'Pre-serialized original reset inputs; release bridge parsing, image setup, named reset/strategy/helpers/RNG and serialization included; preparation/output decoding excluded; after untimed original/composed replay',
         'limits':'Named quick logical reset; no045C buffers,6B2F/78B7, continuous DD02/match/week/setup, arbitrary physical/scratch/timer/render parity or emulator comparison'}
    with path.open('x') as f:f.write(json.dumps(rec,indent=2)+'\n')
    print(f'{path.relative_to(ROOT)}: {rec["calls"]} resets / {rec["rng_draws"]} draws in {elapsed/1e9:.6f} s')


def main(argv=None):
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true');ap.add_argument('--benchmark',action='store_true')
    ap.add_argument('--out',default='records/T09');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args(argv)
    if not(a.check or a.benchmark) or a.repetitions<1:ap.error('choose --check or --benchmark; positive repetitions required')
    cases,pin=load_cases();check(cases)
    if a.benchmark:benchmark(cases,pin,a.repetitions,a.out)
    return 0
if __name__=='__main__':
    try:sys.exit(main())
    except (ValueError,KeyError,OSError,ImportError,StopIteration) as exc:print(f'quick-reset replay failed: {exc}',file=sys.stderr);sys.exit(1)
