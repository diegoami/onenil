#!/usr/bin/env python3
"""Release fresh/retained completion and immutable timing receipt;no original backend."""
import argparse
import hashlib
import json
import platform
import subprocess
import sys
import time
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tools'))
import replay_prepared_match as replay
import onenil_engine as api
from onenil_engine import onenil_engine as native
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--development-reference-only',action='store_true')
args=parser.parse_args()
if subprocess.check_output(['git','status','--porcelain','--','engine','tools'],cwd=ROOT,text=True):raise ValueError('commit engine/tool sources before measuring')
if native.__debug_assertions__:raise ValueError('release extension required')
if not args.development_reference_only:replay.field_preservation()
value=replay.admission();request=json.dumps(value,sort_keys=True,separators=(',',':'))
started=datetime.now(timezone.utc).isoformat();durations=[];sizes=[];clock_digests=[];output_digests=[]
for prefix_steps in (0,171):
    began=time.perf_counter_ns()
    session=api.NativePreparedWatchedSession(request)
    for _ in range(prefix_steps):session.step()
    serialized=session.finish()
    del session
    durations.append(time.perf_counter_ns()-began);sizes.append(len(serialized))
    packet=json.loads(serialized)
    if set(packet)!={'ok'}:raise ValueError('release full completion failed:'+str(packet.get('error')))
    replay.compare(packet['ok'],not args.development_reference_only)
    output_digests.append(hashlib.sha256(replay.canonical(packet)).hexdigest())
    clock_digests.append(hashlib.sha256(replay.canonical(packet['ok']['caller_clocks'])).hexdigest())
    del packet,serialized
if len(set(output_digests))!=1:raise ValueError('fresh/retained release outputs differ')
record=dict(schema='prepared-match-release/1',code_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
    engine_tree=subprocess.check_output(['git','rev-parse','HEAD:engine'],cwd=ROOT,text=True).strip(),started=started,repetitions=2,
    prefix_steps=[0,171],elapsed_ns=sum(durations),durations_ns=durations,ticks=4199*2,computed_draws=182811*2,watched_actions=831*2,
    actual_return='0AB9:EBE2',serialized_output_bytes=sizes,debug_assertions=native.__debug_assertions__,extension_sha256=replay.sha(Path(native.__file__)),
    cargo_lock_sha256=replay.sha(ROOT/'engine/Cargo.lock'),admission_sha256=replay.sha(replay.FIXTURE),reference_index_sha256=replay.reference.INDEX_SHA,
    outputs_sha256=output_digests,caller_clocks_sha256=clock_digests,full_field_and_clock_proof_checked=not args.development_reference_only,
    platform=platform.platform(),python=sys.version,rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),maturin=version('maturin'),
    method='Pre-serialized initial input;release parse,tracked selected setup,optional171 caller steps,all retained native frames through actual outer return,full output serialization and session drop. Reference comparisons and output decoding outside timing.',
    scope='Two complete native reference sessions,fresh and retained. Additional lineup/seed/viewer acceptance remains required;no emulator speed comparison. Full original field/clock proof pending when its checked flag is false.')
directory=ROOT/'records/T45'
number=max((int(p.stem.split('-r')[1])for p in directory.glob('release-r*.json')),default=0)+1
path=directory/f'release-r{number:04d}.json'
with path.open('x')as output:json.dump(record,output,indent=2);output.write('\n')
print(path.relative_to(ROOT),record['elapsed_ns'],'ns for two complete native matches;full reference parity passed;full field/clock proof checked:',record['full_field_and_clock_proof_checked'])
