#!/usr/bin/env python3
"""Verify hand-specified static neutral-input cases; no new original observations."""
import argparse,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/neutral_input'
sha,same,encoded=common.sha,common.same,common.encoded
SOURCE_PINS={'static/1-0-exe/code/seg_0AB9_13.asm': '4318632415043eb7dbbe490ce7d637d720423466e2792dc1b584b91507a8e9cc', 'static/1-0-exe/code/seg_0AB9_20.asm': '263fbe73489ef8e34f6e36a5b5b15c36b2174eb6dd032820b240d1f717c24c4c'}

def load_cases():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(FIXTURES/'cases.jsonl').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['coverage']!='static/synthetic' or pin['original_observations']!=0 or pin['sources']!=SOURCE_PINS or pin['cases']!=14 or sha(data)!=pin['cases_sha256']:raise ValueError('neutral static source/case pin differs')
    rows=[json.loads(l) for l in data.splitlines()]
    if len(rows)!=14 or any(r['schema']!='neutral-input-static-case/1' or r['evidence']!='static/synthetic;zero new original observations' for r in rows):raise ValueError('neutral static case identity differs')
    return rows,pin

def check(cases):
    import onenil_engine as native
    for c in cases:
        if not same(json.loads(native.neutral_match_input(encoded(c['input']))),c['output']):raise ValueError(f"neutral static case {c['name']} differs")
    for field in ('output','expected','presentation','key','keyboard_pending','result'):
        try:native.neutral_match_input(encoded(dict(cases[0]['input'],**{field:0})))
        except ValueError:pass
        else:raise ValueError('neutral-only API accepted extra/keyboard/expected input')
    print('Rust neutral78B7/DB68:14 hand-specified static/synthetic cases;zero new original observations;no keyboard-present or continuous-match claim.')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_neutral_input.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.neutral_match_input(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T23-r*.json')),default=0)+1;path=out/f'T23-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'neutral78B7/conditionalDB68 logical input adapter','source_weeks':0,'original_observations':0,'synthetic_cases':14,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Static/synthetic only;zero new original observations;no-key branch only;original keyboard-present/hardware/render/log scratch/timer/physical parity excluded;not a complete match or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T23');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        cases,_=project();data=''.join(encoded(c)+'\n' for c in cases);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing rearrangement projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    cases,pin=load_cases();check(cases)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(cases,pin,a.repetitions,a.out)
if __name__=='__main__':main()
