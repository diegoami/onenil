#!/usr/bin/env python3
"""[designed:T74] Direct paused native parity with both accepted original streams."""
import argparse,hashlib,json,subprocess
from pathlib import Path
import replay_fresh_watched as fresh
import replay_edited_watched as edited
ROOT=Path(__file__).resolve().parents[1]
sha=lambda data:hashlib.sha256(data).hexdigest()
def main():
 p=argparse.ArgumentParser();p.add_argument('--probe',type=Path,required=True);p.add_argument('--record',type=Path);a=p.parse_args()
 if a.record and (a.record.exists() or subprocess.check_output(['git','status','--porcelain'],cwd=ROOT).strip()):raise ValueError('fresh record and committed producer required')
 fresh.reference.sources();reference,_=edited.require_acceptance();reference.sources()
 results=[]
 for profile,request,directory in [('fresh',fresh.request(),fresh.reference.DIRECTORY),('edited',edited.incoming.request(),reference.DIRECTORY)]:
  raw=subprocess.check_output([str(a.probe.resolve())],input=json.dumps(dict(input=request,profile=profile)).encode(),cwd=ROOT)
  packet=json.loads(raw)
  if 'error' in packet:raise ValueError(packet['error'])
  got=packet['ok'];interval=packet['interval']
  if interval['site']!='0AB9:7E4A' or interval['control']!=1 or interval['state']['remaining']<=0 or interval['state']['elapsed']<45:raise ValueError('native interval boundary differs')
  (fresh.compare if profile=='fresh' else edited.compare)(got,directory)
  results.append(dict(profile=profile,ticks=len(got['ticks']),draws=len(got['draws']),actions=len(got['events']),actual_return=got['return'],interval_ticks=interval['ticks'],interval_draws=interval['draws'],interval_sha256=sha(fresh.reference.encoded(interval)),observation_sha256={key:sha(fresh.reference.encoded(got[key])) for key in ('ticks','draws','events','final')},original_index_sha256=sha((directory/'index.json').read_bytes())))
 record=dict(schema='native-owned-interval-parity/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),probe_sha256=sha(a.probe.read_bytes()),cargo_lock_sha256=sha((ROOT/'engine/Cargo.lock').read_bytes()),results=results,status='passed')
 if a.record:
  with a.record.open('x') as out:json.dump(record,out,indent=2);out.write('\n')
 print('PASS direct original interval parity',[(r['profile'],r['ticks'],r['draws'],r['actions']) for r in results])
if __name__=='__main__':main()
