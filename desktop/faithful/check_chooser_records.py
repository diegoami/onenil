#!/usr/bin/env python3
"""Authenticate real-file chooser transitions, producer/source identity and images."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
sha=lambda data:hashlib.sha256(data).hexdigest()
def check(r):
 assert r['schema']=='native-save-chooser-window/1' and r['status']=='passed' and not r['error'] and not r['x11_errors']
 s={step['name']:step['state'] for step in r['steps']};assert len(s)==len(r['steps'])==15
 initial=s['initial'];saved=s['single-direct']['campaign'];manual=s['chosen-manual-committed'];edited=s['save-only-chosen-target']['campaign']
 assert initial['campaign']['players'][1]['current_position']==2 and saved['players'][1]['current_position']==1
 assert manual['campaign']['players'][17]['individual_instructions']==16 and manual['campaign']['players'][1]['current_position']==2
 assert manual['save_path']!=initial['save_path'] and manual['game_name']=='MANUAL'
 for name in ['multiple-first-selected','escape-keeps-selection','native-cancel','malformed-refused','version-refused','source-refused']:
  assert s[name]['campaign']==saved and s[name]['save_path']==initial['save_path'] and s[name]['game_name']=='squad'
 assert s['no-files']['campaign']==initial['campaign'] and s['no-files']['save_path']==initial['save_path']
 assert s['multiple-first-selected']['load_entries']==['MANUAL','squad'] and s['multiple-first-selected']['load_index']==0
 assert s['escape-keeps-selection']['screen']=='SavedGames' and s['escape-keeps-selection']['load_index']==1
 assert s['native-cancel']['screen']=='Main'
 for name in ['malformed-refused','version-refused','source-refused']:assert s[name]['screen']=='SavedGames' and 'refused' in s[name]['message']
 assert edited!=manual['campaign'] and edited['club']['first_team'][:2]==[2582,2581]
 assert s['fresh-startup-list']['screen']=='SavedGames' and s['fresh-startup-list']['load_index']==0
 assert s['fresh-manual-restore']['campaign']==edited and s['fresh-manual-restore']['game_name']=='MANUAL'
 for name in ['other-save-restore','fresh-direct-load']:assert s[name]['campaign']==saved and s[name]['save_path']==initial['save_path']
 assert len(set(r['desktop_process_ids']))==3
 assert r['font_sha256']=='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
def verify(path,research):
 r=json.loads(path.read_text());check(r)
 subprocess.check_call(['git','cat-file','-e',r['producer']+'^{commit}'],cwd=ROOT)
 seen=set()
 for shot in r['screenshots']:
  assert Path(shot['path']).name==shot['path'] and shot['path'] not in seen;seen.add(shot['path'])
  image=path.parent/shot['path'];assert sha(image.read_bytes())==shot['sha256'] and list(Image.open(image).size)==shot['size']
 assert len(seen)==10
 asset=json.loads(subprocess.check_output(['git','show',r['producer']+':desktop/faithful/assets/cells.json'],cwd=ROOT))
 source=next(s for s in asset['sources'] if s['id']=='saved-games')
 for field in ['record','reference']:
  data=subprocess.check_output(['git','-C',str(research),'show',asset['research_commit']+':'+source[field]])
  assert sha(data)==source[field+'_sha256']
  if field=='record':
   record=json.loads(data);lines=record['text'].splitlines();lines=[line.ljust(80) for line in lines]+[' '*80]*(25-len(lines))
   assert ''.join(lines)==''.join(c[0] for c in asset['screens']['saved-games'])
 for kind in ['target','selection','restored','escape']:
  bad=copy.deepcopy(r);s={v['name']:v['state'] for v in bad['steps']}
  if kind=='target':s['source-refused']['save_path']='wrong.json'
  elif kind=='selection':s['escape-keeps-selection']['load_index']=0
  elif kind=='restored':s['fresh-manual-restore']['campaign']['players'][1]['current_position']=9
  else:s['escape-keeps-selection']['screen']='Main'
  try:check(bad)
  except AssertionError:pass
  else:raise AssertionError('accepted corruption: '+kind)
 print('PASS 15 native steps,10 authenticated images,3 fresh processes,4 negative controls')
p=argparse.ArgumentParser();p.add_argument('record',type=Path);p.add_argument('--research-root',type=Path,required=True);a=p.parse_args();verify(a.record,a.research_root)
