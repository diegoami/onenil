#!/usr/bin/env python3
"""[derived] Append the authenticated W006 decoded layout using the T51 cell exporter."""
import argparse, importlib.util, json, subprocess, tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
# Execute the same exporter over the expanded plan, producing to a temporary target.
spec=importlib.util.spec_from_file_location('study',ROOT/'docs/desktop-fidelity/mockups/build.py');study=importlib.util.module_from_spec(spec);spec.loader.exec_module(study)
record='runs/T43/session-r0003/step-0013.json'; reference=record.replace('.json','.png')
row=dict(id='saved-games',record=record,reference=reference,step=13,record_sha256=study.sha(study.original(a.research_root,record)),reference_sha256=study.sha(study.original(a.research_root,reference)),pitch_layer=False)
# Extract only the existing per-cell conversion, so styling is identical to T51.
source=(ROOT/'desktop/faithful/export_cells.py').read_text()
source=source[source.index('screens={};sources=[]'):source.index('output=dict(')]
scope=dict(rows=[row],study=study,args=a)
import collections
scope['collections']=collections
exec(source,scope)
path=ROOT/'desktop/faithful/assets/cells.json';value=json.loads(path.read_text())
assert 'saved-games' not in value['screens'], 'already exported'
value['screens'].update(scope['screens']);value['sources'].extend(scope['sources'])
path.write_text(json.dumps(value,ensure_ascii=True,separators=(',',':'))+'\n')
print('Added W006 decoded cell sheet; original record/image hashes retained')
