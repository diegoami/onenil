#!/usr/bin/env python3
"""Replay raw A4F2 strategy inputs through Rust, never Python engine formulas."""
import argparse
import json
import platform
import subprocess
import sys
import time
from datetime import datetime, timezone
from importlib.metadata import version
from pathlib import Path
import replay_extra_event as common
import replay_step_tail as tail

ROOT=tail.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/post_score'
sha,same,encoded=tail.sha,tail.same,tail.encoded
SITES={'0AB9:A305','0AB9:A38C','0AB9:A3A4','0AB9:A417','0AB9:A4A5'}


def load_cases():
    old,old_index,_=tail.load_cases();prefixes={(c['week'],c['step']):c for c in old}
    index,pin=common.pinned_index(FIXTURES,'post-score-corpus/1')
    if (tuple(index[k] for k in ('weeks','calls','draws','steps','late_calls','traces'))!=(6,17,12,558,1317,15)
            or index['t16_index_sha256']!=sha((FIXTURES.parent/'step_tail/index.json').read_bytes())):
        raise ValueError('strategy coverage or retained index differs')
    prior_meta={r['week']:r for r in old_index['records']};cases=[];seen=set();files=set();weeks=set()
    for r in index['records']:
        if r['cases'] in files or r['week'] in weeks:raise ValueError('duplicate strategy week/file')
        rows=common.read_rows(FIXTURES,r['cases'],r['cases_sha256'])
        if len(rows)!=r['calls']:raise ValueError('incomplete strategy week')
        for c in rows:
            key=c['week'],c['step']
            if c['schema']!='post-score-case/1' or c['week']!=r['week'] or key in seen or key not in prefixes or not same(c['prefix'],prefixes[key]):
                raise ValueError('strategy identity/literal prefix differs')
            if any(c['source'][a]!=prior_meta[c['week']][b] for a,b in (
                ('t16_record','record'),('t16_record_sha256','record_sha256'),('t16_cases','cases'),('t16_cases_sha256','cases_sha256'))):
                raise ValueError('strategy source metadata differs')
            if (c['source']['t08_trace']!=c['prefix']['source']['t08_trace']
                    or c['source']['t08_trace_sha256']!=c['prefix']['source']['t08_trace_sha256']):raise ValueError('strategy T08 trace differs')
            cases.append(c);seen.add(key)
        files.add(r['cases']);weeks.add(r['week'])
    wanted={k for k,c in prefixes.items() if any(p['routine']=='0AB9:52B6' for p in c['prefix']['pending_calls'])}
    prior_refs={r['trace']:r for r in old_index['coverage']};new_week={r['cases']:r['week'] for r in index['records']};old_week={r['cases']:r['week'] for r in old_index['records']}
    for r in index['coverage']:
        p=prior_refs[r['trace']]
        if r['trace_sha256']!=p['trace_sha256'] or new_week[r['cases']]!=old_week[p['cases']]:raise ValueError('strategy coverage trace differs')
    if seen!=wanted or len(files)!=6 or len(cases)!=17 or len(index['coverage'])!=15 or len({r['trace'] for r in index['coverage']})!=15:
        raise ValueError('strategy call/source coverage incomplete')
    return cases,pin


def request(c):return {'state':c['input'],'argument':c['argument']}


def expected(c):
    draws=[]
    for d in c['draws']:
        sites=SITES.intersection(d['callers'])
        if len(sites)!=1:raise ValueError('unattributed strategy draw')
        draws.append({'site':sites.pop(),**{k:d[k] for k in ('n','value','seed_before','seed_after')}})
    return {'draws':draws,**{k:c[k] for k in ('output','paths','setters','normalizers','getters')}}


def check(cases):
    import onenil_engine as native
    counts=[0,0,0,0]
    for c in cases:
        inp=encoded(request(c));got=json.loads(native.post_score_strategy(inp))
        if not same(got,expected(c)):raise ValueError(f"{c['week']} step {c['step']}: strategy/helper mismatch")
        old=c['prefix'];composed=json.loads(native.placement_prefix_post_score(encoded(old['placement']['input']),encoded(old['prefix']['input']),inp))
        if not same(composed['strategy'],got) or not same(composed['predecessor']['prefix'],tail.expected_prefix(old)):
            raise ValueError('actual prefix shared footprint differs')
        for i,k in enumerate(('draws','setters','normalizers','getters')):counts[i]+=len(got[k])
    if counts!=[12,32,566,34]:raise ValueError('strategy native coverage differs')
    c=cases[0];old=c['prefix']
    for field in ('seed','scores'):
        bad=json.loads(encoded(request(c)))
        if field=='seed':bad['state']['seed']^=1
        else:bad['state']['scores'][0]^=1
        try:native.placement_prefix_post_score(encoded(old['placement']['input']),encoded(old['prefix']['input']),encoded(bad))
        except ValueError:pass
        else:raise ValueError('bridge accepted inconsistent prefix footprint')
    bad=dict(request(c),expected_getters=c['getters'])
    try:native.post_score_strategy(encoded(bad))
    except ValueError:pass
    else:raise ValueError('bridge accepted expected getters as inputs')
    print('Rust post-score: 17 strategies / 12 draws / 32 setters / 566 normalizers / 34 getters; shared prefix footprint preserved')


def benchmark(cases,pin,repetitions,output):
    from onenil_engine import onenil_engine as native
    if tail.git('status','--porcelain','--','engine','tools/replay_post_score.py','pyproject.toml'):raise ValueError('commit code and fixtures before measuring')
    if native.__debug_assertions__:raise ValueError('release extension required')
    prepared=[encoded(request(c)) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();serialized=0
    for _ in range(repetitions):
        for data in prepared:serialized+=len(native.post_score_strategy(data))
    elapsed=time.perf_counter_ns()-begin;directory=ROOT/output;directory.mkdir(parents=True,exist_ok=True)
    n=1+max((int(p.stem.split('-r')[-1]) for p in directory.glob('T08-r*.json')),default=0);path=directory/f'T08-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':tail.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,
         'maturin':version('maturin'),'extension_sha256':sha(Path(native.__file__).read_bytes())},'archaeology':pin,'routine':'A4F2 post-score strategy/flag closure',
         'distinct_calls':len(cases),'source_weeks':6,'source_traces':15,'repetitions':repetitions,'calls':len(cases)*repetitions,'rng_draws':12*repetitions,
         'elapsed_ns':elapsed,'serialized_output_bytes':serialized,'method':'Pre-serialized raw strategy inputs; bridge parsing, image setup, native strategy/flags/RNG and serialization included; preparation/output decoding excluded; after untimed original/composed replay',
         'limits':'Shared-field strategy projection only; no complete52B6/6B2F/78B7, DD02/match/week/setup, physical/scratch/timer/render parity or emulator comparison'}
    with path.open('x') as f:f.write(json.dumps(rec,indent=2)+'\n')
    print(f'{path.relative_to(ROOT)}: {rec["calls"]} strategies / {rec["rng_draws"]} draws in {elapsed/1e9:.6f} s')


def main(argv=None):
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true');ap.add_argument('--benchmark',action='store_true')
    ap.add_argument('--out',default='records/T08');ap.add_argument('--repetitions',type=int,default=20);args=ap.parse_args(argv)
    if not(args.check or args.benchmark) or args.repetitions<1:ap.error('choose --check or --benchmark; positive repetitions required')
    cases,pin=load_cases();check(cases)
    if args.benchmark:benchmark(cases,pin,args.repetitions,args.out)
    return 0


if __name__=='__main__':
    try:sys.exit(main())
    except (ValueError,KeyError,OSError,ImportError,StopIteration) as exc:print(f'post-score replay failed: {exc}',file=sys.stderr);sys.exit(1)
