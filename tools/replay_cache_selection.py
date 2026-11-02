#!/usr/bin/env python3
"""Verify pinned original T32 cases through Rust; no selector/rating formulas."""
import argparse,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/cache_selection'
sha,same,encoded=common.sha,common.same,common.encoded


def load_cases():
    data=(FIXTURES/'index.json').read_bytes();pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or sha(data)!=pin['index_sha256']:raise ValueError('cached selector source/index pin differs')
    index=json.loads(data)
    if index['schema']!='cache-selection-corpus/1' or index['weeks']!=1 or set(index['streams'])!={'selectors','ratings'}:raise ValueError('cache corpus schema/domain differs')
    streams={}
    for name,count,schema in [('selectors',2,'cached-selector-case/1'),('ratings',44,'cached-rating-case/1')]:
        meta=index['streams'][name]
        if meta['cases']!=f'traces/T32/{name}.jsonl' or meta['count']!=count or meta['draws']!=0:raise ValueError('cache stream provenance differs')
        data=(FIXTURES/(name+'.jsonl')).read_bytes()
        if sha(data)!=meta['cases_sha256']:raise ValueError('original cached stream digest differs')
        rows=[json.loads(s) for s in data.splitlines()]
        if len(rows)!=count or any(c['schema']!=schema or c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025) for c in rows):raise ValueError('cache original coverage differs')
        keys=[(c['week'],c['step'],c['replacement'],c['event']) for c in rows]
        if len(set(keys))!=count or keys!=sorted(keys):raise ValueError('cache original order/identity differs')
        streams[name]=rows
    if [c['replacement'] for c in streams['selectors']]!=[0,1] or any(c['source']!=streams['selectors'][0]['source'] for rows in streams.values() for c in rows):raise ValueError('cache retained original source differs')
    return streams,pin


def check(streams):
    import onenil_engine as native
    for name,method in [('selectors',native.cached_player_selector),('ratings',native.cached_player_rating)]:
        for c in streams[name]:
            got=json.loads(method(encoded(c['input'])))
            if not same(got['output'],c['output']) or got['draws']!=[]:raise ValueError(f"{c['week']} replacement {c['replacement']} event {c['event']}: cached {name} differs")
    for method,key in [(native.cached_player_selector,'selectors'),(native.cached_player_rating,'ratings')]:
        for field in ('output','expected','visits'):
            bad=dict(streams[key][0]['input'],**{field:{}})
            try:method(encoded(bad))
            except ValueError:pass
            else:raise ValueError('cache bridge accepted expected-output fields')
    bad=json.loads(encoded(streams['selectors'][0]['input']));bad['players'].pop()
    try:native.cached_player_selector(encoded(bad))
    except ValueError:pass
    else:raise ValueError('cache bridge accepted incomplete player input')
    print('Rust cache selection: 2 original selectors / 44 original ratings / zero draws; native visits are not an original nested-call census')


def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_cache_selection.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in [('selectors',native.cached_player_selector),('ratings',native.cached_player_rating)] for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T11-r*.json')),default=0)+1;path=out/f'T11-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'read-only13B8 cached selector and4EBF remainder rating','source_weeks':1,'recorded_selectors':2,'recorded_ratings':44,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'T32 helper slice only; synthetic edge cases separate from original evidence; no1455/1539/164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))


def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    group.add_argument('--check',action='store_true');group.add_argument('--benchmark',action='store_true');ap.add_argument('--out',default='records/T11');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
