"""[designed] Independent original-root RAM observations;private pages never enter git.
Python records/normalizes source execution only. It computes no engine outcome.
"""
import gzip,hashlib,json,signal,sys
from pathlib import Path
import replay_step_tail as common
ROOT=common.ROOT
ROOTS={'action5':(0x448C,0x4610),'contact':(0x4DFF,0x4E59)}
REGS=('ax','cx','dx','bx','sp','bp','si','di','es','cs','ss','ds')
def write_json(path,value):
    path.parent.mkdir(parents=True,exist_ok=True)
    with path.open('x')as f:json.dump(value,f,indent=2);f.write('\n')
def read_json(path):return json.loads(path.read_text())
class Pages:
    def __init__(self,directory):
        self.directory=directory;self.pages=directory/'pages';self.pages.mkdir(parents=True,exist_ok=True)
        self.seen={p.name.removesuffix('.gz')for p in self.pages.glob('*.gz')}
    def snapshot(self,g,name):
        raw=g.mem(0,0x110000);hashes=[]
        for start in range(0,len(raw),4096):
            block=raw[start:start+4096];digest=common.sha(block);hashes.append(digest)
            if digest not in self.seen:
                with(self.pages/(digest+'.gz')).open('xb')as f:f.write(gzip.compress(block,compresslevel=1,mtime=0))
                self.seen.add(digest)
        metadata={'schema':'private-original-ram-pages/1','size':len(raw),'ram_sha256':common.sha(raw),'pages':hashes}
        path=self.directory/'snapshots'/(name+'.json')
        if path.exists():
            if read_json(path)!=metadata:raise ValueError('private snapshot resume differs:'+name)
        else:write_json(path,metadata)
        return {'path':str(path.relative_to(ROOT)),'sha256':common.sha(path.read_bytes()),'ram_sha256':metadata['ram_sha256']}
def memory(reference):
    path=ROOT/reference['path'];raw=path.read_bytes()
    if common.sha(raw)!=reference['sha256']:raise ValueError('private snapshot index digest differs')
    metadata=json.loads(raw);pages=path.parent.parent/'pages';blocks=[]
    for digest in metadata['pages']:
        block=gzip.decompress((pages/(digest+'.gz')).read_bytes())
        if len(block)!=4096 or common.sha(block)!=digest:raise ValueError('private page digest differs')
        blocks.append(block)
    raw=b''.join(blocks)
    if len(raw)!=metadata['size'] or common.sha(raw)!=reference['ram_sha256'] or common.sha(raw)!=metadata['ram_sha256']:raise ValueError('private RAM digest differs')
    return raw
def scene(g,index):
    a=0xCCEF+25*index
    return {'index':index,**{f'word{k}':g.rd(a+k)for k in(0,2,8,10,12,14,20,23)},**{f'byte{k}':g.rd(a+k,1)for k in(4,5,6,7,16,17,18,19,22)}}
def normalized(draw):
    callers=draw['callers'];site=callers[1]if callers[0]=='24BC:0D4F'else callers[2]
    return {k:draw[k]for k in('n','seed_before','seed_after','value')}|{'site':site}
def validate(directory):
    manifest=read_json(directory/'manifest.json')
    if manifest['status']!='completed' or not manifest['first_tick_join'] or manifest['window']!=[47,128]:raise ValueError('incomplete/wrong original action window')
    cases=[]
    for number,item in enumerate(manifest['cases']):
        path=ROOT/item['path'];raw=path.read_bytes()
        if common.sha(raw)!=item['sha256']:raise ValueError('original root record digest differs')
        c=json.loads(raw)
        if c['case']!=number or c['kind']not in ROOTS or not 47<=c['tick']<=128 or c['exit_sp']!=c['registers']['sp'] or c['exit_bp']!=c['registers']['bp']:raise ValueError('original root boundary differs')
        if c['input_scene']['index']!=c['scene'] or c['output_scene']['index']!=c['scene'] or c['draw_end']-c['draw_start']!=len(c['draws']):raise ValueError('original root state/draw interval differs')
        cases.append(c)
    codes={e['code']for c in cases for e in c['events']}
    if not codes.intersection({8,12})or not codes.intersection({11,14}):raise ValueError('missing original pass/contact observation')
    return cases,manifest
def capture(research_root,directory=ROOT/'records/T37/oracle-r0001'):
    if(directory/'manifest.json').exists():validate(directory);print('completed original action oracle unchanged');return
    if common.git('status','--porcelain','--','tools/watched_oracle.py','tools/replay_watched_action.py'):raise ValueError('commit original observer before capture')
    root=research_root.resolve();sys.path.insert(0,str(root/'runtime/emul'))
    import watched_actions as source
    source_pin=source.check();record=read_json(root/source_pin['record']);reference=source.setup.contests.outputs(record)
    _,data=source.match.find_state(record['start_state']['sha256']);week=source.match.Week(data)
    private=ROOT/'.cache/T37'/directory.name;pages=Pages(private)
    producer=common.git('rev-parse','HEAD');observer_files={name:common.sha((ROOT/name).read_bytes())for name in('tools/watched_oracle.py','tools/replay_watched_action.py')}
    metadata={'schema':'original-action-oracle-start/1','producer':producer,'observer_files':observer_files,'source':source_pin,'start_state_sha256':record['start_state']['sha256'],'game_files':{k:common.sha(v)for k,v in week.files_at_start.items()},'window':[47,128],'private':str(private.relative_to(ROOT))}
    if(directory/'start.json').exists():
        old=read_json(directory/'start.json')
        if any(old[k]!=metadata[k]for k in('observer_files','source','start_state_sha256','game_files','window','private')):raise ValueError('original oracle resume source differs')
    else:write_json(directory/'start.json',metadata)
    ticks=0;joined=False;pending=[];completed={};next_case=0;events=[];pending_event=None;cancelled=[False]
    class Complete(Exception):pass
    def request_cancel(signum,frame):cancelled[0]=True
    signal.signal(signal.SIGINT,request_cancel);signal.signal(signal.SIGTERM,request_cancel)
    def check_cancel(g):
        if cancelled[0]:raise RuntimeError('original oracle cancelled at safe boundary')
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def tick(g):
        nonlocal ticks,joined
        ticks+=1
        if ticks==1:
            if not common.same(source.snapshot(g),reference['first_ticks'][0]['state']):raise ValueError('original watched admission differs')
            joined=True
        if ticks%16==0:print(f'action oracle tick{ticks}/cases{len(completed)}/draws{week.draws}',flush=True)
        if ticks>128:
            if pending:raise ValueError('root spans watched tick boundary')
            raise Complete()
    def enter(g,kind):
        nonlocal next_case
        if not 47<=ticks<=128:return
        address=g.stack_word(4)+16*g.stack_word(6);delta=address-g.DS*16-0xCCEF
        if delta%25 or not 1<=delta//25<=22:raise ValueError('original root scene pointer differs')
        index=delta//25;number=next_case;next_case+=1
        value={'case':number,'kind':kind,'parent':pending[-1]['case']if pending else None,'tick':ticks,'scene':index,'registers':{r:g.reg(r)for r in REGS},'flags':g.reg('eflags'),
               'stack_words':[g.stack_word(i)for i in(0,2,4,6)],'input_scene':scene(g,index),'seed_before':g.rd(0xA51C),'draw_start':week.draws,'event_start':len(events),
               'entry':pages.snapshot(g,f'{number:05d}-entry')}
        pending.append(value)
    def leave(g,kind):
        if not pending:return
        if pending[-1]['kind']!=kind:raise ValueError('original root nesting differs')
        value=pending.pop();value.update(exit_sp=g.reg('sp'),exit_bp=g.reg('bp'),output_scene=scene(g,value['scene']),seed_after=g.rd(0xA51C),draw_end=week.draws,
            draws=[normalized(d)for d in[e for e in week.events if e['kind']=='draw'][value['draw_start']:]],events=events[value.pop('event_start'):],exit=pages.snapshot(g,f'{value["case"]:05d}-exit'))
        path=directory/f'case-{value["case"]:05d}.json'
        if path.exists():
            if not common.same(read_json(path),value):raise ValueError('recorded original root resume differs')
        else:write_json(path,value)
        completed[value['case']]={'path':str(path.relative_to(ROOT)),'sha256':common.sha(path.read_bytes())}
    def event_enter(g):
        nonlocal pending_event
        if not ticks:return
        if pending_event is not None:raise ValueError('nested original commentary')
        a=source.arguments(g);side=a['side'];club=g.rd(0xD6E4+2*side)
        def player(slot):return 0 if slot==0 else int.from_bytes(g.mem(g.far(0xD7C2)+62*(club-1)+2*(slot-1),2),'little')
        pending_event={**a,'club':club,'player':player(a['actor_slot']),'secondary':player(a['secondary_slot']),'minute':g.rd(0xD0B2),'tick':ticks,'draw_start':week.draws,'seed_before':g.rd(0xA51C),'head_before':g.rd(0x380),'caller':g.image_address(g.reg('cs'),g.stack_word(0))}
    def event_leave(g):
        nonlocal pending_event
        if pending_event is None:return
        value=pending_event;pending_event=None
        if g.rd(0x380)!=value['head_before']:
            value.update(event=len(events),draw_end=week.draws,seed_after=g.rd(0xA51C),queue=source.row(g));events.append(value)
    week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',tick);week.g.code_hook('24BC:0CDC',check_cancel)
    for kind,(entry,exit_ip)in ROOTS.items():
        week.g.code_hook(f'0AB9:{entry:04X}',lambda g,k=kind:enter(g,k));week.g.code_hook(f'0AB9:{exit_ip:04X}',lambda g,k=kind:leave(g,k))
    week.g.code_hook('0AB9:0F94',event_enter);week.g.code_hook('0AB9:0FFC',event_leave)
    status='completed';reason=None;started=source.match.utc()
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before oracle boundary')
        draws=[e for e in week.events if e['kind']=='draw']
        if not joined or week.g.unsupported or not common.same(draws,reference['draws'][:len(draws)])or pending:raise ValueError('original oracle admission/RNG/completeness differs')
    except(Exception,KeyboardInterrupt)as error:status='aborted';reason=f'{type(error).__name__}: {error}'
    result={'schema':'original-action-oracle/1','status':status,'reason':reason,'producer':producer,'started':started,'ended':source.match.utc(),'window':[47,128],'first_tick_join':joined,'next_tick_entry':ticks,'draws':week.draws,'cases':[completed[i]for i in sorted(completed)],'unique_event_codes':sorted({e['code']for e in events}),'private':str(private.relative_to(ROOT)),'scope':'Original root observations only;private guest RAM pages are never engine runtime or public fixture inputs'}
    if status!='completed':
        number=len(list(directory.glob('aborted-*.json')))+1;write_json(directory/f'aborted-{number:04d}.json',result);raise SystemExit(1)
    write_json(directory/'manifest.json',result);validate(directory);print('original action oracle completed:',len(completed),'roots',flush=True)
