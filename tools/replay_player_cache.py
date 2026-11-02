#!/usr/bin/env python3
"""Join original T31 parents to T33 raw closure/role traces; no engine formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/player_cache'
sha,same,encoded=common.sha,common.same,common.encoded


def source(pin,task,fixture,schema,count):
    meta=pin['sources'][task];directory=ROOT/'engine/tests/fixtures'/fixture;data=(directory/'index.json').read_bytes()
    if meta['fixture']!=fixture or sha(data)!=meta['index_sha256']:raise ValueError('parent source index pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['draws'])!=(schema,1,count,0) or len(index['records'])!=1:raise ValueError('parent source census differs')
    entry=index['records'][0]
    if entry['cases']!=task+'-r0001.jsonl':raise ValueError('parent source path differs')
    data=(directory/entry['cases']).read_bytes()
    if sha(data)!=entry['cases_sha256']:raise ValueError('parent source rows differ')
    rows=[json.loads(line) for line in data.splitlines()]
    if len(rows)!=count:raise ValueError('parent source row count differs')
    return rows


def normalize(value):
    if isinstance(value,dict):return {k:0 if k in ('draw_start','draw_end','draw') else normalize(v) for k,v in value.items()}
    if isinstance(value,list):return [normalize(v) for v in value]
    return value


def outer(s):return s['context']['rebuild']['outer']

def closure(parent,roles):
    result={};tables=[];cells={};trace=[]
    for c in roles:
        if c['schema']!='role-scoring-case/1' or (c['draw_start'],c['draw_end'])!=(4025,4025):raise ValueError('joined role identity/span differs')
        e=parent['events'][c['event']]
        if e['kind']!='role' or e['arguments']!=c['arguments'] or e['input']!=c['input']['context'] or e['output']!=c['output']['context']:raise ValueError('exact T31/T33 role join differs')
        raw=c['input'];criterion=c['arguments'][1]
        if c['output']['bonus']!=raw['bonus'] or c['output']['criterion_cell']!=raw['criterion_cell'] or c['output']['lookups']!=raw['lookups'] or c['output']['club_rosters']!=raw['club_rosters']:raise ValueError('role extra raw read set changed')
        if result and (result['bonus']!=raw['bonus'] or result['rosters']!=raw['club_rosters']):raise ValueError('shared parent closure differs')
        result.update(bonus=raw['bonus'],rosters=copy.deepcopy(raw['club_rosters']))
        if criterion in cells and cells[criterion]!=raw['criterion_cell']:raise ValueError('criterion alias differs')
        cells[criterion]=raw['criterion_cell']
        for p in raw['lookups']:
            tables.append({'criterion':criterion,'player':{'id':p['player'],**{k:p[k] for k in ('p8_4','p8_5','p8_7')}},'first':{'index':p['first_index'],'value':p['first_cell']},'second':{'index':p['second_index'],'value':p['second_cell']}})
        trace.append({'event':c['event'],'arguments':c['arguments'],'clear':normalize(c['clear']),'visits':normalize(c['visits'])})
    if len(roles)!=4 or [c['arguments'][0] for c in roles]!=[2,3,4,5]:raise ValueError('parent role closure incomplete')
    tables.sort(key=lambda t:(t['criterion'],t['player']['id']))
    if len({(t['criterion'],t['player']['id']) for t in tables})!=len(tables):raise ValueError('duplicate raw table closure')
    result.update(tables=tables,criterion_cells=[{'criterion':k,'value':v} for k,v in sorted(cells.items())])
    return result,trace


def state(raw,extra):
    o=outer(raw);s=o['selection'];cache=[c['players'] for c in raw['cache']]
    if [c[0] for c in cache]!=o['cached_players']:raise ValueError('kind1 cache alias differs')
    players=[{'id':p['id'],'club':p['p8_0'],'p8_1':p['p8_1'],'p8_3':p['p8_3'],'p8_4':p['p8_4'],'p8_5':p['p8_5'],'p8_7':p['p8_7'],'byte6':p['p15_6'],'byte8':p['p15_8'],'flags11':p['p15_11']} for p in o['players']]
    ids={p['id']:p for p in players}
    for table in extra['tables']:
        p=ids[table['player']['id']]
        if any(p[k]!=table['player'][k] for k in ('p8_4','p8_5','p8_7')):raise ValueError('parent/captured table canonical aliases differ')
    rosters={r['club']:r['roster'] for r in extra['rosters']}
    if any(rosters[c['id']]!=c['roster'][:11] for c in s['clubs']):raise ValueError('parent/role roster alias differs')
    return {'seed':s['seed'],'gate':raw['gate'],'criteria':raw['criteria'],'cache':cache,'managed_clubs':s['managed_clubs'],'clubs':[c['id'] for c in s['clubs']],
            'side_offsets':s['side_offsets'],'scenes':copy.deepcopy(o['scenes']),'players':players,**copy.deepcopy(extra)}


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('player-cache source pin differs')
    parents=source(pin,'T31','cache_helpers','player-cache-corpus/1',2);roles=source(pin,'T33','role_scores','role-scoring-corpus/1',8);cases=[]
    for p in parents:
        if p['schema']!='player-cache-case/1' or p['week']!='T04-week-r0017' or (p['draw_start'],p['draw_end'])!=(4025,4025):raise ValueError('parent identity/span differs')
        matched=[c for c in roles if all(c[k]==p[k] for k in ('week','step','replacement'))]
        extra,traces=closure(p,matched);events=[]
        for e in p['events']:
            if (e['draw_start'],e['draw_end'])!=(4025,4025):raise ValueError('parent event span differs')
            event=normalize(copy.deepcopy(e))
            if e['kind']!='store':event.update(input=state(e['input'],extra),output=state(e['output'],extra))
            events.append(event)
        cases.append({**{k:p[k] for k in ('week','step','replacement')},'schema':'native-player-cache-case/1','input':state(p['input'],extra),
                      'output':{'output':state(p['output'],extra),'events':events,'roles':traces,'draws':[]}})
    if sum(len(c['output']['events']) for c in cases)!=106 or sum(len(c['output']['roles']) for c in cases)!=8:raise ValueError('parent direct/role census differs')
    return cases,pin


def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('parent projection differs')
    return cases,pin


def check(cases):
    import onenil_engine as native
    for c in cases:
        if not same(json.loads(native.player_cache_refresh(encoded(c['input']))),c['output']):raise ValueError(f"parent replacement{c['replacement']} differs")
    for field in ('output','expected','events','roles','selected','rating'):
        try:native.player_cache_refresh(encoded(dict(cases[0]['input'],**{field:0})))
        except ValueError:pass
        else:raise ValueError('parent accepted expected/intermediate result')
    print('Rust1539 parent: 2 original roots / 106 direct events / 8 role selections / 82 visits / 148 scene-table pairs / 27 role stores / 44 caller stores;one image,zero draws')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_player_cache.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.player_cache_refresh(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T15-r*.json')),default=0)+1;path=out/f'T15-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'bounded1539 parent with actual native selector/roles/rating stores','source_weeks':1,'recorded_roots':2,'recorded_direct_events':106,'recorded_roles':8,'recorded_caller_stores':44,'recorded_visits':82,'recorded_slot_calls':74,'recorded_scene_calls':148,'recorded_table_calls':148,'recorded_stores':27,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Raw P15+6 restricted0–31;static/synthetic edges separate from original;noB6BD/164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T15');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        cases,_=project();data=''.join(encoded(c)+'\n' for c in cases);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing parent projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    cases,pin=load_cases();check(cases)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(cases,pin,a.repetitions,a.out)
if __name__=='__main__':main()
