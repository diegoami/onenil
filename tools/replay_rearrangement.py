#!/usr/bin/env python3
"""Join pinned original rearrangement/read sets; no sorting or match-engine formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
import replay_scene_assignment as assignment
import replay_role_pairing as pairing
import replay_scene_codes as codes
import replay_player_cache as cache
import replay_terminal as terminal
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/rearrangement'
sha,same,encoded=common.sha,common.same,common.encoded

def core(raw):
    o=raw['rebuild']['outer'];s=o['selection'];players=[]
    if [{'index':r['index'],'player':r['word14'],'slot':r['byte7']} for r in o['scenes']]!=s['scenes']:raise ValueError('scene selection aliases differ')
    for p in o['players']:
        players.append({'id':p['id'],'p8':[p[f'p8_{i}'] for i in range(8)],'p15':[p['p15_word0']&255,p['p15_word0']>>8]+[p[f'p15_{i}'] for i in range(2,15)]})
    return {'assignment':assignment.state(raw['rebuild'],1,1),'players':players,'rosters':[c['roster'] for c in s['clubs']],
        'side_offsets':s['side_offsets'],'actor':raw['actor'],'secondary':raw['secondary'],'cpu_class':raw['cpu_class'],
        'club_totals':o['club_totals'],'event_totals':s['event_totals'],'cache_selectors':s['cache_selectors'],'cached_players':o['cached_players']}

def state(raw,extra,final=False):
    p,c,k,a,b,closure=extra;o=raw['rebuild']['outer'];s=o['selection']
    pr=copy.deepcopy(p['input']);pr['context']=copy.deepcopy(raw)
    cr=copy.deepcopy(c['output' if final else 'input']);cr['context']=copy.deepcopy(raw)
    kr=copy.deepcopy(k['output' if final else 'input']);kr['context']=copy.deepcopy(raw)
    ex=copy.deepcopy(closure)
    for r in ex['rosters']:
        match=[c for c in s['clubs'] if c['id']==r['club']]
        if match:r['roster']=copy.deepcopy(match[0]['roster'][:11])
    # Terminal target/signs/clock are raw observations; positions/selectors are final only after their native callees.
    tr=copy.deepcopy(b['output' if final else 'input'])
    return {'core':core(raw),'pairing':pairing.state(pr),'codes':codes.state(cr),'cache':cache.state(kr,ex),
        'terminal':terminal.state({'context':raw},a['output' if final else 'input']['positions'],tr)}

def raw_rows(directory,filename):return [json.loads(l) for l in (ROOT/'engine/tests/fixtures'/directory/filename).read_bytes().splitlines()]

def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('rearrangement provenance differs')
    for name,want in pin['prior_pins'].items():
        if sha((ROOT/'engine/tests/fixtures'/name/'provenance.json').read_bytes())!=want:raise ValueError('rearrangement dependency pin differs')
    # Existing independent loaders recheck every original source/index and projection before joining.
    assignment.load_cases();pairing.load_cases();codes.load_cases();cache.load_cases();terminal.load_cases()
    parents=assignment.source(json.loads((assignment.FIXTURES/'provenance.json').read_text()),'T28','rearrangement-corpus/1')
    pp=raw_rows('role_pairing','T29-r0002.jsonl');cc=raw_rows('scene_codes','T30-r0001.jsonl');kk=raw_rows('cache_helpers','T31-r0001.jsonl');rr=raw_rows('role_scores','T33-r0001.jsonl')
    (aa,bb),_=terminal.sources();cases=[]
    for root,p,c,k,a,b in zip(parents,pp,cc,kk,aa,bb):
        if root['schema']!='rearrangement-case/1':raise ValueError('rearrangement root schema differs')
        if any(any(x[key]!=root[key] for key in ('week','step','replacement')) for x in (p,c,k,a,b)):raise ValueError('rearrangement identity join differs')
        tails=[e for e in root['events'] if e['kind']=='opaque']
        contexts=[(p['input']['context'],p['output']['context']),(c['input']['context'],c['output']['context']),
            (k['input']['context'],k['output']['context']),(a['input']['context']['context'],a['output']['context']['context']),
            (b['input']['context']['context'],b['output']['context']['context'])]
        if len(tails)!=5 or any((e['input'],e['output'])!=pair for e,pair in zip(tails,contexts)):raise ValueError('exact original tail boundary join differs')
        closure,_=cache.closure(k,[r for r in rr if all(r[key]==root[key] for key in ('week','step','replacement'))]);extra=p,c,k,a,b,closure
        events=[]
        for e in root['events']:
            if (e['draw_start'],e['draw_end'])!=(4025,4025):raise ValueError('rearrangement event RNG span differs')
            event=assignment.normalize(copy.deepcopy(e))
            if e['kind'] in ('assign','opaque'):event.update(input=core(e['input']),output=core(e['output']))
            events.append(event)
        cases.append({**{key:root[key] for key in ('week','step','replacement')},'input':state(root['input'],extra),
            'output':{'output':state(root['output'],extra,True),'events':events,'draws':[]}})
    if len(cases)!=2 or sum(len(c['output']['events']) for c in cases)!=164:raise ValueError('rearrangement census differs')
    return cases,pin

def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('rearrangement projection differs')
    return cases,pin

def check(cases):
    import onenil_engine as native
    for c in cases:
        if not same(json.loads(native.rearrange(encoded(c['input']))),c['output']):raise ValueError(f"rearrangement replacement{c['replacement']} differs")
    for field in ('output','expected','events','roles','quotient','result'):
        try:native.rearrange(encoded(dict(cases[0]['input'],**{field:0})))
        except ValueError:pass
        else:raise ValueError('rearrangement accepted expected/intermediate input')
    print('Rust164F:2 original parents/164 caller events/10 native tail boundaries on one image; first-playable gate1,not a complete match.')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_rearrangement.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.rearrange(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T19-r*.json')),default=0)+1;path=out/f'T19-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'164F rearrangement with assignment and five native tail stages','source_weeks':1,'recorded_roots':2,'recorded_direct_events':164,'recorded_tail_boundaries':10,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Fixedr0017 only;static/synthetic edges separate from original;noA94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T19');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
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
