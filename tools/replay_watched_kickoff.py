#!/usr/bin/env python3
"""Original56EE kickoff root observations;Rust alone computes readiness/launch."""
import argparse,hashlib,json,sys,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_kickoff'
STATIC=tuple('static/1-0-exe/code/seg_'+n+'.asm' for n in ('0AB9_01','0AB9_03','0AB9_04','0AB9_09','1998_01','26C9_01','24BC_01'))
def load():
    cases=[];pins=[]
    for prefix in ('',):
        pin=json.loads((FIXTURES/(prefix+'provenance.json')).read_text())
        raw=(FIXTURES/(prefix+'cases.jsonl')).read_bytes()
        if common.sha(raw)!=pin['cases_sha256']:raise ValueError('kickoff original cases digest differs')
        capture_bytes=(ROOT/pin['capture']).read_bytes()
        if common.sha(capture_bytes)!=pin['capture_sha256']:raise ValueError('kickoff capture digest differs')
        capture_record=json.loads(capture_bytes)
        if capture_record['status']!='completed' or not capture_record['first_tick_join'] or capture_record['source']!=pin['source']:raise ValueError('kickoff capture provenance differs')
        rows=[json.loads(line) for line in raw.splitlines()]
        if not rows or [c['case'] for c in rows]!=list(range(len(rows))):raise ValueError('original kickoff call stream differs')
        if not capture_record.get('first_active_tick'):raise ValueError('complete admission boundary missing')
        if not common.same(rows,capture_record['cases']):raise ValueError('kickoff capture/export differs')
        cases.extend(rows);pins.append(pin)
    return cases,pins

def check(cases):
    import onenil_engine as native
    for number,c in enumerate(cases):
        got=json.loads(native.watched_kickoff(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original kickoff case{number} differs')
    print(f'Native watched kickoff:{len(cases)} exact original readiness/scenes/ball/seed/ordered RNG cases;standalone kernel,not a watched match.')

def capture(research_root):
    if common.git('status','--porcelain','--','tools/replay_watched_kickoff.py'):raise ValueError('commit observer before capture')
    pin_name='provenance.json'
    case_name='cases.jsonl'
    if (FIXTURES/pin_name).exists():load();print('original kickoff capture unchanged');return
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
        if ticks>reference['ticks']:raise ValueError('kickoff capture exceeded retained original prefix')
        if ticks%25==0:print(f'kickoff admission tick{ticks}/roots{len(cases)}/draws{week.draws}',flush=True)
        if ticks>1 and g.rd(0xD0B8,1)!=0:raise Complete()
        if ticks==1:
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('kickoff reached different watched admission')
            joined=True
    def snapshot(g,index):
        scenes=[scene(g,i) for i in range(1,23)];p15=g.far(0xD7BE);p8=g.far(0xD7BA)
        ids=list(dict.fromkeys(r['word14'] for r in scenes));actor=scenes[index-1];pid=actor['word14']
        attrs={'id':pid,**{f'p8_{k}':g.mem(p8+8*pid+k,1)[0] for k in (4,5,7)}}
        criteria=[]
        for kind in (6,7):
            criterion=g.rd(0xA15+kind,1);first=attrs['p8_7']*22+criterion;value=g.rd((0x957D+first)&65535,1);second=attrs['p8_5']*22+value
            criteria.append({'kind':kind,'criterion':criterion,'first':{'index':first,'value':value},'second':{'index':second,'value':g.rd((0x96B1+second)&65535,1)},'bonus_flag':g.rd(0x9FB+criterion,1)})
        state={'index':index,'scenes':scenes,'players':[{'id':i,'byte8':g.mem(p15+15*i+8,1)[0],'flags11':g.mem(p15+15*i+11,1)[0]} for i in ids],
               'score':{'player':attrs,'byte6':g.mem(p15+15*pid+6,1)[0],'byte8':g.mem(p15+15*pid+8,1)[0],'bonus':g.rd(0xD29,1),'criteria':criteria},
               'target':g.rd(0xABC),'phase':g.rd(0xD0AC,1),'remaining':g.rd(0xD0AE),'signs':[g.rd(0xA338+i,1) for i in range(2)],'sound':g.mem(g.far(0xD7D2)+2,1)[0],
               'gate_bounds':[g.rd(0xD12A+2*i) for i in range(2)]}
        for name,at,size in [('actor',0xD09F,1),('secondary',0xD0A0,1),('active',0xD0B8,1),('flag',0xCF53,1),('b6',0xD0B6,1),('mode',0xD0BB,1),('ball',0xCF46,2),('dx',0xCF4D,2),('dy',0xCF4F,2),('distance',0xCF51,2),('endpoint',0xD0A6,2),('seed',0xA51C,2)]:state[name]=g.rd(at,size)
        return state
    def enter(g):
        nonlocal pending
        if pending is not None:raise ValueError('nested56EE')
        address=g.stack_word(4)+16*g.stack_word(6);offset=address-g.DS*16-0xCCEF
        if offset%25 or not 1<=offset//25<=22:raise ValueError('kickoff caller scene pointer differs')
        index=offset//25
        pending={'case':len(cases),'tick':ticks,'draw_start':week.draws,'input':snapshot(g,index),'ready':None}
    def readiness(g):
        if pending is not None:pending['ready']=g.reg('ax')&255
    def leave(g):
        nonlocal pending
        if pending is None:return
        c=pending;pending=None
        output=snapshot(g,c['input']['index'])
        # Score facts are raw entry witnesses;the actor scene itself is in scenes.
        if c['ready'] is None:raise ValueError('missing original readiness return')
        draws=[e for e in week.events if e['kind']=='draw'][c['draw_start']:]
        def normalize(d):
            callers=d['callers'];site=callers[1] if callers[0]=='24BC:0D4F' else callers[2]
            return {k:d[k] for k in ('n','seed_before','seed_after','value')}|{'site':site}
        c.update(draw_end=week.draws,output={'output':output,'ready':c.pop('ready'),'draws':[normalize(d) for d in draws]});cases.append(c)
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:56EE',enter);week.g.code_hook('0AB9:56F6',readiness);week.g.code_hook('0AB9:5724',leave)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before kickoff capture boundary')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('kickoff original RNG prefix differs')
        if not cases:raise ValueError('kickoff call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T33';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-kickoff-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'first_active_tick':ticks if status=='completed' else None,'cases':cases,
            'scope':'Every original56EE root through first active tick;raw named entry inputs,separate readiness/state/RNG assertions;initial kickoff only'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/case_name).open('xb') as f:f.write(raw)
    pin={'schema':'watched-kickoff-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/pin_name).open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def benchmark(cases,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_watched_kickoff.py','pyproject.toml'):raise ValueError('commit source before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    values=[common.encoded(c['input']) for c in cases];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in values:size+=len(native.watched_kickoff(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in out.glob('T33-r*.json')),default=0)+1;path=out/f'T33-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
            'source_pins':pins,'routine':'original56EE kickoff readiness and initial2625 ball launch',
            'repetitions':repetitions,'cases':len(cases),'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
            'method':'Pre-serialized inputs;release parsing,one native image/setup,readiness/native score/RNG/ball preparation and serialization included;fixture preparation and output decoding excluded',
            'limits':'Initial inactive56EE kickoff only,not active launch/72BB full dispatch,ball/clock,complete watched ticks or detailed commentary;no full-match/emulator speed comparison'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true');p.add_argument('--out',default='records/T33');p.add_argument('--repetitions',type=int,default=100);g.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root);return
    if args.repetitions<1:p.error("positive repetitions required")
    cases,pins=load();check(cases)
    if args.benchmark:benchmark(cases,pins,args.repetitions,args.out)
if __name__=='__main__':main()
