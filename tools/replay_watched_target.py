#!/usr/bin/env python3
"""Original561E root observations;Rust computes target and numeric commentary."""
import argparse,hashlib,json,sys,platform,subprocess,time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_target'
STATIC=('static/1-0-exe/code/seg_0AB9_09.asm','static/1-0-exe/code/seg_0AB9_03.asm','static/1-0-exe/code/seg_0AB9_02.asm')
def load():
    cases=[];pins=[]
    for prefix in ('','active-'):
        pin=json.loads((FIXTURES/(prefix+'provenance.json')).read_text())
        raw=(FIXTURES/(prefix+'cases.jsonl')).read_bytes()
        if common.sha(raw)!=pin['cases_sha256']:raise ValueError('target original cases digest differs')
        capture_bytes=(ROOT/pin['capture']).read_bytes()
        if common.sha(capture_bytes)!=pin['capture_sha256']:raise ValueError('target capture digest differs')
        capture_record=json.loads(capture_bytes)
        if capture_record['status']!='completed' or not capture_record['first_tick_join'] or capture_record['source']!=pin['source']:raise ValueError('target capture provenance differs')
        rows=[json.loads(line) for line in raw.splitlines()]
        if len(rows)!=64 or [c['case'] for c in rows]!=list(range(64)):raise ValueError('64 original target calls per stream required')
        if not common.same(rows,capture_record['cases']):raise ValueError('target capture/export differs')
        if prefix and any(c['active']==0 for c in rows):raise ValueError('active stream contains inactive calls')
        cases.extend(rows);pins.append(pin)
    return cases,pins

def check(cases):
    import onenil_engine as native
    for number,c in enumerate(cases):
        got=json.loads(native.watched_prepare_target(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original target case{number} differs')
    print(f'Native watched target:{len(cases)} exact original scene/seed/numeric commentary cases;standalone kernel,not a watched match.')

def capture(research_root,active_only=False):
    if common.git('status','--porcelain','--','tools/replay_watched_target.py'):raise ValueError('commit observer before capture')
    pin_name='active-provenance.json' if active_only else 'provenance.json'
    case_name='active-cases.jsonl' if active_only else 'cases.jsonl'
    if (FIXTURES/pin_name).exists():load();print('original target capture unchanged');return
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
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('target reached different watched admission')
            joined=True
    def numeric(g,template=None):
        clubs=[g.rd(0xD6E4),g.rd(0xD6E6)]
        head=g.rd(0x380)
        def next(i):return 1 if i==1700 else i+1
        indices=list(dict.fromkeys([head,next(head),next(next(head))])) if template is None else [r['index'] for r in template['rows']]
        rows=[]
        for index in indices:
            at=0xA52A+6*index
            rows.append({'index':index,'code':g.rd(at,1),'first_player':g.rd(at+1),'second_player':g.rd(at+3),'side':g.rd(at+5,1)})
        roster=g.far(0xD7C2)
        return {'clubs':clubs,'rosters':[[g.word_at(roster+62*(club-1)+2*i) for i in range(20)] for club in clubs],
                'weight':g.rd(0x97A+49,1),'weighted_words':[g.rd(0xD146+2*i) for i in range(41)],
                'event_counts':[g.rd(0xD1C0+2*i) for i in range(2)],'source_pair':[g.rd(0xD08A),g.rd(0xD08C)],
                'target_pair':[g.rd(0xD09A),g.rd(0xD09C)],'head':head,'rows':rows}
    def enter(g):
        nonlocal pending
        if active_only and g.rd(0xD0B8,1)==0:return
        if pending is not None:raise ValueError('nested561E')
        address=g.stack_word(4)+16*g.stack_word(6)
        offset=address-g.DS*16-0xCCEF
        if offset%25 or not 1<=offset//25<=22:raise ValueError('target caller scene pointer differs')
        index=offset//25;s=scene(g,index);role=g.mem(g.far(0xD7BA)+8*s['word14']+6,1)[0]
        actor=g.rd(0xD09F,1);secondary=g.rd(0xD0A0,1)
        identities=[{'index':i,'slot':g.rd(0xCCEF+25*i+7,1)} for i in dict.fromkeys([index,actor,secondary]) if i!=0]
        state={'scene':s,'actor':actor,'secondary':secondary,'role':role,'role_offset':g.rd(0xAAA+role,1),
               'seed':g.rd(0xA51C),'flag':g.rd(0xCF53,1),'hold':g.rd(0xD0B9,1),'phase':g.rd(0xD0AC,1),'remaining':g.rd(0xD0AE),
               'direction_signs':[g.rd(0xA338+i,1) for i in range(2)],'side_offsets':[g.rd(0xD56+i,1) for i in range(2)],
               'special_target':g.rd(0xABA),'actor_target':g.rd(0xAB8),'identities':identities,'numeric':numeric(g)}
        pending={'case':len(cases),'tick':ticks,'active':g.rd(0xD0B8,1),'draw_start':week.draws,'input':state,'commentary':None}
    def commentary(g):
        if pending is None:return
        if pending['commentary'] is not None:raise ValueError('multiple561E commentary calls')
        pending['commentary']={'code':g.stack_word(2)&255,'actor_slot':g.stack_word(6),'secondary_slot':g.stack_word(4),'event_side':g.stack_word(8)&255}
    def leave(g):
        nonlocal pending
        if pending is None:return
        c=pending;pending=None;output=json.loads(json.dumps(c['input']));output['scene']=scene(g,output['scene']['index']);output['seed']=g.rd(0xA51C);output['flag']=g.rd(0xCF53,1)
        output['numeric']=numeric(g,c['input']['numeric'])
        event=c.pop('commentary')
        if event is not None:event['accepted']=int(output['numeric']['head']!=c['input']['numeric']['head'])
        if week.draws!=c['draw_start']:raise ValueError('561E unexpectedly drew RNG')
        c.update(draw_end=week.draws,output={'output':output,'draws':[],'commentary':event});cases.append(c)
        if len(cases)==64:raise Complete()
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:561E',enter);week.g.code_hook('0AB9:0F94',commentary);week.g.code_hook('0AB9:56EA',leave)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before64 target calls')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('target original RNG prefix differs')
        if len(cases)!=64:raise ValueError('target call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T32';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-target-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'active_only':active_only,'cases':cases,
            'scope':'64 original561E root calls from retained watched admission/RNG prefix;raw named input facts and separate output/numeric commentary assertions;not a complete watched simulation'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/case_name).open('xb') as f:f.write(raw)
    pin={'schema':'watched-target-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/pin_name).open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def benchmark(cases,pins,repetitions,directory):
    from onenil_engine import onenil_engine as native
    if common.git('status','--porcelain','--','engine','tools/replay_watched_target.py','pyproject.toml'):raise ValueError('commit source before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    values=[common.encoded(c['input']) for c in cases];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in values:size+=len(native.watched_prepare_target(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/directory;out.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in out.glob('T32-r*.json')),default=0)+1;path=out/f'T32-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),
            'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':common.sha((ROOT/'engine/Cargo.lock').read_bytes()),
            'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':common.sha(Path(native.__file__).read_bytes())},
            'source_pins':pins,'routine':'original561E action-one target preparation and numeric commentary',
            'repetitions':repetitions,'cases':len(cases),'calls':len(cases)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
            'method':'Pre-serialized inputs;release parsing,one native image/setup,target/orientation/numeric commentary and serialization included;fixture preparation and output decoding excluded',
            'limits':'Standalone561E preparation,not72BB full action dispatch,ball/clock,complete watched ticks or detailed commentary;no full-match/emulator speed comparison'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--benchmark',action='store_true');p.add_argument('--out',default='records/T32');p.add_argument('--repetitions',type=int,default=100);g.add_argument('--capture',action='store_true');g.add_argument('--capture-active',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture or args.capture_active:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root,args.capture_active);return
    if args.repetitions<1:p.error("positive repetitions required")
    cases,pins=load();check(cases)
    if args.benchmark:benchmark(cases,pins,args.repetitions,args.out)
if __name__=='__main__':main()
