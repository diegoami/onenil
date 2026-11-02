#!/usr/bin/env python3
"""Join original substitution/selection observations; no dispatcher or selection formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
import replay_ai_selection as selection
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/substitution'
sha,same,encoded=common.sha,common.same,common.encoded

def compact(context):
    out={k:copy.deepcopy(context[k]) for k in ('seed','managed_clubs','clubs','scenes','side_offsets','remaining','added','allowance','sub_flags','reserves','pending','control','key','event_totals')}
    out['players']=[{k:p[k] for k in ('id','byte8','flags11')} for p in context['players']]
    return out

def context(raw,selection_context):
    # Observed P8/table/candidate/cache fields are carried from the adjacent T23 boundary;
    # T22 supplies every dispatcher field. No dispatcher decision or effect is modeled.
    out=copy.deepcopy(selection_context);players={p['id']:p for p in raw['players']}
    for p in out['players']:p.update(players[p['id']])
    for k,v in raw.items():
        if k!='players':out[k]=copy.deepcopy(v)
    if compact(out)!=raw:raise ValueError('substitution compact aliases differ')
    return out

def readset(raw,ctx):
    native={'match_state':raw['match_state'],'mode':raw['display_mode'],'position_sentinel':raw['position_sentinel'],'speaker_gate':raw['speaker_gate']}
    return selection.readset(native,ctx)

def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('substitution source pin differs')
    if sha((selection.FIXTURES/'provenance.json').read_bytes())!=pin['selection_pin']:raise ValueError('substitution selection pin differs')
    ai,_=selection.load_cases();data=(FIXTURES/'index.json').read_bytes()
    if sha(data)!=pin['index_sha256']:raise ValueError('substitution source index differs')
    index=json.loads(data);meta=[r for r in index['records'] if r['week']=='T04-week-r0017']
    if index['schema']!='substitution-corpus/1' or len(meta)!=1 or(meta[0]['calls'],meta[0]['draws'],meta[0]['cases'])!=(91,0,'T22-r0006.jsonl'):raise ValueError('substitution selected source differs')
    data=(FIXTURES/meta[0]['cases']).read_bytes()
    if sha(data)!=meta[0]['cases_sha256']:raise ValueError('substitution original rows differ')
    roots=[json.loads(l) for l in data.splitlines()];cases=[];cursor=0
    for root in roots:
        if root['schema']!='substitution-case/1' or root['week']!='T04-week-r0017' or root['draw_start']!=root['draw_end'] or root['draws']!=[]:raise ValueError('substitution original identity/span differs')
        prior=[c for c in ai if c['step']<root['step']];current=[c for c in ai if c['step']<=root['step']]
        before=prior[-1]['output']['output'] if prior else ai[0]['input']['context'];after=current[-1]['output']['output'] if current else ai[0]['input']['context']
        inp=context(root['input'],before);out=context(root['output'],after);events=[];calls=[];presentation=[]
        replacements=[c for c in ai if c['step']==root['step'] and c['input']['replacement'] is not None];rinput=routput=None
        if replacements:rinput=readset(replacements[0]['input']['replacement'],inp);routput=readset(replacements[-1]['output']['replacement'],out)
        for event,e in enumerate(root['events']):
            ev=copy.deepcopy(e)
            if e['kind']=='ai':
                c=ai[cursor];cursor+=1
                if c['step']!=root['step'] or e['arguments']!=[c['input']['mode'],c['input']['side'],c['input']['scene'],c['input']['context']['clubs'][c['input']['side']]['id']] or e['input']!=compact(c['input']['context']) or e['output']!=compact(c['output']['output']):raise ValueError('exact T22/T23 AI join differs')
                ev=selection.replacement.rearrangement.assignment.normalize(ev)
                calls.append({'event':event,'input':copy.deepcopy(c['input']['context']),**{k:copy.deepcopy(c['output'][k]) for k in ('output','helpers','presentation','draws')}})
                presentation.extend(copy.deepcopy(c['output']['presentation']))
            elif e['kind'] not in ('managed','keeper','capacity'):raise ValueError('unobserved UI path in fixed reference')
            events.append(ev)
        cases.append({'week':root['week'],'step':root['step'],'input':{'context':inp,'replacement':rinput},
            'output':{'output':out,'replacement':routput,'events':events,'ai':calls,'presentation':presentation,'draws':[]}})
    if len(cases)!=91 or cursor!=37 or sum(len(c['output']['events']) for c in cases)!=401 or sum(c['input']['replacement'] is not None for c in cases)!=1:raise ValueError('substitution projected census differs')
    return cases,pin

def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('substitution projection differs')
    return cases,pin

def check(cases):
    import onenil_engine as native
    for c in cases:
        if not same(json.loads(native.substitutions(encoded(c['input']))),c['output']):raise ValueError(f"substitution step{c['step']} differs")
    for field in ('output','expected','events','ai','chosen','result'):
        try:native.substitutions(encoded(dict(cases[0]['input'],**{field:0})))
        except ValueError:pass
        else:raise ValueError('substitution accepted expected/intermediate inputs')
    print('Rust6B2F:91 original dispatchers/401 direct events/37 native A94B continuations;two replacements on one parent image;no prompt/editor reach. Not a continuous match.')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_substitution.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.substitutions(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T22-r*.json')),default=0)+1;path=out/f'T22-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'6B2F substitution dispatcher with nativeA94B/AEB8','source_weeks':1,'recorded_roots':91,'recorded_events':401,'recorded_ai':37,'recorded_replacements':2,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Fixedr0017 dispatcher only;managed prompt/editor unsupported when reached;no78B7/continuousDD02/match/week or emulator comparison;UI/render/timer/scratch parity excluded'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T22');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
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
