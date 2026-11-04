"""[designed] Native-only presentation experiment;not a prepared-state runtime."""
import hashlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_match as replay
import onenil_engine as api
from onenil_engine import onenil_engine as native

def main():
 value=replay.admission();state=value['initial'];changed=0
 for fact in state['cells']:
  if fact['area']in('main','background','background_shown'):
   offset=fact['offset'];fact['value']=7 if offset%2 else(9 if offset==0x90E else32);changed+=1
 result=json.loads(api.watched_match(json.dumps({'initial':state,'no_pending_key':True})))
 record={'scope':'Native-only derived floor-page experiment using T41 debug kernel. This is not the prepared-image bridge and does not establish original presentation preservation.','changed_page_facts':changed,'extension_sha256':hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),'debug_assertions':native.__debug_assertions__}
 if 'ok'in result:
  try:replay.compare(result['ok']);record['status']='full-original-reference-gameplay-parity'
  except ValueError as error:record.update(status='native-gameplay-mismatch',error=str(error))
 else:record.update(status='native-fault',error=result['error']['message'])
 directory=Path(__file__).resolve().parent;n=max((int(p.stem.split('-r')[1])for p in directory.glob('headless-native-r*.json')),default=0)+1
 target=directory/f'headless-native-r{n:04d}.json'
 with target.open('x')as output:json.dump(record,output,indent=2);output.write('\n')
 print(record,flush=True)
 if record['status']!='full-original-reference-gameplay-parity':raise SystemExit(1)
if __name__=='__main__':main()
