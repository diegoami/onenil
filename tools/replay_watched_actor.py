#!/usr/bin/env python3
"""[designed] Native72BB replay against independently computed original microkernels."""
import argparse,hashlib,json,time,subprocess
from pathlib import Path
import watched_oracle as oracle
import watched_action_export as facts
import watched_parent as parent
ROOT=oracle.ROOT
DIRECTORY=ROOT/'records/T38/oracle-r0002'
FIXTURES=ROOT/'engine/tests/fixtures/watched_actor'
MARGIN_SHA256='590b9ec023f70ff22fad6fef444a225aa221516c0e329ba829f0bd22679d65ef'
CORPUS_SHA256='f0dc8f33fce736501835c4b19e50a3ca98ee0a20b9ec46980b7580811f0945c8'

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):return json.loads(path.read_text())

def originals():
    if sha(DIRECTORY/'manifest.json')!=CORPUS_SHA256:raise ValueError('original parent corpus pin differs')
    manifest=read(DIRECTORY/'manifest.json');proof=read(ROOT/'records/T38/admission-proof-r0001.json')
    if manifest['status']!='completed'or manifest['modes']!=list(parent.MODES)or len(manifest['cases'])!=50:raise ValueError('parent corpus coverage differs')
    if sha(ROOT/'records/T38/admission-proof-r0001.json')!=manifest['proof_sha256']:raise ValueError('parent proof digest differs')
    anchors,source_manifest=oracle.validate(ROOT/'records/T37/oracle-r0001');by_id={c['case']:c for c in anchors};proofs={e['source_case']:e for e in proof['entries']}
    if len(proofs)!=32:raise ValueError('parent proof cardinality differs')
    for e in proofs.values():
        c=by_id[e['source_case']];j=e['joined_nested_entry']
        if e['source_sha256']!=source_manifest['cases'][c['case']]['sha256']or j['registers']!=c['registers']or j['flags']!=c['flags']or j['stack_words']!=c['stack_words']or j['seed']!=c['seed_before']or j['ram_sha256']!=c['entry']['ram_sha256']or j['game_timer_ticks']!=0:raise ValueError('derived parent prefix join differs')
    result=[]
    for number,item in enumerate(manifest['cases']):
        path=ROOT/item['path']
        if sha(path)!=item['sha256']:raise ValueError('original parent case digest differs')
        c=read(path);anchor=by_id[c['source_case']];p=proofs[c['source_case']]
        if c['case']!=number or c['source_sha256']!=p['source_sha256']or c['registers']!=p['parent_registers']or c['stack_words']!=p['parent_stack_words']or c['flags']!=anchor['flags']or c['exit_sp']!=c['registers']['sp']or c['exit_bp']!=c['registers']['bp']:raise ValueError('derived parent source/ABI differs')
        if c['input_scene']['index']!=anchor['scene']or c['mode']not in parent.MODES:raise ValueError('parent actor/mode differs')
        for change in c['changes']:
            if set(change)!={'area','offset','before','after'}or not all(type(change[k])is int and 0<=change[k]<=255 for k in('before','after')):raise ValueError('undecoded parent change')
        result.append((c,anchor))
    margin_dir=ROOT/'records/T38/margin-r0001'
    if sha(margin_dir/'manifest.json')!=MARGIN_SHA256:raise ValueError('original logical margin pin differs')
    margin=read(margin_dir/'manifest.json')
    if len(margin['cases'])!=50:raise ValueError('original logical margin coverage differs')
    for (c,anchor),item in zip(result,margin['cases'],strict=True):
        path=ROOT/item['path']
        if sha(path)!=item['sha256']:raise ValueError('original logical margin case digest differs')
        value=read(path)
        if value['case']!=c['case']or value['original_sha256']!=manifest['cases'][c['case']]['sha256']or value['prior_outcomes_equal']is not True:raise ValueError('original margin provenance differs')
        for d in value['changes']:
            if set(d)!={'area','offset','before','after'}or d['area']not in('main','main_shown','background','background_shown')or facts.pitch(d['offset'])or not pitch(d['offset']):raise ValueError('unexpected original logical margin fact')
        c['changes']+=value['changes']
    return result,manifest

def admitted(c,anchor):
    raw,state=parent.parent_state(anchor,c['mode'],c.get('input_overlay'));ds=16*state['regs'][11]
    state['pointers'].append({'kind':'style','offset':facts.word(raw,ds+0xD0C0),'segment':facts.word(raw,ds+0xD0C2)})
    return raw,state

def address(state,area,offset):
    if area=='style':
        p=state['pointers'][-1]
        if p['kind']!='style'or offset!=0:raise ValueError('invalid logical style fact')
        return p['offset']+16*p['segment']
    return facts.address(state,area,offset)

def pitch(offset):return any(0x90E+160*row-74<=offset<0x90E+160*row+76 for row in range(-9,10))

def named(state,at):
    if at==address(state,'style',0):return 'style',0
    for area in ('main','main_shown','background','background_shown'):
        offset=at-address(state,area,0)
        if 0<=offset<4000 and pitch(offset):return area,offset
    return facts.named(state,at)

def select_entry(c,anchor,native):
    raw,state=admitted(c,anchor);seen=set()
    for _ in range(20000):
        result=json.loads(native.watched_action(json.dumps(state)))
        if 'ok'in result:return raw,state,result['ok']
        fault=result['error'];missing=fault['missing']
        if missing is None:raise ValueError(f'original parent{c["case"]}: '+fault['message'])
        try:area,offset=named(state,missing['address'])
        except ValueError as error:raise ValueError(f'original parent{c["case"]}: {fault}')from error
        at=address(state,area,offset)
        if at in seen:raise ValueError('native parent repeated admitted missing fact:'+str(fault))
        seen.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
    raise ValueError('parent fact selection did not converge')

def expected(c,raw,state):
    changes={(d['area'],d['offset']):d for d in c['changes']}
    cells=[]
    for f in state['cells']:
        if f['area']=='frame':continue
        change=changes.get((f['area'],f['offset']));value=change['after']if change else raw[address(state,f['area'],f['offset'])]
        cells.append({**f,'value':value})
    ds=16*state['regs'][11];scene=ds+0xCCEF+25*state['scene']['index']
    known={address(state,f['area'],f['offset'])for f in state['cells']}
    known.update(range(scene,scene+25));known.update(range(ds+0xA51C,ds+0xA51E))
    for slot in facts.SLOTS.values():known.update(range(ds+slot,ds+slot+4))
    known.update(range(ds+0xD0C0,ds+0xD0C4))
    for change in c['changes']:
        at=address(state,change['area'],change['offset'])
        if raw[at]!=change['before']:raise ValueError('original change baseline differs from admitted ENTRY')
        if at not in known:raise ValueError(f'original parent{c["case"]} changed undeclared gameplay field '+str(change))
    return {'scene':c['output_scene'],'seed':c['seed_after'],'pointers':c['output_pointers']+[state['pointers'][-1]],'cells':cells,'draws':c['draws'],'events':c['events']}

class Probe:
    def __init__(self,path):self.process=subprocess.Popen([str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
    def watched_action(self,value):
        self.process.stdin.write(value+'\n');self.process.stdin.flush();line=self.process.stdout.readline()
        if not line:raise ValueError('native probe ended unexpectedly')
        return line
    def close(self):
        self.process.stdin.close()
        if self.process.wait()!=0:raise ValueError('native probe failed')

def export(probe=None):
    if probe:native=Probe(probe)
    else:import onenil_engine as native
    values,manifest=originals();items=[];FIXTURES.mkdir(parents=True,exist_ok=True)
    for c,anchor in values:
        path=FIXTURES/f'case-{c["case"]:04d}.json'
        if not path.exists():
            raw,state,actual=select_entry(c,anchor,native);want=expected(c,raw,state)
            if actual!=want:
                keys=[key for key in want if actual[key]!=want[key]]
                raise ValueError(f'parent{c["case"]} numeric gameplay differs: '+str(keys))
            value={'case':c['case'],'admission':c['admission'],'source_sha256':manifest['cases'][c['case']]['sha256'],'input':state,'expected':want}
            oracle.write_json(path,value)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('native original parent',c['case'],'verified',flush=True)
    value={'schema':'decoded-watched-actor/1','producer':oracle.common.git('rev-parse','HEAD'),'original_manifest_sha256':CORPUS_SHA256,'cases':items,'scope':'Original ENTRY facts only;original expected changes/results never feed native helper decisions'}
    out=FIXTURES/'manifest.json'
    if not out.exists():oracle.write_json(out,value)
    if probe:native.close()
    return check()

def check():
    originals_values,original_manifest=originals();manifest=read(FIXTURES/'manifest.json')
    if manifest['original_manifest_sha256']!=CORPUS_SHA256 or len(manifest['cases'])!=50:raise ValueError('decoded parent provenance/coverage differs')
    values=[]
    for (c,anchor),item in zip(originals_values,manifest['cases'],strict=True):
        path=ROOT/item['path']
        if sha(path)!=item['sha256']:raise ValueError('decoded parent fixture digest differs')
        value=read(path)
        if value['case']!=c['case']or value['source_sha256']!=original_manifest['cases'][c['case']]['sha256']or value['admission']!=c['admission']:raise ValueError('decoded parent source differs')
        state=value['input']
        if state['kind']!='actor'or state['scene']!=c['input_scene']or state['seed']!=c['seed_before']or state['regs']!=[c['registers'][r]for r in oracle.REGS]or state['flags']!=c['flags']or state['stack_words']!=c['stack_words']or state['pointers'][:-1]!=c['pointers']or state['pointers'][-1]['kind']!='style':raise ValueError('decoded parent entry context differs')
        out=value['expected']
        for key,field in [('scene','output_scene'),('seed','seed_after'),('pointers','output_pointers'),('draws','draws'),('events','events')]:
            want=c[field]+[state['pointers'][-1]]if key=='pointers'else c[field]
            if out[key]!=want:raise ValueError('decoded original parent expected '+key+' differs')
        values.append(value)
    return values

def check_research(research_root):
    values,_=originals()
    for c,anchor in values:
        overlay=c.get('input_overlay')
        if overlay:
            source=overlay['source'];path=research_root/source['path']
            if sha(path)!=source['sha256']:raise ValueError('original tick input source digest differs')
            before=next(t['before']for t in read(path)['payload']['ticks']if t['tick']==source['tick'])
            if before!=overlay['before']:raise ValueError('original tick BEFORE input overlay differs')
    print('original tick-entry overlay provenance verified')

def replay(values):
    import onenil_engine as native
    for c in values:
        got=json.loads(native.watched_action(json.dumps(c['input'])))
        if got.get('ok')!=c['expected']:raise ValueError(f'native parent{c["case"]} mismatch: '+str(got.get('error')))
    print('native watched actor:',len(values),'original parent transitions matched')

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path);p.add_argument('--check',action='store_true');p.add_argument('--replay',action='store_true');p.add_argument('--check-research',type=Path);a=p.parse_args()
    if a.export:export(a.probe)
    if a.check_research:check_research(a.check_research)
    if a.check or a.replay:
        values=check();print('decoded actor corpus:',len(values),'roots verified')
        if a.replay:replay(values)
    if not(a.export or a.check or a.replay or a.check_research):p.error('choose export/check/replay/check-research')
if __name__=='__main__':main()
