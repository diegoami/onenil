#!/usr/bin/env python3
"""Project pinned T30 scene/cache/code observations; no code-selection formulas."""
import argparse,copy,json,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/scene_codes'
sha,same,encoded=common.sha,common.same,common.encoded


def state(raw):
    context=raw['context'];o=context['rebuild']['outer'];selection=o['selection'];aliases={p['id']:p for p in selection['players']}
    if raw['clock']!=selection['remaining']:raise ValueError('scene-code clock alias differs')
    for p in o['players']:
        if p['p8_6']!=aliases[p['id']]['position']:raise ValueError('scene-code role alias differs')
    return {'seed':selection['seed'],'side_offsets':selection['side_offsets'],'actor':context['actor'],'directions':o['directions'],
            'mode':raw['mode'],'threshold':raw['threshold'],'clock':raw['clock'],'direction_signs':raw['direction_signs'],'codes':raw['codes'],
            'role_codes':raw['role_codes'],'cache':copy.deepcopy(raw['cache']),'scenes':copy.deepcopy(o['scenes']),
            'players':[{'id':p['id'],'role':p['p8_6']} for p in o['players']]}


def normalize(value):
    if isinstance(value,dict):return {k:0 if k in ('draw_start','draw_end') else normalize(v) for k,v in value.items()}
    if isinstance(value,list):return [normalize(v) for v in value]
    return value


def project():
    pin=json.loads((FIXTURES/'provenance.json').read_text());data=(FIXTURES/'index.json').read_bytes()
    if pin['repository']!='https://github.com/diegoami/onenil-archaeology' or pin['commit']!='218490d846e365babdd158d09c5ab865124e30ed' or pin['index_sha256']!=sha(data):raise ValueError('scene-code source pin differs')
    index=json.loads(data)
    if (index['schema'],index['weeks'],index['calls'],index['scenes'],index['cache'],index['codes'],index['draws'])!=('scene-codes-corpus/1',1,2,44,2,6,0) or len(index['records'])!=1:raise ValueError('scene-code source census differs')
    meta=index['records'][0]
    if meta['cases']!='T30-r0001.jsonl':raise ValueError('scene-code source path differs')
    data=(FIXTURES/meta['cases']).read_bytes()
    if sha(data)!=meta['cases_sha256']:raise ValueError('scene-code original rows differ')
    roots=[json.loads(line) for line in data.splitlines()];streams={'codes':[],'parents':[]}
    for c in roots:
        if c['schema']!='scene-codes-case/1' or c['week']!='T04-week-r0017' or (c['draw_start'],c['draw_end'])!=(4025,4025):raise ValueError('scene-code identity/span differs')
        base={k:c[k] for k in ('week','step','replacement')};events=[]
        if len(c['events'])!=23:raise ValueError('scene-code direct coverage differs')
        for e in c['events']:
            if (e['draw_start'],e['draw_end'])!=(4025,4025) or e['result'] is not None:raise ValueError('scene/cache event span differs')
            item=normalize(copy.deepcopy(e))
            if e['kind']=='cache':
                inp=state(e['input']);item.update(input=inp,output=state(e['output']))
                if len(e['calls'])!=3:raise ValueError('cache code coverage differs')
                for call_no,h in enumerate(e['calls']):
                    index,a,b=h['arguments']
                    if h['input']['index']!=index or h['output']['index']!=index or (h['draw_start'],h['draw_end'])!=(4025,4025):raise ValueError('cache code scene/span differs')
                    code_input={'scene':copy.deepcopy(h['input']),'a':a,'b':b,'threshold':inp['threshold'],'clock':inp['clock'],'direction_signs':inp['direction_signs'],'codes':inp['codes'][:2]}
                    streams['codes'].append(dict(base,call=call_no,input=code_input,output={'output':{'value':h['result'],'scene':copy.deepcopy(h['output'])},'draws':[]}))
            elif e['kind']!='scene':raise ValueError('unknown scene-code event')
            events.append(item)
        streams['parents'].append(dict(base,input=state(c['input']),output={'output':state(c['output']),'events':events,'draws':[]}))
    if len(roots)!=2 or len(streams['codes'])!=6:raise ValueError('scene-code projected census differs')
    return streams,pin


def load_cases():
    streams,pin=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows).encode()
    if sha(data)!=pin['projection_sha256'] or data!=(FIXTURES/'cases.jsonl').read_bytes():raise ValueError('scene-code projection differs')
    return streams,pin


def methods():
    import onenil_engine as native
    return [('codes',native.scene_code),('parents',native.scene_codes_refresh)]


def check(streams):
    for name,method in methods():
        for c in streams[name]:
            if not same(json.loads(method(encoded(c['input']))),c['output']):raise ValueError(f"scene-code {name}/replacement{c['replacement']} differs")
        for field in ('output','expected','events','result'):
            try:method(encoded(dict(streams[name][0]['input'],**{field:0})))
            except ValueError:pass
            else:raise ValueError('scene-code accepted expected result')
    print('Rust scene codes:2 original parents/44 scenes/2 cache boundaries/6 nested and standalone code calls;zero draws. First-playable gate1 dependency only,not a complete match.')

def benchmark(streams,pin,repetitions,directory):
    import onenil_engine as native
    from onenil_engine import onenil_engine as extension
    if common.git('status','--porcelain','--','engine','tools/replay_scene_codes.py','pyproject.toml'):raise ValueError('commit code/fixtures before measuring')
    if extension.__debug_assertions__:raise ValueError('release bridge required')
    prepared=[(method,encoded(c['input'])) for name,method in methods() for c in streams[name]]
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for method,value in prepared:size+=len(method(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('T17-r*.json')),default=0)+1;path=out/f'T17-r{n:04d}.json'
    rec={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),
         'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
         'cargo_lock_sha256':sha((ROOT/'engine/Cargo.lock').read_bytes()),'build':{'crate_version':extension.__version__,'debug_assertions':extension.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':sha(Path(extension.__file__).read_bytes())},
         'archaeology':pin,'routine':'03BB scene/cache refresh with native0143/0207/02D7/033E','source_weeks':1,'recorded_parents':2,'recorded_scenes':44,'recorded_cache_boundaries':2,'recorded_codes':6,
         'repetitions':repetitions,'calls':len(prepared)*repetitions,'rng_draws':0,'elapsed_ns':elapsed,'serialized_output_bytes':size,
         'method':'Pre-serialized raw inputs; release bridge parsing,image setup,native computation and observations/serialization included; preparation/output decoding excluded',
         'limits':'T30 scene-code slice;not a complete playable match; synthetic edge cases separate from original evidence; no164F/A94B/AEB8/full AI/match/week/render/timer/scratch parity or emulator comparison'}
    with path.open('x') as f:json.dump(rec,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))





def main():
    ap=argparse.ArgumentParser(description=__doc__);group=ap.add_mutually_exclusive_group(required=True)
    for flag in ('write','check','benchmark'):group.add_argument('--'+flag,action='store_true')
    ap.add_argument('--out',default='records/T17');ap.add_argument('--repetitions',type=int,default=20);a=ap.parse_args()
    if a.write:
        streams,_=project();data=''.join(encoded({'stream':name,**c})+'\n' for name,rows in streams.items() for c in rows);path=FIXTURES/'cases.jsonl'
        if path.exists():
            if path.read_text()!=data:raise ValueError('existing scene-code projection differs')
        else:
            with path.open('x') as f:f.write(data)
        return
    streams,pin=load_cases();check(streams)
    if a.benchmark:
        if a.repetitions<1:raise ValueError('positive repetitions required')
        benchmark(streams,pin,a.repetitions,a.out)
if __name__=='__main__':main()
