#!/usr/bin/env python3
"""[designed] Original passing/contact capture,decoded ENTRY export,and native replay."""
import argparse,hashlib,json,time,platform,subprocess
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import watched_oracle as oracle
import watched_action_export as export
ROOT=oracle.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/watched_action'

def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def generate(directory=FIXTURES):
    import onenil_engine as native
    cases,manifest=oracle.validate(ROOT/'records/T37/oracle-r0001')
    directory.mkdir(parents=True,exist_ok=True);items=[];started=time.monotonic()
    for case in cases:
        path=directory/f'case-{case["case"]:05d}.json'
        if path.exists():value=json.loads(path.read_text())
        else:
            state,actual,selected,restarts=export.export_one(case,native)
            export.validate_ownership(case,state);expected=export.expected(case,state)
            if actual!=expected:raise ValueError(f'case{case["case"]} native gameplay/draw/commentary differs')
            value={'case':case['case'],'kind':case['kind'],'tick':case['tick'],'oracle_sha256':manifest['cases'][case['case']]['sha256'],'entry_sha256':case['entry']['sha256'],'input':state,'expected':expected}
            oracle.write_json(path,value)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':digest(path)})
        if case['case']%64==0:print(f'export{case["case"]+1}/{len(cases)} elapsed{time.monotonic()-started:.1f}s',flush=True)
    result={'schema':'decoded-watched-actions/1','producer':oracle.common.git('rev-parse','HEAD'),'oracle_manifest_sha256':oracle.MANIFEST_SHA256,'cases':items,'scope':'Named original ENTRY facts only;all expected outputs are independently decoded from original EXIT observations'}
    path=directory/'manifest.json'
    if path.exists():
        if json.loads(path.read_text())!=result:raise ValueError('existing export manifest differs')
    else:oracle.write_json(path,result)
    return check(directory)

def check(directory=FIXTURES):
    cases,original=oracle.validate(ROOT/'records/T37/oracle-r0001')
    manifest=json.loads((directory/'manifest.json').read_text())
    if manifest['schema']!='decoded-watched-actions/1'or manifest['oracle_manifest_sha256']!=oracle.MANIFEST_SHA256 or len(manifest['cases'])!=len(cases):raise ValueError('decoded fixture provenance/completeness differs')
    values=[]
    for source,item in zip(cases,manifest['cases'],strict=True):
        path=ROOT/item['path']
        if digest(path)!=item['sha256']:raise ValueError('decoded action fixture digest differs')
        c=json.loads(path.read_text());s=c['input'];out=c['expected']
        if c['case']!=source['case']or c['kind']!=source['kind']or c['tick']!=source['tick']or c['oracle_sha256']!=original['cases'][source['case']]['sha256']or c['entry_sha256']!=source['entry']['sha256']:raise ValueError('decoded root provenance differs')
        if s['kind']!=source['kind']or s['scene']!=source['input_scene']or s['seed']!=source['seed_before']or s['regs']!=[source['registers'][r]for r in oracle.REGS]or s['flags']!=source['flags']or s['stack_words']!=source['stack_words']:raise ValueError('decoded action admission differs')
        if out['scene']!=source['output_scene']or out['seed']!=source['seed_after']or out['draws']!=source['draws']:raise ValueError('decoded expected original state/draws differ')
        events=[]
        for event in source['events']:
            e={k:v for k,v in event.items()if k not in('tick','event')}
            for key in('draw_start','draw_end'):e[key]-=source['draw_start']
            events.append(e)
        if out['events']!=events:raise ValueError('decoded expected commentary differs')
        values.append(c)
    return values

def replay(values):
    import onenil_engine as native
    for case in values:
        result=json.loads(native.watched_action(json.dumps(case['input'])))
        if result.get('ok')!=case['expected']:raise ValueError(f'native original action parity differs at case{case["case"]}: '+str(result.get('error')))
    print('native watched actions:',len(values),'original roots matched',flush=True)

def benchmark(values,repetitions):
    from onenil_engine import onenil_engine as native
    source_paths=['engine','tools/replay_watched_action.py','tools/watched_action_export.py','tools/watched_oracle.py','tools/compile_watched.py','pyproject.toml']
    if oracle.common.git('status','--porcelain','--',*source_paths):raise ValueError('commit source and decoded fixtures before measurement')
    if native.__debug_assertions__:raise ValueError('release extension required')
    inputs=[json.dumps(c['input'],sort_keys=True,separators=(',',':'))for c in values];size=0
    started=datetime.now(timezone.utc).isoformat();begin=time.perf_counter_ns()
    for _ in range(repetitions):
        for value in inputs:size+=len(native.watched_action(value))
    elapsed=time.perf_counter_ns()-begin;out=ROOT/'records/T37'
    number=max((int(p.stem.split('-r')[-1])for p in out.glob('T37-r*.json')),default=0)+1;path=out/f'T37-r{number:04d}.json'
    result={'schema':'routine-benchmark/1','run_id':path.stem,'code_commit':oracle.common.git('rev-parse','HEAD'),'started':started,'ended':datetime.now(timezone.utc).isoformat(),'platform':platform.platform(),'python':platform.python_version(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'cargo_lock_sha256':digest(ROOT/'engine/Cargo.lock'),
      'build':{'crate_version':native.__version__,'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'extension_sha256':digest(Path(native.__file__))},
      'source_pins':{'oracle_manifest_sha256':oracle.MANIFEST_SHA256,'decoded_manifest_sha256':digest(FIXTURES/'manifest.json')},'routine':'native448C/4DFF source-addressed compiled action graphs','repetitions':repetitions,'cases':len(values),'calls':len(values)*repetitions,'elapsed_ns':elapsed,'serialized_output_bytes':size,
      'method':'Pre-serialized inputs;release parsing,one native data image,compiled arithmetic/control flow/native helpers/RNG/commentary and serialization included;fixture preparation/output decoding excluded',
      'limits':'Action/contact root kernels only;not a continuous watched tick,clock,full native match or emulator speed comparison'}
    oracle.write_json(path,result);print(path.relative_to(ROOT))

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);p.add_argument('--export',action='store_true');p.add_argument('--check',action='store_true');p.add_argument('--replay',action='store_true');p.add_argument('--benchmark',action='store_true');p.add_argument('--repetitions',type=int,default=3);a=p.parse_args()
    if a.repetitions<1:p.error('positive repetitions required')
    if a.capture:
        if a.research_root is None:p.error('--capture requires --research-root')
        oracle.capture(a.research_root)
    if a.export:generate()
    if a.check or a.replay or a.benchmark:
        values=check();print('decoded watched-action corpus:',len(values),'roots verified')
        if a.replay or a.benchmark:replay(values)
        if a.benchmark:benchmark(values,a.repetitions)
    if not any((a.capture,a.export,a.check,a.replay,a.benchmark)):p.error('choose --capture/--export/--check/--replay')
if __name__=='__main__':main()
