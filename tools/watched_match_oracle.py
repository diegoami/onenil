"""[designed] Select numeric initial facts from the single retained original ENTRY.
No clock/gameplay calculation;native missing-input diagnostics select locations only.
"""
import argparse,copy,json,subprocess,sys
from pathlib import Path
import watched_oracle as oracle
import replay_watched_tick as ticks
import replay_watched_match as replay
ROOT=oracle.ROOT
SLOTS={'results':(0xD7CE,58*176),'fixtures':(0xD7CA,174*176),'screen':(0xDD58,4000),'screen_shown':(0xDD5C,4000),'screen_saved':(0xDD60,4000)}
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
  if any(lo<=offset<hi for lo,hi in[(0x380,0x382),(0x900,0xC00),(0xD00,0xDA0),(0x957D,0xD7BA),(0xD7D6,0xD7DA),(0xDCBA,0xDCBC),(0xDD64,0xDD66),(0x0C66,0x0C68),(0x93F7,0x94F7),(0xD886,0xD936)])or offset in(0xDD56,0xDE88,0xDE8A,0xE5DD):return 'ds',offset
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
 batches=[('ds',range(0xA52A,0xA52A+6*1701)),('ds',range(0xC66,0xC68))]
 for side in(0,1):
  club=int.from_bytes(raw[ds+0xD6E4+2*side:ds+0xD6E6+2*side],'little')
  baseline=0xD885+club;table=0x93F7+raw[ds+baseline];batches.extend([('ds',[baseline]),('ds',[table])])
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
   if missing is None:raise ValueError('native match unsupported:'+error['message'])
   if 'missing initial native write baseline'in error['message']and not missing['writing']:raise ValueError('native diagnostic overwrote the original missing-write descriptor')
   (cache/'last-diagnostic.json').write_text(json.dumps(error,indent=2));area,offset=named(state,missing['address']);at=address(state,area,offset)
   if at in known:raise ValueError('native repeated missing initial fact:'+str(error))
   offsets=[offset]
   if area in('p8','p15'):
    length=8 if area=='p8'else 15;offsets=range(offset//length*length,offset//length*length+length)
   elif area=='ds'and 0xA52A<=offset<0xCCEF:
    start=0xA52A+(offset-0xA52A)//6*6;offsets=range(start,start+6)
   else:offsets=range(offset,offset+(1 if area=='ds'and offset==0xE5DD else 2))
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

def prove_statistics(research,wait=False):
 if oracle.common.git('status','--porcelain','--','tools/watched_match_oracle.py'):raise ValueError('commit observer before original statistics proof')
 import sys
 sys.path.insert(0,str(research.resolve()/'runtime/emul'));import guest,match
 anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');raw=bytearray(oracle.memory(anchor['entry']));state=anchor['context'];regs=dict(zip(oracle.REGS,state['regs']));ds=16*regs['ds'];ss=16*regs['ss'];load=regs['cs']-0x0AB9
 # Explicitly derived original call:statistics variant0,no wait,and a presentation key value7.
 # Original clears the key at0812;this is not a historical halftime-state capture.
 sp=regs['sp']-8;raw[ss+sp:ss+sp+8]=(0x7E2F).to_bytes(2,'little')+regs['cs'].to_bytes(2,'little')+bytes(2)+int(wait).to_bytes(2,'little');raw[ds+0xDE88]=7
 directory=ROOT/('records/T41/statistics-boundary-r0002'if wait else'records/T41/statistics-boundary-r0001');directory.mkdir(parents=True,exist_ok=True)
 protocol={'schema':'original-statistics-boundary-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':replay.sha(Path(__file__)),'admission_sha256':replay.sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'),'root':'1998:045C','return':'0AB9:7E2F','arguments':[0,int(wait)],'external_key':'Return'if wait else None,'initial_presentation_key':7,'scope':'Derived isolated original statistics display from retained initial ENTRY;check complete memory-change footprint. Not a historical halftime capture.'};start=directory/'start.json'
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

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--select',type=Path);p.add_argument('--prove-transient',type=Path);p.add_argument('--prove-statistics',type=Path);p.add_argument('--statistics-wait',action='store_true');a=p.parse_args()
 if a.select:select(a.select)
 if a.prove_transient:prove_transient(a.prove_transient)
 if a.prove_statistics:prove_statistics(a.prove_statistics,a.statistics_wait)
 if not(a.select or a.prove_transient or a.prove_statistics):p.error('choose selection or original transient proof')
if __name__=='__main__':main()
