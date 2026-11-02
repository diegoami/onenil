#!/usr/bin/env python3
"""Join pinned original T34/T35 records and replay Rust; no engine formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/terminal'
sha,same,encoded=common.sha,common.same,common.encoded


def sources():
    pin=json.loads((FIXTURES/'provenance.json').read_text());records=[]
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('terminal source pin differs')
    indices={}
    for directory,experiment in (('positions','T34'),('proximity','T35')):
        path=FIXTURES/directory/'index.json';data=path.read_bytes()
        if sha(data)!=pin['indices'][directory+'/index.json']:raise ValueError('terminal source index differs')
        index=json.loads(data);indices[directory]=index
        if index['schema']!=('scene-positions-corpus/1' if directory=='positions' else 'proximity-selectors-corpus/1'):raise ValueError('terminal corpus schema differs')
        if (index['weeks'],index['calls'],index['draws'])!=(1,2,0) or len(index['records'])!=1:raise ValueError('terminal source coverage differs')
        meta=index['records'][0];name=meta['cases']
        if Path(name).name!=name or not name.startswith(experiment+'-r'):raise ValueError('terminal source path differs')
        data=(FIXTURES/directory/name).read_bytes()
        if sha(data)!=meta['cases_sha256']:raise ValueError('terminal original fixture differs')
        rows=[json.loads(s) for s in data.splitlines()]
        if len(rows)!=2 or [c['replacement'] for c in rows]!=[0,1] or any(c['week']!='T04-week-r0017' or c['schema']!=('scene-positions-case/1' if directory=='positions' else 'proximity-selectors-case/1') for c in rows):raise ValueError('terminal original week differs')
        records.append(rows)
    if indices['proximity']['t34_index_sha256']!=pin['indices']['positions/index.json']:raise ValueError('terminal T34 handoff provenance differs')
    return records,pin


def state(context,positions,extra):
    outer=context['context']['rebuild']['outer'];selection=outer['selection']
    return {'seed':selection['seed'],'side_offsets':selection['side_offsets'],'actor':context['context']['actor'],
            'scenes':copy.deepcopy(outer['scenes']),
            'players':[{'id':p['id'],'position':p['p8_6'],'byte8':p['p15_8'],'flags11':p['p15_11']} for p in outer['players']],
            'positions':copy.deepcopy(positions),'signs':extra['signs'],'target':extra['target'],
            'phase':extra['phase'],'time':extra['time'],'selectors':copy.deepcopy(extra['selectors'])}


def normalized(value,start):
    if isinstance(value,list):return [normalized(v,start) for v in value]
    if isinstance(value,dict):return {k:(v-start if k in ('draw_start','draw_end','draw') else normalized(v,start)) for k,v in value.items()}
    return value


def project():
    (positions,proximity),pin=sources();cases=[]
    for a,b in zip(positions,proximity):
        if any(a[k]!=b[k] for k in ('week','step','replacement','arguments')) or a['output']['context']!=b['input']['context']:raise ValueError('original terminal handoff differs')
        if a['input']['signs']!=b['input']['signs'] or a['output']['signs']!=b['input']['signs']:raise ValueError('original terminal signs differ')
        if (a['draw_start'],a['draw_end'],b['draw_start'],b['draw_end'])!=(4025,4025,4025,4025):raise ValueError('original terminal RNG span differs')
        cases.append({'schema':'native-terminal-case/1','week':a['week'],'step':a['step'],'replacement':a['replacement'],
                      'input':state(a['input']['context'],a['input']['positions'],b['input']),
                      'proximity_input':state(b['input']['context'],a['output']['positions'],b['input']),
                      'positions':{'output':state(a['output']['context'],a['output']['positions'],b['input']),
                                   'visits':normalized(a['visits'],4025),'draws':[]},
                      'proximity':{'output':state(b['output']['context'],a['output']['positions'],b['output']),
                                   'visits':normalized(b['visits'],4025),'initializations':normalized(b['initializations'],4025),'draws':[]}})
    return cases,pin


def load_cases():
    cases,pin=project();data=b''.join((encoded(c)+'\n').encode() for c in cases)
    if sha(data)!=pin['projection_sha256'] or (FIXTURES/'cases.jsonl').read_bytes()!=data:raise ValueError('terminal projection differs from pinned original rows')
    return cases,pin


def check(cases):
    import onenil_engine as native
    for c in cases:
        for method,key,arg in ((native.scene_positions,'positions','input'),(native.proximity_selectors,'proximity','proximity_input')):
            if not same(json.loads(method(encoded(c[arg]))),c[key]):raise ValueError(f"{c['week']} replacement {c['replacement']}: {key} differs")
        combined=json.loads(native.terminal_rearrangement(encoded(c['input'])))
        if not same(combined,{k:c[k] for k in ('positions','proximity')}):raise ValueError('actual shared-image terminal handoff differs')
    for extra in ('output','expected','visits'):
        try:native.terminal_rearrangement(encoded(dict(cases[0]['input'],**{extra:{}})))
        except ValueError:pass
        else:raise ValueError('bridge accepted expected-output fields')
    print('Rust terminal helpers: 2 positions / 44 visits; 2 selectors / 40 visits / 224 helpers / 4 stores; zero draws; actual shared-image handoff')


def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_terminal.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release extension required')
    inputs=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in inputs:size+=len(native.terminal_rearrangement(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T10-r*.json')),default=0)+1;path=out/f'T10-r{n:04d}.json'
    record={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
            'archaeology':pin,'routine':'3155 followed by21D7 on one native memory image','source_weeks':1,'distinct_inputs':len(cases),
            'repetitions':repetitions,'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,'rng_draws':0,
            'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native helpers/intermediate observations and serialization included; preparation/output decoding excluded',
            'limits':'Two terminal helper calls in r0017; no A94B/AEB8/earlier rearrangement/full AI/DD02/match/week/render/scratch/timer parity or emulator speed comparison'}
    with path.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))


def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T10');ap.add_argument('--repetitions',type=int,default=20);args=ap.parse_args()
    if args.write:
        cases,_=project();data=''.join(encoded(c)+'\n' for c in cases);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    cases,pin=load_cases();check(cases)
    if args.benchmark:
        if args.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(cases,pin,args.repetitions,args.out)
if __name__=='__main__':main()
