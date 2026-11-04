#!/usr/bin/env python3
"""Release API parity and receipt for native setup plus171 retained caller iterations."""
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
import replay_prepared_watched as replay
import onenil_engine as api
from onenil_engine import onenil_engine as native
if subprocess.check_output(['git','status','--porcelain','--','engine','tools'],cwd=ROOT,text=True):raise ValueError('commit engine/tool sources before release measurement')
if native.__debug_assertions__:raise ValueError('release extension required')
value=replay.verify(replay.read(replay.FIXTURE));request=json.dumps(value,sort_keys=True,separators=(',',':'))
started=datetime.now(timezone.utc).isoformat();durations=[];size=0
for _ in range(3):
    began=time.perf_counter_ns()
    session=api.NativePreparedWatchedSession(request)
    first=session.snapshot();clocks=[session.step()for _ in range(171)];observation=session.observation()
    del session
    durations.append(time.perf_counter_ns()-began)
    size+=len(first)+sum(map(len,clocks))+len(observation)
    replay.compare(dict(first_entry=json.loads(first),clocks=list(map(json.loads,clocks)),observation=json.loads(observation)))
record=dict(schema='prepared-watched-release/1',code_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
    engine_tree=subprocess.check_output(['git','rev-parse','HEAD:engine'],cwd=ROOT,text=True).strip(),started=started,repetitions=3,
    elapsed_ns=sum(durations),durations_ns=durations,caller_iterations=171*3,computed_draws=7280*3,watched_actions=33*3,
    serialized_output_bytes=size,debug_assertions=native.__debug_assertions__,extension_sha256=replay.sha(Path(native.__file__)),
    cargo_lock_sha256=replay.sha(ROOT/'engine/Cargo.lock'),admission_sha256=replay.sha(replay.FIXTURE),original_manifests=replay.MANIFESTS,
    platform=platform.platform(),python=sys.version,rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),maturin=version('maturin'),
    method='Pre-serialized initial selected data;release parse,tracked setup,compiled E008 entry,171 input-free caller advances,first-entry/per-caller/full-trace JSON serialization and drop. All original parity assertions run outside each timing interval.',
    scope='Four-minute prepared native passage only;no fulltime/other-session or emulator speed claim.')
directory=ROOT/'records/T44'
number=max((int(p.stem.split('-r')[1])for p in directory.glob('release-r*.json')),default=0)+1
path=directory/f'release-r{number:04d}.json'
with path.open('x')as output:json.dump(record,output,indent=2);output.write('\n')
print(path.relative_to(ROOT),record['elapsed_ns'],'ns for3 prepared passages;all parity assertions passed')
