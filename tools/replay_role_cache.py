#!/usr/bin/env python3
"""Project pinned original T33 role-cache continuations; no selection/scoring formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/role_cache'
SOURCE=ROOT/'engine/tests/fixtures/role_scores'
sha,same,encoded=common.sha,common.same,common.encoded


def state(raw,args):
    context=raw['context'];outer=context['context']['rebuild']['outer'];selection=outer['selection']
    lookup={p['player']:p for p in raw['lookups']};players=[]
    for p in outer['players']:
        cell=lookup[p['id']]
        if any(cell[k]!=p[k] for k in ('p8_4','p8_5','p8_7')):raise ValueError('canonical P8 aliases differ')
        table={'criterion':args[1],'player':{'id':p['id'],**{k:cell[k] for k in ('p8_4','p8_5','p8_7')}},'first':{'index':cell['first_index'],'value':cell['first_cell']},'second':{'index':cell['second_index'],'value':cell['second_cell']}}
        players.append({'id':p['id'],'club':p['p8_0'],'byte6':p['p15_6'],'byte8':p['p15_8'],'flags11':p['p15_11'],'table':table})
    return {'kind':args[0],'criterion':args[1],'side':args[2],'seed':selection['seed'],'side_offsets':selection['side_offsets'],'clubs':[c['id'] for c in selection['clubs']],
            'starts':context['criteria'],'cache':[c['players'] for c in context['cache']],'scenes':copy.deepcopy(outer['scenes']),'players':players,
            'rosters':copy.deepcopy(raw['club_rosters']),'bonus':raw['bonus'],'criterion_cell':raw['criterion_cell']}


def normalize(value):
    if isinstance(value,dict):return {k:0 if k in ('draw_start','draw_end','draw') else normalize(v) for k,v in value.items()}
    if isinstance(value,list):return [normalize(v) for v in value]
    return value


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(SOURCE/'index.json').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['source_fixture']!='role_scores' or pin['index_sha256']!=sha(data):raise ValueError('role-cache source pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['draws'])!=('role-scoring-corpus/1',1,8,0) or len(index['records'])!=1:raise ValueError('role-cache corpus differs')
    meta=index['records'][0]
    if meta['cases']!='T33-r0001.jsonl':raise ValueError('role-cache source path differs')
    data=(SOURCE/meta['cases']).read_bytes()
    if sha(data)!=meta['cases_sha256']:raise ValueError('original role-cache fixture differs')
    roots=[json.loads(s) for s in data.splitlines()];cases=[];counts={}
    for c in roots:
        if c['schema']!='role-scoring-case/1' or c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025):raise ValueError('role root identity/span differs')
        for v in c['visits']:
            if v['scene_input']!=v['scene_output'] or (v['draw_start'],v['draw_end'])!=(4025,4025):raise ValueError('role visit read-only/span differs')
            for h in v['helpers']:
                counts[h['kind']]=counts.get(h['kind'],0)+1
                if (h['draw_start'],h['draw_end'])!=(4025,4025):raise ValueError('role helper span differs')
                for child in h['children']:
                    counts[child['kind']]=counts.get(child['kind'],0)+1
                    if (child['draw_start'],child['draw_end'])!=(4025,4025):raise ValueError('table span differs')
        inp=state(c['input'],c['arguments']);out=state(c['output'],c['arguments'])
        cases.append({'schema':'native-role-cache-case/1','week':c['week'],'step':c['step'],'replacement':c['replacement'],'event':c['event'],
                      'input':inp,'output':{'output':out,'clear':normalize(c['clear']),'visits':normalize(c['visits']),'draws':[]}})
    if len(cases)!=8 or counts!={'available':82,'slot':74,'old-score':74,'new-score':74,'table':148} or sum(v['stored'] for c in roots for v in c['visits'])!=27:raise ValueError('original role-cache census differs')
    return cases,pin


def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('role-cache projection differs')
    return cases,pin


def check(cases):
    import onenil_engine as native
    for c in cases:
        got=json.loads(native.role_cache_selection(encoded(c['input'])))
        if not same(got,c['output']):raise ValueError(f"role cache replacement{c['replacement']}/kind{c['input']['kind']} differs")
    for field in ('output','expected','visits','scores','selected'):
        bad=dict(cases[0]['input'],**{field:0})
        try:native.role_cache_selection(encoded(bad))
        except ValueError:pass
        else:raise ValueError('role cache accepted expected/intermediate output')
    print('Rust role cache: 8 original roots / 82 visits / 74 slot lookups / 148 scene/table pairs / 27 stores; ordered actual native helper composition,zero draws')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_role_cache.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.role_cache_selection(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T13-r*.json')),default=0)+1;path=out/f'T13-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'1455 role selection with actual8973/1AE2/9F6E','source_weeks':1,'recorded_roots':8,'recorded_visits':82,'recorded_slot_calls':74,'recorded_scene_calls':148,'recorded_table_calls':148,'recorded_stores':27,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Raw P15+6 restricted0–31;static/synthetic edges separate from original;no1539/164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))



def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T13');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        cases,_=project();data=''.join(encoded(c)+'\n' for c in cases);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing role projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    cases,pin=load_cases();check(cases)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(cases,pin,a.repetitions,a.out)
if __name__=='__main__':main()
