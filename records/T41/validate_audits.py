"""[designed] T41 ownership, immutable evidence and runtime embedding checks."""
import hashlib,re,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,text=True)
def main():
 base='4c4598a';prior=git('ls-tree','-r',base,'records','engine/tests/fixtures').splitlines()
 for line in prior:
  header,path=line.split('\t');expected=header.split()[2];raw=(ROOT/path).read_bytes();actual=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()
  if actual!=expected:raise ValueError('prior evidence changed:'+path)
 for line in git('diff','--name-status',base).splitlines():
  status,path=line.split('\t')
  if not(path=='README.md'or path.startswith(('engine/','records/T41/'))or path in('tools/compile_watched.py','tools/watched_match_oracle.py','tools/replay_watched_match.py')):raise ValueError('unowned change:'+path)
  if path.startswith('records/T41/')and status!='A':raise ValueError('measured record modified:'+path)
 test_modules=set()
 for p in(ROOT/'engine/src').rglob('*.rs'):
  for name in re.findall(r'#\[cfg\(test\)\]\s*mod (\w+);',p.read_text()):test_modules.add(p.parent/f'{name}.rs')
 for p in(ROOT/'engine/src').rglob('*.rs'):
  text=p.read_text()
  for match in re.finditer(r'include_(?:str|bytes)!',text):
   if p not in test_modules and '#[cfg(test)]'not in text[:match.start()]:raise ValueError('runtime embedded bytes:'+str(p))
 for name in('watched_match.rs','match_graph.rs','lineup_validation.rs'):
  runtime=(ROOT/'engine/src/watched_compiled'/name).read_text().split('#[cfg(test)]')[0]
  if re.search(r'fixtures|records/T40|\.cache/|oracle|Unicorn|include_bytes!',runtime):raise ValueError('match runtime contains oracle dependency')
 print(f'{len(prior)}prior evidence/fixture files preserved;all changed paths owned;no modified measured records;includes confined to test modules;match runtime has no oracle/file/byte dependency')
 print('Embedding audit accounts for external #[cfg(test)] module declarations;initial inline-only audit incorrectly flagged existing shared_image_tests.rs. No engine change required.')
if __name__=='__main__':main()
