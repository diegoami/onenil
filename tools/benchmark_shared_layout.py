#!/usr/bin/env python3
"""Check reused legacy/new bridge cases after shared-layout migration;no new original census."""
import argparse,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as tail
import replay_extra_event as extra
import replay_substitution as substitutions
import replay_neutral_input as neutral
ROOT=tail.ROOT

def prepare():
    import onenil_engine as native
    ps,_,pp=tail.load_cases();es,ep=extra.load_cases();ss,sp=substitutions.load_cases();ns,np=neutral.load_cases()
    p=next(c for c in ps if c['week']=='T04-week-r0017' and c['step']==0)
    e=next(c for c in es if c[0]['week']=='T04-week-r0017')
    s=next(c for c in ss if c['step']==57);n=next(c for c in ns if c['name']=='requested-switch')
    rows=[('prefix',native.step_tail_prefix,p['prefix']['input'],tail.expected_prefix(p)),
          ('extra-event',native.extra_event_decision,e[1],extra.expected(e[0])),
          ('substitution',native.substitutions,s['input'],s['output']),
          ('neutral-static',native.neutral_match_input,n['input'],n['output'])]
    for name,method,inp,want in rows:
        if not tail.same(json.loads(method(tail.encoded(inp))),want):raise ValueError('shared-layout reused '+name+' result differs')
    print('Shared-layout bridge smoke:reused original prefix/extra-event/substitution and static neutral case match. Same-image contracts are tested in Rust;no new original observations or continuous-match claim.')
    return [(method,tail.encoded(inp)) for _,method,inp,_ in rows],{'prefix':pp,'extra_event':ep,'substitution':sp,'neutral_static':np}

def benchmark(rows,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if tail.git('status','--porcelain','--','engine','tools/benchmark_shared_layout.py','pyproject.toml'):raise ValueError('commit source before measuring')
    if native.__debug_assertions__:raise ValueError('release bridge required')
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in rows:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T24-r*.json')),default=0)+1;path=out/f'T24-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':tail.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':tail.sha((ROOT/'engine/Cargo.lock').read_bytes()),
         'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':tail.sha(Path(native.__file__).read_bytes())},
         'source_pins':pins,'routine':'mixed standalone bridges after canonical shared-pool migration','profiles':['original-prefix','original-extra-event','original-substitution','static-neutral'],
         'new_original_observations':0,'repetitions':repetitions,'calls':len(rows)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized inputs;release parsing,image setup,native calculation/observations and serialization included;preparation/output decoding excluded',
         'limits':'Independent public bridge calls,not a single continuing match;shared-image correctness tested separately in Rust;reused observations plus static neutral case;no new original census or emulator/full-game speed comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    ap=argparse.ArgumentParser(description=__doc__);g=ap.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true')
    ap.add_argument('--out',default='records/T24');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.repetitions<1:raise ValueError('positive repetitions required')
    rows,pins=prepare()
    if a.benchmark:benchmark(rows,pins,a.repetitions,a.out)
if __name__=='__main__':main()
