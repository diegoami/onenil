#!/usr/bin/env python3
"""T76: run a real owned worker/child lifecycle proof and retain a compact receipt."""
import argparse,hashlib,json,subprocess
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--probe',type=Path,required=True);p.add_argument('--worker',type=Path,required=True);p.add_argument('--record',type=Path);a=p.parse_args()
 if a.record and (a.record.exists() or subprocess.check_output(['git','status','--porcelain'],cwd=ROOT).strip()):raise ValueError('clean producer and new immutable record required')
 packet=json.loads(subprocess.check_output([str(a.probe.resolve()),str(a.worker.resolve())],cwd=ROOT))
 if packet['status']!='passed' or [(c['profile'],c['decision']) for c in packet['cases']]!=[('original','continue'),('edited','continue'),('original','replace_verified')]:raise ValueError('real-worker case census differs')
 for c in packet['cases']:
  if c['interval_boundary']!='0AB9:7E4A' or c['actual_return']!='0AB9:EBE2' or not 0<c['interval_frames']<c['complete_frames'] or not all(c[k] for k in ('interval_delivered_alive','prefix_preserved','campaign_unchanged','completion_once','child_reaped')):raise ValueError('owned worker invariant differs')
 if [c['drop'] for c in packet['cancellations']]!=[False,True] or not all(c['child_reaped'] for c in packet['cancellations']):raise ValueError('cancel/drop proof differs')
 record=dict(schema='live-halftime-worker-proof/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),probe_sha256=sha(a.probe),worker_sha256=sha(a.worker),engine_lock_sha256=sha(ROOT/'engine/Cargo.lock'),desktop_lock_sha256=sha(ROOT/'desktop/faithful/Cargo.lock'),result=packet,scope='Backend transport only; no interactive desktop controls or saved-substitution persistence')
 if a.record:
  with a.record.open('x') as out:json.dump(record,out,indent=2);out.write('\n')
 print('PASS real live workers:',[(c['profile'],c['decision'],c['interval_frames'],c['complete_frames']) for c in packet['cases']])
if __name__=='__main__':main()
