#!/usr/bin/env python3
"""[designed] Original passing/contact capture,decoded ENTRY export,and native replay."""
import argparse,hashlib,json,time
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

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--capture',action='store_true');p.add_argument('--research-root',type=Path);p.add_argument('--export',action='store_true');p.add_argument('--check',action='store_true');p.add_argument('--replay',action='store_true');a=p.parse_args()
    if a.capture:
        if a.research_root is None:p.error('--capture requires --research-root')
        oracle.capture(a.research_root)
    if a.export:generate()
    if a.check or a.replay:
        values=check();print('decoded watched-action corpus:',len(values),'roots verified')
        if a.replay:replay(values)
    if not any((a.capture,a.export,a.check,a.replay)):p.error('choose --capture/--export/--check/--replay')
if __name__=='__main__':main()
