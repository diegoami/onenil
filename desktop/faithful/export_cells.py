#!/usr/bin/env python3
"""[derived] T51 independently drawn cell styles and authenticated decoded original text."""
import argparse, collections, hashlib, importlib.util, json, subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);args=p.parse_args()
spec=importlib.util.spec_from_file_location('study',ROOT/'docs/desktop-fidelity/mockups/build.py');study=importlib.util.module_from_spec(spec);spec.loader.exec_module(study)
plan=study.plan();rows=plan['screens'].copy()
for name,step in [('pearce-details',5),('pearce-skills',6),('mimms-details',20)]:
 record=f'runs/T45/session-r0001/step-{step:04d}.json';reference=record.replace('.json','.png')
 rows.append(dict(id=name,record=record,reference=reference,step=step,record_sha256=study.sha(study.original(args.research_root,record)),reference_sha256=study.sha(study.original(args.research_root,reference)),pitch_layer=False))
screens={};sources=[]
for row in rows:
 lines,image=study.input_screen(row,args.research_root)
 ground=collections.Counter(image.getdata()).most_common(1)[0][0]
 cells=[]
 for y,line in enumerate(lines):
  for x,ch in enumerate(line):
   counts=collections.Counter(image.crop((x*9,y*16,x*9+9,y*16+16)).getdata())
   primitive=row['pitch_layer'] and 5<=y<24 and ch in 'ôÆ£¥öò•'
   if ch in study.BOX_ARMS or ch in study.BLOCKS:
    bg=(0,0,0) if counts[(0,0,0)]>counts[ground] else ground
   elif primitive:bg=ground
   else:bg=counts.most_common(1)[0][0]
   candidates=[item for item in counts.most_common() if item[0]!=bg];fg=candidates[0][0] if candidates else bg
   pack=lambda rgb:rgb[0]<<16|rgb[1]<<8|rgb[2]
   cells.append([ch,pack(fg),pack(bg)])
 screens[row['id']]=cells
 sources.append(dict(id=row['id'],record=row['record'],record_sha256=row['record_sha256'],reference=row['reference'],reference_sha256=row['reference_sha256']))
output=dict(schema='desktop-cells/1',research_commit=study.PIN,sources=sources,screens=screens)
path=ROOT/'desktop/faithful/assets/cells.json'
with path.open('x') as f:json.dump(output,f,ensure_ascii=True,separators=(',',':'));f.write('\n')
print('Exported',len(screens),'authenticated decoded cell sheets; no image/font bytes')
