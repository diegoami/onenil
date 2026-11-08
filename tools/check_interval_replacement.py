#!/usr/bin/env python3
"""T75: authenticate original transaction inputs and check the native decision path."""
import argparse
import hashlib
import json
import subprocess
import time
from pathlib import Path
import replay_fresh_watched as fresh
import watched_oracle
ROOT=Path(__file__).resolve().parents[1]
SOURCE_COMMIT='1187bce05efbfca38bdbe7c2d697ddabcc224371'
SOURCE_TREE='b32570e2855277e5646f03ebf20f1a06cc9ccbaa'
FIXTURES=ROOT/'engine/tests/fixtures/halftime_replacement'
encoded=fresh.reference.encoded
sha=lambda b:hashlib.sha256(b).hexdigest()
def load(path):return json.loads(path.read_text())
def pinned(root,item):
 path=root/item['path']
 if sha(path.read_bytes())!=item['sha256']:raise ValueError('source digest differs: '+item['path'])
 return load(path)
def capture(record):
 if record['status']!='completed' or not record['processes_exited'] or sha(encoded(record['payload']))!=record['payload_sha256']:raise ValueError('incomplete/unbound original capture')
 rows={s['name']:s for s in record['payload']['stages']}
 for row in rows.values():
  if sha(encoded(row['state']))!=row['state_sha256']:raise ValueError('original state digest differs')
  if 'support' in row and sha(encoded(row['support']))!=row['support_sha256']:raise ValueError('original support digest differs')
 return rows
def sources(root):
 if subprocess.check_output(['git','rev-parse','HEAD^{tree}'],cwd=root,text=True).strip()!=SOURCE_TREE:raise ValueError('use the pinned accepted source tree '+SOURCE_COMMIT)
 witness=load(FIXTURES/'witness.json');incoming=load(FIXTURES/'incoming.json')
 originals=[pinned(root,item) for item in witness['sources']]
 if len({r['payload']['start']['savestate_sha256'] for r in originals})!=2:raise ValueError('original transaction repeat is not independent')
 for item,r in zip(witness['sources'],originals):
  if item['producer']!=r['producer']:raise ValueError('original transaction producer differs')
  rows=capture(r)
  if witness['before']!=rows['gale']['state'] or witness['after']!=rows['confirmed']['state']:raise ValueError('native witness differs from original transaction')
 inputs=[];starts=[]
 for item in incoming['sources']:
  query=pinned(root,item);record=pinned(root,query['source']);rows=capture(record);starts.append(record['payload']['start']['savestate_sha256'])
  if query['source']['producer']!=record['producer']:raise ValueError('input capture producer differs')
  facts=[]
  for f in query['facts']:
   row=rows[f['stage']]
   if f['savestate_sha256']!=row['savestate_sha256'] or f['screen_sha256']!=row['screen_sha256']:raise ValueError('input stage identity differs')
   if f['stage']=='halftime':
    if f['width'] not in (8,16) or not 0<=f['value']<1<<f['width']:raise ValueError('invalid decoded input width/value')
    facts.extend(dict(area=f['area'],offset=f['offset']+i,value=(f['value']>>(8*i))&255) for i in range(f['width']//8))
   elif (f['area'],f['offset'],f['value'])!=('club',8048,18):raise ValueError('post-editor observation differs')
  if facts!=incoming['support'] or len({(f['area'],f['offset']) for f in facts})!=len(facts):raise ValueError('runtime input bytes differ from original query')
  inputs.append(facts)
 if len(inputs)!=2 or len(set(starts))!=2 or inputs[0]!=inputs[1]:raise ValueError('incoming original repeat differs')
 return witness,incoming

def original_prefix(got,interval):
 index,_=fresh.reference.sources();expected={k:[] for k in ('ticks','draws','events')}
 n=interval['ticks'];d=interval['draws']
 for item in index['chunks']:
  p=load(fresh.reference.DIRECTORY/item['path'])['payload']
  expected['ticks'].extend(p['ticks'])
  expected['draws'].extend(watched_oracle.normalized(v) for v in p['draws'])
  expected['events'].extend({k:v for k,v in a.items() if k!='event'} for a in p['actions'] if a['tick']<=n)
 for key,limit in [('ticks',n),('draws',d),('events',len(expected['events']))]:
  if got[key][:limit]!=expected[key][:limit]:raise ValueError('computed original first-half prefix differs: '+key)
 return dict(ticks=n,draws=d,actions=len(expected['events']))

def verify(packet,witness,cancel):
 if 'error' in packet:raise ValueError(packet['error'])
 interval=packet['interval'];result=packet['ok'];got=result['match'];rows=result['transaction']
 if interval['site']!='0AB9:7E4A' or interval['control']!=1 or interval['state']['elapsed']!=witness['before']['elapsed']:raise ValueError('owned native interval differs')
 keys=[32,6,2]+([1,2] if cancel else [])+[6]*10+[2,1,None]
 if [r['key'] for r in rows]!=keys:raise ValueError('original editor key sequence differs')
 confirm=15 if cancel else 13
 for i,row in enumerate(rows):
  if row['state']!=witness['before' if i<=confirm else 'after']:raise ValueError('bounded original transaction state differs at input '+str(i))
  if row['club_roster_count']!=18:raise ValueError('original roster count differs')
 if [rows[i]['selected_slot'] for i in (2,confirm,confirm+1)]!=[2,12,12] or rows[confirm]['picked']!=1 or rows[confirm+1]['picked']!=0:raise ValueError('original selection/confirmation differs')
 if cancel and [rows[i]['picked'] for i in (3,4)]!=[1,0]:raise ValueError('original cancellation differs')
 if got['return']!='0AB9:EBE2':raise ValueError('native match did not reach actual outer return')
 prefix=original_prefix(got,interval)
 scenes=got['ticks'][interval['ticks']]['before']['scenes']
 if not any(s['word14']==2592 for s in scenes) or any(s['word14']==2582 for s in scenes):raise ValueError('second half did not use the replaced live player')
 return got,dict(cancel_first=cancel,original_first_half=prefix,ticks=len(got['ticks']),draws=len(got['draws']),actions=len(got['events']),actual_return=got['return'],match_sha256=sha(encoded(got)),transaction_sha256=sha(encoded(rows)),final_sha256=sha(encoded(got['final'])))

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);p.add_argument('--probe',type=Path,required=True);p.add_argument('--record',type=Path);a=p.parse_args()
 if a.record and (a.record.exists() or subprocess.check_output(['git','status','--porcelain'],cwd=ROOT).strip()):raise ValueError('committed producer and fresh output required')
 witness,incoming=sources(a.research_root.resolve());request=fresh.request();request['support']+=incoming['support']
 results=[];previous=None
 for cancel in (False,True):
  start=time.perf_counter()
  raw=subprocess.check_output([str(a.probe.resolve())],input=encoded(dict(profile='fresh',input=request,cancel_first=cancel)),cwd=ROOT)
  got,receipt=verify(json.loads(raw),witness,cancel);receipt['elapsed_seconds']=time.perf_counter()-start
  if previous is not None and previous!=got:raise ValueError('cancel-first changed the computed substituted match')
  previous=got;results.append(receipt)
 record=dict(schema='native-original-halftime-replacement/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),research_checked_commit=SOURCE_COMMIT,research_tree=SOURCE_TREE,probe_sha256=sha(a.probe.read_bytes()),incoming_sha256=sha((FIXTURES/'incoming.json').read_bytes()),witness_sha256=sha((FIXTURES/'witness.json').read_bytes()),results=results,status='passed',scope='Original transaction and first-half stream parity; substituted second-half output is native-only, not original full-match parity')
 if a.record:
  with a.record.open('x') as out:json.dump(record,out,indent=2);out.write('\n')
 print('PASS original halftime replacement and first-half parity',[(r['cancel_first'],r['ticks'],r['draws'],r['actions']) for r in results])
if __name__=='__main__':main()
