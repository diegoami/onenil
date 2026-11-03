"""[designed] Reject changed original admission/overlays and decoded native fixtures."""
import copy,hashlib,json,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_watched_actor as replay

def main():
    values=replay.check();sources,manifest=replay.originals();checks=0
    original_root=replay.ROOT;original_fixture=replay.FIXTURES;original_read=replay.read
    # Simulate coherently rehashed records at the outer trusted-digest boundary.
    # The immutable root pins must reject a changed manifest even when leaf hashes agree.
    with tempfile.TemporaryDirectory()as tmp:
        root=Path(tmp);fixtures=root/'engine/tests/fixtures/watched_actor';fixtures.mkdir(parents=True)
        original_manifest=original_read(original_fixture/'manifest.json')
        for field,value in [('case',999),('admission','live-parent-entry'),('source_sha256','0'*64),('input',{}),('expected',{})]:
            altered=copy.deepcopy(values[0]);altered[field]=value
            leaf=root/original_manifest['cases'][0]['path'];leaf.write_text(json.dumps(altered))
            m=copy.deepcopy(original_manifest);m['cases'][0]['sha256']=hashlib.sha256(leaf.read_bytes()).hexdigest()
            (fixtures/'manifest.json').write_text(json.dumps(m));replay.FIXTURES=fixtures
            try:replay.check()
            except ValueError:checks+=1
            else:raise AssertionError('coherently rehashed decoded fixture accepted')
        replay.FIXTURES=original_fixture
    # Independent semantic gates still reject altered proof joins and source overlays.
    proof=original_read(original_root/'records/T38/admission-proof-r0001.json')
    for field in ('registers','flags','stack_words','seed','ram_sha256','game_timer_ticks'):
        changed=copy.deepcopy(proof);entry=changed['entries'][0]['joined_nested_entry']
        if field=='registers':entry[field]['sp']+=2
        elif field=='stack_words':entry[field][0]^=1
        elif field=='ram_sha256':entry[field]='0'*64
        else:entry[field]+=1
        def read(path,changed=changed):
            if path.name=='admission-proof-r0001.json':return copy.deepcopy(changed)
            return original_read(path)
        replay.read=read
        try:replay.originals()
        except ValueError:checks+=1
        else:raise AssertionError('altered semantic admission proof accepted')
    replay.read=original_read
    original_originals=replay.originals
    research=Path('/home/diego/projects/onenil-archaeology')
    if research.is_dir():
        for field in ('source_digest','seed','scene'):
            altered=copy.deepcopy(sources);overlay=altered[42][0]['input_overlay']
            if field=='source_digest':overlay['source']['sha256']='0'*64
            elif field=='seed':overlay['before']['seed']^=1
            else:overlay['before']['scenes'][0]['word0']^=1
            replay.originals=lambda altered=altered:(altered,manifest)
            try:replay.check_research(research)
            except ValueError:checks+=1
            else:raise AssertionError('changed original tick overlay accepted')
        replay.originals=original_originals
    print(f'valid50 parent roots;{checks} coherently rehashed fixture/proof checks passed')
if __name__=='__main__':main()
