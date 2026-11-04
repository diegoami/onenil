"""[designed] Full original watched reference assertions;never a gameplay model."""
import argparse,hashlib,json,subprocess,time,platform
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
REFERENCE=ROOT/'records/T41/reference'
INDEX_SHA='30a4fe8003f4677c2e2af0966d5fe81055c728a1b41c998a9d03766637414886'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())
def sources():
 if sha(REFERENCE/'index.json')!=INDEX_SHA:raise ValueError('full original reference index differs')
 index=read(REFERENCE/'index.json');ticks=[];draws=[];events=[]
 for item in[index['start'],*index['chunks'],index['finish']]:
  path=REFERENCE/Path(item['path']).name
  if sha(path)!=item['sha256']:raise ValueError('full original reference source differs')
  value=read(path)
  if 'payload'in value:
   payload=value['payload']
   if hashlib.sha256(json.dumps(payload,sort_keys=True,separators=(',',':')).encode()).hexdigest()!=value['payload_sha256']:raise ValueError('full original payload differs')
   if payload['draw_start']!=len(draws)or payload['action_start']!=len(events)or payload['tick_start']!=len(ticks)+1:raise ValueError('full original chunk continuity differs')
   ticks+=payload['ticks'];draws+=payload['draws'];events+=payload['actions']
 finish=read(REFERENCE/Path(index['finish']['path']).name)
 if(len(ticks),len(draws),len(events))!=(4199,182811,831)or finish['status']!='completed'or finish['boundary']!='0AB9:EBE2':raise ValueError('full original coverage/return differs')
 if[entry['tick']for entry in ticks]!=list(range(1,4200))or[entry['event']for entry in events]!=list(range(831)):raise ValueError('full original observation ordering differs')
 if not any(e['code']==64 for e in events)or not any(e['code']==63 for e in events):raise ValueError('full original phase boundaries missing')
 print('pinned original4199ticks/182811draws/831actions and actual fulltime return verified',flush=True)
 return ticks,draws,events,finish

TRANSIENT_SHA='428868f09d57b08c528c7a2035b0f0681247d32f7546fddfcee6de49a529cfe2'
def transient_proof():
 directory=ROOT/'records/T41/transient-r0001'
 if sha(directory/'manifest.json')!=TRANSIENT_SHA:raise ValueError('original transient proof root differs')
 manifest=read(directory/'manifest.json')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('original transient proof source differs')
 result=read(directory/'result.json');start=read(directory/'start.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or result['original_ticks_joined']!=250 or result['draws_joined']!=11176 or result['initial_value']!=0:raise ValueError('original transient proof coverage differs')
 if result['reads']!=[{'tick':250,'draw':11138,'actor':1,'es':12423,'di':58838,'ds_offset':58845,'value':0,'source_return':7590}]or start['reference_index_sha256']!=INDEX_SHA:raise ValueError('original transient read differs')
 print('original250tick prefix and source1D89/1EA6 secondary255 field read verified',flush=True)
 return result

STATISTICS_SHA=['14c1d8bca4c843ddafc33e3ee68a657ef03ae40a2749e4b452ccc9c22c28307d', 'e02f432a7baed77bdb08d29b36603fdd79029cd0b9e48c34e0134432fa79b5e4', 'ac32bc8d541b67cf6a8303d5fc27391b1f531adcc342fd062d3b9734508d0b29', '131e557ee32c63ccae86c4e42e60dc98607cc130e2081cdda0c4b22347fc1299']
def statistics_proof():
 for number,pin in enumerate(STATISTICS_SHA,1):
  directory=ROOT/f'records/T41/statistics-boundary-r{number:04d}'
  if sha(directory/'manifest.json')!=pin:raise ValueError('statistics proof root differs')
  manifest=read(directory/'manifest.json')
  for name in('start','result'):
   if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('statistics original observation differs')
  start=read(directory/'start.json');result=read(directory/'result.json')
  if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or not result['all_other_memory_equal']or result['unexpected_change_count']!=0 or result['unexpected_first_addresses']:raise ValueError('statistics preservation footprint differs')
  if start['root']!='1998:045C'or start['return']!='0AB9:7E2F'or start['arguments']!=[(number-1)//2,(number-1)%2]or result['key_before']!=7 or result['key_after']!=0:raise ValueError('statistics source/input clearing differs')
  if(result['returned_sp'],result['returned_bp'],result['returned_ds'])!=(16026,16032,12423):raise ValueError('statistics far ABI differs')
 print('all4original statistics mode/wait calls preserve all non-presentation memory and clear the consumed key',flush=True)

PREFIX_SOURCE=None
def compare_prefix(got):
 global PREFIX_SOURCE
 if PREFIX_SOURCE is None:PREFIX_SOURCE=sources()
 ticks,draws,events,_=PREFIX_SOURCE;base=258
 for index,actual in enumerate(got['ticks']):
  source=ticks[index];expected=dict(source,draw_start=source['draw_start']-base,draw_end=source['draw_end']-base)
  if actual!=expected:
   path=ROOT/'.cache/T41/tick-mismatch.json';path.write_text(json.dumps({'tick':index+1,'actual':actual,'expected':expected},indent=2));raise ValueError(f'native completed tick{index+1} differs before missing input;private diagnostic retained')
 import watched_oracle as oracle
 expected=[oracle.normalized(d)for d in draws[base:base+len(got['draws'])]]
 if got['draws']!=expected:raise ValueError('native ordered RNG prefix differs before missing input')
 expected=[]
 for source in events[:len(got['events'])]:
  event={k:v for k,v in source.items()if k!='event'}
  for key in('draw_start','draw_end'):event[key]-=base
  expected.append(event)
 if got['events']!=expected:raise ValueError('native accepted-action prefix differs before missing input')
 if got.get('pending')is not None:
  original=ticks[len(got['ticks'])];pending=got['pending']
  if pending!={'tick':original['tick'],'draw_start':original['draw_start']-base,'before':original['before']}:raise ValueError('native pending tick entry differs before missing input')
 return len(got['ticks']),len(got['draws'])

def compare(got):
 import watched_oracle as oracle
 ticks,draws,events,finish=sources();base=258
 expected_ticks=[]
 for original in ticks:
  tick=dict(original,draw_start=original['draw_start']-base,draw_end=original['draw_end']-base);expected_ticks.append(tick)
 if got['ticks']!=expected_ticks:
  for index,(actual,expected)in enumerate(zip(got['ticks'],expected_ticks),1):
   if actual!=expected:
    path=ROOT/'.cache/T41/tick-mismatch.json';path.write_text(json.dumps({'tick':index,'actual':actual,'expected':expected},indent=2));raise ValueError(f'full native tick{index} differs;private diagnostic retained')
  raise ValueError('full native watched tick count differs')
 expected_draws=[oracle.normalized(d)for d in draws[base:]]
 if got['draws']!=expected_draws:raise ValueError('full native ordered RNG differs')
 expected_events=[]
 for entry in events:
  event={k:v for k,v in entry.items()if k!='event'}
  for key in('draw_start','draw_end'):event[key]-=base
  expected_events.append(event)
 if got['events']!=expected_events:raise ValueError('full native ordered accepted actions differ')
 if got['final']!=finish['final']or got['return']!=finish['boundary']:raise ValueError('full native final snapshot/return differs')
 # [confirmed] OriginalE008:E069-E06E swaps C4/C5 into final D086/D087.
 if got['live_goals']!=list(reversed(finish['final']['scores'])):raise ValueError('native live-goal/final-result mapping differs')
 print('native4199watched ticks,182553computed draws plus258setup context,831actions and fulltime return match',flush=True)

INITIAL_SHA='pending-positive-admission'
def admission():
 path=ROOT/'engine/tests/fixtures/watched_match/initial.json'
 if sha(path)!=INITIAL_SHA:raise ValueError('full watched admission root differs')
 value=read(path);anchor=read(ROOT/'records/T40/oracle-r0001/iteration-001.json');state=value['initial']
 if value['source_admission_sha256']!=sha(ROOT/'records/T40/oracle-r0001/iteration-001.json')or value['no_pending_key']is not True or state['kind']!='watched_match':raise ValueError('full watched admission provenance differs')
 for key in('scene','seed','regs','flags','stack_words'):
  if state[key]!=anchor['context'][key]:raise ValueError('full watched initial context differs')
 original={p['kind']:p for p in anchor['context']['pointers']};pointers={p['kind']:p for p in state['pointers']}
 if len(pointers)!=len(state['pointers'])or set(pointers)!=set(original)|{'results','fixtures','screen','screen_shown','screen_saved','counts'}:raise ValueError('full watched pointer kinds differ')
 for key,pointer in original.items():
  if pointers[key]!=pointer:raise ValueError('full watched source pointer differs')
 for ui,logical in[('screen','main'),('screen_shown','main_shown'),('screen_saved','background_shown')]:
  if any(pointers[ui][k]!=pointers[logical][k]for k in('segment','offset')):raise ValueError('full watched UI page alias differs')
 return value

def verify_private(value):
 import watched_match_oracle as facts
 raw,_=facts.entry();state=value['initial'];seen=set();ds=16*state['regs'][11]
 for fact in state['cells']:
  at=facts.address(state,fact['area'],fact['offset'])
  if at in seen or facts.named(state,at)!=(fact['area'],fact['offset'])or raw[at]!=fact['value']:raise ValueError('full watched fact is not a unique typed initial ENTRY value')
  seen.add(at)
 for pointer in state['pointers']:
  slot=facts.SLOTS[pointer['kind']][0]if pointer['kind']in facts.SLOTS else(None if pointer['kind']=='style'else facts.ticks.facts.SLOTS[pointer['kind']])
  if slot is None:slot=0xD0C0
  if int.from_bytes(raw[ds+slot:ds+slot+4],'little')!=pointer['offset']+(pointer['segment']<<16):raise ValueError('full watched pointer not from original ENTRY')
 print('all full-match admission facts and typed aliases verified against the single original ENTRY',flush=True)

def replay(value):
 import onenil_engine as native
 output=json.loads(native.watched_match(json.dumps({k:value[k]for k in('initial','no_pending_key')})))
 if 'ok'not in output:raise ValueError('native full watched match failed:'+output['error']['message'])
 compare(output['ok'])

def benchmark(value,repetitions):
 from onenil_engine import onenil_engine as native
 import watched_oracle as oracle
 if oracle.common.git('status','--porcelain','--','engine','tools','pyproject.toml'):raise ValueError('commit native benchmark sources first')
 if native.__debug_assertions__:raise ValueError('release extension required')
 request=json.dumps({k:value[k]for k in('initial','no_pending_key')},sort_keys=True,separators=(',',':'))
 began=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();size=0
 for _ in range(repetitions):size+=len(native.watched_match(request))
 elapsed=time.perf_counter_ns()-start;directory=ROOT/'records/T41';n=len(list(directory.glob('T41-r*.json')))+1;path=directory/f'T41-r{n:04d}.json'
 record={'schema':'routine-benchmark/1','code_commit':oracle.common.git('rev-parse','HEAD'),'started':began,'elapsed_ns':elapsed,'matches':repetitions,'watched_ticks':4199*repetitions,'computed_draws':182553*repetitions,'setup_context_draws_per_match':258,'serialized_output_bytes':size,'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'extension_sha256':sha(Path(native.__file__)),'cargo_lock_sha256':sha(ROOT/'engine/Cargo.lock'),'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'pins':{'reference':INDEX_SHA,'initial':INITIAL_SHA,'transient':TRANSIENT_SHA},'method':'Pre-serialized initial admission;native parsing,one persistent image,complete watched lifecycle and output serialization','limits':'One reference admission after initial setup;selected-lineup native watched setup and playable viewer remain required. No emulator-speed comparison.'}
 if path.exists():raise ValueError('immutable benchmark record exists')
 path.write_text(json.dumps(record,indent=2)+'\n');print(path.relative_to(ROOT),flush=True)

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--check',action='store_true');a=p.parse_args()
 if a.check:sources()
 else:p.error('choose --check')
if __name__=='__main__':main()
