#!/usr/bin/env python3
"""Project pinned original T29 parents/range arrays; no pairing or search formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/role_pairing'
sha,same,encoded=common.sha,common.same,common.encoded


def preferences(raw):
    if [p['pass'] for p in raw['preferences']]!=[1,2,3,4] or any(len(p['roles'])!=13 for p in raw['preferences']):raise ValueError('preference coverage differs')
    return [p['roles'] for p in raw['preferences']]


def state(raw):
    o=raw['context']['rebuild']['outer'];s=o['selection'];flags=raw['club_flags']
    if [c['id'] for c in flags]!=[c['id'] for c in s['clubs']]:raise ValueError('club flag alias differs')
    aliases={p['id']:p for p in s['players']}
    for p in o['players']:
        q=aliases[p['id']]
        if (p['p8_6'],p['p15_8'],p['p15_11'])!=(q['position'],q['byte8'],q['flags11']):raise ValueError('canonical role/player aliases differ')
    return {'seed':s['seed'],'side_offsets':s['side_offsets'],
            'clubs':[{'id':c['id'],'roster':c['roster'][:11],'byte57':f['byte57']} for c,f in zip(s['clubs'],flags)],
            'scenes':copy.deepcopy(o['scenes']),'players':[{'id':p['id'],'role':p['p8_6'],'byte8':p['p15_8'],'flags11':p['p15_11']} for p in o['players']],
            'preferences':preferences(raw)}


def normalize(value):
    if isinstance(value,dict):return {k:0 if k in ('draw_start','draw_end') else normalize(v) for k,v in value.items()}
    if isinstance(value,list):return [normalize(v) for v in value]
    return value


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(FIXTURES/'index.json').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['index_sha256']!=sha(data):raise ValueError('pairing source pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['ranges'],index['draws'])!=('role-pairing-corpus/1',1,2,20,0) or len(index['records'])!=1:raise ValueError('pairing original census differs')
    meta=index['records'][0]
    if meta['cases']!='T29-r0002.jsonl':raise ValueError('pairing requires successful r0002 source')
    data=(FIXTURES/meta['cases']).read_bytes()
    if sha(data)!=meta['cases_sha256']:raise ValueError('pairing original source differs')
    roots=[json.loads(line) for line in data.splitlines()];streams={'ranges':[],'pairings':[]}
    for c in roots:
        if c['schema']!='role-pairing-case/1' or c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025):raise ValueError('pairing identity/span differs')
        base={k:c[k] for k in ('week','step','replacement')};inp=state(c['input']);out=state(c['output'])
        if c['input']['preferences']!=c['output']['preferences'] or c['input']['club_flags']!=c['output']['club_flags']:raise ValueError('pairing read-only tables differ')
        streams['pairings'].append(dict(base,input=inp,output={'output':out,'ranges':normalize(c['ranges']),'draws':[]}))
        if len(c['ranges'])!=10:raise ValueError('pairing range coverage differs')
        for event,e in enumerate(c['ranges']):
            if (e['draw_start'],e['draw_end'])!=(4025,4025) or e['result'] is not None:raise ValueError('range void/span differs')
            streams['ranges'].append(dict(base,event=event,site=e['site'],side=e['arguments'][0],input={'low':e['arguments'][1],'high':e['arguments'][2],'locals':copy.deepcopy(e['input']),'preferences':inp['preferences']},output={'output':copy.deepcopy(e['output']),'draws':[]}))
    if len(roots)!=2 or len(streams['ranges'])!=20:raise ValueError('pairing projected coverage differs')
    return streams,pin


def load_cases():
    streams,pin=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('pairing projection differs')
    return streams,pin


def methods():
    import onenil_engine as native
    return [('ranges',native.pairing_range),('pairings',native.role_pairing)]


def check(streams):
    for name,method in methods():
        for c in streams[name]:
            if not same(json.loads(method(encoded(c['input']))),c['output']):raise ValueError(f"pairing {name}/replacement{c['replacement']} differs")
        for field in ('output','expected','ranges','assignments'):
            try:method(encoded(dict(streams[name][0]['input'],**{field:0})))
            except ValueError:pass
            else:raise ValueError('pairing accepted expected/intermediate result')
    print('Rust pairing: 2 original parents / 20 ordered composed and standalone ranges / zero draws;actual local-array handoff and scene.word20 results')

def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_role_pairing.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in methods() for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T16-r*.json')),default=0)+1;path=out/f'T16-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'10B9 role pairing with composed1000 and standalone ranges','source_weeks':1,'recorded_parents':2,'recorded_ranges':20,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'T29 pairing slice only; synthetic edge cases separate from original evidence; no164F/scene codes/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T16');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        streams,_=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing pairing projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
