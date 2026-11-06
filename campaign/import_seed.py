#!/usr/bin/env python3
"""Pin decoded initial/changed club projections from the original evidence repository."""
import argparse,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
COMMIT='39e289a'
RECORD='runs/T65/seed-r0001.json'
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);p.add_argument('--build',action='store_true');args=p.parse_args()
commit=subprocess.check_output(['git','rev-parse',COMMIT],cwd=args.research_root,text=True).strip()
raw=subprocess.check_output(['git','show',commit+':'+RECORD],cwd=args.research_root);value=json.loads(raw)
assert value['schema']=='campaign-seed-evidence/1' and value['initial']['namespace']['league']=='ENGL-95.DAT'
for name in ['initial','changed']:
 digest=hashlib.sha256(json.dumps(value[name],sort_keys=True,separators=(',',':')).encode()).hexdigest();assert digest==value['projection_sha256'][name]
manifest=dict(research_repository='https://github.com/diegoami/onenil-archaeology',research_commit=commit,record=RECORD,record_sha256=hashlib.sha256(raw).hexdigest(),producer=value['producer'],save_sha256=value['save_sha256'],league_sha256=value['league_sha256'],projection_sha256=value['projection_sha256'])
for name,data in [('research-seed.json',raw),('source.json',(json.dumps(manifest,indent=2)+'\n').encode())]:
 path=ROOT/'campaign/fixtures'/name
 if args.build:
  with path.open('xb') as f:f.write(data)
 else:assert path.read_bytes()==data,name
print('PASS pinned original decoded source',commit,manifest['record_sha256'])
