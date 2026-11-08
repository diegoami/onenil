#!/usr/bin/env python3
"""T77: authenticate the actual live desktop phase/ownership/raster/save workflow."""
import argparse,copy,hashlib,json,re
from pathlib import Path
from check_numbered_pitch_records import native_rgb
ROOT=Path(__file__).resolve().parents[2]
PINS={}
FONT='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
sha=lambda b:hashlib.sha256(b).hexdigest()
def check(r,directory,images=True):
 assert r['schema']=='native-live-desktop-window/1' and r['case'] in ('original','edited')
 assert r['status']=='passed' and not r['error'] and not r['x11_errors']
 assert r['owned_windows_closed'] and r['cancelled_and_closed_workers_reaped'] and r['fresh_process_count']==3
 assert r['font_sha256']==FONT and r['runtime']['window_backend']=='software/minifb'
 assert r['runtime']['cargo_lock_sha256']==sha((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
 assert r['names_sha256']==sha((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes())
 deps={'desktop/faithful/live_desktop_window_check.py','desktop/faithful/src/application.rs','desktop/faithful/src/controller.rs','desktop/faithful/src/timeline.rs','desktop/faithful/src/render.rs'}
 assert set(r['producer_files'])==deps and all(re.fullmatch('[0-9a-f]{64}',v) for v in r['producer_files'].values())
 steps={s['name']:s['state'] for s in r['steps']};assert len(steps)==len(r['steps'])
 expected=['main']+(['players','edited-team','edited-main'] if r['case']=='edited' else [])+['initial-save','preparation','preparing-worker','cancelled-start','first-half','history','history-Home','history-Down','history-End','first-half-bench','first-half-resume','half-time','interval-bench','resuming','second-half','second-half-history','full-time','recorded','saved-result','fresh-reload','duplicate-refused','close-incomplete']
 assert list(steps)==expected
 initial=steps['initial-save']['campaign'];original=steps['main']['campaign'];want=copy.deepcopy(original)
 if r['case']=='edited':want['club']['first_team'][1:3]=[2583,2582];want['revision']+=1
 assert initial==want
 before=['initial-save','preparation','preparing-worker','cancelled-start','first-half','history','history-Home','history-Down','history-End','first-half-bench','first-half-resume','half-time','interval-bench','resuming','second-half','second-half-history','full-time']
 assert all(steps[k]['campaign']==initial for k in before)
 assert not steps['cancelled-start']['worker_present'] and steps['cancelled-start']['screen']=='Preparation'
 held=['history','history-Home','history-Down','history-End','first-half-bench','first-half-resume','half-time','interval-bench','resuming']
 assert all(steps[k]['worker_present'] and steps[k]['worker_same'] and steps[k]['live_worker'] and steps[k]['match_stage']=='incomplete' for k in held)
 history=steps['history']
 for key in ['history-Home','history-Down','history-End']:
  row=steps[key];assert (row['frames'],row['events'],row['pitch_markers'])==(history['frames'],history['events'],history['pitch_markers'])
 assert steps['history-Home']['history_top']==0 and steps['history-Down']['history_top']==1
 assert steps['history-End']['history_top']>1
 half=steps['half-time'];expected_frames=1940 if r['case']=='edited' else 1765
 assert (half['screen'],half['phase'],half['frames'],half['loaded_frames'],half['clock'])==('HalfTime','HalfTime',expected_frames,expected_frames,46)
 resume=steps['resuming'];bench=steps['interval-bench']
 assert resume['screen']=='Preparing' and resume['resuming'] and resume['phase']=='HalfTime'
 assert all(resume[k]==bench[k] for k in ('frames','events','recent_actions','pitch_markers'))
 after=steps['second-half'];assert after['screen']=='Pitch' and after['phase']=='SecondHalf' and after['match_stage']=='complete' and not after['worker_present'] and after['frames']>=half['frames'] and after['events']>=half['events']
 final=steps['full-time'];score=[0,2] if r['case']=='edited' else [1,4];frames=3970 if r['case']=='edited' else 3825;events=870 if r['case']=='edited' else 749
 assert (final['screen'],final['phase'],final['frames'],final['events'],final['score'])==('FullTime','FullTime',frames,events,score)
 assert not final['worker_present']
 saved={}
 for proof in r['save_proofs']:
  assert Path(proof['path']).name==proof['path'];raw=(directory/proof['path']).read_bytes();assert sha(raw)==proof['sha256'];saved[proof['name']]=(json.loads(raw),raw)
 assert set(saved)=={'initial','completed'} and saved['initial'][0]==initial
 recorded=steps['recorded']['campaign'];expected=copy.deepcopy(initial);expected['revision']+=1;expected['played_fixture']=dict(fixture=initial['next_fixture'],season=initial['season'],week_index=initial['week_index'],request_revision=initial['revision'],request_sha256=sha(saved['initial'][1]),score=score,native_return='0AB9:EBE2')
 assert recorded==expected and saved['completed'][0]==recorded
 assert all(steps[k]['campaign']==recorded and not steps[k]['worker_present'] for k in ['recorded','saved-result','fresh-reload','duplicate-refused'])
 assert steps['fresh-reload']['frames'] is None and steps['duplicate-refused']['screen']=='Preparation' and 'already' in steps['duplicate-refused']['message']
 assert steps['close-incomplete']['worker_present'] and steps['close-incomplete']['match_stage']=='incomplete' and 'played_fixture' not in steps['close-incomplete']['campaign']
 assert r['keyboard']==[key for row in r['steps'] for key in row['keys']]
 names=json.loads((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_text())['players'];messages=json.loads((ROOT/'desktop/faithful/messages.json').read_text())
 for row in [s for s in steps.values() if s.get('recent_actions')]:
  for action in row['recent_actions']:
   event=action['native'];text=messages[event['code']][1]
   for marker,field in [('{p}','player'),('{s}','secondary')]:
    if marker in text:text=text.replace(marker,names[str(event[field])])
   assert action['text']==text and 'Play continues' not in text and event['minute']<=row['clock']
 expected_shots={'history-default','history-resized','half-time-default','half-time-resized','second-half-default','second-half-resized','full-time'}
 assert {s['name'] for s in r['screenshots']}==expected_shots
 for shot in r['screenshots']:
  assert Path(shot['path']).name==shot['path'];s=shot['state'];assert s['players']==22 and len(s['pitch_markers'])==22
  for marker in s['pitch_markers']:
   selector=marker['selector'];assert marker['label']==('G' if selector in (1,12) else chr(((selector-1)&65535)%11+47)) and 0<=marker['attribute']<=255
  if images:
   raw=(directory/shot['path']).read_bytes();assert sha(raw)==shot['sha256'];assert sha(native_rgb(raw))==s['raster_sha256']
 assert {tuple(s['size']) for s in r['screenshots']}=={(1440,900),(1280,800)}
def verify(path):
 raw=path.read_bytes();pin=PINS[path.name];assert sha(raw)==pin['sha256'];r=json.loads(raw);assert r['producer']==pin['producer'] and r['binary_sha256']==pin['binary'];check(r,path.parent)
 for damage in range(10):
  bad=copy.deepcopy(r);steps={s['name']:s['state'] for s in bad['steps']}
  if damage==0:steps['history-Home']['worker_same']=False
  elif damage==1:steps['resuming']['frames']=0
  elif damage==2:steps['second-half']['frames']=0
  elif damage==3:steps['full-time']['score']=steps['full-time']['score'][::-1]
  elif damage==4:steps['half-time']['campaign']['played_fixture']={}
  elif damage==5:bad['cancelled_and_closed_workers_reaped']=False
  elif damage==6:bad['screenshots'][0]['state']['raster_sha256']='0'*64
  elif damage==7:bad['producer_files']={}
  elif damage==8:bad['save_proofs'][0]['sha256']='0'*64
  else:steps['half-time']['match_stage']='complete'
  try:check(bad,path.parent,images=damage==6)
  except (AssertionError,KeyError):pass
  else:raise AssertionError('corrupted live desktop proof accepted: '+str(damage))
 print('PASS live desktop',r['case'],len(r['steps']),'steps,7 raster bindings,10 corruption controls;real continuation and saved result')
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('record',type=Path);a=p.parse_args();verify(a.record)
