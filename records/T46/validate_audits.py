"""[designed] Owned paths, immutable evidence and independent native runtime."""
import hashlib
import re
import subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
BASE='71c9723398f63ca0fcdc9c83324782725d787c1e'
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
prior=git('ls-tree','-r',BASE,'records','engine/tests/fixtures','docs/tasks/T46.md').splitlines()
for line in prior:
    header,name=line.split('\t');raw=(ROOT/name).read_bytes()
    digest=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()
    if digest!=header.split()[2]:raise ValueError('prior evidence/task changed:'+name)
allowed={'README.md','docs/first-playable.md','tools/play_watched.py','tools/test_play_watched.py',
         'tools/smoke_watched_viewer.py','tools/replay_prepared_choices.py'}
for line in git('diff','--name-status',BASE).splitlines():
    status,name=line.split('\t')
    if name not in allowed and not name.startswith(('engine/','records/T46/','web/watched/')):
        raise ValueError('unowned path:'+name)
measured=0
names=set(git('log','--diff-filter=A','--name-only','--format=','--','records/T46','engine/tests/fixtures/prepared_choices').splitlines())
for name in sorted(names):
    if not re.search(r'-r\d{4}(?:[-/.])',name):continue
    history=git('log','--diff-filter=A','--format=%H','--',name).splitlines()
    original=subprocess.check_output(['git','show',history[-1]+':'+name],cwd=ROOT)
    path=ROOT/name
    if not path.is_file()or path.read_bytes()!=original:raise ValueError('measured evidence changed/deleted:'+name)
    measured+=1
for line in git('diff','--unified=0',BASE,'--','engine/src').splitlines():
    if line.startswith('+')and not line.startswith('+++')and re.search(r'include_(bytes|str)!',line):
        raise ValueError('new embedded data')
for name in ['engine/src/watched_compiled/prepared.rs','engine/src/native_image.rs']:
    runtime=(ROOT/name).read_text().split('#[cfg(test)]')[0]
    if re.search(r'\.cache/|Unicorn|include_bytes!|std::fs|serde_json::from_slice|records/T46',runtime):
        raise ValueError('native runtime external/oracle dependency:'+name)
product=(ROOT/'tools/play_watched.py').read_text()
for token in ('watched_match(', 'fixtures/watched_match/initial.json','records/T41','records/T46','Unicorn','unicorn'):
    if token in product:raise ValueError('product outcome/backend dependency:'+token)
print(f'{len(prior)} prior evidence/fixtures/task and{measured} task measurements preserved;owned paths only;no embedded or captured runtime backend')
