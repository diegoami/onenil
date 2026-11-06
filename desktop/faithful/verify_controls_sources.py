#!/usr/bin/env python3
"""[designed] Verify the source-bound UI controls read map and decoded source cells."""
import argparse,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
PIN='68cd3396ec3621d55df8c9377cd5c4b7b74c1314'
FILES={
 'static/1-0-exe/code/seg_0AB9_17.asm':['AC00','AC07','AC27','AC3B','AC6A','AC71','AC75','B2EB','B34A','B365','B379','B391','B3A9','B3C2','B3FD'],
 'static/1-0-exe/code/seg_0AB9_19.asm':['C4D1','C73C','C748','C774'],
 'static/1-0-exe/code/seg_232A_01.asm':['0098','00D5','00F8','011D'],
 'static/1-0-exe/code/seg_232A_02.asm':['0EC4'],
 'static/1-0-exe/code/seg_24BC_01.asm':['042B','048C'],
 'docs/manual/selection.md':[],
}
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);p.add_argument('--record',type=Path);a=p.parse_args();sources={}
for name,addresses in FILES.items():
 data=subprocess.check_output(['git','-C',str(a.research_root),'show',PIN+':'+name]);assert data==(a.research_root/name).read_bytes()
 for address in addresses:assert any(line.startswith(address+' ') for line in data.decode().splitlines()),(name,address)
 sources[name]=dict(sha256=hashlib.sha256(data).hexdigest(),listing_addresses=addresses)
assets=json.loads((ROOT/'desktop/faithful/assets/cells.json').read_text());captures=[]
for name in ['player-options','player-instructions']:
 row=next(r for r in assets['sources'] if r['id']==name);capture={}
 for field in ['record','reference']:
  data=subprocess.check_output(['git','-C',str(a.research_root),'show',assets['research_commit']+':'+row[field]])
  assert hashlib.sha256(data).hexdigest()==row[field+'_sha256'];capture[field]=row[field];capture[field+'_sha256']=row[field+'_sha256']
  if field=='record':
   value=json.loads(data);lines=value['text'].splitlines();lines=[line.ljust(80) for line in lines]+[' '*80]*(25-len(lines));assert ''.join(lines)==''.join(c[0] for c in assets['screens'][name])
 captures.append(capture)
result=dict(schema='original-player-controls-source/1',research_commit=PIN,sources=sources,captures=captures,confirmed=['Club-role zero disables Reposition; current role is independent','Current role wraps across14 codes via one-based helper','Club-position restore requires age18 or greater','Positioning inherited -> explicit zero -> explicit one -> inherited with lower choice copied from club','Only bits0 and4 change for Positioning','Options and instruction entry force details; instruction row is remembered; Escape returns to options'],designed_bounds=['Only goalkeeper Positioning is editable in the instruction slice','Other management actions and instruction rows diagnose unavailable','Supported prototype availability marks replace unsupported original ticks'])
if a.record:
 with a.record.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
print('PASS pinned original handlers/helpers,2 decoded control layouts and source image hashes')
