"""[designed] Replay one persistent native watched caller against original outcomes."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
import watched_caller_oracle as caller
import watched_tick_oracle as tick
import watched_oracle as oracle
import replay_watched_tick as tick_replay
ROOT=oracle.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/watched_caller'

def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def sources():
 values=[]
 for name in('oracle-r0001','clock-r0001'):
  directory=ROOT/'records/T40'/name;manifest=read(directory/'manifest.json')
  if manifest['status']!='completed'or sha(directory/'start.json')!=manifest['protocol_sha256']:raise ValueError('original caller/clock protocol differs')
  cases=[]
  for item in manifest['cases']:
   path=ROOT/item['path']
   if sha(path)!=item['sha256']:raise ValueError('original caller/clock case differs')
   cases.append(read(path))
  values.append((cases,manifest))
 return values
class Probe:
 def __init__(self,path):self.p=subprocess.Popen([str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
 def run(self,initial,steps,clock_only):
  self.p.stdin.write(json.dumps({'initial':initial,'steps':steps,'clock_only':clock_only,'no_pending_key':True})+'\n');self.p.stdin.flush();line=self.p.stdout.readline()
  if not line:raise ValueError('native caller probe stopped unexpectedly')
  return json.loads(line)
 def close(self):self.p.stdin.close();assert self.p.wait()==0

def expected(source,state,base_draw):
 output=tick_replay.expected(source,state)
 # Caller outputs are part of one persistent session,not isolated per-step calls.
 for event in output['events']:
  for key in('draw_start','draw_end'):event[key]-=base_draw
 output['phase_complete']=source['phase_complete']
 return output

def admission(anchor,clock_case=None):
 raw=bytearray(oracle.memory(anchor['entry']));state=copy.deepcopy(anchor['context']);state['cells']=[]
 if clock_case is not None:
  ds=16*state['regs'][11]
  for key,value in clock_case['input_overlay'].items():
   offset,width=caller.CLOCK_FIELDS[key];raw[ds+offset:ds+offset+width]=value.to_bytes(width,'little')
  raw[ds+0xD0B9]=0
 return bytes(raw),state

def export(probe):
 (actual,am),(clocks,cm)=sources();anchor=actual[0];FIXTURES.mkdir(parents=True,exist_ok=True);native=Probe(probe);items=[]
 catalog=[(actual,False,None)]+[([c],True,c)for c in clocks]
 for number,(outputs,clock_only,clock_case)in enumerate(catalog):
  path=FIXTURES/f'case-{number:02d}.json'
  if not path.exists():
   raw,state=admission(anchor,clock_case);ds=16*state['regs'][11];seen=set()
   for i in range(2,23):
    for k in range(25):
     offset=0xCCEF+25*i+k;at=ds+offset;seen.add(at);state['cells'].append({'area':'ds','offset':offset,'value':raw[at]})
   for attempt in range(30000):
    got=native.run(state,len(outputs),clock_only)
    if 'ok'in got:break
    error=got['error'];missing=error['missing']
    if missing is None:raise ValueError(f'caller session{number} unsupported: '+error['message'])
    try:area,offset=tick.named(state,missing['address'])
    except ValueError as e:raise ValueError(f'caller session{number} unclassified: '+str(error))from e
    at=tick_replay.address(state,area,offset)
    if at in seen:raise ValueError('native caller repeated missing initial fact:'+str(error))
    seen.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
    if attempt%256==0:print('native initial fact union session',number,'facts',len(state['cells']),flush=True)
   else:raise ValueError('native caller initial fact selection did not converge')
   base=actual[0]['draw_start']if not clock_only else 0
   want=[expected(c,state,base)for c in outputs]
   if got['ok']!=want:
    for step,(a,b)in enumerate(zip(got['ok'],want,strict=True),1):
     for field in b:
      if a[field]!=b[field]:
       target=ROOT/'.cache/T40/caller-parity-mismatch.json';target.write_text(json.dumps({'session':number,'step':step,'field':field,'actual':a[field],'expected':b[field]},indent=2));raise ValueError(f'caller session{number} step{step} {field} differs;private diagnostic: {target}')
   oracle.write_json(path,{'case':number,'mode':'derived-clock-boundary'if clock_only else'actual-persistent-caller','initial':state,'steps':len(outputs),'clock_only':clock_only,'no_pending_key':True,'expected':want})
  items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('native caller session',number,'matched',len(outputs),'original transitions',flush=True)
 native.close();manifest={'schema':'decoded-native-watched-caller/1','producer':oracle.common.git('rev-parse','HEAD'),'actual_manifest_sha256':sha(ROOT/'records/T40/oracle-r0001/manifest.json'),'clock_manifest_sha256':sha(ROOT/'records/T40/clock-r0001/manifest.json'),'cases':items,'scope':'One initial ENTRY fact union per session. Clock input overlays are offline derived cases;runtime advance has no intermediate inputs.'};path=FIXTURES/'manifest.json'
 if not path.exists():oracle.write_json(path,manifest)
 elif read(path)!=manifest:raise ValueError('native caller manifest differs')

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path,required=True);a=p.parse_args()
 if a.export:export(a.probe)
 else:p.error('choose export')
if __name__=='__main__':main()
