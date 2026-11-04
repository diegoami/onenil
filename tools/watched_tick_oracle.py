"""[designed] Observe actual original7A02 entries and independent repeated calls.
No Python gameplay model;private RAM pages never enter git or the native runtime.
"""
import argparse,json,sys
from pathlib import Path
import watched_oracle as oracle
import watched_action_export as facts
import watched_parent as parent
ROOT=oracle.ROOT
DIRECTORY=ROOT/'records/T39/oracle-r0001'
REGS=oracle.REGS
COUNT=8

def read(path):return oracle.read_json(path)
def sha(path):return oracle.common.sha(path.read_bytes())
def context(g):
    c={'kind':'tick','registers':{r:g.reg(r)for r in REGS},'flags':g.reg('eflags'),'stack_words':[g.stack_word(i)for i in(0,2,4,6)],'input_scene':oracle.scene(g,1),'seed_before':g.rd(0xA51C)}
    raw=g.mem(0,0x110000);state=facts.context(c,raw);ds=16*g.reg('ds')
    state['pointers'].append({'kind':'style','offset':facts.word(raw,ds+0xD0C0),'segment':facts.word(raw,ds+0xD0C2)})
    return state

def named(state,at):
    p=state['pointers'][-1]
    if at==16*p['segment']+p['offset']:return 'style',0
    for area in('main','main_shown','background','background_shown'):
        offset=at-facts.address(state,area,0)
        if any(0x90E+160*r-74<=offset<0x90E+160*r+76 for r in range(-9,10)):return area,offset
    return facts.named(state,at)

def owned(state):return parent.owned_addresses(state,True)
def observed(g):return {'scenes':[oracle.scene(g,i)for i in range(1,23)],'seed':g.rd(0xA51C),'pointers':context(g)['pointers']}
def changes(state,before,after):
    out=[]
    for at in owned(state):
        if before[at]!=after[at]:
            area,offset=named(state,at);out.append({'area':area,'offset':offset,'before':before[at],'after':after[at]})
    return out

def source(research):
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    pin=full.prefix.check();record=read(research/pin['record']);expected=full.prefix.setup.contests.outputs(record)
    first=read(research/'runs/T40/chunk-000000.json')['payload'];return full,pin,record,expected,first

def completed(directory,pin):
    path=directory/'manifest.json'
    if not path.exists():return False
    if sha(path)!=pin:raise ValueError('completed original tick manifest pin differs')
    value=read(path)
    if value['status']!='completed'or value['count']!=COUNT or len(value['cases'])!=COUNT or sha(directory/'start.json')!=value['protocol_sha256']:raise ValueError('completed original tick protocol differs')
    for item in value['cases']:
        if sha(ROOT/item['path'])!=item['sha256']:raise ValueError('completed original tick case differs')
    print('completed original corpus verified unchanged:',directory.name,flush=True);return True

def actual(research):
    import replay_watched_tick as replay
    if completed(DIRECTORY,replay.ACTUAL_SHA):return
    if oracle.common.git('status','--porcelain','--','tools/watched_tick_oracle.py'):raise ValueError('commit observer before capture')
    full,pin,record,expected,reference=source(research);match=full.match
    _,data=match.find_state(record['start_state']['sha256']);week=match.Week(data)
    private=ROOT/'.cache/T39/oracle-r0001';pages=oracle.Pages(private);DIRECTORY.mkdir(parents=True,exist_ok=True)
    protocol={'schema':'original-watched-tick-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':sha(Path(__file__)),'reference_commit':match.git('rev-parse','HEAD'),'reference_prefix':pin,'source_state_sha256':record['start_state']['sha256'],'reference_chunk_sha256':sha(research/'runs/T40/chunk-000000.json'),'count':COUNT,'game_files':{k:oracle.common.sha(v)for k,v in week.files_at_start.items()},'scope':'Actual original7A02 entries/returns;presentation timers excluded from gameplay ownership'}
    start=DIRECTORY/'start.json'
    if start.exists():
        old=read(start)
        if any(old[k]!=protocol[k]for k in('observer_sha256','reference_prefix','source_state_sha256','reference_chunk_sha256','count','game_files')):raise ValueError('tick oracle resume source differs')
        protocol=old
    else:oracle.write_json(start,protocol)
    pending=[None];number=[0];events=[];event_pending=[None];items=[]
    class Complete(Exception):pass
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def event_enter(g):event_pending[0]=parent.event_entry(g,week)
    def event_leave(g):
        e=event_pending[0];event_pending[0]=None
        if e is not None and g.rd(0x380)!=e['head_before']:
            e.update(draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g));events.append(e)
    def enter(g):
        if pending[0]is not None:raise ValueError('nested original tick')
        number[0]+=1
        if number[0]>COUNT:raise Complete()
        n=number[0];ref=reference['ticks'][n-1]
        if full.state(g)!=ref['before']or week.draws!=ref['draw_start']:raise ValueError('actual original tick entry does not join T40')
        if n==1 and [e for e in week.events if e['kind']=='draw']!=expected['setup_draws']:raise ValueError('setup RNG prefix differs')
        state=context(g);pending[0]={'tick':n,'context':state,'entry':pages.snapshot(g,f'actual-{n:02d}-entry'),'input':observed(g),'draw_start':week.draws,'event_start':len(events),'before':g.mem(0,0x110000)}
    def leave(g):
        p=pending[0]
        if p is None:return
        n=p['tick'];ref=reference['ticks'][n-1]
        if full.state(g)!=ref['after']or week.draws!=ref['draw_end']:raise ValueError('actual original tick return does not join T40')
        if g.reg('sp')!=p['context']['regs'][4]or g.reg('bp')!=p['context']['regs'][5]:raise ValueError('original near tick ABI differs')
        after=g.mem(0,0x110000);p.update(output=observed(g),exit=pages.snapshot(g,f'actual-{n:02d}-exit'),exit_registers={r:g.reg(r)for r in REGS},exit_flags=g.reg('eflags'),draw_end=week.draws,draws=[oracle.normalized(d)for d in week.events if d['kind']=='draw'][p['draw_start']:],events=events[p.pop('event_start'):],changes=changes(p['context'],p.pop('before'),after));pending[0]=None
        p['schema']='original-watched-tick/1';path=DIRECTORY/f'actual-{n:02d}.json'
        if path.exists():
            if read(path)!=p:raise ValueError('retained original tick differs on resume')
        else:oracle.write_json(path,p)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('original actual tick',n,'verified',flush=True)
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',enter);week.g.code_hook('0AB9:7B7A',leave)
    week.g.code_hook('0AB9:0F94',event_enter);week.g.code_hook('0AB9:0FFC',event_leave)
    try:
        try:week.play()
        except Complete:pass
        if len(items)!=COUNT or pending[0]is not None or week.g.unsupported:raise ValueError('incomplete original tick observations')
        draws=[e for e in week.events if e['kind']=='draw']
        if draws!=expected['draws'][:len(draws)]:raise ValueError('original RNG prefix differs')
    except(Exception,KeyboardInterrupt)as error:
        n=len(list(DIRECTORY.glob('aborted-*.json')))+1;oracle.write_json(DIRECTORY/f'aborted-{n:04d}.json',{'status':'aborted','reason':f'{type(error).__name__}: {error}','producer':protocol['producer'],'completed':len(items)});raise
    out={'schema':'original-watched-ticks/1','status':'completed','protocol_sha256':sha(start),'cases':items,'count':COUNT}
    path=DIRECTORY/'manifest.json'
    if not path.exists():oracle.write_json(path,out)
    elif read(path)!=out:raise ValueError('original tick manifest differs')

def repeat(research):
    import replay_watched_tick as replay
    if completed(ROOT/'records/T39/repeat-r0001',replay.REPEAT_SHA):return
    if oracle.common.git('status','--porcelain','--','tools/watched_tick_oracle.py'):raise ValueError('commit observer before capture')
    sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest,match
    c=read(DIRECTORY/'actual-01.json');raw=oracle.memory(c['entry']);state=c['context'];registers=dict(zip(REGS,state['regs']))
    g=guest.Guest(memory=raw,load=registers['cs']-0x0AB9,regs={k.upper():v for k,v in registers.items()}|{'IP':0x7A02,'EFLAGS':state['flags']});g.files=match.game_files();g.divisor=0x400
    week=match.Week.__new__(match.Week);week.g=g;week.events=[];week.draws=0;week._pending=None;week.install_hooks()
    full,_,_,_,_=source(research);directory=ROOT/'records/T39/repeat-r0001';directory.mkdir(parents=True,exist_ok=True);pages=oracle.Pages(ROOT/'.cache/T39/repeat-r0001')
    protocol={'schema':'original-repeated-tick-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':sha(Path(__file__)),'admission_sha256':sha(DIRECTORY/'actual-01.json'),'count':COUNT,'scope':'Isolated successive7A02 calls from captured tick1 ENTRY;outer caller/clock omitted. No per-call state or helper result injection. Only IP reset to7A02 before repeating the same unconsumed near return frame.'}
    start=directory/'start.json'
    if start.exists():
        old=read(start)
        if any(old[k]!=protocol[k]for k in('observer_sha256','admission_sha256','count','scope')):raise ValueError('repeated tick resume source differs')
    else:oracle.write_json(start,protocol)
    class Returned(Exception):pass
    def returned(g):raise Returned()
    events=[];pending=[None]
    def event_enter(g):pending[0]=parent.event_entry(g,week)
    def event_leave(g):
        e=pending[0];pending[0]=None
        if e is not None and g.rd(0x380)!=e['head_before']:
            e.update(draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g));events.append(e)
    g.code_hook('0AB9:7B7A',returned);g.code_hook('0AB9:0F94',event_enter);g.code_hook('0AB9:0FFC',event_leave);items=[]
    for n in range(1,COUNT+1):
        before=g.mem(0,0x110000);start_draw=week.draws;start_events=len(events);g.set_reg('ip',0x7A02)
        entry_regs={r:g.reg(r)for r in REGS};entry_flags=g.reg('eflags')
        returned_cleanly=False
        try:
            for _ in range(40):g.run(max_steps=500000);g.other_polls=0
        except Returned:returned_cleanly=True
        if not returned_cleanly or g.unsupported or pending[0]is not None or week._pending is not None:raise ValueError('original repeated tick failed to return')
        if g.reg('sp')!=state['regs'][4]or g.reg('bp')!=state['regs'][5]:raise ValueError('original repeated tick ABI changed')
        value={'schema':'original-repeated-watched-tick/1','step':n,'entry_registers':entry_regs,'entry_flags':entry_flags,'output':observed(g),'exit_registers':{r:g.reg(r)for r in REGS},'exit_flags':g.reg('eflags'),'changes':changes(state,before,g.mem(0,0x110000)),'exit':pages.snapshot(g,f'repeat-{n:02d}-exit'),'draws':[oracle.normalized(d)for d in week.events if d['kind']=='draw'][start_draw:],'events':events[start_events:],'draw_start':start_draw,'draw_end':week.draws}
        path=directory/f'step-{n:02d}.json'
        if path.exists():
            if read(path)!=value:raise ValueError('original repeated outcome differs on resume')
        else:oracle.write_json(path,value)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('original repeated tick',n,'verified',flush=True)
    result={'schema':'original-repeated-watched-ticks/1','status':'completed','protocol_sha256':sha(start),'cases':items,'count':COUNT};path=directory/'manifest.json'
    if not path.exists():oracle.write_json(path,result)
    elif read(path)!=result:raise ValueError('original repeated manifest differs')

def prove_neutral(research):
    if oracle.common.git('status','--porcelain','--','tools/watched_tick_oracle.py'):raise ValueError('commit observer before measurement')
    full,pin,record,expected,reference=source(research);_,data=full.match.find_state(record['start_state']['sha256']);week=full.match.Week(data)
    count=[0];calls=[]
    class Complete(Exception):pass
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def enter(g):
        count[0]+=1
        if count[0]>COUNT:raise Complete()
        if full.state(g)!=reference['ticks'][count[0]-1]['before']:raise ValueError('neutral audit original tick join differs')
    def key_return(g):
        if 1<=count[0]<=COUNT:
            value=g.reg('ax')&255
            if value!=0:raise ValueError('captured original tick does not take no-pending-key route')
            calls.append({'tick':count[0],'AL':value,'site':'0AB9:791B'})
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',enter);week.g.code_hook('0AB9:791B',key_return)
    try:week.play()
    except Complete:pass
    if len(calls)!=COUNT*22 or week.g.unsupported:raise ValueError('neutral audit did not visit every original player')
    directory=ROOT/'records/T39/neutral-input';n=len(list(directory.glob('run-*.json')))+1
    oracle.write_json(directory/f'run-{n:04d}.json',{'schema':'original-neutral-input-proof/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':sha(Path(__file__)),'source':pin,'reference_chunk_sha256':sha(research/'runs/T40/chunk-000000.json'),'count':len(calls),'calls':calls,'scope':'Read-only observation of actual keyboard-test return at791B;all176 player visits take the no-pending-key branch. No helper result feeds native gameplay.'})
    print('original no-pending-key route:',len(calls),'visits verified',flush=True)

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);p.add_argument('--neutral-proof',action='store_true');p.add_argument('--capture',action='store_true');p.add_argument('--repeat',action='store_true');a=p.parse_args()
    if a.neutral_proof:prove_neutral(a.research_root)
    if a.capture:actual(a.research_root)
    if a.repeat:repeat(a.research_root)
    if not(a.capture or a.repeat or a.neutral_proof):p.error('choose capture or repeat')
if __name__=='__main__':main()
