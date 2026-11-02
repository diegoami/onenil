#!/usr/bin/env python3
"""Project fixed-match T26 rebuilds and T28 assignments; no assignment/lookup formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/scene_assignment'
sha,same,encoded=common.sha,common.same,common.encoded


def source(pin,task,schema):
    meta=pin['sources'][task];d=FIXTURES/task;data=(d/'index.json').read_bytes()
    if sha(data)!=meta['index_sha256']:raise ValueError('assignment source index differs')
    index=json.loads(data)
    if index['schema']!=schema:raise ValueError('assignment source schema differs')
    records=[r for r in index['records'] if r['week']=='T04-week-r0017']
    if len(records)!=1 or records[0]['cases']!=meta['cases'] or records[0]['calls']!=2 or records[0]['draws']!=0:raise ValueError('fixed-match source selection differs')
    data=(d/meta['cases']).read_bytes()
    if sha(data)!=records[0]['cases_sha256']:raise ValueError('assignment original rows differ')
    rows=[json.loads(l) for l in data.splitlines()]
    if len(rows)!=2 or any(c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025) for c in rows):raise ValueError('assignment original identity/span differs')
    return rows


def state(raw,index,selector):
    o=raw['outer'];selection=o['selection'];aliases={p['id']:p for p in selection['players']};players=[]
    for p in o['players']:
        q=aliases[p['id']]
        if (p['p8_6'],p['p8_7'])!=(q['position'],q['style']):raise ValueError('assignment canonical role/style alias differs')
        players.append({'id':p['id'],'club':p['p8_0'],'age':p['p8_1'],'role':p['p8_6'],'style':p['p8_7'],'value':p['p15_word0'],'hand_selector':q['hand_selector'],'hand_cell':q['hand_cell']})
    return {'index':index,'selector':selector,'seed':selection['seed'],'position_default':o['globals']['position_default'],'position12':o['globals']['position12'],
            'directions':o['directions'],'clubs':[{'id':c['id'],'roster':c['roster'][:11]} for c in selection['clubs']],
            'players':players,'club_counts':copy.deepcopy(raw['club_counts']),'scenes':copy.deepcopy(o['scenes'])}


def normalize(value):
    if isinstance(value,dict):return {k:0 if k in ('draw_start','draw_end') else normalize(v) for k,v in value.items()}
    if isinstance(value,list):return [normalize(v) for v in value]
    return value


def helper_count(events):return sum(1+helper_count(e['children']) for e in events)

def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('assignment provenance differs')
    rebuilds=source(pin,'T26','rebuild-corpus/1');parents=source(pin,'T28','rearrangement-corpus/1');streams={'assignments':[],'rebuilds':[]}
    for c in rebuilds:
        if c['schema']!='rebuild-case/1':raise ValueError('rebuild case schema differs')
        base={k:c[k] for k in ('week','step','replacement')};index=c['scene']
        streams['rebuilds'].append(dict(base,input=state(c['input'],index,index),output={'output':state(c['output'],index,index),'helpers':normalize(c['helpers']),'draws':[]}))
    for c in parents:
        for event,e in enumerate(c['events']):
            if e['kind']!='assign':continue
            if e['site']!='0AB9:1971' or e['arguments'][1]!=e['arguments'][2] or e['result'] is not None or (e['draw_start'],e['draw_end'])!=(4025,4025):raise ValueError('assignment boundary differs')
            index,selector=e['arguments'][1:];base={k:c[k] for k in ('week','step','replacement')}
            streams['assignments'].append(dict(base,event=event,input=state(e['input']['rebuild'],index,selector),output={'output':state(e['output']['rebuild'],index,selector),'draws':[]}))
    if len(streams['assignments'])!=44 or sum(helper_count(c['helpers']) for c in rebuilds)!=20:raise ValueError('assignment/rebuild census differs')
    return streams,pin


def load_cases():
    streams,pin=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('assignment projection differs')
    return streams,pin


def methods():
    import onenil_engine as native
    return [('assignments',native.assign_scene),('rebuilds',native.rebuild_scene)]

def check(streams):
    for name,method in methods():
        for c in streams[name]:
            got=json.loads(method(encoded(c['input'])))
            if name=='assignments':got={k:got[k] for k in ('output','draws')} #T28 records outer boundary,not nested helper census.
            if not same(got,c['output']):raise ValueError(f"scene assignment {name}/replacement{c['replacement']} differs")
        for field in ('output','expected','helpers','count','minimum','half'):
            try:method(encoded(dict(streams[name][0]['input'],**{field:0})))
            except ValueError:pass
            else:raise ValueError('assignment accepted expected/intermediate output')
    print('Rust scene assignment:44 original T28 boundaries/2 T26 rebuilds/20 observed rebuild helpers;T28 nested helpers are computed diagnostics. First-playable gate1 prerequisite,not a complete match.')

def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_scene_assignment.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in methods() for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T18-r*.json')),default=0)+1;path=out/f'T18-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'8599 assignment and8663 rebuild with actual native lookups/predicates','source_weeks':1,'recorded_assignments':44,'recorded_rebuilds':2,'recorded_rebuild_helpers':20,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'T26/T28 fixed-match assignment slice;not a complete playable match; synthetic edge cases separate from original evidence; no164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))






def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T18');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        streams,_=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing assignment projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
