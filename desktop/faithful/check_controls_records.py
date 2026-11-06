#!/usr/bin/env python3
"""Authenticate native keyboard-only original position/instruction transitions and restart."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
def projection(c):
 c=copy.deepcopy(c)
 for name in ['origin','revision','version']:c.pop(name)
 c['schema']='original-club-seed/1'
 for name in ['first_team','reserves']:c['club'][name]=[p or 0 for p in c['club'][name]]
 c['players'].sort(key=lambda p:p['id']);return c
def check(r):
 assert r['schema']=='native-player-controls-window/1' and r['status']=='passed' and not r['error'] and not r['x11_errors'] and r['headless_campaign_edits']==0
 source=json.loads((ROOT/'campaign/fixtures/research-seed.json').read_text())
 steps=r['steps'];s={v['name']:v['state'] for v in steps};assert len(steps)==len(s)==40
 assert projection(s['initial']['campaign'])==source['initial'];expected=copy.deepcopy(s['initial']['campaign'])
 changes={'keeper-cycle-'+str(i+1):v for i,v in enumerate([1,16,17,1,16])}
 for row in steps[1:]:
  name=row['name'];state=row['state']
  if name=='original-goalkeeper-swap':expected['club']['first_team'][0],expected['club']['first_team'][17]=expected['club']['first_team'][17],expected['club']['first_team'][0];expected['revision']+=1
  elif name in changes:expected['players'][17]['individual_instructions']=changes[name];expected['revision']+=1
  elif name.startswith('pearce-first-') or name.startswith('pearce-cycle-'):expected['players'][1]['current_position']=int(name.rsplit('-',1)[1]);expected['revision']+=1
  elif name=='pearce-club-restored':expected['players'][1]['current_position']=2;expected['revision']+=1
  assert state['campaign']==expected,name
 assert projection(expected)==source['changed'] and expected['revision']==23
 assert s['keeper-options-details']['panel']=='Details' and not s['keeper-options-details']['option_available'][0]
 assert s['keeper-instructions']['screen']=='Instructions' and s['keeper-instructions']['instruction']==0
 assert s['instruction-escape-options']['screen']=='Options' and s['instruction-escape-options']['menus'][2]==4
 assert s['remembered-instruction-row']['instruction']==1
 assert s['instruction-insert-options-skills']['screen']=='Options' and s['instruction-insert-options-skills']['panel']=='Skills'
 assert s['options-escape-players']['screen']=='Players'
 assert s['pearce-remembered-option']['menus'][2]==4 and s['pearce-remembered-option']['card']==2582 and s['pearce-remembered-option']['panel']=='Details'
 for name in ['disabled-keeper-reposition','unimplemented-instruction-refused','unsupported-player-action-refused']:assert 'unavailable' in s[name]['message']
 assert s['fresh-startup-save-restored']['screen']=='Main' and s['fresh-keeper-card']['card']==2598 and s['fresh-pearce-positions']['card']==2582
 assert len(set(r['desktop_process_ids']))==2
 assert r['font_sha256']=='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
def verify(path):
 r=json.loads(path.read_text());check(r);subprocess.check_call(['git','cat-file','-e',r['producer']+'^{commit}'],cwd=ROOT)
 seen=set()
 for shot in r['screenshots']:
  assert Path(shot['path']).name==shot['path'] and shot['path'] not in seen;seen.add(shot['path']);image=path.parent/shot['path']
  assert hashlib.sha256(image.read_bytes()).hexdigest()==shot['sha256'] and list(Image.open(image).size)==shot['size']
 assert len(seen)==15
 for kind in ['disabled-change','unrelated-bit','role-cycle','club-role','remembered-row','fresh-state']:
  bad=copy.deepcopy(r);s={v['name']:v['state'] for v in bad['steps']}
  if kind=='disabled-change':s['disabled-keeper-reposition']['campaign']['revision']+=1
  elif kind=='unrelated-bit':s['keeper-cycle-2']['campaign']['players'][17]['individual_instructions']^=2
  elif kind=='role-cycle':s['pearce-cycle-0']['campaign']['players'][1]['current_position']=1
  elif kind=='club-role':s['pearce-club-restored']['campaign']['players'][1]['club_position']=3
  elif kind=='remembered-row':s['remembered-instruction-row']['instruction']=0
  else:s['fresh-keeper-card']['campaign']['players'][17]['individual_instructions']=17
  try:check(bad)
  except AssertionError:pass
  else:raise AssertionError('accepted corrupted record: '+kind)
 print('PASS 40 original-control native steps,15 images,2 fresh processes,6 negative controls; no headless edits')
p=argparse.ArgumentParser();p.add_argument('record',type=Path);a=p.parse_args();verify(a.record)
