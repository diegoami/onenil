"""[designed] Replay one persistent native watched caller against original outcomes."""
import argparse,copy,hashlib,json,subprocess,time,platform
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import watched_caller_oracle as caller
import watched_tick_oracle as tick
import watched_oracle as oracle
import replay_watched_tick as tick_replay
ROOT=oracle.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/watched_caller'
ACTUAL_SHA='de9ca39c278d16539f7fa59ef893676a3c159ac2b5e66c5e58a5b6ba7d8fe7fd'
CLOCK_SHA='2bd8417fea36a46988e4b013bbef58d7d7caff93dce7adc7be58eabddf819e2f'
FIXTURE_SHA='1a39241a0ea36324e10ab726f5278a9fc3d7d44717c620f579ddf9e117c0e286'
PREFIX_SHA='0afdfe47c7244f833371ab65f25011d2a709e290a8d3b783388036146f0122bf'

def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def sources():
 values=[]
 for name in('oracle-r0001','clock-r0001'):
  directory=ROOT/'records/T40'/name
  if sha(directory/'manifest.json')!=(ACTUAL_SHA if name=='oracle-r0001'else CLOCK_SHA):raise ValueError('original caller/clock manifest pin differs')
  manifest=read(directory/'manifest.json')
  if manifest['status']!='completed'or sha(directory/'start.json')!=manifest['protocol_sha256']:raise ValueError('original caller/clock protocol differs')
  cases=[]
  for item in manifest['cases']:
   path=ROOT/item['path']
   if sha(path)!=item['sha256']:raise ValueError('original caller/clock case differs')
   cases.append(read(path))
  values.append((cases,manifest))
 actual,am=values[0];clocks,cm=values[1]
 if am['count']!=171 or len(actual)!=171 or am['accepted_actions']!=33 or am['no_key_visits']!=3762 or am['retained32prefix_joined']is not True:raise ValueError('original caller coverage/prefix differs')
 if cm['count']!=8 or len(clocks)!=8:raise ValueError('original clock coverage differs')
 for i,c in enumerate(actual,1):
  if c['iteration']!=i or c['parent_return']!=0x7DCB or c['no_key_visits']!=22 or c['context']['regs'][5]!=c['context']['regs'][4]+6:raise ValueError('original caller identity/ABI differs')
  if c['exit_registers']['sp']!=c['context']['regs'][4]or c['exit_registers']['bp']!=c['context']['regs'][5]:raise ValueError('original caller return frame differs')
  if c['draw_end']-c['draw_start']!=len(c['draws']):raise ValueError('original caller RNG interval differs')
  if i>1 and(c['input']!=actual[i-2]['output']or c['draw_start']!=actual[i-2]['draw_end']):raise ValueError('original caller continuity differs')
 for i,c in enumerate(clocks):
  if c['case']!=i or c['input_overlay']!=caller.CLOCK_CASES[i]or c['admission_sha256']!=am['cases'][0]['sha256']:raise ValueError('derived original clock admission differs')
 path=ROOT/'records/T40/reference-prefix-r0001.json'
 if sha(path)!=PREFIX_SHA:raise ValueError('retained original prefix pin differs')
 prefix=read(path);protocol=read(ROOT/'records/T40/oracle-r0001/start.json')
 if prefix['source']!=protocol['reference_prefix']:raise ValueError('original caller prefix source differs')
 draws=prefix['setup_draws']+[d for c in actual for d in c['draws']];n=prefix['prefix_draw_count']
 if len(prefix['setup_draws'])!=258 or n!=7248 or len(draws)<n or hashlib.sha256(json.dumps(draws[:n],sort_keys=True,separators=(',',':')).encode()).hexdigest()!=prefix['normalized_prefix_sha256']:raise ValueError('retained7248draw prefix differs')
 actions=[]
 for c in actual:
  for e in c['events']:actions.append(dict(e,tick=c['iteration'],event=len(actions)))
 if len(actions)!=33 or actions[:32]!=prefix['actions']:raise ValueError('retained32accepted actions differ')
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
 if (FIXTURES/'manifest.json').exists():return check()
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

def check():
 (actual,am),(clocks,cm)=sources();manifest=read(FIXTURES/'manifest.json')
 if sha(FIXTURES/'manifest.json')!=FIXTURE_SHA:raise ValueError('native caller fixture manifest pin differs')
 if manifest['actual_manifest_sha256']!=ACTUAL_SHA or manifest['clock_manifest_sha256']!=CLOCK_SHA or len(manifest['cases'])!=9:raise ValueError('native caller corpus provenance differs')
 values=[];anchor=actual[0]
 for number,item in enumerate(manifest['cases']):
  path=ROOT/item['path']
  if sha(path)!=item['sha256']:raise ValueError('native caller fixture digest differs')
  v=read(path);state=v['initial'];outputs=actual if number==0 else[clocks[number-1]];source_items=am['cases']if number==0 else[cm['cases'][number-1]]
  if v['case']!=number or v['steps']!=len(outputs)or v['clock_only']!=(number>0)or v['no_pending_key']is not True or v['originals']!=source_items or v['event_draw_base']!=(258 if number==0 else 0):raise ValueError('decoded native caller source/context differs')
  for key in anchor['context']:
   if key!='cells'and state[key]!=anchor['context'][key]:raise ValueError('native initial caller admission differs')
  # Expected state derives only from original ENTRY values and sparse original writes.
  for _ in public_outputs(v,outputs):pass
  values.append(v)
 return values

def public_outputs(value,outputs):
 state=value['initial'];facts=tick_replay.values_at_entry(state)
 for source in outputs:
  if source.get('context',state)['pointers']!=state['pointers']or source['output']['pointers']!=state['pointers']:raise ValueError('caller corpus pointer mapping changed')
  for d in source['changes']:
   if set(d)!={'area','offset','before','after'}or type(d['before'])is not int or type(d['after'])is not int or not(0<=d['before']<=255 and 0<=d['after']<=255):raise ValueError('invalid decoded original caller change')
   key=d['area'],d['offset']
   if key not in facts or facts[key]!=d['before']:raise ValueError('original caller sparse baseline/ownership differs')
   facts[key]=d['after']
  def word_at(offset,width):return sum(facts['ds',offset+i]<<(8*i)for i in range(width))
  decoded_clock={key:word_at(offset,width)for key,(offset,width)in caller.CLOCK_FIELDS.items()}
  if decoded_clock!=source['output']['clock']or word_at(0xA51C,2)!=source['output']['seed']:raise ValueError('original clock/seed snapshot does not agree with sparse physical writes')
  for scene in source['output']['scenes']:
   for key,field_value in scene.items():
    if key!='index'and word_at(0xCCEF+25*scene['index']+int(key[4:]),2 if key.startswith('word')else 1)!=field_value:raise ValueError('original scene snapshot does not agree with physical writes')
  out=copy.deepcopy(source['output']);out['cells']=[{**f,'value':facts[f['area'],f['offset']]}for f in state['cells']if f['area']!='frame'];out['draws']=source['draws'];out['events']=copy.deepcopy(source['events']);out['phase_complete']=source['phase_complete']
  for e in out['events']:
   for field in('draw_start','draw_end'):e[field]-=value['event_draw_base']
  yield out

def verify_private(values):
 (actual,_),(clocks,_)=sources();anchor=actual[0]
 for i,v in enumerate(values):
  raw,context=admission(anchor,None if i==0 else clocks[i-1]);state=v['initial'];seen=set()
  for f in state['cells']:
   at=tick_replay.address(state,f['area'],f['offset'])
   if at in seen or tick.named(state,at)!=(f['area'],f['offset'])or raw[at]!=f['value']:raise ValueError('native caller fact is not a unique initial ENTRY value')
   seen.add(at)
  outputs=actual if i==0 else[clocks[i-1]];make=expected_builder(state,v['event_draw_base'])
  for source,want in zip(outputs,public_outputs(v,outputs),strict=True):
   if make(source)!=want:raise ValueError('public sparse expectation differs from actual original exit/ownership')
 print('all9 caller sessions/179original transitions:initial ENTRY facts,exit expectations and ownership verified',flush=True)

def replay(values):
 import onenil_engine as native
 (actual,_),(clocks,_)=sources()
 for i,v in enumerate(values):
  outputs=actual if i==0 else[clocks[i-1]]
  if i==0:
   session=native.NativeWatchedCallerSession(json.dumps(v['initial']),v['no_pending_key'])
   for step,want in enumerate(public_outputs(v,outputs),1):
    got=json.loads(session.step())
    if got!=want:raise ValueError(f'native caller iteration{step} mismatch')
  else:
   request={k:v[k]for k in('initial','steps','clock_only','no_pending_key')};got=json.loads(native.watched_caller(json.dumps(request)))
   if got.get('ok')!=list(public_outputs(v,outputs)):raise ValueError(f'native clock boundary{i-1} mismatch:'+str(got.get('error')))
 print('native caller:171persistent iterations/33actions and8clock boundaries matched',flush=True)

def check_research(research):
 (actual,_),_=sources();protocol=read(ROOT/'records/T40/oracle-r0001/start.json')
 if sha(research/'traces/T40/index.json')!=protocol['reference_index_sha256']:raise ValueError('complete original reference index differs')
 ticks=[];draws=[];actions=[]
 for item in protocol['reference_chunks']:
  path=research/item['path']
  if sha(path)!=item['sha256']:raise ValueError('complete original reference chunk differs')
  chunk=read(path);payload=chunk['payload']
  if hashlib.sha256(json.dumps(payload,sort_keys=True,separators=(',',':')).encode()).hexdigest()!=chunk['payload_sha256']:raise ValueError('complete original reference payload differs')
  ticks+=payload['ticks'];draws+=payload['draws'];actions+=payload['actions']
 def decoded(scene):
  raw=bytearray(25)
  for key,value in scene.items():
   if key!='index':
    width=2 if key.startswith('word')else 1;offset=int(key[4:]);raw[offset:offset+width]=value.to_bytes(width,'little')
  return raw
 for i,c in enumerate(actual):
  for side,reference in [('input',ticks[i]['before']),('output',ticks[i+1]['before'])]:
   observed=c[side]
   if observed['seed']!=reference['seed']or[decoded(scene)for scene in observed['scenes']]!=[decoded(scene)for scene in reference['scenes']]:raise ValueError('original caller/reference scenes or seed differ')
   for key in ('active','remaining','stoppage','elapsed','phase','hold'):
    if observed['clock'][key]!=reference[key]:raise ValueError('original caller/reference logical clock differs')
  if(c['draw_start'],c['draw_end'])!=(ticks[i]['draw_start'],ticks[i+1]['draw_start']):raise ValueError('original caller/reference draw cursors differ')
  if c['draws']!=[oracle.normalized(d)for d in draws[c['draw_start']:c['draw_end']]]:raise ValueError('original caller/reference ordered draws differ')
  expected=[{k:v for k,v in event.items()if k not in('tick','event')}for event in actions if event['tick']==i+1]
  if c['events']!=expected:raise ValueError('original caller/reference accepted actions differ')
 print('171original caller entries/cutpoints join pinned complete reference scenes,clock,RNG and33actions',flush=True)

def benchmark(values,repetitions):
 from onenil_engine import onenil_engine as native
 if oracle.common.git('status','--porcelain','--','engine','tools','pyproject.toml'):raise ValueError('commit native benchmark sources first')
 if native.__debug_assertions__:raise ValueError('release extension required')
 initial=json.dumps(values[0]['initial'],sort_keys=True,separators=(',',':'))
 clocks=[json.dumps({k:v[k]for k in('initial','steps','clock_only','no_pending_key')},sort_keys=True,separators=(',',':'))for v in values[1:]]
 began=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();size=0
 for _ in range(repetitions):
  session=native.NativeWatchedCallerSession(initial,True)
  for _ in range(values[0]['steps']):size+=len(session.step())
  for request in clocks:size+=len(native.watched_caller(request))
 elapsed=time.perf_counter_ns()-start;directory=ROOT/'records/T40';n=len(list(directory.glob('T40-r*.json')))+1;path=directory/f'T40-r{n:04d}.json'
 oracle.write_json(path,{'schema':'routine-benchmark/1','code_commit':oracle.common.git('rev-parse','HEAD'),'started':began,'elapsed_ns':elapsed,'sessions':len(values)*repetitions,'transitions':179*repetitions,'repetitions':repetitions,'serialized_output_bytes':size,'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'extension_sha256':sha(Path(native.__file__)),'cargo_lock_sha256':sha(ROOT/'engine/Cargo.lock'),'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'pins':{'actual':ACTUAL_SHA,'clock':CLOCK_SHA,'fixtures':FIXTURE_SHA},'method':'Pre-serialized initial admissions;native parsing,one persistent image,171computed caller advances and serialization;8independent clock cases per repetition','limits':'First171caller iterations and derived clock cases only;no full match,lifecycle,viewer or emulator-speed comparison'})
 print(path.relative_to(ROOT),flush=True)

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path);p.add_argument('--check',action='store_true');p.add_argument('--check-private',action='store_true');p.add_argument('--replay',action='store_true');p.add_argument('--check-research',type=Path);p.add_argument('--benchmark',action='store_true');p.add_argument('--repetitions',type=int,default=3);a=p.parse_args()
 if a.repetitions<1:p.error('positive repetitions required')
 if a.check_research:check_research(a.check_research)
 if a.export:
  if a.probe is None:p.error('--export requires --probe')
  export(a.probe)
 if a.check or a.check_private or a.replay or a.benchmark:
  values=check();print('decoded caller corpus:171actual iterations/8clock boundaries verified',flush=True)
  if a.check_private:verify_private(values)
  if a.replay or a.benchmark:replay(values)
  if a.benchmark:benchmark(values,a.repetitions)
 if not(a.export or a.check or a.check_private or a.replay or a.benchmark or a.check_research):p.error('choose export/check/private/replay')
if __name__=='__main__':main()
