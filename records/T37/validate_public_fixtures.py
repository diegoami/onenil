"""[designed] Public decoded evidence rejects altered state,RNG,events and provenance."""
import hashlib,json,shutil,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_action as replay
import watched_oracle as oracle

def main():
    actual=replay.ROOT;assert len(replay.check())==3768;checks=0
    with tempfile.TemporaryDirectory()as name:
        root=Path(name);fixtures=root/'engine/tests/fixtures/watched_action';source=root/'records/T37/oracle-r0001'
        shutil.copytree(replay.FIXTURES,fixtures);shutil.copytree(actual/'records/T37/oracle-r0001',source)
        replay.ROOT=root;oracle.ROOT=root
        assert len(replay.check(fixtures))==3768
        manifest_path=fixtures/'manifest.json';manifest_bytes=manifest_path.read_bytes()
        for label,index,mutate in [
          ('scene',0,lambda v:v['input']['scene'].__setitem__('word14',0)),
          ('seed',0,lambda v:v['expected'].__setitem__('seed',0)),
          ('draw',0,lambda v:v['expected']['draws'][0].__setitem__('value',0)),
          ('event',113,lambda v:v['expected']['events'][0].__setitem__('code',99)),
          ('provenance',0,lambda v:v.__setitem__('oracle_sha256','0'*64)),
          ('entry fact',0,lambda v:v['input']['cells'][0].__setitem__('value',(v['input']['cells'][0]['value']+1)%256)),
        ]:
            path=fixtures/f'case-{index:05d}.json';before=path.read_bytes();value=json.loads(before);mutate(value);path.write_text(json.dumps(value))
            try:replay.check(fixtures)
            except ValueError as error:assert str(error)=='decoded action fixture digest differs',(label,error)
            else:raise AssertionError(label+' mutation accepted')
            checks+=1
            m=json.loads(manifest_bytes);m['cases'][index]['sha256']=hashlib.sha256(path.read_bytes()).hexdigest();manifest_path.write_text(json.dumps(m))
            try:replay.check(fixtures)
            except ValueError as error:assert str(error)=='decoded fixture manifest pin differs',(label,error)
            else:raise AssertionError('coherently rehashed '+label+' accepted')
            checks+=1;path.write_bytes(before);manifest_path.write_bytes(manifest_bytes)
    print(f'public fixtures: valid3768 roots;{checks} altered/coherently rehashed state,draw,event,entry and provenance mutations rejected')
if __name__=='__main__':main()
