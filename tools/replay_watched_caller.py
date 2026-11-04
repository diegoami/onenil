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
 def compare(self,state,outputs,clock_only,base_draw):
  make_expected=expected_builder(state,base_draw)
  self.p.stdin.write(json.dumps({'initial':state,'steps':len(outputs),'clock_only':clock_only,'no_pending_key':True,'stream':True})+'\n');self.p.stdin.flush()
  for index,source in enumerate(outputs):
   line=self.p.stdout.readline()
   if not line:raise ValueError('native caller probe stopped unexpectedly')
   packet=json.loads(line)
   if 'error'in packet:return packet['error']
   if packet['step']!=index:raise ValueError('native caller stream index differs')
   actual=packet['ok'];want=make_expected(source)
   if actual!=want:
    fields=[key for key in want if actual[key]!=want[key]]
    target=ROOT/'.cache/T40/caller-parity-mismatch.json';target.write_text(json.dumps({'step':index+1,'fields':fields,'actual':{k:actual[k]for k in fields},'expected':{k:want[k]for k in fields}},indent=2));raise ValueError(f'caller step{index+1} {fields} differs;private diagnostic: {target}')
  if json.loads(self.p.stdout.readline())!={'done':len(outputs)}:raise ValueError('native caller stream completion differs')
  return None
 def close(self):self.p.stdin.close();assert self.p.wait()==0

def expected(source,state,base_draw):
 output=tick_replay.expected(source,state)
 # Caller outputs are part of one persistent session,not isolated per-step calls.
 for event in output['events']:
  for key in('draw_start','draw_end'):event[key]-=base_draw
 output['phase_complete']=source['phase_complete']
 return output

def expected_builder(state,base_draw):
 # Physical field locations and ownership do not depend on an expected outcome.
 locations=[(f,tick_replay.address(state,f['area'],f['offset']))for f in state['cells']if f['area']!='frame']
 known={at for _,at in locations};ds=16*state['regs'][11]
 known.update(range(ds+0xCCEF+25,ds+0xCCEF+50));known.update(range(ds+0xA51C,ds+0xA51E))
 for slot in [*tick_replay.facts.SLOTS.values(),0xD0C0]:known.update(range(ds+slot,ds+slot+4))
 def make(source):
  reference=source['exit'];key=reference['ram_sha256']
  if key not in EXIT_CACHE:EXIT_CACHE[key]=oracle.memory(reference)
  raw=EXIT_CACHE[key]
  for change in source['changes']:
   if tick_replay.address(source.get('context',state),change['area'],change['offset'])not in known:raise ValueError('original changed undeclared gameplay field')
  out=copy.deepcopy(source['output']);out['cells']=[{**f,'value':raw[at]}for f,at in locations];out['draws']=source['draws'];out['events']=copy.deepcopy(source['events'])
  for event in out['events']:
   for field in('draw_start','draw_end'):event[field]-=base_draw
  out['phase_complete']=source['phase_complete'];return out
 return make
EXIT_CACHE={}

def admission(anchor,clock_case=None):
 raw=bytearray(oracle.memory(anchor['entry']));state=copy.deepcopy(anchor['context']);state['cells']=[]
 if clock_case is not None:
  ds=16*state['regs'][11]
  for key,value in clock_case['input_overlay'].items():
   offset,width=caller.CLOCK_FIELDS[key];raw[ds+offset:ds+offset+width]=value.to_bytes(width,'little')
  raw[ds+0xD0B9]=0
 return bytes(raw),state

def initial_fields(raw,state):
    """[designed] Batch typed logical-pitch fields and prior validated field locations.
    Every value is re-decoded from THIS initial ENTRY;no EXIT/helper result is used.
    """
    ds=16*state['regs'][11];seen=set();reserved=set(range(ds+0xCCEF+25,ds+0xCCEF+50))|set(range(ds+0xA51C,ds+0xA51E))
    for slot in [*tick_replay.facts.SLOTS.values(),0xD0C0]:reserved.update(range(ds+slot,ds+slot+4))
    def add(at):
        if at in seen or at in reserved:return
        area,offset=tick.named(state,at);seen.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
    for i in range(2,23):
        for k in range(25):add(ds+0xCCEF+25*i+k)
    for area in('main','main_shown','background','background_shown'):
        for row in range(-9,10):
            for offset in range(0x90E+160*row-74,0x90E+160*row+76):add(tick_replay.address(state,area,offset))
    # Conservative typed match controls/tables;hardware presentation counters stay excluded.
    for lo,hi in((0x380,0x382),(0x900,0xC00),(0xD00,0xD80),(0xCCEF,0xD7BA)):
        for offset in range(lo,hi):
            if offset not in(0xD5A,0xD5B,0xD088,0xD089):add(ds+offset)
    # Numeric commentary rows:code,primary,secondary,side;not rendered strings.
    for offset in range(0xA52A,0xA52A+6*65):add(ds+offset)
    # The two selected clubs'20-player rosters and their typed8/15-byte player records.
    for side in(0,1):
        club=int.from_bytes(raw[ds+0xD6E4+2*side:ds+0xD6E6+2*side],'little')
        row=tick_replay.address(state,'club',62*(club-1))
        for offset in range(40):add(row+offset)
        for slot in range(20):
            player=int.from_bytes(raw[row+2*slot:row+2*slot+2],'little')
            if 0<player<=3920:
                for area,length in(('p8',8),('p15',15)):
                    for offset in range(length):add(tick_replay.address(state,area,length*player+offset))
    # F33/9F6E table-address witnesses,not a score/gameplay calculation.
    # Only addresses derive from ENTRY attributes/table bytes;every supplied value is ENTRY data.
    for side in(0,1):
        club=int.from_bytes(raw[ds+0xD6E4+2*side:ds+0xD6E6+2*side],'little');row=tick_replay.address(state,'club',62*(club-1))
        for slot in range(20):
            player=int.from_bytes(raw[row+2*slot:row+2*slot+2],'little')
            if not 0<player<=3920:continue
            attributes=tick_replay.address(state,'p8',8*player)
            for criterion in range(256):
                first=ds+((0x957D+22*raw[attributes+7]+criterion)&65535);add(first)
                second=ds+((0x96B1+22*raw[attributes+5]+raw[first])&65535);add(second)
    for path in sorted((ROOT/'engine/tests/fixtures/watched_tick').glob('case-*.json')):
        previous=read(path)['initial']
        for f in previous['cells']:
            at=tick_replay.address(previous,f['area'],f['offset'])
            add(at)
    return seen

def export(probe):
 (actual,am),(clocks,cm)=sources();anchor=actual[0];FIXTURES.mkdir(parents=True,exist_ok=True);native=Probe(probe);items=[]
 cache=ROOT/'.cache/T40/selection';cache.mkdir(parents=True,exist_ok=True)
 catalog=[(actual,False,None)]+[([c],True,c)for c in clocks]
 for number,(outputs,clock_only,clock_case)in enumerate(catalog):
  path=FIXTURES/f'case-{number:02d}.json'
  if not path.exists():
   raw,state=admission(anchor,clock_case);saved=cache/f'session-{number:02d}.json';seen=initial_fields(raw,state)
   if saved.exists():
    prior=read(saved)
    for f in prior['cells']:
     at=tick_replay.address(state,f['area'],f['offset'])
     if raw[at]!=f['value']:raise ValueError('cached fact differs from THIS original ENTRY')
     if at not in seen:seen.add(at);state['cells'].append(f)
   base=actual[0]['draw_start']if not clock_only else 0
   for attempt in range(30000):
    # Checkpoint candidate inputs before each replay;these are ENTRY facts only.
    saved.write_text(json.dumps({'cells':state['cells']}))
    error=native.compare(state,outputs,clock_only,base)
    if error is None:break
    missing=error['missing']
    if missing is None:raise ValueError(f'caller session{number} unsupported: '+error['message'])
    try:area,offset=tick.named(state,missing['address'])
    except ValueError as e:raise ValueError(f'caller session{number} unclassified: '+str(error))from e
    at=tick_replay.address(state,area,offset)
    if at in seen:raise ValueError('native caller repeated missing initial fact:'+str(error))
    offsets=[offset]
    if area in('p8','p15'):
        length=8 if area=='p8'else 15;offsets=list(range(offset//length*length,offset//length*length+length))
    elif area=='ds'and 0xA52A<=offset<0xA52A+6*353:
        start=0xA52A+(offset-0xA52A)//6*6;offsets=list(range(start,start+6))
    for value_offset in offsets:
        value_at=tick_replay.address(state,area,value_offset)
        if value_at not in seen:seen.add(value_at);state['cells'].append({'area':area,'offset':value_offset,'value':raw[value_at]})
    if attempt%8==0:print('native batched ENTRY union session',number,'facts',len(state['cells']),'additional',attempt+1,flush=True)
   else:raise ValueError('native caller initial fact selection did not converge')
   source_items=am['cases']if not clock_only else[cm['cases'][number-1]]
   oracle.write_json(path,{'case':number,'mode':'derived-clock-boundary'if clock_only else'actual-persistent-caller','initial':state,'steps':len(outputs),'clock_only':clock_only,'no_pending_key':True,'event_draw_base':base,'originals':source_items,'expected_format':'full-cell snapshots reconstructed from original initial values and sparse original changes;other outputs from original observations'})
  items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('native caller session',number,'matched',len(outputs),'original transitions',flush=True)
 native.close();manifest={'schema':'decoded-native-watched-caller-sparse/1','producer':oracle.common.git('rev-parse','HEAD'),'actual_manifest_sha256':sha(ROOT/'records/T40/oracle-r0001/manifest.json'),'clock_manifest_sha256':sha(ROOT/'records/T40/clock-r0001/manifest.json'),'cases':items,'scope':'One initial ENTRY union per session;all declared cells compared each step. Sparse original changes avoid duplicating unchanged initial facts. No intermediate runtime inputs.'};path=FIXTURES/'manifest.json'
 if not path.exists():oracle.write_json(path,manifest)
 elif read(path)!=manifest:raise ValueError('native caller manifest differs')

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path,required=True);a=p.parse_args()
 if a.export:export(a.probe)
 else:p.error('choose export')
if __name__=='__main__':main()
