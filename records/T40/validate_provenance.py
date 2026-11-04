"""[designed] Trusted roots and semantic source joins reject altered caller evidence."""
import copy,hashlib,json,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_caller as replay

def main():
 values=replay.check();original_read=replay.read;original_fixture=replay.FIXTURES;manifest=original_read(original_fixture/'manifest.json');checks=0
 with tempfile.TemporaryDirectory()as temp:
  root=Path(temp);directory=root/'fixtures';directory.mkdir()
  for field,value in [('case',99),('steps',170),('no_pending_key',False),('clock_only',True),('event_draw_base',0),('initial',{}),('originals',[])]:
   c=copy.deepcopy(values[0]);c[field]=value;leaf=root/'leaf.json';leaf.write_text(json.dumps(c));m=copy.deepcopy(manifest);m['cases'][0]={'path':str(leaf),'sha256':hashlib.sha256(leaf.read_bytes()).hexdigest()};(directory/'manifest.json').write_text(json.dumps(m));replay.FIXTURES=directory
   try:replay.check()
   except ValueError:checks+=1
   else:raise AssertionError('coherently rehashed caller fixture accepted')
 replay.FIXTURES=original_fixture
 first=replay.ROOT/'records/T40/oracle-r0001/iteration-001.json';clock=replay.ROOT/'records/T40/clock-r0001/case-02.json'
 mutations=[(first,lambda c:c.update(iteration=99)),(first,lambda c:c.update(parent_return=0)),(first,lambda c:c['context']['regs'].__setitem__(5,c['context']['regs'][5]+2)),(first,lambda c:c['exit_registers'].update(sp=c['exit_registers']['sp']+2)),(first,lambda c:c.update(draw_end=c['draw_end']+1)),(first,lambda c:c.update(no_key_visits=21)),(clock,lambda c:c['input_overlay'].update(elapsed=99)),(clock,lambda c:c['output']['clock'].update(elapsed=99)),(clock,lambda c:c['changes'][0].update(before=(c['changes'][0]['before']+1)%256))]
 for path,mutate in mutations:
  altered=copy.deepcopy(original_read(path));mutate(altered)
  def read(p,path=path,altered=altered):return copy.deepcopy(altered)if p==path else original_read(p)
  replay.read=read
  try:replay.check()
  except ValueError:checks+=1
  else:raise AssertionError('altered original caller/clock provenance accepted')
 replay.read=original_read
 print(f'valid171caller/8clock transitions;{checks} altered/coherently-rehashed provenance cases rejected')
if __name__=='__main__':main()
