"""[designed] Trusted full-reference roots reject altered and rehashed evidence."""
import copy,hashlib,json,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_match as replay

def main():
 replay.sources();original_read=replay.read;original_sha=replay.sha;root=replay.REFERENCE;index=original_read(root/'index.json');checks=0
 # Leaf edits with honestly recomputed leaf digests still fail the trusted index root.
 with tempfile.TemporaryDirectory()as temp:
  directory=Path(temp);altered=copy.deepcopy(index);altered['chunks'][0]['sha256']='0'*64;(directory/'index.json').write_text(json.dumps(altered));replay.REFERENCE=directory
  try:replay.sources()
  except ValueError:checks+=1
  else:raise AssertionError('altered reference index accepted')
 replay.REFERENCE=root
 target=root/Path(index['chunks'][0]['path']).name
 mutations=[lambda c:c['payload']['ticks'][0].update(tick=99),lambda c:c['payload']['ticks'][0]['before'].update(elapsed=99),lambda c:c['payload']['draws'][0].update(seed_after=0),lambda c:c['payload']['actions'][0].update(code=0),lambda c:c['payload'].update(draw_start=1)]
 for mutate in mutations:
  changed=copy.deepcopy(original_read(target));mutate(changed);changed['payload_sha256']=hashlib.sha256(json.dumps(changed['payload'],sort_keys=True,separators=(',',':')).encode()).hexdigest()
  raw=json.dumps(changed).encode();digest=hashlib.sha256(raw).hexdigest();altered=copy.deepcopy(index);altered['chunks'][0]['sha256']=digest
  def read(path):
   if path==target:return changed
   if path==root/'index.json':return altered
   return original_read(path)
  def sha(path):return digest if path==target else(hashlib.sha256(json.dumps(altered).encode()).hexdigest()if path==root/'index.json'else original_sha(path))
  replay.read=read;replay.sha=sha
  try:replay.sources()
  except ValueError:checks+=1
  else:raise AssertionError('coherently rehashed reference accepted')
 replay.read=original_read;replay.sha=original_sha
 print(f'{checks} altered/coherently-rehashed full-reference cases rejected')
if __name__=='__main__':main()
