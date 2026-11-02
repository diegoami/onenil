#!/usr/bin/env python3
"""Project fixed-match original selection and exact replacement joins; no selection formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
import replay_replacement as replacement
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/ai_selection'
sha,same,encoded=common.sha,common.same,common.encoded

def readset(native,context):
    # Copy original pre-selection fields into the supplemental canonical read set.
    # T25 begins after A94B's prefix;T23 supplies each actual earlier value.
    m=copy.deepcopy(native['match_state']);c=m['core'];a=c['assignment'];ps={p['id']:p for p in context['players']}
    c.update(rosters=[x['roster'] for x in context['clubs']],side_offsets=context['side_offsets'],event_totals=context['event_totals'],cache_selectors=context['cache_selectors'])
    a['seed']=context['seed'];a['clubs']=[{'id':x['id'],'roster':x['roster'][:11]} for x in context['clubs']]
    mini={r['index']:r for r in context['scenes']}
    for row in a['scenes']:row.update(word14=mini[row['index']]['player'],byte7=mini[row['index']]['slot'])
    for p in a['players']:
        q=ps[p['id']];p.update(role=q['position'],style=q['style'],hand_selector=q['hand_selector'],hand_cell=q['hand_cell'])
    for p in c['players']:
        q=ps[p['id']];p['p8'][6:8]=[q['position'],q['style']];p['p15'][8]=q['byte8'];p['p15'][11]=q['flags11']
    for name in ['pairing','codes','cache','terminal']:
        t=m[name];t['seed']=context['seed'];t['side_offsets']=copy.deepcopy(context['side_offsets']);t['scenes']=copy.deepcopy(a['scenes'])
        for p in t['players']:
            q=ps[p['id']]
            for field,key in [('role','position'),('position','position'),('p8_7','style'),('byte8','byte8'),('flags11','flags11')]:
                if field in p:p[field]=q[key]
    for i,cx in enumerate(m['pairing']['clubs']):cx['id']=context['clubs'][i]['id'];cx['roster']=copy.deepcopy(context['clubs'][i]['roster'][:11])
    m['cache']['managed_clubs']=context['managed_clubs'];m['cache']['clubs']=[x['id'] for x in context['clubs']]
    for r in m['cache']['rosters']:
        for cx in context['clubs']:
            if cx['id']==r['club']:r['roster']=copy.deepcopy(cx['roster'][:11])
    m['codes']['clock']=context['remaining'];m['terminal']['time']=context['remaining']
    return {'match_state':m,'display_mode':native['mode'],'position_sentinel':native['position_sentinel'],'speaker_gate':native['speaker_gate']}

def helper_count(es):return sum(1+helper_count(e['children']) for e in es)

def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('selection source pin differs')
    if sha((replacement.FIXTURES/'provenance.json').read_bytes())!=pin['replacement_pin']:raise ValueError('selection supplemental pin differs')
    replacements,_=replacement.load_cases();native=replacements['replacements']
    data=(FIXTURES/'index.json').read_bytes()
    if sha(data)!=pin['index_sha256']:raise ValueError('selection source index differs')
    index=json.loads(data);meta=[r for r in index['records'] if r['week']=='T04-week-r0017']
    if index['schema']!='ai-selection-corpus/1' or len(meta)!=1 or (meta[0]['calls'],meta[0]['draws'],meta[0]['cases'])!=(37,0,'T23-r0006.jsonl'):raise ValueError('selection selected source differs')
    data=(FIXTURES/meta[0]['cases']).read_bytes()
    if sha(data)!=meta[0]['cases_sha256']:raise ValueError('selection original rows differ')
    rows=[json.loads(l) for l in data.splitlines()];roots=replacement.sources(json.loads((replacement.FIXTURES/'provenance.json').read_text()))[0];cases=[]
    for call,c in enumerate(rows):
        if c['schema']!='ai-selection-case/1' or c['week']!='T04-week-r0017' or c['draw_start']!=c['draw_end']:raise ValueError('selection identity/span differs')
        inp=out=None;presentation=[];matched=[(i,r) for i,r in enumerate(roots) if r['a94b']==call]
        if matched:
            if len(matched)!=1:raise ValueError('duplicate replacement join')
            i,r=matched[0];h=[h for h in c['helpers'] if h['kind']=='replace']
            if len(h)!=1 or h[0]['arguments']!=r['arguments'] or h[0]['input']!=r['input']['selection'] or h[0]['output']!=r['output']['selection']:raise ValueError('exact T23/T25 replacement join differs')
            inp=readset(native[i]['input'],c['input']);out=readset(native[i]['output']['output'],c['output']);presentation=copy.deepcopy(native[i]['output']['presentation'])
        cases.append({'week':c['week'],'step':c['step'],'call':call,'input':{'context':copy.deepcopy(c['input']),'mode':c['mode'],'scene':c['scene'],'side':c['side'],'replacement':inp},
            'output':{'output':copy.deepcopy(c['output']),'replacement':out,'helpers':replacement.rearrangement.assignment.normalize(c['helpers']),'presentation':presentation,'draws':[]}})
    if len(cases)!=37 or sum(helper_count(c['output']['helpers']) for c in cases)!=33 or sum(c['input']['replacement'] is not None for c in cases)!=2:raise ValueError('selection projected census differs')
    return cases,pin

def load_cases():
    cases,pin=project();data=''.join(encoded(c)+'\n' for c in cases).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('selection projection differs')
    return cases,pin

def check(cases):
    import onenil_engine as native
    for c in cases:
        if not same(json.loads(native.ai_select(encoded(c['input']))),c['output']):raise ValueError(f"selection call{c['call']} differs")
    for field in ('output','expected','helpers','chosen','result','rating'):
        try:native.ai_select(encoded(dict(cases[0]['input'],**{field:0})))
        except ValueError:pass
        else:raise ValueError('selection accepted expected/intermediate input')
    print('RustA94B:37 original calls/33 observed helpers/two native replacements on one image;no chosen-slot inputs. Not a complete playable match.')

def benchmark(cases,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_ai_selection.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[encoded(c['input']) for c in cases];started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in prepared:size+=len(native.ai_select(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T21-r*.json')),default=0)+1;path=out/f'T21-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'A94B selection with nativeAEB8 replacements','source_weeks':1,'recorded_roots':37,'recorded_helpers':33,'recorded_replacements':2,'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,validated memory setup,native role/cache/slot/score computation and serialization included; preparation/output decoding excluded',
         'limits':'Fixedr0017 only;A837 unsupported;static weighted fallback separate from observed preferred path;presentation adapter excludes original hardware/render/log scratch/timer parity;not a complete match/week or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))




def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T21');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
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
