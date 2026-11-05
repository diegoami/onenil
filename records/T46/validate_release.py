#!/usr/bin/env python3
"""[designed] Committed-source release replay of every supported prepared input."""
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
from play_watched import compute_prepared
from replay_prepared_choices import CHOICES,check_inputs,validate_output
from onenil_engine import onenil_engine as native
if subprocess.check_output(['git','status','--porcelain','--','engine','tools','web/watched',str(Path(__file__).relative_to(ROOT))],cwd=ROOT,text=True):
    raise ValueError('commit release/product sources before measuring')
if native.__debug_assertions__:raise ValueError('release extension required')
started=datetime.now(timezone.utc).isoformat()
check_inputs();samples=[]
for choices in CHOICES:
    began=time.perf_counter_ns()
    output,initial,draws=compute_prepared(choices)
    elapsed=time.perf_counter_ns()-began
    summary=validate_output(choices,output,initial,draws)
    summary.update(elapsed_ns=elapsed,terminal_clock=output['caller_clocks'][-1]['clock'])
    samples.append(summary)
    print('Release complete:',choices,elapsed,'ns',flush=True)
    del output,initial,draws
record=dict(schema='supported-prepared-release/1',started=started,completed=datetime.now(timezone.utc).isoformat(),
    source_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
    engine_tree=subprocess.check_output(['git','rev-parse','HEAD:engine'],cwd=ROOT,text=True).strip(),
    extension_sha256=hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),debug_assertions=native.__debug_assertions__,
    cargo_lock_sha256=hashlib.sha256((ROOT/'engine/Cargo.lock').read_bytes()).hexdigest(),
    python=sys.version,platform=platform.platform(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),
    maturin=version('maturin'),sessions=samples,reference_full_original_proof_checked=True,
    method='One fresh NativePreparedWatchedSession perinitialchoice:initialdata validation/serialization,nativeparse+selectedsetup+allretainedframes throughactualEBE2,outputserialization/Pythondecodeandnativedrop. Assertions,referencecomparisonanddigests outside timing. GUIframeconversion is not timed.',
    scope='Four supported native completions;onlyreference64 claims originalparity. Single host samples,not emulator comparison or guaranteed UI latency.')
directory=ROOT/'records/T46';numbers=[int(p.stem.rsplit('-r',1)[1])for p in directory.glob('release-r*.json')]
path=directory/f'release-r{max(numbers,default=0)+1:04d}.json'
with path.open('x')as out:json.dump(record,out,indent=2);out.write('\n')
print(path.relative_to(ROOT),flush=True)
