#!/usr/bin/env python3
"""Original35F8 root observations;compiled Rust computes the complete helper graph."""
import argparse,hashlib,json,sys,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_adjustment'
STATIC=tuple('static/1-0-exe/code/seg_'+n+'.asm' for n in ('0AB9_03','0AB9_04','0AB9_05','0AB9_06','24BC_01','24BC_02'))
def load():
    cases=[];pins=[]
    for prefix in ('',):
        pin=json.loads((FIXTURES/(prefix+'provenance.json')).read_text())
        raw=(FIXTURES/(prefix+'cases.jsonl')).read_bytes()
        if common.sha(raw)!=pin['cases_sha256']:raise ValueError('adjustment original cases digest differs')
        capture_bytes=(ROOT/pin['capture']).read_bytes()
        if common.sha(capture_bytes)!=pin['capture_sha256']:raise ValueError('adjustment capture digest differs')
        capture_record=json.loads(capture_bytes)
        if capture_record['status']!='completed' or not capture_record['first_tick_join'] or capture_record['source']!=pin['source']:raise ValueError('adjustment capture provenance differs')
        rows=[json.loads(line) for line in raw.splitlines()]
        if len(rows)!=64 or [c['case'] for c in rows]!=list(range(64)):raise ValueError('original adjustment call stream differs')
        if not common.same(rows,capture_record['cases']):raise ValueError('adjustment capture/export differs')
        cases.extend(rows);pins.append(pin)
    return cases,pins

def check(cases):
    import onenil_engine as native
    for number,c in enumerate(cases):
        got=json.loads(native.watched_adjust_target(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original adjustment case{number} differs')
    print(f'Native watched adjustment:{len(cases)} exact original scenes/seeds/ordered draws from original35F8 calls;standalone kernel,not a watched match.')

def capture(research_root):
    if common.git('status','--porcelain','--','tools/replay_watched_adjustment.py'):raise ValueError('commit observer before capture')
    pin_name='provenance.json'
    case_name='cases.jsonl'
    if (FIXTURES/pin_name).exists():load();print('original adjustment capture unchanged');return
    root=research_root.resolve();sys.path.insert(0,str(root/'runtime/emul'))
    import watched_actions as observer
    source=observer.check();record=json.loads((root/source['record']).read_text());reference=observer.setup.contests.outputs(record)
    _,data=observer.match.find_state(record['start_state']['sha256']);week=observer.match.Week(data)
    pending=None;cases=[];ticks=0;joined=False;entry_memory={};regions=[];hook=None;entry_sp=0
    class Complete(Exception):pass
    def scene(g,index):
        a=0xCCEF+25*index
        return {'index':index,**{f'word{k}':g.rd(a+k) for k in (0,2,8,10,12,14,20,23)},**{f'byte{k}':g.rd(a+k,1) for k in (4,5,6,7,16,17,18,19,22)}}
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def tick(g):
        nonlocal ticks,joined
        ticks+=1
        if ticks>reference['ticks']:raise ValueError('adjustment capture exceeded retained original prefix')
        if ticks%25==0:print(f'adjustment admission tick{ticks}/roots{len(cases)}/draws{week.draws}',flush=True)
        if ticks==1:
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('adjustment reached different watched admission')
            joined=True
    from unicorn import UC_HOOK_MEM_READ,UC_HOOK_MEM_WRITE,UC_MEM_WRITE
    builtin=set(range(0xD7BA,0xD7C6))|set(range(0xDE84,0xDE88))
    def enter(g):
        nonlocal pending,entry_memory,regions,hook,entry_sp
        if pending is not None:raise ValueError('nested35F8')
        address=g.stack_word(4)+16*g.stack_word(6);offset=address-g.DS*16-0xCCEF
        if offset%25 or not 1<=offset//25<=22:raise ValueError('adjustment scene pointer differs')
        index=offset//25
        regions=[('p8',g.far(0xD7BA),8*3921),('p15',g.far(0xD7BE),15*3921),('club',g.far(0xD7C2),62*175),('ds',g.DS*16,65536)]
        entry_memory={name:g.mem(base,length) for name,base,length in regions};entry_sp=g.reg('sp')
        pending={'case':len(cases),'tick':ticks,'draw_start':week.draws,'input':{'scene':scene(g,index),'seed':g.rd(0xA51C),'cells':[]}}
        def memory(uc,access,address,size,value,user):
            if pending is None:return
            stack_base=g.reg('ss')*16;lo=(g.reg('sp')-8)&65535;span=(entry_sp+8-lo)&65535
            for linear in range(address,address+size):
                if 0<=linear-stack_base<65536 and ((linear-stack_base-lo)&65535)<=span:continue
                found=None
                for name,base,length in regions:
                    if base<=linear<base+length:found=(name,linear-base);break
                if found is None:raise ValueError(f'unmapped original data access {g.where()} {linear:06X}')
                name,offset=found;root=0xCCEF+25*index
                owned=name=='ds' and (offset in (root+12,root+13,0xA51C,0xA51D))
                if access==UC_MEM_WRITE:
                    if not owned:raise ValueError(f'unexpected original35F8 write {g.where()} {name}:{offset:04X}')
                    continue
                if name=='ds' and (root<=offset<root+25 or offset in builtin or offset in (0xA51C,0xA51D)):continue
                cell={'region':name,'offset':offset,'value':entry_memory[name][offset]}
                cells=pending['input']['cells']
                if not any(c['region']==name and c['offset']==offset for c in cells):cells.append(cell)
        hook=g.mu.hook_add(UC_HOOK_MEM_READ|UC_HOOK_MEM_WRITE,memory)
    def leave(g):
        nonlocal pending,hook
        if pending is None:return
        g.mu.hook_del(hook);hook=None;c=pending;pending=None;c['input']['cells'].sort(key=lambda r:(r['region'],r['offset']))
        output=json.loads(json.dumps(c['input']));output['scene']=scene(g,output['scene']['index']);output['seed']=g.rd(0xA51C)
        for field in output['cells']:
            base=next(base for name,base,length in regions if name==field['region']);actual=g.mem(base+field['offset'],1)[0]
            if actual!=field['value']:raise ValueError('readonly adjustment entry fact changed')
        draws=[e for e in week.events if e['kind']=='draw'][c['draw_start']:]
        def normalize(d):
            callers=d['callers'];site=callers[1] if callers[0]=='24BC:0D4F' else callers[2]
            return {k:d[k] for k in ('n','seed_before','seed_after','value')}|{'site':site}
        c.update(draw_end=week.draws,output={'output':output,'draws':[normalize(d) for d in draws]});cases.append(c)
        if len(cases)==64:raise Complete()
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:35F8',enter);week.g.code_hook('0AB9:365F',leave)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before adjustment capture boundary')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('adjustment original RNG prefix differs')
        if len(cases)!=64:raise ValueError('adjustment call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T35';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-adjustment-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'pending_on_abort':pending if status!='completed' else None,'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'cases':cases,
            'scope':'First64 original35F8 roots;readonly root-entry decoded data witnesses,canonical pointers external;full scenes/seeds/ordered draws separate'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/case_name).open('xb') as f:f.write(raw)
    pin={'schema':'watched-adjustment-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/pin_name).open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def benchmark(cases,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_watched_adjustment.py','pyproject.toml'):raise ValueError('commit source before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    values=[common.encoded(c['input']) for c in cases];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in values:size+=len(native.watched_adjust_target(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in out.glob('T35-r*.json')),default=0)+1;path=out/f'T35-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
            'source_pins':pins,'routine':'compiled native35F8 full target-adjustment helper graph',
            'repetitions':repetitions,'cases':len(cases),'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
            'method':'Pre-serialized inputs;release parsing,one native image/setup,compiled adjustment graph/Pascal frames/native helpers/RNG and serialization included;fixture preparation and output decoding excluded',
            'limits':'35F8 graph only,no full action5/72BB/clock,complete watched ticks or detailed commentary;no full-match/emulator speed comparison'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true');p.add_argument('--out',default='records/T35');p.add_argument('--repetitions',type=int,default=100);g.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root);return
    if args.repetitions<1:p.error("positive repetitions required")
    cases,pins=load();check(cases)
    if args.benchmark:benchmark(cases,pins,args.repetitions,args.out)
if __name__=='__main__':main()
