"""T44 ownership,evidence preservation and absence of runtime outcome dependencies."""
import hashlib
import re
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
base='c8e2b3bf2572b27df3f4278c6855a1219af2322e'
prior=git('ls-tree','-r',base,'records','engine/tests/fixtures').splitlines()
for line in prior:
    header,path=line.split('\t');raw=(ROOT/path).read_bytes()
    if hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()!=header.split()[2]:raise ValueError('prior evidence changed:'+path)
for line in git('diff','--name-status',base).splitlines():
    status,path=line.split('\t')
    if not(path=='README.md' or path.startswith(('engine/','records/T44/')) or path in ('tools/compile_watched.py','tools/prepared_watched_oracle.py','tools/replay_prepared_watched.py')):raise ValueError('unowned change:'+path)
    if path.startswith('records/T44/') and status!='A':raise ValueError('prior task record modified:'+path)
measured=0
for path in (ROOT/'records/T44').rglob('*'):
    if not path.is_file() or not re.search(r'-r\d{4}(?:/|\.)',str(path.relative_to(ROOT))):continue
    name=str(path.relative_to(ROOT));history=git('log','--diff-filter=A','--format=%H','--',name).splitlines()
    if not history:continue
    initial=subprocess.check_output(['git','show',history[-1]+':'+name],cwd=ROOT)
    if path.read_bytes()!=initial:raise ValueError('measured task record changed after first commit:'+name)
    measured+=1
for line in git('diff','--unified=0',base,'--','engine/src').splitlines():
    if line.startswith('+') and not line.startswith('+++') and re.search(r'include_(str|bytes)!',line):raise ValueError('new embedded data')
for name in ['engine/src/watched_compiled/prepared.rs','engine/src/native_image.rs']:
    runtime=(ROOT/name).read_text().split('#[cfg(test)]')[0]
    if re.search(r'fixtures|records/T44|\.cache/|Unicorn|include_bytes!|std::fs|serde_json::from_slice',runtime):raise ValueError('prepared runtime depends on captured data:'+name)
runtime=(ROOT/'tools/replay_prepared_watched.py').read_text()
if 'native.NativePreparedWatchedSession(json.dumps(value))' not in runtime:raise ValueError('Python native factory input boundary differs')
print(f'{len(prior)} prior evidence/fixtures and {measured} committed task records preserved;owned paths only;no embedded data or native runtime fixture/oracle/file dependency')
