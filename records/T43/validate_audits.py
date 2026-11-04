"""[designed] T43 path ownership,evidence preservation and runtime dependencies."""
import hashlib,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def main():
 base='b7de4be';prior=git('ls-tree','-r',base,'records','engine/tests/fixtures').splitlines()
 for line in prior:
  header,path=line.split('\t');raw=(ROOT/path).read_bytes()
  if hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()!=header.split()[2]:raise ValueError('prior evidence changed:'+path)
 for line in git('diff','--name-status',base).splitlines():
  status,path=line.split('\t')
  if not(path=='README.md'or path.startswith(('engine/','records/T43/'))or path in('tools/watched_setup_oracle.py','tools/replay_watched_setup.py')):raise ValueError('unowned change:'+path)
  if path.startswith('records/T43/')and status!='A':raise ValueError('measured record modified:'+path)
 # No new includes,including after pre-existing inline test modules. This avoids
 # treating all code after the first cfg(test) marker as test-only runtime.
 for line in git('diff','--unified=0',base,'--','engine/src').splitlines():
  if line.startswith('+')and not line.startswith('+++')and re.search(r'include_(str|bytes)!',line):raise ValueError('new embedded source data')
 runtime=(ROOT/'engine/src/watched_setup.rs').read_text().split('#[cfg(test)]')[0]
 if re.search(r'fixtures|records/T43|\.cache/|oracle|Unicorn|include_bytes!|std::fs',runtime):raise ValueError('watched preparation runtime depends on files/outcomes')
 runtime=(ROOT/'tools/replay_watched_setup.py').read_text()
 # The normalizer/assertions may read expected states;the Rust factory may only
 # receive the separately constructed initial-input envelope.
 if "NativeWatchedPreparation(json.dumps(value['input']))"not in runtime:raise ValueError('native bridge initial-input boundary changed')
 print(f'{len(prior)}prior evidence/fixture files preserved;all changed paths owned;no modified measured records;no added includes;owned watched preparation has no fixture/file/oracle dependency')
if __name__=='__main__':main()
