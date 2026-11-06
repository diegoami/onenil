#!/usr/bin/env python3
"""[derived] Decode bounded presentation labels at the original documented draw tables."""
import argparse,hashlib,importlib.util,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);args=p.parse_args()
spec=importlib.util.spec_from_file_location('original_templates',args.research_root/'tools/static/ui_blueprint.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
data,covered,paths=module.data_segment()
def text(address,width):
 n=data[address];assert n<width and set(range(address,address+1+n))<=covered
 return bytes(data[address+1:address+1+n]).decode('cp437').strip()
labels=dict(schema='original-campaign-labels/1',positions=[text(0x8724+i*19,19) for i in range(14)],role_glyphs=[module.glyph(data[0x8716+i]) for i in range(14)],instructions=[[text(0x85E8+i*32+j*16,16) for j in range(2)] for i in range(1,9)],goalkeeper_position=[text(0x0B70+j*16,16) for j in range(2)],goalkeeper_passing=[text(0x0B90+j*16,16) for j in range(2)],sources={str(path.relative_to(args.research_root)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths})
for path in paths:assert subprocess.check_output(['git','show','6aca98caeebc1b9b5410852139fa305462224a05:'+str(path.relative_to(args.research_root))],cwd=args.research_root)==path.read_bytes()
with (ROOT/'desktop/faithful/assets/campaign-labels.json').open('w') as f:json.dump(labels,f,indent=2);f.write('\n')
print('Decoded14position labels and bounded two-choice instruction text; no original resource bytes')
