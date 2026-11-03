#!/usr/bin/env python3
"""Original7423 root observations;Rust alone computes flight/logical pitch transport."""
import argparse,hashlib,json,sys,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_ball'
STATIC=tuple('static/1-0-exe/code/seg_'+n+'.asm' for n in ('0AB9_12','1998_01','1998_02','26C9_01'))
def load():
    cases=[];pins=[]
    for prefix in ('',):
        pin=json.loads((FIXTURES/(prefix+'provenance.json')).read_text())
        raw=(FIXTURES/(prefix+'cases.jsonl')).read_bytes()
        if common.sha(raw)!=pin['cases_sha256']:raise ValueError('ball original cases digest differs')
        capture_bytes=(ROOT/pin['capture']).read_bytes()
        if common.sha(capture_bytes)!=pin['capture_sha256']:raise ValueError('ball capture digest differs')
        capture_record=json.loads(capture_bytes)
        if capture_record['status']!='completed' or not capture_record['first_tick_join'] or capture_record['source']!=pin['source']:raise ValueError('ball capture provenance differs')
        rows=[json.loads(line) for line in raw.splitlines()]
        if len(rows)!=32 or [c['case'] for c in rows]!=list(range(32)):raise ValueError('original ball call stream differs')
        if any(c['output']['within']!=1 for c in rows):raise ValueError('original reached unported restart branch')
        if not common.same(rows,capture_record['cases']):raise ValueError('ball capture/export differs')
        cases.extend(rows);pins.append(pin)
    return cases,pins

def check(cases):
    import onenil_engine as native
    for number,c in enumerate(cases):
        got=json.loads(native.watched_advance_ball(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original ball case{number} differs')
    print(f'Native watched ball:{len(cases)} exact original ball/pitch/metric/bounds/seed/draw cases;standalone kernel,not a watched match.')

def capture(research_root):
    if common.git('status','--porcelain','--','tools/replay_watched_advance_ball.py'):raise ValueError('commit observer before capture')
    pin_name='provenance.json'
    case_name='cases.jsonl'
    if (FIXTURES/pin_name).exists():load();print('original ball capture unchanged');return
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
        if ticks>reference['ticks']:raise ValueError('ball capture exceeded retained original prefix')
        if ticks%25==0:print(f'ball admission tick{ticks}/roots{len(cases)}/draws{week.draws}',flush=True)
        if ticks==1:
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('ball reached different watched admission')
            joined=True
    def packet(g):
        return {**{f'word{k}':g.rd(0xCF46+k) for k in (0,2,7,9,11)},**{f'byte{k}':g.rd(0xCF46+k,1) for k in (4,5,6,13,14)}}
    def indexed(g,bank,coordinate):
        x,y=coordinate.to_bytes(2,'little');x=x if x<128 else x-256;y=y if y<128 else y-256
        address=g.far(0xD08A if bank=='main' else 0xD092)+160*x+2*y+0x90E
        return {'coordinate':coordinate,'occupant':g.mem(address,1)[0],'tag':g.mem(address+1,1)[0]}
    def enter(g):
        nonlocal pending
        if pending is not None:raise ValueError('nested7423')
        address=g.stack_word(4)+16*g.stack_word(6)
        if address!=g.DS*16+0xCF46:raise ValueError('non-ball7423 pointer')
        state={'ball':packet(g),'sentinel':g.rd(0xD2E),'glyphs':[g.rd(0xD70+i,1) for i in range(2)],'seed':g.rd(0xA51C),'main':[],'background':[]}
        pending={'case':len(cases),'tick':ticks,'draw_start':week.draws,'input':state,'within':None,'metrics':[]}
    def read_cell(g,bank,offset):
        if pending is None:return
        coordinate=g.rd(0xCF46+offset);address=g.reg('es')*16+g.reg('di')+0x90E
        cell={'coordinate':coordinate,'occupant':g.mem(address,1)[0],'tag':g.mem(address+1,1)[0]}
        if not common.same(cell,indexed(g,bank,coordinate)):raise ValueError('indexed pitch pointer differs')
        cells=pending['input'][bank]
        for previous in cells:
            if previous['coordinate']==coordinate:
                if previous!=cell:raise ValueError('entry witness read after same-cell write')
                return
        cells.append(cell)
    def bounds(g):
        if pending is not None:pending['within']=g.reg('ax')&255
    def metric(g):
        if pending is not None:pending['metrics'].append(g.reg('ax')&65535)
    def leave(g):
        nonlocal pending
        if pending is None:return
        c=pending;pending=None;output=json.loads(json.dumps(c['input']));output['ball']=packet(g);output['seed']=g.rd(0xA51C)
        for bank in ('main','background'):output[bank]=[indexed(g,bank,v['coordinate']) for v in c['input'][bank]]
        if c['within'] is None:raise ValueError('missing original pitch-bound result')
        draws=[e for e in week.events if e['kind']=='draw'][c['draw_start']:]
        def normalize(d):
            callers=d['callers'];site=callers[1] if callers[0]=='24BC:0D4F' else callers[2]
            return {k:d[k] for k in ('n','seed_before','seed_after','value')}|{'site':site}
        c.update(draw_end=week.draws,output={'output':output,'within':c.pop('within'),'metrics':c.pop('metrics'),'draws':[normalize(d) for d in draws]});cases.append(c)
        if len(cases)==32:raise Complete()
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:7423',enter);week.g.code_hook('0AB9:7496',bounds);week.g.code_hook('0AB9:7596',leave)
    week.g.code_hook('1998:0A49',lambda g:read_cell(g,'main',2));week.g.code_hook('1998:0A73',lambda g:read_cell(g,'background',2))
    week.g.code_hook('1998:0AEC',lambda g:read_cell(g,'background',0));week.g.code_hook('1998:0B0D',lambda g:read_cell(g,'main',0));week.g.code_hook('1998:0B72',metric)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before ball capture boundary')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('ball original RNG prefix differs')
        if len(cases)!=32:raise ValueError('ball call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T34';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-ball-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'cases':cases,
            'scope':'First32 original7423 roots from retained admission/RNG prefix;raw reached indexed main/background facts and separate state/bounds/metric/draw assertions'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/case_name).open('xb') as f:f.write(raw)
    pin={'schema':'watched-ball-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/pin_name).open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def benchmark(cases,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_watched_advance_ball.py','pyproject.toml'):raise ValueError('commit source before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    values=[common.encoded(c['input']) for c in cases];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in values:size+=len(native.watched_advance_ball(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in out.glob('T34-r*.json')),default=0)+1;path=out/f'T34-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
            'source_pins':pins,'routine':'original7423 in-bounds ball flight and0A16 logical pitch transport',
            'repetitions':repetitions,'cases':len(cases),'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
            'method':'Pre-serialized inputs;release parsing,one native image/setup,ball flight/metric/bounds/logical pitch transport and serialization included;fixture preparation and output decoding excluded',
            'limits':'In-bounds ball transport only,no outside restarts/full72BB/clock,complete watched ticks or detailed commentary;no full-match/emulator speed comparison'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true');p.add_argument('--out',default='records/T34');p.add_argument('--repetitions',type=int,default=100);g.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root);return
    if args.repetitions<1:p.error("positive repetitions required")
    cases,pins=load();check(cases)
    if args.benchmark:benchmark(cases,pins,args.repetitions,args.out)
if __name__=='__main__':main()
