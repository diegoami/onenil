"""[designed] Reject edited and coherently-rehashed original UI preservation records."""
import copy,hashlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_match as replay

def main():
 original_read,original_sha=replay.read,replay.sha
 families=[(f'statistics-boundary-r{i:04d}',replay.statistics_proof)for i in range(1,5)]
 families += [('template-boundary-r0001',replay.template_proof),('ui-leaves-boundary-r0001',replay.ui_leaves_proof)]
 for check in (replay.statistics_proof,replay.template_proof,replay.ui_leaves_proof):check()
 checks=0
 for name,check in families:
  directory=replay.ROOT/'records/T41'/name
  for coherent in (False,True):
   result_path=directory/'result.json';manifest_path=directory/'manifest.json'
   result=copy.deepcopy(original_read(result_path));manifest=copy.deepcopy(original_read(manifest_path))
   target=result['observations'][0]if 'observations'in result else result
   target['all_other_memory_equal']=False
   changed={result_path:result}
   if coherent:
    manifest['result_sha256']=hashlib.sha256(json.dumps(result).encode()).hexdigest();changed[manifest_path]=manifest
   def read(path):return changed[path]if path in changed else original_read(path)
   def sha(path):return hashlib.sha256(json.dumps(changed[path]).encode()).hexdigest()if path in changed else original_sha(path)
   replay.read,replay.sha=read,sha
   try:
    try:check()
    except ValueError:checks+=1
    else:raise AssertionError(f'altered UI footprint accepted:{name}/{coherent}')
   finally:replay.read,replay.sha=original_read,original_sha
 print(f'{checks} edited/coherently-rehashed original UI footprint records rejected')
if __name__=='__main__':main()
