#!/usr/bin/env python3
"""Original watched6F98 inputs/assertions;Rust alone computes movement legality."""
import argparse,hashlib,json,sys
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT;FIXTURES=ROOT/'engine/tests/fixtures/watched_movement'
STATIC=('static/1-0-exe/code/seg_0AB9_12.asm','static/1-0-exe/code/seg_1998_01.asm','static/1-0-exe/code/seg_26C9_02.asm')
def load():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    raw=(FIXTURES/'cases.jsonl').read_bytes()
    if common.sha(raw)!=pin['cases_sha256']:raise ValueError('movement original cases digest differs')
    if common.sha((ROOT/pin['capture']).read_bytes())!=pin['capture_sha256']:raise ValueError('movement capture digest differs')
    cases=[json.loads(line) for line in raw.splitlines()]
    if len(cases)!=64 or [c['case'] for c in cases]!=list(range(64)):raise ValueError('64 original movement calls required')
    if any(c['draw_start']!=c['draw_end'] or c['input']['seed']!=c['output']['output']['seed'] or c['output']['draws'] for c in cases):raise ValueError('original6F98 unexpectedly drew RNG')
    return cases,pin

def check(cases):
    import onenil_engine as native
    for c in cases:
        got=json.loads(native.watched_move_candidate(common.encoded(c['input'])))
        if not common.same(got,c['output']):raise ValueError(f'original movement case{c["case"]} differs')
    print('Native watched movement:64 exact original candidate/legality/seed cases,no RNG;standalone kernel,not a watched match.')

def capture(research_root):
    if common.git('status','--porcelain','--','tools/replay_watched_movement.py'):raise ValueError('commit observer before capture')
    if (FIXTURES/'provenance.json').exists():load();print('original movement capture unchanged');return
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
            if not common.same(observer.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('movement reached different watched admission')
            joined=True
    def enter(g):
        nonlocal pending
        if pending is not None:raise ValueError('nested6F98')
        ss=g.reg('ss')*16;parent=g.stack_word(2);owner=g.word_at(ss+parent+4)
        address=g.word_at(ss+owner+6)+16*g.word_at(ss+owner+8)
        offset=address-g.DS*16-0xCCEF
        if offset%25 or not 1<=offset//25<=22:raise ValueError('movement caller scene pointer differs')
        index=offset//25;s=scene(g,index);role=g.mem(g.far(0xD7BA)+8*s['word14']+6,1)[0]
        state={'scene':s,'active':g.rd(0xD0B8,1),'actor':g.rd(0xD09F,1),'secondary':g.rd(0xD0A0,1),
               'role':role,'blocking_mask':[g.rd(0xD36+i,1) for i in range(32)],'cell':None,'seed':g.rd(0xA51C)}
        pending={'case':len(cases),'tick':ticks,'draw_start':week.draws,'input':state}
    def cell(g):
        if pending is None:return
        index=pending['input']['scene']['index'];address=g.reg('es')*16+g.reg('di')+0x90E
        pending['input']['cell']={'coordinate':g.rd(0xCCEF+25*index),'occupant':g.mem(address,1)[0],'tag':g.mem(address+1,1)[0]}
    def leave(g):
        nonlocal pending
        if pending is None:return
        c=pending;pending=None;output=json.loads(json.dumps(c['input']));output['scene']=scene(g,output['scene']['index']);output['seed']=g.rd(0xA51C)
        c.update(draw_end=week.draws,output={'output':output,'allowed':g.reg('ax')&255,'draws':[]});cases.append(c)
        if len(cases)==64:raise Complete()
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick)
    week.g.code_hook('0AB9:6F98',enter);week.g.code_hook('0AB9:7082',cell);week.g.code_hook('0AB9:70D1',leave)
    producer=common.git('rev-parse','HEAD');started=observer.match.utc();status='completed';reason=None
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before64 movement calls')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported:raise ValueError('unsupported/incomplete original capture')
        if not common.same(draws,reference['draws'][:len(draws)]):raise ValueError('movement original RNG prefix differs')
        if len(cases)!=64:raise ValueError('movement call count differs')
    except (Exception,KeyboardInterrupt) as e:status='aborted';reason=f'{type(e).__name__}: {e}'
    directory=ROOT/'records/T30';directory.mkdir(parents=True,exist_ok=True)
    number=max((int(p.stem.split('-r')[-1]) for p in directory.glob('capture-r*.json')),default=0)+1;path=directory/f'capture-r{number:04d}.json'
    result={'schema':'watched-movement-capture/1','code_commit':producer,'started':started,'ended':observer.match.utc(),
            'status':status,'reason':reason,'source':{'repository':'https://github.com/diegoami/onenil-archaeology','commit':observer.match.git('rev-parse','HEAD'),'prefix_record':source['record'],'prefix_record_sha256':source['record_sha256'],'start_state_sha256':record['start_state']['sha256'],'static_sources':{p:common.sha((root/p).read_bytes()) for p in STATIC}},
            'first_tick_join':joined,'ticks':ticks,'draws':week.draws,'cases':cases,
            'scope':'64 original6F98 calls from same watched admission/RNG prefix;raw indexed occupancy witnesses only,expected candidate/result never inputs;not a complete watched simulation'}
    with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print(path.relative_to(ROOT),status,reason,flush=True)
    if status!='completed':raise SystemExit(1)
    FIXTURES.mkdir(parents=True,exist_ok=True);raw=''.join(common.encoded(c)+'\n' for c in cases).encode()
    with (FIXTURES/'cases.jsonl').open('xb') as f:f.write(raw)
    pin={'schema':'watched-movement-source/1','capture':str(path.relative_to(ROOT)),'capture_sha256':common.sha(path.read_bytes()),'cases_sha256':common.sha(raw),'source':result['source']}
    with (FIXTURES/'provenance.json').open('x') as f:json.dump(pin,f,indent=2);f.write('\n')

def main():
    p=argparse.ArgumentParser(description=__doc__);g=p.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);args=p.parse_args()
    if args.capture:
        if args.research_root is None:p.error('--capture requires --research-root')
        capture(args.research_root);return
    cases,_=load();check(cases)
if __name__=='__main__':main()
