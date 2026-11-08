#!/usr/bin/env python3
"""[designed:T73] Authenticate immutable native history window evidence."""
import hashlib,json,sys
from pathlib import Path
from check_numbered_pitch_records import native_rgb
p=Path(sys.argv[1]);r=json.loads(p.read_text())
assert r['schema']=='native-history-navigation-window/1' and r['status']=='passed' and not r['x11_errors'] and not r['error']
assert r['producer']=='d61d78b80ee25cb090869c4c23d5c99ad4b77b76'  # exact checked implementation
assert r['binary_sha256']=='08bf00157d8b39717d270e050e2bbc3345a0354beeb0bc68c804ac35ed29108d'
assert r['font_sha256']=='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
steps={s['name']:s['state'] for s in r['steps']};base=steps['latest'];top=base['history_top']
assert len(r['steps'])==13 and len(r['screenshots'])==3 and top>18
for name,expected in [('Up',top-1),('Next',18),('Home',0),('Down',1),('Prior',0),('End',top),('opening',0)]:
 s=steps[name];assert s['screen']=='History' and s['history_top']==expected
 assert (s['frames'],s['events'],s['phase'])==(base['frames'],base['events'],base['phase'])
assert steps['resume']['screen']=='Pitch' and steps['resume']['frames']>base['frames']
assert steps['reopen-latest']['screen']=='History' and steps['reopen-latest']['history_top']>=top
assert all(s['campaign']==steps['main']['campaign'] for s in steps.values())
for shot in r['screenshots']:
 raw=p.with_name(shot['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==shot['sha256']
 assert hashlib.sha256(native_rgb(raw)).hexdigest()==shot['state']['raster_sha256']
 assert shot['state']['visible_history'] and len(shot['state']['visible_history'])==18
 if shot['name'].startswith('opening'):assert shot['state']['history_top']==0
assert {tuple(s['size']) for s in r['screenshots']}=={(1440,900),(1280,800)}
print('PASS history window:13 keyboard stages,3 raster bindings,2 sizes,paused cursor/events and campaign preserved')
