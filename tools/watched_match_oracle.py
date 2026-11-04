"""[designed] Select numeric initial facts from the single retained original ENTRY.
No clock/gameplay calculation;native missing-input diagnostics select locations only.
"""
import argparse,copy,json,subprocess
from pathlib import Path
import watched_oracle as oracle
import replay_watched_tick as ticks
import replay_watched_match as replay
ROOT=oracle.ROOT
SLOTS={'results':(0xD7CE,58*176),'fixtures':(0xD7CA,174*176)}
def read(p):return json.loads(p.read_text())
def address(state,area,offset):
 if area in SLOTS or area=='style':
  p=next(p for p in state['pointers']if p['kind']==area);return 16*p['segment']+p['offset']+offset
 if area=='frame':return 16*state['regs'][10]+state['regs'][4]+offset
 return ticks.address(state,area,offset)
def named(state,at):
 frame=16*state['regs'][10]+state['regs'][4]
 if -4096<=at-frame<64:return 'frame',at-frame
 ds=16*state['regs'][11]
 if 0<=at-ds<65536:return 'ds',at-ds
 for name,(_,length)in SLOTS.items():
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
 if saved.exists():state=read(saved)
 for fact in state['cells']:
  if raw[address(state,fact['area'],fact['offset'])]!=fact['value']:raise ValueError('cached initial fact differs from original ENTRY')
 known={address(state,f['area'],f['offset'])for f in state['cells']}
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
   if missing is None:raise ValueError('native match unsupported:'+error['message'])
   area,offset=named(state,missing['address']);at=address(state,area,offset)
   if at in known:raise ValueError('native repeated missing initial fact:'+str(error))
   offsets=[offset]
   if area in('p8','p15'):
    length=8 if area=='p8'else 15;offsets=range(offset//length*length,offset//length*length+length)
   elif area=='ds'and 0xA52A<=offset<0xCCEF:
    start=0xA52A+(offset-0xA52A)//6*6;offsets=range(start,start+6)
   else:offsets=range(offset,offset+2)
   for field in offsets:
    at=address(state,area,field)
    if at not in known:known.add(at);state['cells'].append({'area':area,'offset':field,'value':raw[at]})
   if attempt%8==0:print('initial ENTRY union',len(state['cells']),'facts;native diagnostic',error,flush=True)
  raise ValueError('full native match admission did not converge')
 finally:p.stdin.close();p.wait()
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--select',type=Path,required=True);a=p.parse_args();select(a.select)
if __name__=='__main__':main()
