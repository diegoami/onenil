"""[designed] Fail-closed full-match admissions and original missing-address guards."""
import copy,json,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_match as replay
import onenil_engine as native

def main():
 value=replay.admission();initial=value['initial'];checks=0
 def reject(state,key=True):
  result=json.loads(native.watched_match(json.dumps({'initial':state,'no_pending_key':key})))
  if 'error'not in result:raise AssertionError('invalid native match admission accepted')
  return result['error']
 def early(state,key=True):
  error=reject(state,key)
  if error['ticks']or error['draws']or error['events']:raise AssertionError('invalid admission advanced gameplay')
  return error
 early(initial,False);checks+=1
 state=copy.deepcopy(initial);screen=next(p for p in state['pointers']if p['kind']=='screen');seed=next(p for p in state['pointers']if p['kind']=='seed');screen.update(segment=seed['segment'],offset=seed['offset']);early(state);checks+=1
 state=copy.deepcopy(initial);next(f for f in state['cells']if f['area']=='frame'and f['offset']==8)['value']^=1;early(state);checks+=1
 state=copy.deepcopy(initial);state['cells'].append({'area':'ds','offset':0xD5A,'value':0});early(state);checks+=1
 state=copy.deepcopy(initial);state['cells'].append(copy.deepcopy(state['cells'][0]));early(state);checks+=1
 state=copy.deepcopy(initial);next(f for f in state['cells']if f['area']=='ds'and f['offset']==0xD074)['value']=1
 if 'quick fallback'not in early(state)['message']:raise AssertionError('quick fallback was not explicit')
 checks+=1
 for area,offset,site,writing in[('main',0xC4,(0x24BC,0x02A9),True),('ds',0xD90B,(0x0AB9,0xA275),False)]:
  state=copy.deepcopy(initial);before=len(state['cells']);state['cells']=[f for f in state['cells']if(f['area'],f['offset'])!=(area,offset)]
  if len(state['cells'])!=before-1:raise AssertionError('guard does not remove one admitted baseline')
  error=reject(state);missing=error['missing'];replay.compare_prefix(error)
  if missing is None or tuple(missing['pc'])!=site or missing['writing']is not writing:raise AssertionError('original missing descriptor overwritten')
  if len(error['ticks'])!=1212:raise AssertionError('missing-field guard stopped at a different original boundary')
  if area=='main':
   pointer=next(p for p in state['pointers']if p['kind']=='main');expected=16*pointer['segment']+pointer['offset']+offset
   if missing['address']!=expected or error['diagnostic']['partial']is not None:raise AssertionError('temporary-DS diagnostic replaced actual glyph write fault')
  checks+=1
 try:native.watched_match(json.dumps({'initial':initial,'no_pending_key':True,'expected_outcome':{}}))
 except ValueError:checks+=1
 else:raise AssertionError('runtime expected-outcome input accepted')
 print(f'{checks} native admission/alias/ABI/read/write/fallback/outcome-input guards passed')
if __name__=='__main__':main()
