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

TEMPLATE_SHA='c044f620ce67bdae78e07caf667e6b969d27615930b3348104ad278c4e8e1092'
def template_proof():
 directory=ROOT/'records/T41/template-boundary-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=TEMPLATE_SHA:raise ValueError('template proof root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('template original observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 expected=[(0x132E,'1B55:346C'),(0x6346,'1B55:347B'),(0x25CC,'0AB9:D6E5'),(0x4F20,'0AB9:971A')]
 if start['root']!='232A:052F'or start['destination_header']!='DS:DD5C'or result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']:raise ValueError('template proof protocol differs')
 if [(v['template_ds_offset'],v['return'])for v in result['observations']]!=expected:raise ValueError('template cases differ')
 for observation in result['observations']:
  if not observation['all_other_memory_equal']or observation['unexpected_change_count']or observation['unexpected_first_addresses']or set(observation['changed_bytes_by_area'])!={'stack','main_shown'}:raise ValueError('template modifies non-presentation memory')
  if(observation['returned_sp'],observation['returned_bp'],observation['returned_ds'])!=(16026,16032,12423):raise ValueError('template far ABI differs')
 print('all4original templates preserve all memory outside MainShown and stack',flush=True)

UI_LEAVES_SHA='0edede71cc96e75eb50ca5e0dab0eb82e03e13ce3e87084abd0d955bc491a3ba'
UI_INPUT_SHA='8da02465ea67354c9dd4931f0a08946d2ecd7ea6db0de9f2994ed225c941a9ba'
def ui_leaves_proof():
 inputs_path=ROOT/'records/T41/ui-leaves-input-r0001.json'
 if sha(inputs_path)!=UI_INPUT_SHA:raise ValueError('UI numeric input root differs')
 inputs=read(inputs_path)
 if inputs['source_admission_sha256']!=sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'):raise ValueError('UI input original ENTRY provenance differs')
 addresses=[v['address']for v in inputs['facts']]
 if addresses!=sorted(set(addresses))or any(not 0<=v['value']<=255 for v in inputs['facts']):raise ValueError('UI numeric input facts differ')
 directory=ROOT/'records/T41/ui-leaves-boundary-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=UI_LEAVES_SHA:raise ValueError('UI leaves proof root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('UI leaves original observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or len(result['observations'])!=48:raise ValueError('UI leaves completion differs')
 if [{'root':v['root'],'arguments':v['arguments']}for v in result['observations']]!=start['cases']:raise ValueError('UI leaves cases differ')
 for observation in result['observations']:
  if not observation['all_other_memory_equal']or observation['unexpected_change_count']or observation['unexpected_first_addresses']or set(observation['changed_bytes_by_area'])!={'stack','main_shown','style'}:raise ValueError('UI leaf modifies non-presentation memory')
  if(observation['returned_sp'],observation['returned_bp'],observation['returned_ds'])!=(16026,16032,12423):raise ValueError('UI leaves far ABI differs')
 print('all48original UI leaf calls preserve all memory outside MainShown/style/stack',flush=True)

FORMATION_SHA='20ac56b45ebaab6b989ae79a819287fca9c473d73a68633b566d7e6dbab29c23'
FORMATION_INPUT_SHA='3f30fbc6b49a365ffe7d94c3476eef229ee25213b98e6d1fc6754b895b676fee'
def formation_proof():
 directory=ROOT/'records/T41/formation-boundary-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=FORMATION_SHA:raise ValueError('formation proof root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('formation original observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='1B55:10AF'or start['observe_counts_at']!='1B55:1125':raise ValueError('formation protocol differs')
 if [v['club']for v in result['observations']]!=start['clubs']:raise ValueError('formation cases differ')
 for v in result['observations']:
  if not v['all_other_memory_equal']or v['unexpected_change_count']or v['unexpected_first_addresses']or set(v['changed_bytes_by_area'])!={'stack','output_text'}:raise ValueError('formation modifies non-presentation memory')
  if(v['returned_sp'],v['returned_bp'],v['returned_ds'])!=(16022,16032,12423)or len(v['counts'])!=4:raise ValueError('formation ABI/counts differ')
 path=ROOT/'records/T41/formation-input-r0001.json'
 if sha(path)!=FORMATION_INPUT_SHA or read(path)['source_admission_sha256']!=sha(ROOT/'records/T40/oracle-r0001/iteration-001.json'):raise ValueError('formation numeric input provenance differs')
 print('both original formation count/format calls preserve non-presentation memory',flush=True)

GROUPED_INTEGER_SHA='d855a83169c5d55a90c08fa8a76490b32b032979f9eaf94c6e57822b18258c60'
def grouped_integer_proof():
 directory=ROOT/'records/T41/grouped-integer-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=GROUPED_INTEGER_SHA:raise ValueError('grouped integer root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('grouped integer observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='1B55:0F95':raise ValueError('grouped integer protocol differs')
 if [v['raw_value']for v in result['observations']]!=start['values']or len(start['values'])!=12:raise ValueError('grouped integer cases differ')
 for v in result['observations']:
  if not v['all_other_memory_equal']or v['unexpected_change_count']or v['unexpected_first_addresses']:raise ValueError('grouped integer modifies non-output memory')
  if(v['returned_sp'],v['returned_bp'],v['returned_ds'])!=(16022,16032,12423)or len(v['pascal_bytes'])!=1+v['pascal_bytes'][0]:raise ValueError('grouped integer ABI/text length differs')
 print('all12original signed32 grouped-number outputs and non-output memory verified',flush=True)

FIXTURE_TITLE_SHA='75d0a72910c5028d581dad2725eff71133cff34a949ad4faa1f36ded524a5f48'
def fixture_title_proof():
 directory=ROOT/'records/T41/fixture-title-boundary-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=FIXTURE_TITLE_SHA:raise ValueError('fixture title root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('fixture title observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='1B55:142B'or start['arguments']!=[6,0]:raise ValueError('fixture title protocol differs')
 if not result['all_other_memory_equal']or result['unexpected_change_count']or result['unexpected_first_addresses']or set(result['changed_bytes_by_area'])!={'stack','output_text'}:raise ValueError('fixture title modifies non-output memory')
 if(result['returned_sp'],result['returned_bp'],result['returned_ds'])!=(16022,16032,12423):raise ValueError('fixture title ABI differs')
 print('original fixture-title formatter preserves non-output memory and hidden ABI',flush=True)

MENU_INPUT_SHA='d05aed7d390c41caeed3b64e5307b03dfad8b08a3cb560aa4d193144c17a5a2e'
def menu_input_proof():
 directory=ROOT/'records/T41/menu-input-boundary-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=MENU_INPUT_SHA:raise ValueError('menu input root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('menu input observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='232A:0F8B'or start['menu_count']!=4:raise ValueError('menu input protocol differs')
 cases=[(1,'Down',2,6),(2,'Down',3,6),(3,'Return',3,2),(4,'Up',3,5)]
 if [(v['cursor_before'],v['external_key'],v['cursor_after'],v['key_code'])for v in result['observations']]!=cases:raise ValueError('menu input normalization/cursor differs')
 for v in result['observations']:
  if not v['all_other_memory_equal']or v['unexpected_change_count']or v['unexpected_first_addresses']:raise ValueError('menu input modifies gameplay memory')
  if(v['returned_sp'],v['returned_bp'],v['returned_ds'])!=(16026,16032,12423)or v['last_key_code']!=v['key_code']or v['pending_key']or v['queue_count']:raise ValueError('menu input ABI/consumption differs')
 print('all4original menu key/cursor calls preserve gameplay memory and consume their key',flush=True)

LINEUP_VALIDATION_SHA='887d2b53be8c8c53912e6edd393d846e3ab6d99bff090f3b8cb94595be1e205c'
def lineup_validation_proof():
 directory=ROOT/'records/T41/lineup-validation-r0002';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=LINEUP_VALIDATION_SHA:raise ValueError('lineup validation root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('lineup validation observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='0AB9:D29E'or len(start['cases'])!=9:raise ValueError('lineup validation protocol differs')
 if [v['name']for v in result['observations']]!=[v['name']for v in start['cases']]:raise ValueError('lineup validation cases differ')
 for v in result['observations']:
  if not v['all_other_memory_equal']or v['unexpected_change_count']or v['unexpected_first_addresses']:raise ValueError('lineup validation modifies gameplay memory')
  if(v['returned_sp'],v['returned_bp'],v['returned_ds'])!=(16026,start['derived_parent_bp'],12423)or v['restored_active_club']!=start['active_club']or v['returned_valid']not in(0,1):raise ValueError('lineup validation ABI/result differs')
 print('all9original lineup validation cases preserve gameplay memory and restore parent selection',flush=True)

REPLACEMENT_BANNER_SHA='a019afeccf9f018adb547224e1c91ab9a37f918a38cc205aef1e78848b05faa8'
def replacement_banner_proof():
 directory=ROOT/'records/T41/replacement-banner-r0002';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=REPLACEMENT_BANNER_SHA:raise ValueError('replacement banner root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('replacement banner observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='0AB9:ADD0'or start['external_keys']or start['pit_divisor']!=1024 or len(start['cases'])!=8:raise ValueError('replacement banner protocol differs')
 if [v['name']for v in result['observations']]!=[v['name']for v in start['cases']]:raise ValueError('replacement banner cases differ')
 for v in result['observations']:
  if not v['all_other_memory_equal']or v['unexpected_change_count']or v['unexpected_first_addresses']or v['changed_bytes_by_area']['keyboard_driver']:raise ValueError('replacement banner modifies non-presentation memory or consumes a key')
  if(v['returned_sp'],v['returned_bp'],v['returned_ds'])!=(16026,16032,12423)or not 0<=v['style_after']<=255:raise ValueError('replacement banner ABI/style differs')
 print('all8original no-key announcements restore Main and preserve gameplay memory',flush=True)

FINAL_FOOTER_SHA='b0bfff8735860d2970eca4d56f21d9c14f5904e57415e843d01a3850acb51cb7'
def final_footer_proof():
 directory=ROOT/'records/T41/final-footer-r0001';manifest=read(directory/'manifest.json')
 if sha(directory/'manifest.json')!=FINAL_FOOTER_SHA:raise ValueError('final footer root differs')
 for name in('start','result'):
  if sha(directory/f'{name}.json')!=manifest[f'{name}_sha256']:raise ValueError('final footer observation differs')
 start=read(directory/'start.json');result=read(directory/'result.json')
 if result['status']!='completed'or result['protocol_sha256']!=manifest['start_sha256']or start['root']!='0AB9:E1B0'or start['template']!='DS:4B12'or start['destination_header']!='DS:DD58'or start['external_key']!='n':raise ValueError('final footer protocol differs')
 if not result['all_other_memory_equal']or result['unexpected_change_count']or result['unexpected_first_addresses']:raise ValueError('final footer modifies gameplay memory')
 if(result['returned_sp'],result['returned_bp'],result['returned_ds'])!=(16026,16032,12423)or result['key_code']!=110 or result['last_key_code']!=110 or result['pending_key']or result['queue_count']:raise ValueError('final footer ABI/key consumption differs')
 print('original final footer/exit parsing preserves gameplay memory and consumes n',flush=True)

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

INITIAL_SHA='88274a14f94fb526c1f0777d06017355b73e41946aabaafc7d65233da5492311'
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
 ui_leaves_proof();formation_proof()
 for fact in read(ROOT/'records/T41/ui-leaves-input-r0001.json')['facts']:
  if raw[fact['address']]!=fact['value']:raise ValueError('UI numeric fact differs from original ENTRY')
 for fact in read(ROOT/'records/T41/formation-input-r0001.json')['facts']:
  if raw[fact['address']]!=fact['value']:raise ValueError('formation numeric fact differs from original ENTRY')
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
 for _ in range(repetitions):
  output=native.watched_match(request)
  if not output.startswith('{"ok":'):raise ValueError('benchmark native match returned an error')
  size+=len(output)
 elapsed=time.perf_counter_ns()-start;directory=ROOT/'records/T41';n=len(list(directory.glob('T41-r*.json')))+1;path=directory/f'T41-r{n:04d}.json'
 record={'schema':'routine-benchmark/1','code_commit':oracle.common.git('rev-parse','HEAD'),'started':began,'elapsed_ns':elapsed,'matches':repetitions,'watched_ticks':4199*repetitions,'computed_draws':182553*repetitions,'accepted_actions':831*repetitions,'setup_context_draws_per_match':258,'serialized_output_bytes':size,'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'extension_sha256':sha(Path(native.__file__)),'cargo_lock_sha256':sha(ROOT/'engine/Cargo.lock'),'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'pins':{'reference':INDEX_SHA,'initial':INITIAL_SHA,'transient':TRANSIENT_SHA},'method':'Pre-serialized initial admission;native parsing,one persistent image,complete watched lifecycle and output serialization','limits':'One reference admission after initial setup;selected-lineup native watched setup and playable viewer remain required. No emulator-speed comparison.'}
 if path.exists():raise ValueError('immutable benchmark record exists')
 path.write_text(json.dumps(record,indent=2)+'\n');print(path.relative_to(ROOT),flush=True)

def main():
 p=argparse.ArgumentParser(description=__doc__)
 for flag in('check','private','replay','benchmark'):p.add_argument('--'+flag,action='store_true')
 p.add_argument('--repetitions',type=int,default=3);a=p.parse_args()
 if not(a.check or a.private or a.replay or a.benchmark):p.error('choose --check,--private,--replay or --benchmark')
 if a.repetitions<1:p.error('repetitions must be positive')
 sources()
 for proof in(transient_proof,statistics_proof,template_proof,ui_leaves_proof,formation_proof,grouped_integer_proof,fixture_title_proof,menu_input_proof,lineup_validation_proof,replacement_banner_proof,final_footer_proof):proof()
 value=admission();print('pinned one-entry watched-match admission verified',flush=True)
 if a.private:verify_private(value)
 if a.replay or a.benchmark:replay(value)
 if a.benchmark:benchmark(value,a.repetitions)
if __name__=='__main__':main()
