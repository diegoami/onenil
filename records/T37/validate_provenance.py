"""[designed] Public oracle provenance rejects altered and coherently rehashed records."""
import hashlib,json,sys,tempfile
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import watched_oracle as oracle

def main():
    directory=oracle.ROOT/'records/T37/oracle-r0001'
    cases,manifest=oracle.validate(directory)
    assert len(cases)==3768
    original_root=oracle.ROOT
    checks=0
    with tempfile.TemporaryDirectory()as tmp:
        root=Path(tmp);d=root/'records/T37/oracle-r0001';d.mkdir(parents=True)
        # A missing case fails without accessing any private bytes.
        (d/'manifest.json').write_bytes((directory/'manifest.json').read_bytes());oracle.ROOT=root
        def reject():
            try:oracle.validate(d)
            except(ValueError,FileNotFoundError):return
            raise AssertionError('altered provenance accepted')
        reject();checks+=1
        for field,value in [('status','aborted'),('first_tick_join',False),('window',[1,64]),('next_tick_entry',128),('cases',manifest['cases'][:-1])]:
            altered={**manifest,field:value};(d/'manifest.json').write_text(json.dumps(altered));reject();checks+=1
        item=manifest['cases'][0];path=root/item['path'];path.write_text(json.dumps({**cases[0],'seed_before':0}))
        altered={**manifest,'cases':[dict(v)for v in manifest['cases']]};altered['cases'][0]['sha256']=hashlib.sha256(path.read_bytes()).hexdigest()
        (d/'manifest.json').write_text(json.dumps(altered));reject();checks+=1
    oracle.ROOT=original_root
    print(f'original provenance: valid3768 roots;{checks} altered/missing/coherently rehashed cases rejected')
if __name__=='__main__':main()
