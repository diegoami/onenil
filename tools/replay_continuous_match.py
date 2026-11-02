#!/usr/bin/env python3
"""Admit original starting data once;compare native full-match steps and caller effects."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_continuous_step as half
import replay_rearrangement as rearrangement
import replay_role_pairing as pairing
import replay_scene_codes as codes
import replay_player_cache as cache
import replay_terminal as terminal
common=half.common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/full_match'

def table_rows(skills):
    return [{'criterion':s['criterion'],'player':{'id':p['player'],**{k:p[k] for k in ('p8_4','p8_5','p8_7')}},'first':{'index':p['first_index'],'value':p['first_cell']},'second':{'index':p['second_index'],'value':p['second_cell']}} for s in skills for p in s['state']['lookups']]

def normalize(s):
    raw=s['pairing']['context'];g=raw['rebuild']['outer']['globals']
    return {'core':rearrangement.core(raw),'pairing':pairing.state(s['pairing']),'codes':codes.state(s['codes']),'selection':raw['rebuild']['outer']['selection'],'global_read_set':{'display_mode':g['mode'],'position_sentinel':g['position_sentinel'],'speaker_gate':g['speaker_gate']},'gates':s['gates'],'ratings':s['ratings'],'seed':s['seed']}

def load():
    first,steps,oldpin=half.load();pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!=oldpin['repository'] or pin['first_half_pin_sha256']!=common.sha((half.FIXTURES/'provenance.json').read_bytes()):raise ValueError('full-match source identity differs')
    for name,sha in pin['sources'].items():
        if common.sha((FIXTURES/name).read_bytes())!=sha:raise ValueError('full-match source digest differs')
    rows=[json.loads(l) for l in (FIXTURES/'T37-r0001.jsonl').read_text().splitlines()]
    if rows[0]['schema']!='caller-admission/1':raise ValueError('initial caller supplement missing')
    initial=rows[0]['state'];a=json.loads((half.FIXTURES/'T36-r0001.jsonl').read_text().splitlines()[0])['state'];raw=initial['pairing']['context']
    if raw!=a['positions']['context']['context'] or initial['codes']['context']!=raw:raise ValueError('initial T36/T37 context join differs')
    wanted=set(a['positions']['context']['criteria'][2:]);skills=[s for s in a['skills'] if s['criterion'] in wanted]
    if {s['criterion'] for s in skills}!=wanted:raise ValueError('initial role lookup criteria incomplete')
    ref=skills[0]['state']
    if any(s['state']['bonus']!=ref['bonus'] or s['state']['club_rosters']!=ref['club_rosters'] for s in skills):raise ValueError('initial role read-set aliases differ')
    extra={'bonus':ref['bonus'],'rosters':ref['club_rosters'],'tables':sorted(table_rows(skills),key=lambda t:(t['criterion'],t['player']['id'])),'criterion_cells':[{'criterion':s['criterion'],'value':s['state']['criterion_cell']} for s in skills]}
    state={'core':rearrangement.core(raw),'pairing':pairing.state(initial['pairing']),'codes':codes.state(initial['codes']),
           'cache':cache.state(a['positions']['context'],extra),'terminal':terminal.state({'context':raw},a['positions']['positions'],a['proximity'])}
    g=raw['rebuild']['outer']['globals']
    readset={'match_state':state,'display_mode':g['mode'],'position_sentinel':g['position_sentinel'],'speaker_gate':g['speaker_gate']}
    admission={'first_half':first,'replacement':readset,'gates':initial['gates'],'skill_tables':table_rows([s for s in a['skills'] if s['criterion'] in (6,14)])}
    if not common.same(admission,json.loads((FIXTURES/'admission.json').read_text())):raise ValueError('initial full-match admission projection differs')
    phases=[r for r in rows if r['schema']=='caller-phase/1']
    if [r['site'] for r in phases]!=['0AB9:7DD3','0AB9:7E4C','0AB9:7E4F','0AB9:7E6E','0AB9:7E71']:raise ValueError('caller continuation coverage differs')
    pin['first_half']=oldpin
    return admission,steps,phases,pin

def check(admission,steps,phases):
    import onenil_engine as native
    session=native.NativeMatchSession(common.encoded(admission))
    if not common.same(json.loads(session.snapshot()),half.normalized(steps[0]['input'])):raise ValueError('full-match initial admission differs')
    draws=[json.loads(l) for l in (half.FIXTURES/'draws.jsonl').read_text().splitlines()];count=0;ai=[]
    while session.phase()!='full-time':
        if session.phase()=='half-time':session.continue_without_changes()
        got=json.loads(session.step());want=steps[count]
        for k in ('input','output'):
            if not common.same(got[k],half.normalized(want[k])):raise ValueError(f'full-match step{count} {k} differs')
        if not common.same(got['clock'],{k:want['clock'][k] for k in ('output','event','result')}) or not common.same(got['clock_input'],want['clock']['input']):raise ValueError(f'full-match step{count} clock differs')
        if not common.same(got['draws'],[{k:v for k,v in d.items() if k!='i'} for d in draws[want['draw_start']:want['clock']['draw_end']]]):raise ValueError(f'full-match step{count} RNG differs')
        ai.extend(got['ai']);count+=1
        if count>len(steps):raise ValueError('native match failed to terminate')
    if count!=95:raise ValueError('original full-match boundary count differs')
    replacements=[h['arguments'] for c in ai for h in c['helpers'] if h['kind']=='replace']
    if len(ai)!=37 or replacements!=[[14,8,1],[13,6,1]]:raise ValueError('actual native AI/replacement census differs')
    actual=json.loads(session.caller_points())
    if not common.same(actual,[normalize(p['state']) for p in phases]):raise ValueError('actual halftime/caller projections differ')
    result=json.loads(session.result())
    if result!={'clubs':steps[-1]['output']['clubs'],'goals':steps[-1]['output']['score_side']}:raise ValueError('original final result differs (verification only)')
    print('One admitted native match:95 exact DD02/clock/RNG boundaries,actual halftime,1-3 original final result;after setup,not yet playable.')

def benchmark(admission,pin,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_continuous_match.py','pyproject.toml'):raise ValueError('commit source before measuring')
    if native.__debug_assertions__:raise ValueError('release extension required')
    value=common.encoded(admission);started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();steps=0;size=0
    for _ in range(repetitions):
        session=native.NativeMatchSession(value)
        while True:
            if session.phase()=='half-time':session.continue_without_changes()
            result=session.step();size+=len(result);steps+=1
            if session.phase()=='full-time':break
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T26-r*.json')),default=0)+1;path=out/f'T26-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
         'source_pins':pin,'routine':'one admitted continuing DD02/7B7B full-match session','repetitions':repetitions,'steps':steps,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized initial admission;release parse/setup/native successive steps/serialization and phase queries for termination included',
         'limits':'Fixed reference full match after setup with explicit no-edit halftime;not native setup,playable acceptance,arbitrary seeds or emulator/full-game speed comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    ap=argparse.ArgumentParser(description=__doc__);g=ap.add_mutually_exclusive_group(required=True)
    g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true')
    ap.add_argument('--out',default='records/T26');ap.add_argument('--repetitions',type=int,default=10);args=ap.parse_args()
    if args.repetitions<1:raise ValueError('positive repetitions required')
    a,s,p,pin=load();check(a,s,p)
    if args.benchmark:benchmark(a,pin,args.repetitions,args.out)
if __name__=='__main__':main()
