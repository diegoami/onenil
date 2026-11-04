"""[designed] Reject changed or coherently rehashed original/native tick evidence."""
import copy,hashlib,json,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_tick as replay

def main():
    values=replay.check();original_read=replay.read;original_fixture=replay.FIXTURES;checks=0
    manifest=original_read(original_fixture/'manifest.json')
    with tempfile.TemporaryDirectory()as temp:
        root=Path(temp);directory=root/'fixtures';directory.mkdir()
        for field,value in [('case',99),('steps',7),('no_pending_key',False),('source_sha256','0'*64),('initial',{}),('expected',[])]:
            c=copy.deepcopy(values[0]);c[field]=value;leaf=root/'leaf.json';leaf.write_text(json.dumps(c))
            m=copy.deepcopy(manifest);m['cases'][0]={'path':str(leaf),'sha256':hashlib.sha256(leaf.read_bytes()).hexdigest()};(directory/'manifest.json').write_text(json.dumps(m));replay.FIXTURES=directory
            try:replay.check()
            except ValueError:checks+=1
            else:raise AssertionError('coherently rehashed native tick fixture accepted')
    replay.FIXTURES=original_fixture
    cases=[(replay.ROOT/'records/T39/oracle-r0001/actual-01.json',lambda c:c.update(tick=99)),
           (replay.ROOT/'records/T39/oracle-r0001/actual-01.json',lambda c:c['context'].update(seed=0)),
           (replay.ROOT/'records/T39/repeat-r0001/step-01.json',lambda c:c['output'].update(seed=0)),
           (replay.ROOT/'records/T39/repeat-r0001/step-01.json',lambda c:c['changes'][0].update(before=(c['changes'][0]['before']+1)%256)),
           (replay.ROOT/'records/T39/neutral-input/run-0001.json',lambda c:c['calls'][0].update(AL=1))]
    for path,mutation in cases:
        changed=copy.deepcopy(original_read(path));mutation(changed)
        def read(p,path=path,changed=changed):return copy.deepcopy(changed)if p==path else original_read(p)
        replay.read=read
        try:replay.check()
        except ValueError:checks+=1
        else:raise AssertionError('altered original admission/outcome/keyboard branch accepted')
    replay.read=original_read
    print('valid9sessions/16outputs;'+str(checks)+' altered/coherently-rehashed provenance cases rejected')
if __name__=='__main__':main()
