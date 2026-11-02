#!/usr/bin/env python3
"""Project original replacement/rating observations and exact read-set joins; no engine formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
import replay_rearrangement as rearrangement
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/replacement'
sha,same,encoded=common.sha,common.same,common.encoded

def observation(raw,context):
    ctx=copy.deepcopy(context);ctx['rebuild']['outer']=copy.deepcopy(raw);s=raw['selection'];g=raw['globals']
    return {'core':rearrangement.core(ctx),'allowance':s['allowance'],'sub_flags':s['sub_flags'],'reserves':s['reserves'],
        'mode':g['mode'],'position_sentinel':g['position_sentinel'],'speaker_gate':g['speaker_gate']}

def state(raw,context,extra,args,final=False):
    ctx=copy.deepcopy(context);ctx['rebuild']['outer']=copy.deepcopy(raw)
    o=observation(raw,context);o.pop('core')
    return {'match_state':rearrangement.state(ctx,extra,final),**o,'incoming':args[0],'index':args[1],'side':args[2]}

def sources(pin):
    if sha((ROOT/'engine/tests/fixtures/rearrangement/provenance.json').read_bytes())!=pin['rearrangement_pin']:raise ValueError('replacement supplemental pin differs')
    rearrangement.load_cases()
    for task in ['T25','T27']:
        if sha((FIXTURES/task/'index.json').read_bytes())!=pin['indices'][task]:raise ValueError('replacement source index differs')
    ix=json.loads((FIXTURES/'T25/index.json').read_text());m=[m for m in ix['records'] if m['week']=='T04-week-r0017']
    if ix['schema']!='replacement-corpus/1' or len(m)!=1 or (m[0]['calls'],m[0]['draws'],m[0]['cases'])!=(2,0,'T25-r0006.jsonl'):raise ValueError('replacement selected source differs')
    data=(FIXTURES/'T25'/m[0]['cases']).read_bytes()
    if sha(data)!=m[0]['cases_sha256']:raise ValueError('replacement source cases differ')
    roots=[json.loads(l) for l in data.splitlines()]
    ix=json.loads((FIXTURES/'T27/index.json').read_text());data=(FIXTURES/'T27/cases.jsonl').read_bytes()
    if ix['schema']!='rating-flags-corpus/1' or ix['count']!=25 or ix['t25_index_sha256']!=pin['indices']['T25'] or sha(data)!=ix['cases_sha256']:raise ValueError('rating-flag source differs')
    return roots,[json.loads(l) for l in data.splitlines()]

def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed':raise ValueError('replacement provenance differs')
    roots,flags=sources(pin);r=rearrangement;rr=r.raw_rows('scene_assignment/T28','T28-r0001.jsonl')
    pp=r.raw_rows('role_pairing','T29-r0002.jsonl');cc=r.raw_rows('scene_codes','T30-r0001.jsonl');kk=r.raw_rows('cache_helpers','T31-r0001.jsonl');roles=r.raw_rows('role_scores','T33-r0001.jsonl');(aa,bb),_=r.terminal.sources()
    streams={'flags':[],'replacements':[]}
    for c in flags:
        if c['schema']!='rating-flags-case/1' or c['draw_start']!=c['draw_end']:raise ValueError('rating case schema/span differs')
        streams['flags'].append({**{key:c[key] for key in ('week','step','replacement','helper')},'input':{'player':c['player'],**c['input']},'output':{'flags10':c['output']['flags10'],'draws':[]}})
    for root,p,pr,cr,kr,a,b in zip(roots,rr,pp,cc,kk,aa,bb):
        if root['schema']!='replacement-case/1' or root['arguments']!=p['arguments'] or root['step']!=p['step'] or root['week']!=p['week'] or (root['draw_start'],root['draw_end'])!=(4025,4025):raise ValueError('replacement identity/span join differs')
        closure,_=r.cache.closure(kr,[x for x in roles if all(x[key]==p[key] for key in ('week','step','replacement'))]);extra=pr,cr,kr,a,b,closure
        refresh=[h for h in root['helpers'] if h['site']=='0AB9:B0C6']
        if len(refresh)!=1 or refresh[0]['input']!=p['input']['rebuild']['outer'] or refresh[0]['output']!=p['output']['rebuild']['outer']:raise ValueError('exact replacement/rearrangement join differs')
        helpers=[];presentation=[]
        for h in root['helpers']:
            if (h['draw_start'],h['draw_end'])!=(4025,4025):raise ValueError('replacement helper span differs')
            item=r.assignment.normalize(copy.deepcopy(h));item.update(input=observation(h['input'],p['input']),output=observation(h['output'],p['output'] if h['site']=='0AB9:B0C6' else p['input']));helpers.append(item)
            kind={'0AB9:AF01':'speaker-off','0AB9:AF5F':'replacement-message','0AB9:AF7F':'replacement-log'}.get(h['site'])
            if kind:presentation.append({'site':h['site'],'kind':kind,'arguments':copy.deepcopy(h['arguments'])})
        streams['replacements'].append({**{key:p[key] for key in ('week','step','replacement')},'input':state(root['input'],p['input'],extra,root['arguments']),
            'output':{'output':state(root['output'],p['output'],extra,root['arguments'],True),'helpers':helpers,'presentation':presentation,'draws':[]}})
    if len(streams['flags'])!=25 or len(streams['replacements'])!=2 or sum(len(c['output']['helpers']) for c in streams['replacements'])!=26:raise ValueError('replacement projection census differs')
    return streams,pin

def load_cases():
    streams,pin=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('replacement projection differs')
    return streams,pin

def methods():
    import onenil_engine as native
    return [('flags',native.rating_flags),('replacements',native.replace_player)]

def check(streams):
    for name,method in methods():
        for c in streams[name]:
            if not same(json.loads(method(encoded(c['input']))),c['output']):raise ValueError(f"replacement {name}/{c['replacement']} differs")
        for field in ('output','expected','helpers','presentation','result','rating'):
            try:method(encoded(dict(streams[name][0]['input'],**{field:0})))
            except ValueError:pass
            else:raise ValueError('replacement accepted expected/intermediate input')
    print('RustAEB8:2 original replacements/26 helpers with8663/164F on one image;25 T27 flags;explicit presentation events;not a complete playable match.')

def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_replacement.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in methods() for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T20-r*.json')),default=0)+1;path=out/f'T20-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'logical AEB8 replacement with native8663/164F and53FB','source_weeks':1,'recorded_replacements':2,'recorded_helpers':26,'recorded_flags':25,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'Fixedr0017 logical replacement plus T27 flags;not a playable match;presentation adapter events exclude original port61/display/log scratch/timer parity;synthetic coverage separate;noA94B/full AI/match/week or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))






def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T20');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        streams,_=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing replacement projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
