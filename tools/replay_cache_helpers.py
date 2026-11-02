#!/usr/bin/env python3
"""Project pinned T31 helper and caller-store observations; no engine arithmetic."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/cache_helpers'
sha,same,encoded=common.sha,common.same,common.encoded


def outer(s):return s['context']['rebuild']['outer']

def boost_state(s,side):
    o=outer(s);selection=o['selection']
    if [c['players'][0] for c in s['cache']]!=o['cached_players']:raise ValueError('kind1 cache alias differs')
    return {'side':side,'seed':selection['seed'],'side_offsets':selection['side_offsets'],'cache':[c['players'] for c in s['cache']],
            'scenes':copy.deepcopy(o['scenes']),'players':[{'id':p['id'],'p8_1':p['p8_1'],'p8_3':p['p8_3'],'p8_5':p['p8_5']} for p in o['players']]}


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(FIXTURES/'index.json').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['index_sha256']!=sha(data):raise ValueError('cache helper source pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['draws'],index['stores'])!=('player-cache-corpus/1',1,2,0,44) or len(index['records'])!=1:raise ValueError('cache helper source census differs')
    meta=index['records'][0]
    if meta['cases']!='T31-r0001.jsonl':raise ValueError('cache helper source path differs')
    data=(FIXTURES/meta['cases']).read_bytes()
    if sha(data)!=meta['cases_sha256']:raise ValueError('original T31 fixture differs')
    roots=[json.loads(line) for line in data.splitlines()];streams={'managed':[],'slots':[],'boosts':[]}
    for c in roots:
        if c['schema']!='player-cache-case/1' or c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025):raise ValueError('T31 root identity/span differs')
        events=c['events'];base={k:c[k] for k in ('week','step','replacement')}
        for event_no,e in enumerate(events):
            if e['kind'] not in ('managed','slot'):continue
            if e['input']!=e['output'] or (e['draw_start'],e['draw_end'])!=(4025,4025):raise ValueError('cache helper read-only/span differs')
            selection=outer(e['input'])['selection']
            if e['kind']=='managed':
                if e['site']!='0AB9:1569' or e['arguments']!=[selection['clubs'][e['side']]['id']]:raise ValueError('managed argument differs')
                inp={'club':e['arguments'][0],'managed_clubs':selection['managed_clubs']};name='managed'
            else:
                kind,side=e['arguments']
                if e['site']!='0AB9:15DF' or side!=e['side']:raise ValueError('slot argument differs')
                club=selection['clubs'][side]
                inp={'side':side,'kind':kind,'club':club['id'],'roster':club['roster'][:11],'cache':[r['players'] for r in e['input']['cache']]};name='slots'
            streams[name].append(dict(base,event=event_no,input=inp,output={'output':{'value':e['result']},'draws':[]}))
        for side in (0,1):
            ratings=[(i,e) for i,e in enumerate(events) if e['kind']=='rating' and e['side']==side]
            if len(ratings)!=11 or [e['arguments'][0] for _,e in ratings]!=list(range(1,12)):raise ValueError('rating phase coverage differs')
            visits=[];first_input=None;last_output=None
            for event_no,e in ratings:
                store=events[event_no+1];slot,index,player=e['arguments'];inp=boost_state(e['input'],side)
                if e['input']!=e['output'] or player!=inp['cache'][side][0] or e['site']!='0AB9:1630':raise ValueError('rating cache/read-only boundary differs')
                if store['kind']!='store' or store['site']!='0AB9:163E' or store['side']!=side or store['arguments']!=e['arguments'] or store['input']!=inp['scenes'][index-1] or store['result'] is not None:raise ValueError('rating/store continuation differs')
                if (e['draw_start'],e['draw_end'],store['draw_start'],store['draw_end'])!=(4025,4025,4025,4025):raise ValueError('rating/store draw span differs')
                if last_output is not None and inp!=last_output:raise ValueError('rating phase state continuation differs')
                if first_input is None:first_input=inp
                last_output=boost_state(e['output'],side);last_output['scenes'][index-1]=copy.deepcopy(store['output'])
                visits.append({'site':e['site'],'store_site':store['site'],'arguments':e['arguments'],'input':copy.deepcopy(store['input']),'rating':e['result'],'output':copy.deepcopy(store['output']),'draw_start':0,'draw_end':0})
            streams['boosts'].append(dict(base,side=side,input=first_input,output={'output':last_output,'visits':visits,'draws':[]}))
    if len(roots)!=2 or {k:len(v) for k,v in streams.items()}!={'managed':4,'slots':2,'boosts':4}:raise ValueError('projected helper census differs')
    return streams,pin


def load_cases():
    streams,pin=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('cache helper projection differs')
    return streams,pin


def methods():
    import onenil_engine as native
    return [('managed',native.managed_club),('slots',native.cached_roster_slot),('boosts',native.cached_side_boost)]

def check(streams):
    for name,method in methods():
        for c in streams[name]:
            if not same(json.loads(method(encoded(c['input']))),c['output']):raise ValueError(f"cache helper {name}/replacement{c['replacement']} differs")
        for field in ('output','expected','visits','rating','result'):
            try:method(encoded(dict(streams[name][0]['input'],**{field:0})))
            except ValueError:pass
            else:raise ValueError('cache helper accepted expected/intermediate return')
    print('Rust cache helpers: 4 original managed calls / 2 first-slot lookups / 4 side phases / 44 actual ratings and byte18 stores;zero draws')

def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_cache_helpers.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in methods() for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T14-r*.json')),default=0)+1;path=out/f'T14-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'0E7A/577F helpers and1539 cached-side4EBF byte18 stores','source_weeks':1,'recorded_managed':4,'recorded_slots':2,'recorded_side_phases':4,'recorded_ratings':44,'recorded_stores':44,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'T31 helper/caller-store slice only; synthetic edge cases separate from original evidence; no1539/164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))



def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T14');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        streams,_=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing cache helper projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
