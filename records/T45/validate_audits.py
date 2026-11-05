"""T45 owned paths,immutable evidence and native runtime independence."""
import hashlib
import json
import re
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
base='efd9f6b'
prior=git('ls-tree','-r',base,'records','engine/tests/fixtures').splitlines()
for line in prior:
    header,path=line.split('\t');raw=(ROOT/path).read_bytes()
    if hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()!=header.split()[2]:raise ValueError('prior evidence changed:'+path)
for line in git('diff','--name-status',base).splitlines():
    status,path=line.split('\t')
    if not(path=='README.md' or path.startswith(('engine/','records/T45/')) or path in ('tools/prepared_match_oracle.py','tools/replay_prepared_match.py')):raise ValueError('unowned change:'+path)
    if path.startswith('records/T45/')and status!='A':raise ValueError('task record modified:'+path)
measured=0
for path in (ROOT/'records/T45').rglob('*'):
    if not path.is_file()or not re.search(r'-r\d{4}(?:/|\.)',str(path.relative_to(ROOT))):continue
    name=str(path.relative_to(ROOT));history=git('log','--diff-filter=A','--format=%H','--',name).splitlines()
    if not history:continue
    initial=subprocess.check_output(['git','show',history[-1]+':'+name],cwd=ROOT)
    if path.name=='progress.jsonl':
        if not path.read_bytes().startswith(initial):raise ValueError('append-only original progress prefix changed')
        rows=[json.loads(line)for line in path.read_text().splitlines()]
        if any(a['tick']>=b['tick']or a['draws']>=b['draws']for a,b in zip(rows,rows[1:])):raise ValueError('original progress does not increase')
    elif path.read_bytes()!=initial:raise ValueError('measured record changed:'+name)
    measured+=1
for line in git('diff','--unified=0',base,'--','engine/src').splitlines():
    if line.startswith('+')and not line.startswith('+++')and re.search(r'include_(str|bytes)!',line):raise ValueError('new embedded data')
for name in ['engine/src/watched_compiled/prepared.rs','engine/src/native_image.rs']:
    runtime=(ROOT/name).read_text().split('#[cfg(test)]')[0]
    if re.search(r'records/T45|\.cache/|Unicorn|include_bytes!|std::fs|serde_json::from_slice',runtime):raise ValueError('native runtime captured-file dependency:'+name)
print(f'{len(prior)} previous evidence/fixtures and {measured} measured task files preserved;owned paths only;no embedded data or native oracle backend')
