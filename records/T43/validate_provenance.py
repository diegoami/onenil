"""[designed] Watched preparation roots reject altered/coherently-rehashed evidence."""
import copy
import hashlib
import json
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_setup as replay


def digest(value):
 return hashlib.sha256(json.dumps(value).encode()).hexdigest()


def main():
 replay.admission();read,sha=replay.read,replay.sha;directory=replay.DIRECTORY;checks=0
 manifest=read(directory/'manifest.json');result=read(directory/'result.json')
 mutations=[lambda r:r['incoming'].update(ball_word7=1),lambda r:r['first_entry'].update(actor=1),lambda r:r['draws'][0].update(seed_after=0),lambda r:r['points'][3].update(draw=0),lambda r:r.update(status='incomplete')]
 for mutate in mutations:
  changed=copy.deepcopy(result);mutate(changed);altered=copy.deepcopy(manifest);altered['files']['result.json']=digest(changed)
  def altered_read(path):
   if path==directory/'manifest.json':return altered
   if path==directory/'result.json':return changed
   return read(path)
  def altered_sha(path):
   if path==directory/'manifest.json':return digest(altered)
   if path==directory/'result.json':return digest(changed)
   return sha(path)
  replay.read,replay.sha=altered_read,altered_sha
  try:replay.sources()
  except ValueError:checks+=1
  else:raise AssertionError('coherently-rehashed watched preparation accepted')
  finally:replay.read,replay.sha=read,sha
 baseline=read(replay.FIXTURE)
 mutations=[lambda v:v['input']['setup']['legacy']['config'].__setitem__(1,2),lambda v:v['input']['incoming'].update(ball_byte6=1),lambda v:v.update(source_manifest_sha256='0'*64),lambda v:v['input'].update(expected_first_entry={}),lambda v:v['input']['incoming'].update(ball_word11=0)]
 for mutate in mutations:
  changed=copy.deepcopy(baseline);mutate(changed)
  replay.read=lambda path:changed if path==replay.FIXTURE else read(path)
  replay.sha=lambda path:digest(changed) if path==replay.FIXTURE else sha(path)
  try:replay.admission()
  except ValueError:checks+=1
  else:raise AssertionError('altered/extended watched preparation fixture accepted')
  finally:replay.read,replay.sha=read,sha
 print(f'{checks} watched setup reference/input/rehash/outcome-injection cases rejected')


if __name__=='__main__':main()
