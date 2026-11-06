#!/usr/bin/env python3
"""[derived] Bounded decoded control layouts from pinned original captures, using T51 styling."""
import argparse,collections,importlib.util,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
spec=importlib.util.spec_from_file_location('study',ROOT/'docs/desktop-fidelity/mockups/build.py');study=importlib.util.module_from_spec(spec);spec.loader.exec_module(study)
path=ROOT/'desktop/faithful/assets/cells.json';assets=json.loads(path.read_text())
for name,step in [('player-options',10),('player-instructions',15)]:
 record=f'runs/T46/session-r0001/step-{step:04d}.json';reference=record.replace('.json','.png')
 row=dict(id=name,record=record,reference=reference,step=step,record_sha256=study.sha(study.original(a.research_root,record)),reference_sha256=study.sha(study.original(a.research_root,reference)))
 lines,image=study.input_screen(row,a.research_root);ground=collections.Counter(image.getdata()).most_common(1)[0][0];cells=[]
 for y,line in enumerate(lines):
  for x,ch in enumerate(line):
   counts=collections.Counter(image.crop((x*9,y*16,x*9+9,y*16+16)).getdata())
   bg=((0,0,0) if counts[(0,0,0)]>counts[ground] else ground) if ch in study.BOX_ARMS or ch in study.BLOCKS else counts.most_common(1)[0][0]
   candidates=[v for v in counts.most_common() if v[0]!=bg];fg=candidates[0][0] if candidates else bg
   pack=lambda c:c[0]<<16|c[1]<<8|c[2]
   cells.append([ch,pack(fg),pack(bg)])
 assert name not in assets['screens'];assets['screens'][name]=cells;assets['sources'].append(row)
path.write_text(json.dumps(assets,ensure_ascii=True,separators=(',',':'))+'\n')
print('Exported two decoded control sheets; no original font/pixel resources')
