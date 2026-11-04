"""[designed] Select numeric initial facts from the single retained original ENTRY.
No clock/gameplay calculation;native missing-input diagnostics select locations only.
"""
import argparse,copy,json,subprocess,sys
from pathlib import Path
import watched_oracle as oracle
import replay_watched_tick as ticks
import replay_watched_match as replay
ROOT=oracle.ROOT
SLOTS={'results':(0xD7CE,58*176),'fixtures':(0xD7CA,174*176),'screen':(0xDD58,4000),'screen_shown':(0xDD5C,4000),'screen_saved':(0xDD60,4000),'counts':(0xDCC8,4004)}
def read(p):return json.loads(p.read_text())
def address(state,area,offset):
 if area in SLOTS or area=='style':
  p=next(p for p in state['pointers']if p['kind']==area);return 16*p['segment']+p['offset']+offset
 if area=='frame':return 16*state['regs'][10]+state['regs'][4]+offset
 return ticks.address(state,area,offset)
def named(state,at):
 style=next(p for p in state['pointers']if p['kind']=='style')
 if at==16*style['segment']+style['offset']:return 'style',0
 frame=16*state['regs'][10]+state['regs'][4]
 if -4096<=at-frame<64:return 'frame',at-frame
 ds=16*state['regs'][11]
 offset=at-ds
 if 0<=offset<65536:
  if offset in (0xD5A,0xD5B,0xD088,0xD089):raise ValueError('presentation timer outside match ownership')
  if any(lo<=offset<hi for lo,hi in[(0x380,0x382),(0x900,0xC00),(0xD00,0xDA0),(0x957D,0xD7BA),(0xD7D6,0xD7DA),(0xDCBA,0xDCBC),(0xDD64,0xDD66),(0xDE88,0xDE90),(0x0BF6,0x0C68),(0x0DA0,0x0E6A),(0x0CD7,0x0CE1),(0x935B,0x94F7),(0xD886,0xD936),(0x0F20,0x0F30),(0x1274,0x1288),(0x129E,0x12CE),(0x8708,0x8716)])or offset in(0xDD56,0xDE88,0xDE8A,0xE5DD):return 'ds',offset
  raise ValueError(f'unclassified source DS field{offset:04X};requires source/original proof')
 for name,(_,length)in {**{k:(0,4000)for k in('main','main_shown','background','background_shown')},**SLOTS}.items():
  p=next(p for p in state['pointers']if p['kind']==name);base=16*p['segment']+p['offset']
  if base<=at<base+length:return name,at-base
 return ticks.ticks.named(state,at)
def entry():
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');raw=oracle.memory(anchor['entry']);state=copy.deepcopy(read(ROOT/'engine/tests/fixtures/watched_caller/case-00.json')['initial']);state['kind']='watched_match';ds=16*state['regs'][11]
 for name,(slot,_)in SLOTS.items():
  value=int.from_bytes(raw[ds+slot:ds+slot+4],'little');state['pointers'].append({'kind':name,'offset':value&65535,'segment':value>>16})
 # Pointer headers become typed inputs instead of duplicate DS byte facts.
 reserved={ds+slot+i for slot,_ in SLOTS.values()for i in range(4)}
 state['cells']=[f for f in state['cells']if address(state,f['area'],f['offset'])not in reserved]
 known={address(state,f['area'],f['offset'])for f in state['cells']}
 base=16*state['regs'][10]+state['regs'][4]
 for offset in range(8,64):
  if base+offset not in known:state['cells'].append({'area':'frame','offset':offset,'value':raw[base+offset]});known.add(base+offset)
 return raw,state

def select(probe):
 raw,state=entry();cache=ROOT/'.cache/T41';cache.mkdir(parents=True,exist_ok=True);saved=cache/'initial-selection.json'
 if saved.exists():
  old=read(saved);existing={p['kind']:p for p in old['pointers']}
  for pointer in state['pointers']:
   if pointer['kind']in existing and existing[pointer['kind']]!=pointer:raise ValueError('cached pointer differs from original ENTRY')
   existing[pointer['kind']]=pointer
  old['pointers']=list(existing.values());state=old
  reserved={16*state['regs'][11]+slot+i for slot,_ in SLOTS.values()for i in range(4)}
  state['cells']=[f for f in state['cells']if address(state,f['area'],f['offset'])not in reserved]
 for fact in state['cells']:
  if raw[address(state,fact['area'],fact['offset'])]!=fact['value']:raise ValueError('cached initial fact differs from original ENTRY')
 known={address(state,f['area'],f['offset'])for f in state['cells']}
 # Original7E21/7E3E copies complete4000-byte text pages at halftime.
 # These are2000numeric character/attribute pairs per page,not font bitmap data.
 for area in('main','main_shown','background','background_shown'):
  for offset in range(4000):
   at=address(state,area,offset)
   if at not in known:known.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
 # Original0E45 advances the numeric queue in1..1700;all baseline rows are ENTRY facts.
 ds=16*state['regs'][11]
 batches=[('ds',range(0xA52A,0xA52A+6*1701)),('ds',range(0xC66,0xC68)),('ds',range(0x12BA,0x12BD)),('ds',range(0x8708,0x8716)),('ds',range(0x0F20,0x0F30)),('ds',range(0x1274,0x1288)),('ds',range(0x12BE,0x12CE)),('ds',range(0x9828,0x9A2A)),('ds',range(0x0CD7,0x0CE1)),('ds',range(0xDE88,0xDE90)),('ds',range(0x0BF6,0x0C66)),('ds',range(0x0DA6,0x0E6A)),('ds',range(0x0D9A,0x0D9B+raw[ds+0x0D9A]))]
 for side in(0,1):
  club=int.from_bytes(raw[ds+0xD6E4+2*side:ds+0xD6E6+2*side],'little')
  baseline=0xD885+club;table=0x93F7+raw[ds+baseline];batches.extend([('ds',[baseline]),('ds',[table])])
  club_at=address(state,'club',62*(club-1))
  for field in(40,42):
   index=int.from_bytes(raw[club_at+field:club_at+field+2],'little');batches.append(('counts',[4*index+2]))
  for slot in range(20):
   player=int.from_bytes(raw[club_at+2*slot:club_at+2*slot+2],'little')
   if 0<player<=3920:
    batches.extend([('p8',range(8*player,8*player+8)),('p15',range(15*player,15*player+15))]);style=raw[address(state,'p8',8*player+7)];selector=raw[ds+0x12BA+player%3];batches.append(('ds',[0x129E+14*selector+style]))
  for area,width in [('club',62),('results',58),('fixtures',174)]:batches.append((area,range(width*(club-1),width*club)))
 for area,offsets in batches:
  for offset in offsets:
   at=address(state,area,offset)
   if at not in known:known.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
 p=subprocess.Popen([str(probe.resolve())],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
 try:
  for attempt in range(10000):
   saved.write_text(json.dumps(state));p.stdin.write(json.dumps({'initial':state,'no_pending_key':True})+'\n');p.stdin.flush();line=p.stdout.readline()
   if not line:raise ValueError('native match probe stopped')
   packet=json.loads(line)
   if 'ok'in packet:
    replay.compare(packet['ok']);target=ROOT/'engine/tests/fixtures/watched_match';target.mkdir(parents=True,exist_ok=True);path=target/'initial.json'
    if path.exists():raise ValueError('immutable measured admission exists')
    oracle.write_json(path,{'initial':state,'no_pending_key':True,'source_admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'scope':'One initial ENTRY union;full original reference used only for assertions;no per-step inputs.'});print('native full match admission selected and parity verified',len(state['cells']),'facts',flush=True);return
   error=packet['error'];missing=error.get('missing')
   if 'ticks'in error:
    count,draws=replay.compare_prefix(error);print('verified completed original prefix',count,'ticks',draws,'draws',flush=True)
    error={k:v for k,v in error.items()if k not in('ticks','draws','events')}
   (cache/'last-diagnostic.json').write_text(json.dumps(error,indent=2))
   if missing is None:raise ValueError('native match unsupported:'+error['message'])
   if 'missing initial native write baseline'in error['message']and not missing['writing']:raise ValueError('native diagnostic overwrote the original missing-write descriptor')
   (cache/'last-diagnostic.json').write_text(json.dumps(error,indent=2));area,offset=named(state,missing['address']);at=address(state,area,offset)
   if at in known:raise ValueError('native repeated missing initial fact:'+str(error))
   offsets=[offset]
   if area in('p8','p15'):
    length=8 if area=='p8'else 15;offsets=range(offset//length*length,offset//length*length+length)
   elif area=='ds'and 0xA52A<=offset<0xCCEF:
    start=0xA52A+(offset-0xA52A)//6*6;offsets=range(start,start+6)
   else:offsets=range(offset,offset+(1 if area=='ds'and(offset in(0xDD56,0xDE88,0xDE8A,0xE5DD)or 0x0F20<=offset<0x0F30 or 0x935B<=offset<0x93F7)else 2))
   for field in offsets:
    at=address(state,area,field)
    if at not in known:known.add(at);state['cells'].append({'area':area,'offset':field,'value':raw[at]})
   if attempt%8==0:print('initial ENTRY union',len(state['cells']),'facts;native diagnostic',{k:error[k]for k in('message','missing')},flush=True)
  raise ValueError('full native match admission did not converge')
 finally:
  if p.poll()is None:
   if sys.exc_info()[0]is not None:p.terminate()
   else:p.stdin.close()
   try:p.wait(timeout=10)
   except subprocess.TimeoutExpired:p.kill();p.wait()
  for stream in(p.stdin,p.stdout):
   try:stream.close()
   except OSError:pass
def prove_transient(research):
 if (ROOT/'records/T41/transient-r0001/result.json').exists():
  replay.transient_proof();print('completed original proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original proof')
 import sys
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import watched_complete as full
 ticks,draws,events,finish=replay.sources();source,base,expected=full.reference();_,data=full.match.find_state(base['start_state']['sha256']);week=full.match.Week(data)
 number=[0];observations=[];initial=[None]
 directory=ROOT/'records/T41/transient-r0001';directory.mkdir(parents=True,exist_ok=True)
 protocol={'schema':'original-transient-scene-proof/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'reference_index_sha256':replay.INDEX_SHA,'target_tick':250,'read_site':'0AB9:1EA6','derived_offset':0xCCEF+25*255+7,'scope':'Observe original transient scene255 byte7 read after joining every original tick through250;Python computes no helper outcome.'}
 start=directory/'start.json'
 if start.exists():
  prior=read(start)
  if prior['observer_sha256']!=protocol['observer_sha256']:raise ValueError('original proof source differs')
  protocol=prior
 else:oracle.write_json(start,protocol)
 class Complete(Exception):pass
 def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
 def enter(g):
  number[0]+=1
  if full.state(g)!=ticks[number[0]-1]['before']or week.draws!=ticks[number[0]-1]['draw_start']:raise ValueError('original proof entry differs from full reference')
  if number[0]==1:initial[0]=g.rd(0xE5DD,1)
 def leave(g):
  if full.state(g)!=ticks[number[0]-1]['after']or week.draws!=ticks[number[0]-1]['draw_end']:raise ValueError('original proof return differs from full reference')
  if number[0]==250:raise Complete()
 def observe(g):
  at=16*g.reg('es')+((g.reg('di')+7)&65535)
  if number[0]==250 and at==16*g.reg('ds')+0xE5DD:
   observations.append({'tick':number[0],'draw':week.draws,'actor':g.rd(0xD09F,1),'es':g.reg('es'),'di':g.reg('di'),'ds_offset':at-16*g.reg('ds'),'value':g.mem(at,1)[0],'source_return':g.word_at(16*g.reg('ss')+g.reg('bp')+2)})
 week.g.code_hook('0AB9:E24C',enable);week.g.code_hook('0AB9:7A02',enter);week.g.code_hook('0AB9:7B79',leave);week.g.code_hook('0AB9:1EA6',observe)
 try:week.play()
 except Complete:pass
 if number[0]!=250 or not observations or week.g.unsupported:raise ValueError('original transient read not confirmed')
 if [oracle.normalized(d)for d in week.events if d['kind']=='draw']!=[oracle.normalized(d)for d in draws[:week.draws]]:raise ValueError('original transient proof RNG prefix differs')
 result={'schema':'original-transient-scene-read/1','status':'completed','protocol_sha256':replay.sha(start),'original_ticks_joined':number[0],'draws_joined':week.draws,'initial_value':initial[0],'reads':observations,'scope':protocol['scope']};path=directory/'result.json'
 if path.exists():
  if read(path)!=result:raise ValueError('original transient proof outcome differs')
 else:oracle.write_json(path,result)
 print('original transient scene255 read confirmed',observations,flush=True)

def prove_statistics(research,wait=False,mode=0):
 directory=ROOT/f'records/T41/statistics-boundary-r{1+2*mode+int(wait):04d}'
 if (directory/'result.json').exists():
  replay.statistics_proof();print('completed statistics proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original statistics proof')
 import sys
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest,match
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');raw=bytearray(oracle.memory(anchor['entry']));state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 # Explicitly derived mode/wait arguments and a presentation key value7.
 # Original clears the key at0812;this is not a historical halftime-state capture.
 sp=regs['sp']-8;raw[ss+sp:ss+sp+8]=(0x7E2F).to_bytes(2,'little')+regs['cs'].to_bytes(2,'little')+mode.to_bytes(2,'little')+int(wait).to_bytes(2,'little');raw[ds+0xDE88]=7
 directory=ROOT/f'records/T41/statistics-boundary-r{1+2*mode+int(wait):04d}';directory.mkdir(parents=True,exist_ok=True)
 protocol={'schema':'original-statistics-boundary-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'1998:045C','return':'0AB9:7E2F','arguments':[mode,int(wait)],'external_key':'Return'if wait else None,'initial_presentation_key':7,'scope':'Derived isolated original statistics display from retained initial ENTRY;check complete memory-change footprint. Not a historical halftime capture.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable statistics proof already started;resume recorded observer explicitly')
 oracle.write_json(start,protocol)
 g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x1998,'IP':0x045C,'SP':sp,'EFLAGS':state['flags']});g.files=match.game_files()
 if wait:g.press('Return')
 before=g.mem(0,0x110000)
 class Returned(Exception):pass
 def returned(g):raise Returned()
 g.code_hook('0AB9:7E2F',returned);complete=False
 try:
  for _ in range(20):g.run(max_steps=500000);g.other_polls=0
 except Returned:complete=True
 if not complete or g.unsupported:raise ValueError('original statistics display did not return cleanly')
 if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original statistics far ABI differs')
 after=g.mem(0,0x110000)
 def pointer(slot):return int.from_bytes(before[ds+slot:ds+slot+2],'little')+16*int.from_bytes(before[ds+slot+2:ds+slot+4],'little')
 pages={'main':pointer(0xDD58),'main_shown':pointer(0xDD5C)}
 allowed_ds={0xDD64,0xDD65,0xDE88,0xD5A,0xD5B,0xD088,0xD089};counts={'stack':0,'main':0,'main_shown':0,'presentation_ds':0,'bios_hardware':0};unexpected=[]
 for at,(old,new)in enumerate(zip(before,after,strict=True)):
  if old==new:continue
  if ss+sp-4096<=at<ss+regs['sp']+64:kind='stack'
  elif ds<=at<ds+65536 and at-ds in allowed_ds:kind='presentation_ds'
  elif pages['main']<=at<pages['main']+4000:kind='main'
  elif pages['main_shown']<=at<pages['main_shown']+4000:kind='main_shown'
  elif 0x400<=at<0x500:kind='bios_hardware'
  else:unexpected.append(at);continue
  counts[kind]+=1
 result={'schema':'original-statistics-preservation/1','status':'completed'if not unexpected else'failed-footprint','protocol_sha256':replay.sha(start),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':counts,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16],'all_other_memory_equal':not unexpected,'key_before':before[ds+0xDE88],'key_after':after[ds+0xDE88],'style_before':int.from_bytes(before[ds+0xDD64:ds+0xDD66],'little'),'style_after':int.from_bytes(after[ds+0xDD64:ds+0xDD66],'little'),'scope':protocol['scope']};oracle.write_json(directory/'result.json',result)
 if unexpected:raise ValueError('original statistics has changes outside the declared presentation footprint')
 if result['key_after']!=0:raise ValueError('original statistics did not clear its presentation key')
 print('original statistics display:all non-presentation memory preserved;far ABI restored;key cleared;changes',counts,flush=True)

def prove_templates(research):
 directory=ROOT/'records/T41/template-boundary-r0001'
 if (directory/'result.json').exists():
  replay.template_proof();print('completed template proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original template proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 cases=[(0x132E,0x1B55,0x346C),(0x6346,0x1B55,0x347B),(0x25CC,0x0AB9,0xD6E5),(0x4F20,0x0AB9,0x971A)]
 protocol={'schema':'original-template-boundary-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'232A:052F','cases':[{'template_ds_offset':t,'return':f'{cs:04X}:{ip:04X}'}for t,cs,ip in cases],'destination_header':'DS:DD5C','scope':'Derived isolated original menu-template calls from retained initial ENTRY;whole-memory footprint. No template bytes or historical halftime state exported.'}
 start=directory/'start.json'
 if start.exists():raise ValueError('immutable template proof already started')
 oracle.write_json(start,protocol);observations=[]
 for template,cs,ip in cases:
  raw=bytearray(initial);sp=regs['sp']-12;words=[ip,load+cs,0xDD5C,regs['ds'],template,regs['ds']]
  raw[ss+sp:ss+sp+12]=b''.join(w.to_bytes(2,'little')for w in words)
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x232A,'IP':0x052F,'SP':sp,'EFLAGS':state['flags']})
  before=g.mem(0,0x110000);page=int.from_bytes(before[ds+0xDD5C:ds+0xDD5E],'little')+16*int.from_bytes(before[ds+0xDD5E:ds+0xDD60],'little')
  class Returned(Exception):pass
  def returned(g):raise Returned()
  g.code_hook(f'{cs:04X}:{ip:04X}',returned);complete=False
  try:g.run(max_steps=500000)
  except Returned:complete=True
  if not complete or g.unsupported:raise ValueError('original template did not return cleanly')
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original template ABI differs')
  after=g.mem(0,0x110000);counts={'stack':0,'main_shown':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+regs['sp']:counts['stack']+=1
   elif page<=address<page+4000:counts['main_shown']+=1
   else:unexpected.append(address)
  observations.append({'template_ds_offset':template,'return':f'{cs:04X}:{ip:04X}','returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':counts,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16],'all_other_memory_equal':not unexpected})
  if unexpected:raise ValueError('original template modifies non-presentation memory')
 result={'schema':'original-template-preservation/1','status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']};oracle.write_json(directory/'result.json',result)
 oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')})
 print('all4original templates preserve all memory outside MainShown and stack',observations,flush=True)

def prove_ui_leaves(research):
 directory=ROOT/'records/T41/ui-leaves-boundary-r0001'
 if (directory/'result.json').exists():
  replay.ui_leaves_proof();print('completed UI leaf proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original UI leaf proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 def word(at):return int.from_bytes(initial[at:at+2],'little')
 def pointer(slot):return word(ds+slot)+16*word(ds+slot+2)
 cases=[]
 for template in(0x132E,0x6346,0x25CC,0x4F20,0x3B82,0x3D72):cases.append((0x232A,0x052F,[0xDD5C,regs['ds'],template,regs['ds']]))
 for root in(0x0283,0x039D,0x041B):cases.append((0x232A,root,[0xDD5C,regs['ds'],50,10,0x0F10,regs['ds']]+([7]if root==0x041B else[])))
 clubs=[word(ds+0xD6E4+2*side)for side in(0,1)]
 for club in[0]+clubs:cases.append((0x1B55,0x0EE3,[23,1,club]))
 players=sorted({0}|{word(pointer(0xD7C2)+62*(club-1)+2*slot)for club in clubs for slot in range(20)})
 for player in players:cases.append((0x1B55,0x1FF1,[10,3,player]))
 protocol={'schema':'original-ui-leaf-boundary-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'return':'0AB9:EBE2','cases':[{'root':f'{cs:04X}:{ip:04X}','arguments':args}for cs,ip,args in cases],'scope':'Derived isolated original UI leaves from retained initial ENTRY;whole-memory footprint including all selected squad player rows. Numeric summary and substitution routines excluded. No original text/name/template bytes exported.'}
 start=directory/'start.json'
 if start.exists():raise ValueError('immutable UI leaf proof already started')
 oracle.write_json(start,protocol);observations=[]
 for cs,ip,args in cases:
  raw=bytearray(initial);sp=regs['sp']-4-2*len(args);words=[0xEBE2,regs['cs']]+args
  raw[ss+sp:ss+sp+2*len(words)]=b''.join(w.to_bytes(2,'little')for w in words)
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+cs,'IP':ip,'SP':sp,'EFLAGS':state['flags']})
  before=g.mem(0,0x110000);page=pointer(0xDD5C)
  class Returned(Exception):pass
  def returned(g):raise Returned()
  g.code_hook('0AB9:EBE2',returned);complete=False
  try:g.run(max_steps=500000)
  except Returned:complete=True
  if not complete or g.unsupported:raise ValueError(f'original UI leaf{cs:04X}:{ip:04X} did not return cleanly')
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original UI leaf ABI differs')
  after=g.mem(0,0x110000);counts={'stack':0,'main_shown':0,'style':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+regs['sp']:counts['stack']+=1
   elif page<=address<page+4000:counts['main_shown']+=1
   elif ds+0xDD64<=address<ds+0xDD66:counts['style']+=1
   else:unexpected.append(address)
  observation={'root':f'{cs:04X}:{ip:04X}','arguments':args,'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':counts,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16],'all_other_memory_equal':not unexpected,'style_after':int.from_bytes(after[ds+0xDD64:ds+0xDD66],'little')};observations.append(observation)
  if unexpected:raise ValueError('original UI leaf modifies non-presentation memory:'+str(observation))
 result={'schema':'original-ui-leaf-preservation/1','status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']};oracle.write_json(directory/'result.json',result)
 oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')})
 print('original UI leaves preserve all memory outside MainShown/style/stack',len(observations),'cases',flush=True)

def prove_formation(research):
 directory=ROOT/'records/T41/formation-boundary-r0001'
 if(directory/'result.json').exists():
  replay.formation_proof();print('completed formation proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original formation proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 clubs=[int.from_bytes(initial[ds+0xD6E4+2*side:ds+0xD6E6+2*side],'little')for side in(0,1)];out=regs['sp']+128
 protocol={'schema':'original-formation-display-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'1B55:10AF','return':'0AB9:EBE2','clubs':clubs,'output_stack_offset':out,'observe_counts_at':'1B55:1125','scope':'Derived isolated original formation-text formatter;observe computed numeric role counts and complete memory footprint. Only displayed text is omitted from native presentation;numeric role counting remains native. No original text bytes exported.'}
 start=directory/'start.json'
 if start.exists():raise ValueError('immutable formation proof already started')
 oracle.write_json(start,protocol);observations=[]
 for club in clubs:
  raw=bytearray(initial);sp=regs['sp']-10;words=[0xEBE2,regs['cs'],club,out,regs['ss']];raw[ss+sp:ss+sp+10]=b''.join(w.to_bytes(2,'little')for w in words)
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x1B55,'IP':0x10AF,'SP':sp,'EFLAGS':state['flags']});before=g.mem(0,0x110000);counts=[]
  class Returned(Exception):pass
  def returned(g):raise Returned()
  def observe(g):counts.append(list(g.mem(16*g.reg('ss')+g.reg('bp')-4,4)))
  g.code_hook('0AB9:EBE2',returned);g.code_hook('1B55:1125',observe);complete=False
  try:g.run(max_steps=500000)
  except Returned:complete=True
  if not complete or g.unsupported or len(counts)!=1:raise ValueError('original formation did not return cleanly')
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp']-4,regs['bp'],regs['ds']):raise ValueError('original formation hidden-output ABI differs')
  after=g.mem(0,0x110000);changed={'stack':0,'output_text':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+regs['sp']:changed['stack']+=1
   elif ss+out<=address<ss+out+256:changed['output_text']+=1
   else:unexpected.append(address)
  observations.append({'club':club,'counts':counts[0],'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16]})
  if unexpected:raise ValueError('original formation changed non-presentation memory')
 oracle.write_json(directory/'result.json',{'status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']})
 oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')})
 print('original formation counts and presentation-only footprint',observations,flush=True)

def prove_grouped_integer(research):
 directory=ROOT/'records/T41/grouped-integer-r0001'
 if(directory/'result.json').exists():
  replay.grouped_integer_proof();print('completed grouped integer proof verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original numeric formatter proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ss=16*regs['ss'];load=regs['cs']-0x0AB9
 values=[0,1,12,999,1000,1001,999999,1000000,2147483647,2147483648,4294967295,4294966296];out=regs['sp']+128
 protocol={'schema':'original-grouped-integer-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'1B55:0F95','return':'0AB9:EBE2','values':values,'output_stack_offset':out,'scope':'Derived isolated original signed32 decimal grouping;decoded dynamically generated numeric output only,no original string/template asset.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable formatter proof already started')
 oracle.write_json(start,protocol);observations=[]
 for value in values:
  raw=bytearray(initial);sp=regs['sp']-12;words=[0xEBE2,regs['cs'],value&65535,value>>16,out,regs['ss']];raw[ss+sp:ss+sp+12]=b''.join(w.to_bytes(2,'little')for w in words)
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x1B55,'IP':0x0F95,'SP':sp,'EFLAGS':state['flags']});before=g.mem(0,0x110000)
  class Returned(Exception):pass
  def returned(g):raise Returned()
  g.code_hook('0AB9:EBE2',returned);complete=False
  try:g.run(max_steps=500000)
  except Returned:complete=True
  if not complete or g.unsupported:raise ValueError('original grouped integer did not return cleanly')
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp']-4,regs['bp'],regs['ds']):raise ValueError('original grouped integer hidden-output ABI differs')
  after=g.mem(0,0x110000);unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old!=new and not(ss+sp-4096<=address<ss+regs['sp']or ss+out<=address<ss+out+256):unexpected.append(address)
  observations.append({'raw_value':value,'pascal_bytes':list(after[ss+out:ss+out+1+after[ss+out]]),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16]})
  if unexpected:raise ValueError('original grouped integer changed non-output memory')
 oracle.write_json(directory/'result.json',{'status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']});oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')});print('original grouped integer',observations,flush=True)

def prove_fixture_title(research):
 directory=ROOT/'records/T41/fixture-title-boundary-r0001'
 if(directory/'result.json').exists():
  replay.fixture_title_proof();print('completed fixture title verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original fixture title proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 def word(at):return int.from_bytes(initial[at:at+2],'little')
 def pointer(slot):return word(ds+slot)+16*word(ds+slot+2)
 week=initial[pointer(0xD7D2)];club=word(ds+0xD6E6);stage=initial[pointer(0xD7CA)+174*(club-1)+3*(week-1)];out=regs['sp']+128
 protocol={'schema':'original-fixture-title-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'1B55:142B','return':'0AB9:EBE2','arguments':[week,stage],'output_stack_offset':out,'scope':'Derived isolated actual fixture-title arguments from retained initial ENTRY;verify complete memory footprint. Original title text is omitted from records/runtime.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable fixture title proof already started')
 oracle.write_json(start,protocol);raw=bytearray(initial);sp=regs['sp']-12;words=[0xEBE2,regs['cs'],week,stage,out,regs['ss']];raw[ss+sp:ss+sp+12]=b''.join(w.to_bytes(2,'little')for w in words)
 g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x1B55,'IP':0x142B,'SP':sp,'EFLAGS':state['flags']});before=g.mem(0,0x110000)
 class Returned(Exception):pass
 def returned(g):raise Returned()
 g.code_hook('0AB9:EBE2',returned);complete=False
 try:g.run(max_steps=500000)
 except Returned:complete=True
 if not complete or g.unsupported:raise ValueError('original fixture title did not return cleanly')
 if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp']-4,regs['bp'],regs['ds']):raise ValueError('original fixture title hidden-output ABI differs')
 after=g.mem(0,0x110000);changed={'stack':0,'output_text':0};unexpected=[]
 for address,(old,new)in enumerate(zip(before,after,strict=True)):
  if old==new:continue
  if ss+sp-4096<=address<ss+regs['sp']:changed['stack']+=1
  elif ss+out<=address<ss+out+256:changed['output_text']+=1
  else:unexpected.append(address)
 result={'status':'completed'if not unexpected else'failed-footprint','protocol_sha256':replay.sha(start),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16],'scope':protocol['scope']};oracle.write_json(directory/'result.json',result);oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')})
 if unexpected:raise ValueError('original fixture title modified non-output memory')
 print('original fixture title preserves all memory outside stack/output text',result,flush=True)

def prove_menu_input(research):
 directory=ROOT/'records/T41/menu-input-boundary-r0001'
 if(directory/'result.json').exists():
  replay.menu_input_proof();print('completed menu input verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original menu-input proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 def word(raw,at):return int.from_bytes(raw[at:at+2],'little')
 if word(initial,ds+0xDE8C)or word(initial,ds+0xDE8E):raise ValueError('menu input requires original empty pending-key queues')
 out=regs['sp']+128;cases=[(1,'Down'),(2,'Down'),(3,'Return'),(4,'Up')]
 protocol={'schema':'original-menu-input-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'232A:0F8B','return':'0AB9:EBE2','menu_count':4,'cases':[{'cursor':cursor,'external_key':key}for cursor,key in cases],'output_stack_offset':out,'scope':'Derived original menu cursor/input calls with external Down/Return/Up. SourceD8A2-D8A6 identifies option3 as continue;original Week.play chooses PLAY GAME without lineup changes. No external key is supplied to watched gameplay ticks.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable menu input proof already started')
 oracle.write_json(start,protocol);observations=[]
 page=word(initial,ds+0xDD58)+16*word(initial,ds+0xDD5A)
 for cursor,key in cases:
  raw=bytearray(initial);sp=regs['sp']-10;words=[0xEBE2,regs['cs'],4,out,regs['ss']];raw[ss+sp:ss+sp+10]=b''.join(w.to_bytes(2,'little')for w in words);raw[ss+out]=cursor
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x232A,'IP':0x0F8B,'SP':sp,'EFLAGS':state['flags']});g.press(key);before=g.mem(0,0x110000)
  class Returned(Exception):pass
  def returned(g):raise Returned()
  g.code_hook('0AB9:EBE2',returned);complete=False
  try:
   for _ in range(20):g.run(max_steps=500000);g.other_polls=0
  except Returned:complete=True
  if not complete or g.unsupported:raise ValueError('original menu input did not return cleanly')
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original menu input ABI differs')
  after=g.mem(0,0x110000);changed={'stack':0,'cursor':0,'main':0,'keyboard_driver':0,'presentation_timers':0,'bios_hardware':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+regs['sp']:kind='stack'
   elif address==ss+out:kind='cursor'
   elif page<=address<page+4000:kind='main'
   elif ds+0xDE88<=address<ds+0xDF00:kind='keyboard_driver'
   elif address-ds in(0xD5A,0xD5B,0xD088,0xD089):kind='presentation_timers'
   elif 0x400<=address<0x500:kind='bios_hardware'
   else:unexpected.append(address);continue
   changed[kind]+=1
  observation={'cursor_before':cursor,'external_key':key,'cursor_after':after[ss+out],'key_code':word(after,ds+0xDE88),'last_key_code':word(after,ds+0xDE8A),'pending_key':word(after,ds+0xDE8C),'queue_count':word(after,ds+0xDE8E),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16]};observations.append(observation)
  if unexpected:raise ValueError('original menu input modifies gameplay memory:'+str(observation))
 oracle.write_json(directory/'result.json',{'status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']});oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')});print('original menu input cursor/key results',observations,flush=True)

def prove_lineup_validation(research,attempt=1):
 directory=ROOT/f'records/T41/lineup-validation-r{attempt:04d}'
 if(directory/'result.json').exists():
  if attempt!=2:raise ValueError('unrecognized completed lineup protocol')
  replay.lineup_validation_proof();print('completed lineup validation verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original lineup-validation proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 def word(at):return int.from_bytes(initial[at:at+2],'little')
 def pointer(slot):return word(ds+slot)+16*word(ds+slot+2)
 club=word(ds+0xD6E4);club_at=pointer(0xD7C2)+62*(club-1);players=[word(club_at+2*slot)for slot in range(11)];roles={player:initial[pointer(0xD7BA)+8*player+6]for player in players};keeper=next(p for p in players if roles[p]==0);outfield=[p for p in players if roles[p]!=0]
 cases=[{'name':'original-prematch','mode':0,'patches':[]},{'name':'original-inmatch','mode':1,'patches':[]},
 {'name':'no-keeper','mode':1,'patches':[{'address':pointer(0xD7BA)+8*keeper+6,'value':1}]},
 {'name':'two-keepers','mode':1,'patches':[{'address':pointer(0xD7BA)+8*outfield[0]+6,'value':0}]},
 {'name':'overfull-role','mode':1,'patches':[{'address':pointer(0xD7BA)+8*p+6,'value':1}for p in outfield[:4]]},
 {'name':'empty-keeper-slot','mode':0,'patches':[{'address':club_at+2*players.index(keeper)+i,'value':0}for i in(0,1)]},
 {'name':'unavailable-prematch','mode':0,'patches':[{'address':pointer(0xD7BE)+15*outfield[0]+8,'value':0}]},
 {'name':'unavailable-inmatch','mode':1,'patches':[{'address':pointer(0xD7BE)+15*outfield[0]+8,'value':0}]},
 {'name':'unmanaged-invalid','mode':1,'patches':[{'address':ds+0xDCBA,'value':0},{'address':ds+0xDCBB,'value':0},{'address':pointer(0xD7BA)+8*keeper+6,'value':1}]}]
 parent_bp=regs['sp']+256
 protocol={'schema':'original-lineup-validation-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'0AB9:D29E','return':'0AB9:EBE2','active_club':club,'derived_parent_bp':parent_bp,'cases':cases,'external_warning_key':'space','presentation_line_backup':'DS:F878..F917,160bytes;source24BC:0568/0598','scope':'Derived original numeric lineup validator,valid/invalid keeper/position/availability/management cases. Observe return,role counts and whole memory. Warning text/assets excluded from records.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable lineup validation proof already started')
 oracle.write_json(start,protocol);observations=[]
 pages=[pointer(slot)for slot in(0xD08E,0xD092,0xD096,0xD09A)]
 for case in cases:
  raw=bytearray(initial);raw[ds+0xD18]=case['mode'];raw[ss+parent_bp-4]=club
  for patch in case['patches']:raw[patch['address']]=patch['value']
  sp=regs['sp']-4;raw[ss+sp:ss+sp+4]=(0xEBE2).to_bytes(2,'little')+parent_bp.to_bytes(2,'little')
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x0AB9,'IP':0xD29E,'SP':sp,'BP':parent_bp,'EFLAGS':state['flags']});g.press('space');before=g.mem(0,0x110000);counts=[]
  class Returned(Exception):pass
  def returned(g):raise Returned()
  def observe(g):counts.append({'side':g.mem(16*g.reg('ss')+g.reg('bp')-0x13,1)[0],'counts':list(g.mem(16*g.reg('ss')+g.reg('bp')-0x10,15))})
  g.code_hook('0AB9:EBE2',returned);g.code_hook('0AB9:D3CF',observe);complete=False
  try:
   for _ in range(20):g.run(max_steps=500000);g.other_polls=0
  except Returned:complete=True
  if not complete or g.unsupported:raise ValueError('original lineup validator did not return cleanly:'+case['name'])
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],parent_bp,regs['ds']):raise ValueError('original lineup validator ABI differs')
  after=g.mem(0,0x110000);changed={'stack':0,'presentation_pages':0,'presentation_ds':0,'presentation_line_backup':0,'bios_hardware':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+parent_bp+64:kind='stack'
   elif any(page<=address<page+4000 for page in pages):kind='presentation_pages'
   elif ds+0xDE00<=address<ds+0xDF00 or address-ds in(0xDD64,0xDD65,0xD5A,0xD5B,0xD088,0xD089):kind='presentation_ds'
   elif ds+0xF878<=address<ds+0xF918:kind='presentation_line_backup'
   elif 0x400<=address<0x500:kind='bios_hardware'
   else:unexpected.append(address);continue
   changed[kind]+=1
  observation={'name':case['name'],'returned_valid':g.reg('ax')&255,'role_counts':counts,'restored_active_club':after[ss+parent_bp-4],'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16]};observations.append(observation)
  if unexpected:raise ValueError('original lineup validator modifies unexpected memory:'+str(observation))
 oracle.write_json(directory/'result.json',{'status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']});oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')});print('original lineup validation completed',observations,flush=True)

def prove_replacement_banner(research,attempt=1):
 directory=ROOT/f'records/T41/replacement-banner-r{attempt:04d}'
 if(directory/'result.json').exists():
  if attempt!=2:raise ValueError('unrecognized completed banner protocol')
  replay.replacement_banner_proof();print('completed banner verified;no emulator constructed',flush=True);return
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original banner proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 def word(at):return int.from_bytes(initial[at:at+2],'little')
 def pointer(slot):return word(ds+slot)+16*word(ds+slot+2)
 club=word(ds+0xD6E4);base=pointer(0xD7C2)+62*(club-1);first=word(base+2);second=word(base)
 cases=[{'name':'healthy','first':first,'second':second,'patches':[]},{'name':'unavailable','first':first,'second':second,'patches':[{'address':pointer(0xD7BE)+15*first+8,'value':0}]}]
 for bit in(8,6,2,3):cases.append({'name':f'flag-bit{bit}','first':first,'second':second,'patches':[{'address':pointer(0xD7BE)+15*first+11,'value':1<<(bit-1)}]})
 cases.extend([{'name':'role13','first':first,'second':second,'patches':[{'address':pointer(0xD7BA)+8*first+6,'value':13}]},{'name':'reversed','first':second,'second':first,'patches':[]}])
 protocol={'schema':'original-replacement-banner-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'0AB9:ADD0','return':'0AB9:EBE2','cases':cases,'external_keys':[],'pit_divisor':1024,'pit_source':'1998:00BA/00C5→0096','maximum_instructions':100000000,'scope':'Derived original substitution announcement;verify Main is restored and only MainShown bottom line/style/stack/presentation hardware may change. Numeric style helper remains native;no name/template assets exported.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable banner proof already started')
 oracle.write_json(start,protocol);observations=[];shown=pointer(0xDD5C)
 for case in cases:
  raw=bytearray(initial)
  for patch in case['patches']:raw[patch['address']]=patch['value']
  sp=regs['sp']-6;words=[0xEBE2,case['first'],case['second']];raw[ss+sp:ss+sp+6]=b''.join(w.to_bytes(2,'little')for w in words)
  g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x0AB9,'IP':0xADD0,'SP':sp,'EFLAGS':state['flags']});g.divisor=1024;before=g.mem(0,0x110000)
  class Returned(Exception):pass
  def returned(g):raise Returned()
  g.code_hook('0AB9:EBE2',returned);complete=False
  try:
   for _ in range(200):g.run(max_steps=500000);g.other_polls=0
  except Returned:complete=True
  if not complete or g.unsupported:
   failure={'status':'aborted-before-acceptance','case':case['name'],'complete':complete,'unsupported':g.unsupported,'budget':g.budget,'game_timer_ticks':g.game_timer_ticks,'pc':g.image_address(g.reg('cs'),g.reg('ip')),'protocol_sha256':replay.sha(start)};oracle.write_json(directory/'failure.json',failure);raise ValueError('original banner did not return cleanly:'+str(failure))
  if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original banner ABI differs')
  after=g.mem(0,0x110000);changed={'stack':0,'main_shown_bottom':0,'style':0,'keyboard_driver':0,'presentation_timers':0,'bios_hardware':0};unexpected=[]
  for address,(old,new)in enumerate(zip(before,after,strict=True)):
   if old==new:continue
   if ss+sp-4096<=address<ss+regs['sp']:kind='stack'
   elif shown+0xF00<=address<shown+0xFA0:kind='main_shown_bottom'
   elif ds+0xDD64<=address<ds+0xDD66:kind='style'
   elif ds+0xDE88<=address<ds+0xDF00:kind='keyboard_driver'
   elif address-ds in(0xD5A,0xD5B,0xD088,0xD089):kind='presentation_timers'
   elif 0x400<=address<0x500:kind='bios_hardware'
   else:unexpected.append(address);continue
   changed[kind]+=1
  observation={'name':case['name'],'style_after':word_from_bytes(after,ds+0xDD64),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16]};observations.append(observation)
  if unexpected:raise ValueError('original banner modifies non-presentation memory:'+str(observation))
 oracle.write_json(directory/'result.json',{'status':'completed','protocol_sha256':replay.sha(start),'observations':observations,'scope':protocol['scope']});oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')});print('original replacement banner',observations,flush=True)

def word_from_bytes(raw,address):return int.from_bytes(raw[address:address+2],'little')

def prove_final_footer(research):
 directory=ROOT/'records/T41/final-footer-r0001'
 if(directory/'result.json').exists():raise ValueError('completed final footer exists;verify pinned records')
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original final-footer proof')
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');initial=oracle.memory(anchor['entry']);state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 protocol={'schema':'original-final-footer-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'0AB9:E1B0','return':'0AB9:EBE2','template':'DS:4B12','destination_header':'DS:DD58','external_key':'n','pit_divisor':1024,'scope':'Derived isolated original final-screen footer plus actual exit-key parsing. Numeric match cleanup remains outside this presentation probe;no original text/template bytes exported.'};start=directory/'start.json'
 if start.exists():raise ValueError('immutable final-footer proof already started')
 oracle.write_json(start,protocol);raw=bytearray(initial);sp=regs['sp']-6;words=[regs['bp'],0xEBE2,0];raw[ss+sp:ss+sp+6]=b''.join(w.to_bytes(2,'little')for w in words)
 g=guest.Guest(memory=bytes(raw),load=load,regs={k.upper():v for k,v in regs.items()}|{'CS':load+0x0AB9,'IP':0xE1B0,'SP':sp,'BP':sp,'EFLAGS':state['flags']});g.divisor=1024;g.press('n');before=g.mem(0,0x110000)
 class Returned(Exception):pass
 def returned(g):raise Returned()
 g.code_hook('0AB9:EBE2',returned);complete=False
 try:
  for _ in range(20):g.run(max_steps=500000);g.other_polls=0
 except Returned:complete=True
 if not complete or g.unsupported:raise ValueError('original final footer did not return cleanly')
 if(g.reg('sp'),g.reg('bp'),g.reg('ds'))!=(regs['sp'],regs['bp'],regs['ds']):raise ValueError('original final-footer ABI differs')
 after=g.mem(0,0x110000);page=word_from_bytes(before,ds+0xDD58)+16*word_from_bytes(before,ds+0xDD5A);changed={'stack':0,'main':0,'keyboard_driver':0,'presentation_timers':0,'bios_hardware':0};unexpected=[]
 for address,(old,new)in enumerate(zip(before,after,strict=True)):
  if old==new:continue
  if ss+sp-4096<=address<ss+regs['sp']:kind='stack'
  elif page<=address<page+4000:kind='main'
  elif ds+0xDE88<=address<ds+0xDF00:kind='keyboard_driver'
  elif address-ds in(0xD5A,0xD5B,0xD088,0xD089):kind='presentation_timers'
  elif 0x400<=address<0x500:kind='bios_hardware'
  else:unexpected.append(address);continue
  changed[kind]+=1
 result={'status':'completed'if not unexpected else'failed-footprint','protocol_sha256':replay.sha(start),'key_code':word_from_bytes(after,ds+0xDE88),'last_key_code':word_from_bytes(after,ds+0xDE8A),'pending_key':word_from_bytes(after,ds+0xDE8C),'queue_count':word_from_bytes(after,ds+0xDE8E),'returned_sp':g.reg('sp'),'returned_bp':g.reg('bp'),'returned_ds':g.reg('ds'),'changed_bytes_by_area':changed,'all_other_memory_equal':not unexpected,'unexpected_change_count':len(unexpected),'unexpected_first_addresses':unexpected[:16],'scope':protocol['scope']};oracle.write_json(directory/'result.json',result);oracle.write_json(directory/'manifest.json',{'start_sha256':replay.sha(start),'result_sha256':replay.sha(directory/'result.json')})
 if unexpected:raise ValueError('original final footer modifies gameplay memory')
 print('original final footer and exit key',result,flush=True)

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--select',type=Path);p.add_argument('--prove-transient',type=Path);p.add_argument('--prove-statistics',type=Path);p.add_argument('--prove-templates',type=Path);p.add_argument('--prove-ui-leaves',type=Path);p.add_argument('--prove-formation',type=Path);p.add_argument('--prove-grouped-integer',type=Path);p.add_argument('--prove-fixture-title',type=Path);p.add_argument('--prove-menu-input',type=Path);p.add_argument('--prove-lineup-validation',type=Path);p.add_argument('--prove-replacement-banner',type=Path);p.add_argument('--prove-final-footer',type=Path);p.add_argument('--replacement-banner-attempt',type=int,default=1);p.add_argument('--lineup-validation-attempt',type=int,default=1);p.add_argument('--statistics-wait',action='store_true');p.add_argument('--statistics-mode',type=int,choices=(0,1),default=0);a=p.parse_args()
 if a.select:select(a.select)
 if a.prove_transient:prove_transient(a.prove_transient)
 if a.prove_statistics:prove_statistics(a.prove_statistics,a.statistics_wait,a.statistics_mode)
 if a.prove_templates:prove_templates(a.prove_templates)
 if a.prove_ui_leaves:prove_ui_leaves(a.prove_ui_leaves)
 if a.prove_formation:prove_formation(a.prove_formation)
 if a.prove_grouped_integer:prove_grouped_integer(a.prove_grouped_integer)
 if a.prove_fixture_title:prove_fixture_title(a.prove_fixture_title)
 if a.prove_menu_input:prove_menu_input(a.prove_menu_input)
 if a.prove_lineup_validation:prove_lineup_validation(a.prove_lineup_validation,a.lineup_validation_attempt)
 if a.prove_replacement_banner:prove_replacement_banner(a.prove_replacement_banner,a.replacement_banner_attempt)
 if a.prove_final_footer:prove_final_footer(a.prove_final_footer)
 if not(a.select or a.prove_transient or a.prove_statistics or a.prove_templates or a.prove_ui_leaves or a.prove_formation or a.prove_grouped_integer or a.prove_fixture_title or a.prove_menu_input or a.prove_lineup_validation or a.prove_replacement_banner or a.prove_final_footer):p.error('choose selection or original transient proof')
if __name__=='__main__':main()
