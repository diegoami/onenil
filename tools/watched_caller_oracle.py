"""[designed] Observe original watched caller iterations and isolated clock cases.
Python computes no gameplay/clock outcome;private original pages never enter git.
"""
import argparse,json,sys
from pathlib import Path
import watched_oracle as oracle
import watched_tick_oracle as tick
import watched_parent as parent
ROOT=oracle.ROOT
DIRECTORY=ROOT/'records/T40/oracle-r0001'
COUNT=171
CLOCK_CASES=[
 {'active':0,'counter':0,'remaining':46,'stoppage':0,'elapsed':44,'phase':45},
 {'active':1,'counter':2,'remaining':46,'stoppage':0,'elapsed':44,'phase':45},
 {'active':1,'counter':0,'remaining':46,'stoppage':0,'elapsed':44,'phase':45},
 {'active':1,'counter':0,'remaining':1,'stoppage':0,'elapsed':89,'phase':255},
 {'active':1,'counter':0,'remaining':0,'stoppage':2,'elapsed':90,'phase':255},
 {'active':1,'counter':0,'remaining':0,'stoppage':0,'elapsed':93,'phase':255},
 {'active':1,'counter':0,'remaining':1,'stoppage':0,'elapsed':128,'phase':255},
 {'active':1,'counter':0,'remaining':65535,'stoppage':1,'elapsed':0,'phase':255},
]
CLOCK_FIELDS={'active':(0xD0B8,1),'counter':(0xD0B0,2),'remaining':(0xD0AE,2),'stoppage':(0xD0B4,2),'elapsed':(0xD0B2,2),'phase':(0xD0AC,1),'hold':(0xD0B9,1)}
def read(p):return oracle.read_json(p)
def sha(p):return oracle.common.sha(p.read_bytes())
def clock(g):return {key:g.rd(offset,width)for key,(offset,width)in CLOCK_FIELDS.items()}
def context(g):
 s=tick.context(g);s['kind']='caller';return s
def observed(g):return tick.observed(g)|{'clock':clock(g)}
def reference(research):
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import watched_complete as full
 source,base,expected=full.reference();index=read(research/'traces/T40/index.json');ticks=[];draws=[];actions=[];pins=[]
 for item in index['chunks'][:3]:
  p=research/item['path']
  if sha(p)!=item['sha256']:raise ValueError('completed watched reference chunk digest differs')
  c=read(p);payload=c['payload']
  if full.sha(full.encoded(payload))!=c['payload_sha256']:raise ValueError('reference payload digest differs')
  ticks+=payload['ticks'];draws+=payload['draws'];actions+=payload['actions'];pins.append(item)
 if len(ticks)<=COUNT:raise ValueError('reference caller continuation missing')
 return full,source,base,expected,ticks,draws,actions,pins

def capture(research):
 if (DIRECTORY/'manifest.json').exists():
  import replay_watched_caller as replay
  replay.sources();replay.check_research(research);print('completed caller capture verified;no original rerun',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_caller_oracle.py'):raise ValueError('commit original observer before capture')
 full,source,base,expected,ref_ticks,ref_draws,ref_actions,pins=reference(research)
 _,data=full.match.find_state(base['start_state']['sha256']);week=full.match.Week(data);pages=oracle.Pages(ROOT/'.cache/T40/oracle-r0001');DIRECTORY.mkdir(parents=True,exist_ok=True)
 protocol={'schema':'original-watched-caller-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':sha(Path(__file__)),'reference_prefix':source,'reference_index_sha256':sha(research/'traces/T40/index.json'),'reference_chunks':pins,'source_state_sha256':base['start_state']['sha256'],'count':COUNT,'game_files':{k:oracle.common.sha(v)for k,v in week.files_at_start.items()},'scope':'Actual7C67 entry to7C94 cutpoint,including tick,logical display/occupancy,offside work and7B7B clock;presentation timers excluded from gameplay ownership'}
 start=DIRECTORY/'start.json'
 if start.exists():
  old=read(start)
  if any(old[k]!=protocol[k]for k in('observer_sha256','reference_prefix','reference_index_sha256','reference_chunks','source_state_sha256','count','game_files')):raise ValueError('caller resume source differs')
  protocol=old
 else:oracle.write_json(start,protocol)
 count=[0];pending=[None];event_pending=[None];events=[];items=[];key_visits=[]
 class Complete(Exception):pass
 def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
 def event_enter(g):
  if pending[0]is not None:event_pending[0]=parent.event_entry(g,week)
 def event_leave(g):
  e=event_pending[0];event_pending[0]=None
  if e is not None and g.rd(0x380)!=e['head_before']:
   e.update(draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g));events.append(e)
 def key_return(g):
  if pending[0]is not None:
   if g.reg('ax')&255:raise ValueError('original caller left no-pending-key domain')
   key_visits.append({'iteration':count[0],'AL':0})
 def enter(g):
  if pending[0]is not None:raise ValueError('nested original caller iteration')
  count[0]+=1
  if count[0]>COUNT:raise Complete()
  number=count[0];ref=ref_ticks[number-1]
  if full.state(g)!=ref['before']or week.draws!=ref['draw_start']:raise ValueError('original caller entry does not join watched reference')
  if g.reg('bp')!=g.reg('sp')+6 or g.word_at(16*g.reg('ss')+g.reg('bp')+2)!=0x7DCB:raise ValueError('original caller parent ABI differs')
  state=context(g);entry=pages.snapshot(g,'initial-entry')if number==1 else None
  pending[0]={'iteration':number,'context':state,'input':observed(g),'before':g.mem(0,0x110000),'draw_start':week.draws,'event_start':len(events),'key_start':len(key_visits),'entry':entry,'parent_return':0x7DCB}
  if number==1 and [e for e in week.events if e['kind']=='draw']!=expected['setup_draws']:raise ValueError('original258setup draws differ')
 def leave(g):
  p=pending[0]
  if p is None:return
  number=p['iteration'];ref=ref_ticks[number]
  if full.state(g)!=ref['before']or week.draws!=ref['draw_start']:raise ValueError('original caller cutpoint does not join next watched entry')
  if g.reg('sp')!=p['context']['regs'][4]or g.reg('bp')!=p['context']['regs'][5]:raise ValueError('original iteration changed parent frame')
  if len(key_visits)-p['key_start']!=22:raise ValueError('original caller player visits differ')
  after=g.mem(0,0x110000);p.update(schema='original-watched-caller-iteration/1',output=observed(g),exit=pages.snapshot(g,f'exit-{number:03d}'),exit_registers={r:g.reg(r)for r in oracle.REGS},exit_flags=g.reg('eflags'),phase_complete=bool(g.reg('ax')&255),draw_end=week.draws,draws=[oracle.normalized(d)for d in week.events if d['kind']=='draw'][p['draw_start']:],events=events[p.pop('event_start'):],changes=tick.changes(p['context'],p.pop('before'),after),no_key_visits=len(key_visits)-p.pop('key_start'))
  pending[0]=None;path=DIRECTORY/f'iteration-{number:03d}.json'
  if path.exists():
   if read(path)!=p:raise ValueError('recorded original caller outcome differs on resume')
  else:oracle.write_json(path,p)
  items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('original caller iteration',number,'clock',p['output']['clock'],'draws',week.draws,flush=True)
 week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7C67',enter);week.g.code_hook('0AB9:7C94',leave);week.g.code_hook('0AB9:791B',key_return)
 week.g.code_hook('0AB9:0F94',event_enter);week.g.code_hook('0AB9:0FFC',event_leave)
 try:
  try:week.play()
  except Complete:pass
  if len(items)!=COUNT or pending[0]is not None or week.g.unsupported:raise ValueError('original caller capture incomplete')
  if [e for e in week.events if e['kind']=='draw']!=ref_draws[:week.draws]:raise ValueError('original caller RNG prefix differs')
  flat=[]
  for item in items:
   c=read(ROOT/item['path']);flat.extend([dict(e,tick=c['iteration'],event=len(flat)+i)for i,e in enumerate(c['events'])])
  if len(flat)<32 or flat[:32]!=expected['actions']:raise ValueError('original caller32-action prefix does not join')
 except(Exception,KeyboardInterrupt)as error:
  n=len(list(DIRECTORY.glob('aborted-*.json')))+1;oracle.write_json(DIRECTORY/f'aborted-{n:04d}.json',{'status':'aborted','reason':f'{type(error).__name__}: {error}','producer':protocol['producer'],'completed':len(items)});raise
 result={'schema':'original-watched-caller-corpus/1','status':'completed','protocol_sha256':sha(start),'count':COUNT,'cases':items,'accepted_actions':len(events),'no_key_visits':len(key_visits),'retained32prefix_joined':True}
 path=DIRECTORY/'manifest.json'
 if not path.exists():oracle.write_json(path,result)
 elif read(path)!=result:raise ValueError('original caller manifest differs')

def clock_cases(research):
 if (ROOT/'records/T40/clock-r0001/manifest.json').exists():
  import replay_watched_caller as replay
  replay.sources();print('completed8original clock cases verified;no original rerun',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_caller_oracle.py'):raise ValueError('commit observer before clock measurements')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest,match
 full,_,_,_,_,_,_,_=reference(research);anchor=read(DIRECTORY/'iteration-001.json');raw_entry=oracle.memory(anchor['entry']);directory=ROOT/'records/T40/clock-r0001';directory.mkdir(parents=True,exist_ok=True);pages=oracle.Pages(ROOT/'.cache/T40/clock-r0001')
 protocol={'schema':'original-watched-clock-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':sha(Path(__file__)),'admission_sha256':sha(DIRECTORY/'iteration-001.json'),'cases':CLOCK_CASES,'scope':'Derived isolated original7B7B calls with explicit initial clock-field overlays and source caller ABI;not historical match observations. Original code computes all outcomes.'};start=directory/'start.json'
 if start.exists():
  old=read(start)
  if any(old[k]!=protocol[k]for k in('observer_sha256','admission_sha256','cases','scope')):raise ValueError('clock capture resume source differs')
 else:oracle.write_json(start,protocol)
 items=[]
 for number,overlay in enumerate(CLOCK_CASES):
  raw=bytearray(raw_entry);state=json.loads(json.dumps(anchor['context']));regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds']
  for key,value in overlay.items():
   offset,width=CLOCK_FIELDS[key];raw[ds+offset:ds+offset+width]=value.to_bytes(width,'little')
  raw[ds+0xD0B9]=0
  # Source caller7C8E pushes BP,then near-call return7C92;other state is unchanged.
  sp=regs['sp']-4;at=16*regs['ss']+sp;raw[at:at+4]=(0x7C92).to_bytes(2,'little')+regs['bp'].to_bytes(2,'little')
  g=guest.Guest(memory=bytes(raw),load=regs['cs']-0x0AB9,regs={k.upper():v for k,v in regs.items()}|{'SP':sp,'IP':0x7B7B,'EFLAGS':state['flags']});g.files=match.game_files();g.divisor=0x400
  week=match.Week.__new__(match.Week);week.g=g;week.events=[];week.draws=0;week._pending=None;week.install_hooks();events=[];pending=[None]
  class Returned(Exception):pass
  def returned(g):raise Returned()
  def event_enter(g):pending[0]=parent.event_entry(g,week)
  def event_leave(g):
   e=pending[0];pending[0]=None
   if e is not None and g.rd(0x380)!=e['head_before']:
    e.update(draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g));events.append(e)
  g.code_hook('0AB9:7C3C',returned);g.code_hook('0AB9:0F94',event_enter);g.code_hook('0AB9:0FFC',event_leave)
  entered_clock=clock(g);returned_cleanly=False
  try:
   for _ in range(40):g.run(max_steps=500000);g.other_polls=0
  except Returned:returned_cleanly=True
  if not returned_cleanly or g.unsupported or pending[0]is not None or week._pending is not None:raise ValueError('original clock call did not return cleanly')
  if g.reg('sp')!=sp or g.reg('bp')!=regs['bp']:raise ValueError('original clock ABI differs')
  value={'schema':'original-derived-watched-clock/1','case':number,'admission_sha256':sha(DIRECTORY/'iteration-001.json'),'input_overlay':overlay,'input_clock':entered_clock,'output':observed(g),'phase_complete':bool(g.reg('ax')&255),'changes':tick.changes(state,bytes(raw),g.mem(0,0x110000)),'exit':pages.snapshot(g,f'clock-{number:02d}-exit'),'draws':[oracle.normalized(d)for d in week.events if d['kind']=='draw'],'events':events,'exit_sp':g.reg('sp'),'exit_bp':g.reg('bp'),'scope':protocol['scope']}
  path=directory/f'case-{number:02d}.json'
  if path.exists():
   if read(path)!=value:raise ValueError('retained original clock outcome differs')
  else:oracle.write_json(path,value)
  items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('original clock case',number,'complete',value['phase_complete'],'events',[e['code']for e in events],flush=True)
 result={'schema':'original-watched-clock-corpus/1','status':'completed','protocol_sha256':sha(start),'count':len(CLOCK_CASES),'cases':items};path=directory/'manifest.json'
 if not path.exists():oracle.write_json(path,result)
 elif read(path)!=result:raise ValueError('original clock manifest differs')

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);p.add_argument('--capture',action='store_true');p.add_argument('--clock-cases',action='store_true');a=p.parse_args()
 if a.capture:capture(a.research_root)
 if a.clock_cases:clock_cases(a.research_root)
 if not(a.capture or a.clock_cases):p.error('choose capture or clock-cases')
if __name__=='__main__':main()
