#!/usr/bin/env python3
"""T78: authenticate the actual-window squad selection, refusal and persisted exchange."""
import argparse,hashlib,json,re
from pathlib import Path
from check_numbered_pitch_records import native_rgb
ROOT=Path(__file__).resolve().parents[2]
PINS={'window-r0003.json': {'sha256': 'f0cab36cc2be65973ecc9caf8d383e1bab17047e7c618594917eb59958919d2b', 'producer': '367af022a3a4f9126b31528214067cb351d63c2e', 'binary': 'd6099d232904ceac1ff11638d7183d64ada94778e439aab6f1238483ce019f29'}, 'window-r0004.json': {'sha256': 'd005a3a1b322a1abb206de866f9b2f37bbb7b26202dc77474c79044408d0255e', 'producer': '40c48944f3a836a178afcd0eeb6c72f6801af6a4', 'binary': 'd6099d232904ceac1ff11638d7183d64ada94778e439aab6f1238483ce019f29'}}
FONT='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
sha=lambda b:hashlib.sha256(b).hexdigest()
def check(r,directory):
 assert r['schema']=='native-live-squad-window/1' and r['case'] in ('original','cancelled')
 assert r['status']=='passed' and not r['error'] and not r['x11_errors']
 assert r['owned_windows_closed'] and r['cancelled_and_closed_workers_reaped'] and r['fresh_process_count']==2
 assert r['font_sha256']==FONT and r['runtime']['window_backend']=='software/minifb'
 assert r['runtime']['cargo_lock_sha256']==sha((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
 assert r['names_sha256']==sha((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes())
 deps={'desktop/faithful/live_squad_window_check.py','desktop/faithful/src/application.rs','desktop/faithful/src/controller.rs','desktop/faithful/src/timeline.rs','desktop/faithful/src/render.rs','desktop/faithful/src/live_worker.rs','campaign/src/result.rs','engine/src/watched_compiled/prepared.rs'}
 assert set(r['producer_files'])==deps and all(re.fullmatch('[0-9a-f]{64}',v) for v in r['producer_files'].values())
 assert re.fullmatch('[0-9a-f]{40}',r['producer']) and re.fullmatch('[0-9a-f]{40}',r['source_research'])
 steps={s['name']:s['state'] for s in r['steps']};assert len(steps)==len(r['steps'])
 expected=['main','initial-save','preparation','preparing-worker','half-time','interval-bench','match-squad','squad-highlight-0','squad-picked-first','squad-put-11','squad-refusal','squad-pick-cancelled','squad-cancelled']
 expected+= (['resuming','second-half','second-half-history','full-time'] if r['case']=='cancelled'
             else ['match-squad-again','squad-picked','squad-put-witnessed','resuming-replace','second-half','second-half-history','full-time'])
 expected+=['recorded','saved-result','fresh-reload','duplicate-refused']
 assert list(steps)==expected
 initial=steps['initial-save']['campaign']
 assert initial==steps['main']['campaign'] and initial['club']['first_team'][1]==2582 and initial['club']['first_team'][11]==2592
 # The whole route keeps the campaign byte-identical until the exactly-once posting.
 pre=['initial-save','preparation','preparing-worker','half-time','interval-bench','match-squad','squad-highlight-0','squad-picked-first','squad-put-11','squad-refusal','squad-pick-cancelled']
 if r['case']=='original':pre+=['match-squad-again','squad-picked','squad-put-witnessed','resuming-replace']
 else:pre+=['resuming']
 assert all(steps[k]['campaign']==initial for k in pre)
 # Squad view reads the owned snapshot and refuses unwitnessed pairs readably.
 squad=steps['match-squad'];assert squad['squad_open'] and squad['squad_highlight']==1 and squad['squad_picked'] is None
 assert squad['remaining_subs']==7 and len(squad['interval_roster'])==20
 assert squad['interval_roster'][1]==2582 and squad['interval_roster'][11]==2592
 assert squad['interval_roster'][0]==2581
 assert 'witnessed' in steps['squad-refusal']['message'] and steps['squad-refusal']['squad_picked']==0
 assert steps['squad-refusal']['campaign']==initial
 if r['case']=='cancelled':
  assert steps['squad-cancelled']['screen']=='Bench' and not steps['resuming']['replace_sent'] if 'replace_sent' in steps['resuming'] else True
  recorded=steps['recorded']['campaign']
  assert recorded['club']['first_team'][1]==2582 and recorded['club']['first_team'][11]==2592
 else:
  assert steps['resuming-replace']['resuming'] and not steps['resuming-replace']['squad_open']
  recorded=steps['recorded']['campaign']
  assert recorded['club']['first_team'][1]==2592 and recorded['club']['first_team'][11]==2582
 assert recorded['played_fixture'] and recorded['revision']==initial['revision']+1
 assert recorded['played_fixture']['score']==steps['full-time']['score']
 # Save proofs: the pre-match save and the completed save, digests bound.
 tag='r0003' if r['case']=='original' else 'r0004'
 proofs={p['name']:p for p in r['save_proofs']}
 assert set(proofs)=={'initial','completed'}
 for key,p in proofs.items():
  path=directory/f'window-{tag}-{key}.json'
  assert p['path']==path.name and re.fullmatch('[0-9a-f]{64}',p['sha256']) and sha(path.read_bytes())==p['sha256']
 # Raster bindings for the squad view and the refusal are pinned by digest.
 names=[s['name'] for s in r['screenshots']]
 assert 'match-squad' in names and 'match-squad-resized' in names and 'squad-refusal' in names
 for s in r['screenshots']:
  raw=(directory/s['path']).read_bytes();assert sha(raw)==s['sha256']
  assert native_rgb(raw)
 assert not steps['duplicate-refused']['worker_present']
 assert 'already' in steps['duplicate-refused']['message']
 return dict(case=r['case'],recorded_slots=[recorded['club']['first_team'][1],recorded['club']['first_team'][11]])
def main():
 p=argparse.ArgumentParser();p.add_argument('records',type=Path,nargs='+');a=p.parse_args()
 out=[]
 for path in a.records:
  pin=PINS[path.name]
  raw=path.read_bytes();assert sha(raw)==pin['sha256']
  r=json.loads(raw)
  assert r['producer']==pin['producer'] and r['binary_sha256']==pin['binary']
  directory=path.parent
  out.append(check(r,directory))
 assert len({o['case'] for o in out})==2 and len(out)==2
 persisted=[o for o in out if o['case']=='original'][0]
 print('PASS live squad selection:',len(out),'records;',persisted)
if __name__=='__main__':main()
