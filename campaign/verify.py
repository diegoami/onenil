#!/usr/bin/env python3
"""Retain fresh native-process state persistence, invalid-load and interrupted-save proof."""
import argparse,hashlib,json,os,subprocess,tempfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser();p.add_argument('--record',type=Path,required=True);args=p.parse_args();assert not args.record.exists()
producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert not subprocess.check_output(['git','status','--porcelain','--','campaign'],cwd=ROOT,text=True),'commit producer'
tests=subprocess.run(['cargo','test','--manifest-path','campaign/Cargo.toml','--locked'],cwd=ROOT,text=True,capture_output=True);assert tests.returncode==0,tests.stdout+tests.stderr
binary=Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'campaign/target'))/'debug/onenil-campaign'
source=json.loads((ROOT/'campaign/fixtures/research-seed.json').read_text());cases=[]
def run(*command,fail=False):
 r=subprocess.run([str(binary),*map(str,command)],capture_output=True,text=True)
 assert bool(r.returncode)==fail,(command,r.stdout,r.stderr)
 cases.append(dict(command=[str(c) for c in command],exit=r.returncode,stdout_sha256=hashlib.sha256(r.stdout.encode()).hexdigest(),stderr=r.stderr))
 return None if fail else json.loads(r.stdout)
with tempfile.TemporaryDirectory(prefix='onenil-campaign-') as directory:
 directory=Path(directory);path=directory/'campaign.json'
 assert run('new',path)['committed'];assert run('project',path)==source['initial']
 assert run('swap',path,'first:0','first:17')['committed'];assert run('position',path,2582,1)['committed'];assert run('instruction',path,2598,16)['committed']
 assert run('project',path)==source['changed'];changed=run('inspect',path);assert changed['revision']==3
 committed=path.read_bytes();digest=hashlib.sha256(committed).hexdigest()
 run('new',path,fail=True);assert path.read_bytes()==committed
 for corruption in ['malformed','version','namespace','duplicate','foreign','fixture','unknown','oversized','legacy-experiment']:
  value=json.loads(committed)
  if corruption=='malformed':raw=b'{'
  elif corruption=='oversized':raw=b' '* (1024*1024+1)
  elif corruption=='legacy-experiment':raw=json.dumps(dict(schema='onenil-captured-editor',version=1,first=list(range(18)),reserve=None)).encode()
  else:
   if corruption=='version':value['version']=2
   elif corruption=='namespace':value['namespace']['league']='ENGL-94.DAT'
   elif corruption=='duplicate':value['club']['first_team'][1]=value['club']['first_team'][0]
   elif corruption=='foreign':value['players'][0]['id']=2661
   elif corruption=='fixture':value['next_fixture']['opponent']=134
   else:value['future']=True
   raw=json.dumps(value).encode()
  bad=directory/(corruption+'.json');bad.write_bytes(raw);run('load',path,bad,fail=True);assert path.read_bytes()==committed;assert run('inspect',path)==changed
 stage=subprocess.Popen([str(binary),'stage',str(path),'first:0','first:17'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 ready=json.loads(stage.stdout.readline());assert ready['stage']=='synced-before-replace';pending=Path(ready['temporary']);assert pending.is_file();assert path.read_bytes()==committed
 stage.kill();stage.wait(timeout=3);assert stage.returncode!=0
 assert path.read_bytes()==committed and run('inspect',path)==changed
 pending.unlink()
record=dict(schema='native-campaign-persistence-check/1',producer=producer,binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),tests=dict(exit=tests.returncode,stdout=tests.stdout,stderr=tests.stderr),fresh_process_cases=cases,original_projection_comparison='Every field of initial and observed T46 changed projections matches decoded original evidence.',committed_save_sha256=digest,interruption=dict(stage=ready['stage'],child_exit=stage.returncode,old_file_unchanged=True,fresh_process_reload_equal=True),scope='Real saved-club state, fixed pending fixture/week only; no engine admission, match result or progression. Linux interruption/directory-sync proof; other platforms unmeasured.')
with args.record.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
print('PASS',len(cases),'fresh Rust processes,9 invalid loads and killed pre-commit write')
