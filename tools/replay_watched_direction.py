#!/usr/bin/env python3
"""Original watched70D5 inputs/assertions;Rust alone computes direction legality."""
import argparse,hashlib,json,sys,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_direction'
STATIC=('static/1-0-exe/code/seg_0AB9_12.asm','static/1-0-exe/code/seg_1998_01.asm','static/1-0-exe/code/seg_26C9_02.asm')
def load():
    cases=[];pins=[]
    for prefix in ('','active-'):
        pin=json.loads((FIXTURES/(prefix+'provenance.json')).read_text())
        raw=(FIXTURES/(prefix+'cases.jsonl')).read_bytes()
        if common.sha(raw)!=pin['cases_sha256']:raise ValueError('direction original cases digest differs')
        capture_bytes=(ROOT/pin['capture']).read_bytes()
        if common.sha(capture_bytes)!=pin['capture_sha256']:raise ValueError('direction capture digest differs')
        capture_record=json.loads(capture_bytes)
        if capture_record['status']!='completed' or not capture_record['first_tick_join'] or capture_record['source']!=pin['source']:raise ValueError('direction capture provenance differs')
        rows=[json.loads(line) for line in raw.splitlines()]
        if len(rows)!=64 or [c['case'] for c in rows]!=list(range(64)):raise ValueError('64 original direction calls per stream required')
        if not common.same(rows,capture_record['cases']):raise ValueError('direction capture/export differs')
        if prefix and any(c['input']['active']==0 for c in rows):raise ValueError('active stream contains inactive calls')
        cases.extend(rows);pins.append(pin)
    return cases,pins

def check(cases):
    import onenil_engine as native
    for number,c in enumerate(cases):
        got=json.loads(native.watched_choose_direction(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original direction case{number} differs')
    print(f'Native watched direction:{len(cases)} exact original scene/seed/ordered RNG/attempt cases;standalone kernel,not a watched match.')

def capture(research_root,active_only=False):
    if common.git('status','--porcelain','--','tools/replay_watched_direction.py'):raise ValueError('commit observer before capture')
    pin_name='active-provenance.json' if active_only else 'provenance.json'
    case_name='active-cases.jsonl' if active_only else 'cases.jsonl'
    if (FIXTURES/pin_name).exists():load();print('original direction capture unchanged');return
    root=research_root.resolve();sys.path.insert(0,str(root/'runtime/emul'))
    import watched_actions as observer
    source=observer.check();record=json.loads((root/source['record']).read_text());reference=observer.setup.contests.outputs(record)
    _,data=observer.match.find_state(record['start_state']['sha256']);week=observer.match.Week(data)
    pending=None;cases=[];ticks=0;joined=False
    class Complete(Exception):pass
    def scene(g,index):
        a=0xCCEF+25*index
        return {'index':index,**{f'word{k}':g.rd(a+k) for k in (0,2,8,10,12,14,20,23)},**{f'byte{k}':g.rd(a+k,1) for k in (4,5,6,7,16,17,18,19,22)}}
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def tick(g):
        nonlocal ticks,joined
        ticks+=1
        if ticks==1:
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('direction reached different watched admission')
            joined=True
    def enter(g):
        nonlocal pending
        if active_only and g.rd(0xD0B8,1)==0:return
        if pending is not None:raise ValueError('nested70D5')
        ss=g.reg('ss')*16;owner=g.stack_word(2)
        address=g.word_at(ss+owner+6)+16*g.word_at(ss+owner+8)
        offset=address-g.DS*16-0xCCEF
        if offset%25 or not 1<=offset//25<=22:raise ValueError('direction caller scene pointer differs')
        index=offset//25;s=scene(g,index);role=g.mem(g.far(0xD7BA)+8*s['word14']+6,1)[0]
        state={'scene':s,'active':g.rd(0xD0B8,1),'actor':g.rd(0xD09F,1),'secondary':g.rd(0xD0A0,1),
               'role':role,'blocking_mask':[g.rd(0xD36+i,1) for i in range(32)],'cells':[],'seed':g.rd(0xA51C),'flag':g.rd(0xCF53,1),'ball_y':g.rd(0xCF47,1),'phase':g.rd(0xD0AC,1),'remaining':g.rd(0xD0AE),'direction_signs':[g.rd(0xA338+i,1) for i in range(2)],'line_bytes':[g.rd(0xCF2F+7*i,1) for i in range(2)]}
        pending={'case':len(cases),'tick':ticks,'draw_start':week.draws,'input':state,'attempts':[]}
    def cell(g):
        if pending is None:return
        index=pending['input']['scene']['index'];address=g.reg('es')*16+g.reg('di')+0x90E
        witness={'coordinate':g.rd(0xCCEF+25*index),'occupant':g.mem(address,1)[0],'tag':g.mem(address+1,1)[0]}
        cells=pending['input']['cells']
        for previous in cells:
            if previous['coordinate']==witness['coordinate']:
                if previous!=witness:raise ValueError('mutable indexed occupancy within direction call')
                return
        cells.append(witness)
    def attempt(g):
        if pending is None:return
        at=0xCCEF+25*pending['input']['scene']['index']
        pending['attempts'].append({'direction':g.rd(at+16,1),'coordinate':g.rd(at),'allowed':g.reg('ax')&255})
    def leave(g):
        nonlocal pending
        if pending is None:return
        c=pending;pending=None;output=json.loads(json.dumps(c['input']));output['scene']=scene(g,output['scene']['index']);output['seed']=g.rd(0xA51C)
        draws=[e for e in week.events if e['kind']=='draw'][c['draw_start']:]
        def normalize(d):
            callers=d['callers'];site=callers[1] if callers[0]=='24BC:0D4F' else callers[2]
            return {k:d[k] for k in ('n','seed_before','seed_after','value')}|{'site':site}
        c.update(draw_end=week.draws,output={'output':output,'draws':[normalize(d) for d in draws],'attempts':c.pop('attempts')});cases.append(c)
        if len(cases)==64:raise Complete()
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:70D5',enter);week.g.code_hook('0AB9:7082',cell);week.g.code_hook('0AB9:70D1',attempt);week.g.code_hook('0AB9:72B7',leave)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before64 direction calls')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('direction original RNG prefix differs')
        if len(cases)!=64:raise ValueError('direction call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T31';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-direction-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'active_only':active_only,'cases':cases,
            'scope':'64 original70D5 root calls composed with6F98 from watched admission/RNG prefix;raw indexed occupancy inputs,assertions separate;not a complete watched simulation'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/case_name).open('xb') as f:f.write(raw)
    pin={'schema':'watched-direction-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/pin_name).open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def benchmark(cases,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_watched_direction.py','pyproject.toml'):raise ValueError('commit source before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    values=[common.encoded(c['input']) for c in cases];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in values:size+=len(native.watched_choose_direction(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in out.glob('T31-r*.json')),default=0)+1;path=out/f'T31-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
            'source_pins':pins,'routine':'original70D5 direction selection and6F98 movement retries',
            'repetitions':repetitions,'cases':len(cases),'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
            'method':'Pre-serialized inputs;release parsing,one native image/setup,direction/bounds/occupancy and serialization included;fixture preparation and output decoding excluded',
            'limits':'Standalone direction/retry composition,not72BB action dispatch,ball/clock,complete watched ticks or detailed commentary;no full-match/emulator speed comparison'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true');p.add_argument('--out',default='records/T31');p.add_argument('--repetitions',type=int,default=100);g.add_argument('--capture',action='store_true');g.add_argument('--capture-active',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture or args.capture_active:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root,args.capture_active);return
    if args.repetitions<1:p.error("positive repetitions required")
    cases,pins=load();check(cases)
    if args.benchmark:benchmark(cases,pins,args.repetitions,args.out)
if __name__=='__main__':main()
