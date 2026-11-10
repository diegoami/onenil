#!/usr/bin/env python3
"""T80: authenticate the actual-window friendly-week route and persistence."""
import argparse,hashlib,json,re
from pathlib import Path
from check_numbered_pitch_records import native_rgb
ROOT=Path(__file__).resolve().parents[2]
PINS={'window-r0001.json': {'sha256': '3ca5b2f7e3313350c67fb721b6bc2e0168c26698c22c6c37f54b0c41e3c6d06d', 'producer': '9498322455ccef6ef6ce023e498ec0dc3b47343d', 'binary': '93829e2be5bbe206ab6adadabdd8d51b8ee14a2f333caf8963cc9a4dcaf00f4f'}, 'window-r0002.json': {'sha256': '0df1cac17c7b194bbf5309fac7431614309e5765ce87f54e13581d734b291b6a', 'producer': '9498322455ccef6ef6ce023e498ec0dc3b47343d', 'binary': '93829e2be5bbe206ab6adadabdd8d51b8ee14a2f333caf8963cc9a4dcaf00f4f'}, 'window-r0003.json': {'sha256': 'a693908e7ba614171cc3d82d2aceb0ee77943dc9734745eed03d59354c6530e4', 'producer': 'aa9df9b684ab3030f15d532f5f251b94f6bb6295', 'binary': 'b88b18a4a7da782ac37ba604e04be375d6556dd0ccaac6f69c73e1bb6deb7e5c'}, 'window-r0004.json': {'sha256': '5197ade4be9dc051d9071ecd1abc19326503d084c5d992c09cd30083e666dcb1', 'producer': 'aa9df9b684ab3030f15d532f5f251b94f6bb6295', 'binary': 'b88b18a4a7da782ac37ba604e04be375d6556dd0ccaac6f69c73e1bb6deb7e5c'}}
FONT='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
sha=lambda b:hashlib.sha256(b).hexdigest()
def check(r,directory):
 assert r['schema']=='native-week-route-window/1' and r['case']=='week'
 assert r['status']=='passed' and not r['error'] and not r['x11_errors']
 assert r['owned_windows_closed'] and r['cancelled_and_closed_workers_reaped'] and r['fresh_process_count']>=2
 assert r['font_sha256']==FONT and r['runtime']['window_backend']=='software/minifb'
 assert r['runtime']['cargo_lock_sha256']==sha((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
 assert r['names_sha256']==sha((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes())
 deps={'desktop/faithful/live_week_window_check.py','desktop/faithful/src/application.rs','desktop/faithful/src/controller.rs','desktop/faithful/src/timeline.rs','desktop/faithful/src/render.rs','desktop/faithful/src/live_worker.rs','campaign/src/result.rs','engine/src/watched_compiled/prepared.rs'}
 assert set(r['producer_files'])==deps and all(re.fullmatch('[0-9a-f]{64}',v) for v in r['producer_files'].values())
 names=[s['name'] for s in r['steps']]
 # The replay may be refused more than once; every other step appears exactly once.
 assert len(names)==len(set(names))+names.count('duplicate-refused')-1
 expected=['main','initial-save','preparation','preparing-worker','half-time','interval-bench','resuming','second-half','second-half-history','full-time','recorded','transfer-news','menu-advanced','saved-week','fresh-reload','duplicate-refused']
 kept=[n for n in names if n!='duplicate-refused']+['duplicate-refused']
 assert kept==expected
 steps={s['name']:s['state'] for s in r['steps']}
 initial=steps['initial-save']['campaign']
 assert initial==steps['main']['campaign'] and initial['club']['first_team'][1]==2582 and initial['club']['first_team'][11]==2592
 pre=['initial-save','preparation','preparing-worker','half-time','interval-bench','resuming','second-half','second-half-history','full-time']
 assert all(steps[k]['campaign']==initial for k in pre), 'the campaign changes before the week transaction'
 assert steps['recorded']['campaign']['played_fixture'] and steps['recorded']['campaign']['revision']==initial['revision']+1
 assert steps['recorded']['campaign']['cash']==initial['cash']
 # The W030 route and the week transaction: cash, week index, following fixture.
 advanced=steps['menu-advanced']
 assert advanced['screen']=='Main' and advanced['campaign']['week_index']==1
 assert advanced['campaign']['cash']==421491
 assert advanced['campaign']['next_fixture']['competition_code']==1
 assert advanced['campaign']['next_fixture']['opponent']==176
 assert advanced['campaign']['next_fixture']['opponent_name']=='Sevilla'
 assert advanced['campaign']['revision']==steps['recorded']['campaign']['revision']+1
 assert steps['saved-week']['campaign']==advanced['campaign']
 # Reload restores everything; replay refuses.
 assert steps['fresh-reload']['campaign']==advanced['campaign']
 assert steps['fresh-reload']['screen']=='Main' and not steps['fresh-reload'].get('worker_pid')
 assert 'already' in steps['duplicate-refused']['message']
 # Raster bindings for the new screens and the route.
 names=[s['name'] for s in r['screenshots']]
 assert 'second-half' in names and 'full-time' in names
 for s in r['screenshots']:
  raw=(directory/s['path']).read_bytes();assert sha(raw)==s['sha256']
  assert native_rgb(raw)
 return dict(cash_after=advanced['campaign']['cash'],week=1)
def main():
 p=argparse.ArgumentParser();p.add_argument('records',type=Path,nargs='+');a=p.parse_args()
 for path in a.records:
  pin=PINS[path.name]
  raw=path.read_bytes();assert sha(raw)==pin['sha256']
  r=json.loads(raw)
  assert r['producer']==pin['producer'] and r['binary_sha256']==pin['binary']
  check(r,path.parent)
 print('PASS week route:',len(a.records),'records validated')
if __name__=='__main__':main()
