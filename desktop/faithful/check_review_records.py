#!/usr/bin/env python3
"""[designed] Review126 fix: identical explicit Mark row before/after edits and restart."""
import argparse,hashlib,json,subprocess,sys
from pathlib import Path
from PIL import Image
HERE=Path(__file__).resolve().parent
p=argparse.ArgumentParser();p.add_argument('record',type=Path);a=p.parse_args()
subprocess.check_call([sys.executable,str(HERE/'check_controls_records.py'),str(a.record)])
r=json.loads(a.record.read_text());shots={s['name']:s for s in r['screenshots']}
assert all('raster_sha256' in shot for shot in shots.values())
names=['keeper-explicit-one']+['keeper-cycle-'+str(i) for i in range(1,6)]+['fresh-instructions']
rows=[]
for name in names:
 shot=shots[name];image=Image.open(a.record.parent/shot['path']).convert('RGB');assert image.size==(1440,900)
 rows.append(image.crop((2*18,50+17*32,36*18,50+18*32)).tobytes())
assert all(row==rows[0] for row in rows),'Mark row differs across captured/changed/fresh instruction cards'
print('PASS explicit Mark row is identical across7 native instruction images including fresh restart; SHA256',hashlib.sha256(rows[0]).hexdigest())
