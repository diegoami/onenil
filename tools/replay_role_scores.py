#!/usr/bin/env python3
"""Project pinned original T33 score/table observations; no scoring formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/role_scores'
sha,same,encoded=common.sha,common.same,common.encoded


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(FIXTURES/'index.json').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['index_sha256']!=sha(data):raise ValueError('role score source pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['draws'])!=('role-scoring-corpus/1',1,8,0) or len(index['records'])!=1:raise ValueError('role scoring corpus differs')
    meta=index['records'][0]
    if meta['cases']!='T33-r0001.jsonl':raise ValueError('role scoring source path differs')
    data=(FIXTURES/meta['cases']).read_bytes()
    if sha(data)!=meta['cases_sha256']:raise ValueError('role scoring original fixture differs')
    roots=[json.loads(s) for s in data.splitlines()];cases=[]
    if len(roots)!=8:raise ValueError('role score root coverage differs')
    for root_no,c in enumerate(roots):
        if c['week']!='T04-week-r0017' or c['schema']!='role-scoring-case/1' or c['draw_start']!=c['draw_end']:raise ValueError('role root identity/span differs')
        state=c['input'];outer=state['context']['context']['rebuild']['outer'];players={p['id']:p for p in outer['players']};lookups={p['player']:p for p in state['lookups']}
        for visit_no,v in enumerate(c['visits']):
            for h in v['helpers']:
                if h['kind'] not in ('old-score','new-score'):continue
                if len(h['children'])!=1 or h['scene_input']!=h['scene_output']:raise ValueError('score child/read-only boundary differs')
                child=h['children'][0];cell=child['input'];player=players[h['scene_input']['word14']];criterion=c['arguments'][1]
                if cell!=lookups[player['id']] or child['arguments']!=[criterion,player['id']] or h['arguments']!=[h['scene_input']['index'],criterion] or any(player[k]!=cell[k] for k in ('p8_4','p8_5','p8_7')):raise ValueError('score raw aliases/arguments differ')
                if child['kind']!='table' or child['children']!=[] or (h['draw_start'],h['draw_end'],child['draw_start'],child['draw_end'])!=(4025,4025,4025,4025):raise ValueError('score original child/span differs')
                table={'criterion':criterion,'player':{'id':player['id'],**{k:cell[k] for k in ('p8_4','p8_5','p8_7')}},'first':{'index':cell['first_index'],'value':cell['first_cell']},'second':{'index':cell['second_index'],'value':cell['second_cell']}}
                inp={'table':table,'scene':copy.deepcopy(h['scene_input']),'byte6':player['p15_6'],'byte8':player['p15_8'],'bonus':state['bonus'],'criterion_cell':state['criterion_cell']}
                expected_table={'output':{'value':child['result']},'draws':[]}
                cases.append({'schema':'native-role-score-case/1','week':c['week'],'step':c['step'],'replacement':c['replacement'],'root':root_no,'visit':visit_no,'site':h['site'],'input':inp,'table':expected_table,
                              'output':{'output':{'value':h['result']},'table':copy.deepcopy(expected_table),'scene':copy.deepcopy(h['scene_output']),'draws':[]}})
    if len(cases)!=148 or sum(c['site']=='0AB9:14FC' for c in cases)!=74 or sum(c['site']=='0AB9:1509' for c in cases)!=74:raise ValueError('score helper census differs')
    return cases,pin


def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('role score projection differs')
    return cases,pin


def check(cases):
    import onenil_engine as native
    for c in cases:
        score=json.loads(native.role_scene_rating(encoded(c['input'])));table=json.loads(native.role_table_rating(encoded(c['input']['table'])))
        if not same(score,c['output']) or not same(table,c['table']):raise ValueError(f"role score {c['root']}/{c['visit']}/{c['site']} differs")
    for method,inp in [(native.role_scene_rating,cases[0]['input']),(native.role_table_rating,cases[0]['input']['table'])]:
        for field in ('output','expected','baseline','table_result'):
            try:method(encoded(dict(inp,**{field:0})))
            except ValueError:pass
            else:raise ValueError('score bridge accepted expected/intermediate result input')
    for raw in (32,127,128,255):
        bad=copy.deepcopy(cases[0]['input']);bad['byte6']=raw
        try:native.role_scene_rating(encoded(bad))
        except ValueError:pass
        else:raise ValueError('score bridge accepted unsupported P15+6 domain')
    print('Rust role scores: 148 original scene scores / 148 original table calls; actual table composition,read-only scenes,zero draws')


def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_role_scores.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.role_scene_rating(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T12-r*.json')),default=0)+1;path=out/f'T12-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'read-only1AE2 with actual9F6E table score','source_weeks':1,'recorded_scene_calls':148,'recorded_table_calls':148,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native table/scene computation and serialization included; preparation/output decoding excluded',
         'limits':'Raw P15+6 restricted0–31;static/synthetic edges separate from original;no1455/1539/164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))


def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T12');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
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
