"""[designed] Original72BB microkernel oracles from source-proven derived entries.
This observes original execution;it models no football decision or helper outcome.
"""
import argparse,json,sys
from pathlib import Path
import watched_oracle as oracle
import watched_action_export as facts
ROOT=oracle.ROOT
MODES=(1,2,3,5,6,7,8,9,10,11,13)

def anchors():
    cases,manifest=oracle.validate(ROOT/'records/T37/oracle-r0001')
    choices=[c for c in cases if c['kind']=='action5'and c['stack_words'][0]==0x72E4]
    chosen={}
    for scene in range(1,23):
        c=next(c for c in choices if c['scene']==scene);chosen[c['case']]=c
    for n in range(128):
        c=choices[(len(choices)-1)*n//127];chosen[c['case']]=c
        if len(chosen)==32:break
    if len(chosen)!=32:raise ValueError('32 distinct parent anchors required')
    return[(c,manifest['cases'][c['case']]['sha256'])for c in sorted(chosen.values(),key=lambda c:c['case'])]

def admitted(c,mode=5):
    raw=bytearray(oracle.memory(c['entry']));r=dict(c['registers']);parent_sp=r['bp']+2
    r['bp']=facts.word(raw,16*r['ss']+r['bp']);r['sp']=parent_sp
    at=16*r['ds']+0xCCEF+25*c['scene'];before=raw[at+19]
    if before!=5:raise ValueError('anchor is not action5')
    if mode not in MODES:raise ValueError('unsupported action-mode intervention')
    raw[at+19]=mode
    stack=16*r['ss']+parent_sp;words=[facts.word(raw,stack+2*i)for i in range(4)]
    if words[1]!=r['cs']or words[2]+16*words[3]!=at:raise ValueError('parent return/argument ABI differs')
    return bytes(raw),r,words

def guest(research_root,raw,registers,flags):
    sys.path.insert(0,str(research_root.resolve()/'runtime/emul'))
    import guest as original
    values={k.upper():v for k,v in registers.items()}|{'IP':0x72BB,'EFLAGS':flags}
    g=original.Guest(memory=raw,load=registers['cs']-0x0AB9,regs=values)
    # [confirmed] Match PIT divisor1998:00BA/00C5 is0400;phase is reset
    # for these isolated original calls. No IRQ is due during the short prefix.
    g.divisor=0x400
    return g

def prove_one(research_root,c,source_sha):
    raw,registers,words=admitted(c);g=guest(research_root,raw,registers,c['flags']);found={}
    class Joined(Exception):pass
    def join(g):
        observed={k:g.reg(k)for k in oracle.REGS};flags=g.reg('eflags');stack=[g.stack_word(i)for i in(0,2,4,6)]
        digest=oracle.common.sha(g.mem(0,0x110000))
        if observed!=c['registers']or flags!=c['flags']or stack!=c['stack_words']or oracle.scene(g,c['scene'])!=c['input_scene']or g.rd(0xA51C)!=c['seed_before']or digest!=c['entry']['ram_sha256']:raise ValueError('original parent prefix does not reproduce retained nested entry')
        found.update(registers=observed,flags=flags,stack_words=stack,seed=g.rd(0xA51C),ram_sha256=digest,game_timer_ticks=g.game_timer_ticks)
        raise Joined()
    g.code_hook('0AB9:448C',join)
    try:g.run(max_steps=4096)
    except Joined:pass
    if not found or g.unsupported or g.game_timer_ticks:raise ValueError('parent prefix proof incomplete or environment-dependent')
    return {'source_case':c['case'],'source_sha256':source_sha,'admission':'derived-parent-entry','parent_registers':registers,'parent_stack_words':words,'joined_nested_entry':found,'liveness':'AX/DI/ES/flags are recomputed by72BB–72E1;BX/CX/DX/SI/DS/SS/CS are unchanged. Parent BP/SP/header come from retained original caller frame. Only dead/overwritten stack slots are prefilled.'}

def prove(research_root):
    if oracle.common.git('status','--porcelain','--','tools/watched_parent.py'):raise ValueError('commit observer before original measurement')
    entries=[prove_one(research_root,c,sha)for c,sha in anchors()]
    return {'schema':'original-parent-admission-proof/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':oracle.common.sha(Path(__file__).read_bytes()),'source_oracle_manifest_sha256':oracle.MANIFEST_SHA256,'clock':'original match divisor0400,isolated initial phase;no IRQ before nested-entry join','entries':entries,'scope':'Source-proven derived admissions;not observations of the original parent entry. No native result supplies any input.'}

def owned_addresses(state,pitch_margin=False):
    ds=16*state['regs'][11];seen=set()
    spans=[(ds+a,ds+b)for a,b in[(0x380,0x382),(0x900,0xC00),(0xD00,0xD80),(0x957D,0xAD9B),(0xCCEF,0xD7BA)]]
    for area,n in facts.LENGTHS.items():
        start=facts.address(state,area,0)
        if area in('p8','p15','club','config'):spans.append((start,start+n))
        else:spans.extend((start+0x90E+160*r-74,start+0x90E+160*r+76)for r in range(-9 if pitch_margin else -8,10 if pitch_margin else 9))
    for lo,hi in spans:
        for at in range(lo,hi):
            if at in seen or at-ds in(0xD5A,0xD5B,0xD088,0xD089):continue
            seen.add(at);yield at

def parent_state(c,mode,overlay=None):
    raw,registers,words=admitted(c,mode);raw=bytearray(raw);ds=16*registers['ds']
    def put(at,value,width):raw[at:at+width]=value.to_bytes(width,'little')
    if overlay is not None:
        before=overlay['before']
        for row in before['scenes']:
            for key,value in row.items():
                if key=='index':continue
                put(ds+0xCCEF+25*row['index']+int(key[4:]),value,2 if key.startswith('word')else 1)
        for key,value in before['ball'].items():put(ds+0xCF46+int(key[4:]),value,2 if key.startswith('word')else 1)
        for key,offset,width in [('seed',0xA51C,2),('actor',0xD09F,1),('secondary',0xD0A0,1),('elapsed',0xD0B2,2),('remaining',0xD0AE,2),('stoppage',0xD0B4,2),('phase',0xD0AC,1),('active',0xD0B8,1),('hold',0xD0B9,1),('quick',0xD074,1)]:put(ds+offset,before[key],width)
    at=ds+0xCCEF+25*c['scene'];put(at+19,mode,1)
    scene={'index':c['scene'],**{f'word{k}':facts.word(raw,at+k)for k in(0,2,8,10,12,14,20,23)},**{f'byte{k}':raw[at+k]for k in(4,5,6,7,16,17,18,19,22)}}
    pseudo={**c,'kind':'actor','registers':registers,'stack_words':words,'input_scene':scene,'seed_before':facts.word(raw,ds+0xA51C)}
    return bytes(raw),facts.context(pseudo,raw)

def event_entry(g,watch):
    args={'code':g.stack_word(2)&255,'actor_slot':g.stack_word(6),'secondary_slot':g.stack_word(4),'side':g.stack_word(8)&255}
    club=g.rd(0xD6E4+2*args['side'])
    def player(slot):return 0 if slot==0 else int.from_bytes(g.mem(g.far(0xD7C2)+62*(club-1)+2*(slot-1),2),'little')
    return args|{'club':club,'player':player(args['actor_slot']),'secondary':player(args['secondary_slot']),'minute':g.rd(0xD0B2),'draw_start':watch.draws,'seed_before':g.rd(0xA51C),'head_before':g.rd(0x380),'caller':g.image_address(g.reg('cs'),g.stack_word(0))}

def run_one(research_root,c,source_sha,mode,number,files,overlay=None,pitch_margin=False):
    sys.path.insert(0,str(research_root.resolve()/'runtime/emul'))
    import match as original
    raw,state=parent_state(c,mode,overlay);registers=dict(zip(oracle.REGS,state['regs']));g=guest(research_root,raw,registers,state['flags']);g.files=dict(files)
    watch=original.Week.__new__(original.Week);watch.g=g;watch.events=[];watch.draws=0;watch._pending=None;watch.install_hooks()
    events=[];pending=[None];child={'joined':False,'returned':False};finished=[False]
    class Returned(Exception):pass
    def on_enter(g):
        if mode!=5 or overlay is not None or child['joined']:return
        if {r:g.reg(r)for r in oracle.REGS}!=c['registers']or g.reg('eflags')!=c['flags']or oracle.common.sha(g.mem(0,0x110000))!=c['entry']['ram_sha256']:raise ValueError('derived parent lost retained nested-entry join')
        child['joined']=True;child['draw_start']=watch.draws;child['event_start']=len(events)
    def on_child_return(g):
        if mode!=5 or overlay is not None or not child['joined']or child['returned']or g.reg('sp')!=c['registers']['sp']:return
        draws=[oracle.normalized(d)for d in watch.events if d['kind']=='draw'][child['draw_start']:]
        if oracle.scene(g,c['scene'])!=c['output_scene']or g.rd(0xA51C)!=c['seed_after']or draws!=c['draws']:raise ValueError('original nested outcome/RNG differs from retained independent root')
        child['returned']=True
    def on_event(g):
        if pending[0]is not None:raise ValueError('nested numeric commentary')
        pending[0]=event_entry(g,watch)
    def on_event_return(g):
        value=pending[0];pending[0]=None
        if value is None:return
        if g.rd(0x380)!=value['head_before']:
            head=g.rd(0x380);at=0xA52A+6*head
            value.update(draw_end=watch.draws,seed_after=g.rd(0xA51C),queue={'index':head,'code':g.rd(at,1),'player':g.rd(at+1),'secondary':g.rd(at+3),'side':g.rd(at+5,1)})
            events.append(value)
    def on_return(g):
        if g.reg('sp')==state['regs'][4]and g.reg('bp')==state['regs'][5]:finished[0]=True;raise Returned()
    g.code_hook('0AB9:448C',on_enter);g.code_hook('0AB9:4610',on_child_return);g.code_hook('0AB9:0F94',on_event);g.code_hook('0AB9:0FFC',on_event_return);g.code_hook('0AB9:7420',on_return)
    try:
        while g.budget<20_000_000:g.run(max_steps=min(500_000,20_000_000-g.budget));g.other_polls=0
    except Returned:pass
    if not finished[0]or g.unsupported or pending[0]is not None or watch._pending is not None:raise ValueError('original parent did not return cleanly within budget')
    if mode==5 and overlay is None and not(child['joined']and child['returned']):raise ValueError('missing original nested-entry/outcome join')
    after=g.mem(0,0x110000);changes=[]
    for at in owned_addresses(state,pitch_margin):
        if raw[at]!=after[at]:
            area,offset=facts.named(state,at);changes.append({'area':area,'offset':offset,'before':raw[at],'after':after[at]})
    ds=16*state['regs'][11]
    pointers=[{'kind':kind,'offset':facts.word(after,ds+slot),'segment':facts.word(after,ds+slot+2)}for kind,slot in facts.SLOTS.items()]
    return {'schema':'original-parent-microkernel/1','case':number,'source_case':c['case'],'source_sha256':source_sha,'mode':mode,'admission':'derived-with-tick-entry-overlay'if overlay else('derived-unmodified'if mode==5 else'derived-with-mode-intervention'),'input_overlay':overlay,'interventions':([{'field':'original-tick-entry-player-ball-clock-inputs','source':overlay['source']}]if overlay else([]if mode==5 else[{'field':'actor.byte19','before':5,'after':mode}])),
       'registers':registers,'flags':state['flags'],'stack_words':state['stack_words'],'input_scene':state['scene'],'seed_before':state['seed'],'pointers':state['pointers'],'output_scene':oracle.scene(g,c['scene']),'seed_after':g.rd(0xA51C),'output_pointers':pointers,
       'changes':changes,'draws':[oracle.normalized(d)for d in watch.events if d['kind']=='draw'],'events':events,'exit_sp':g.reg('sp'),'exit_bp':g.reg('bp'),'nested_join':child,'game_timer_ticks':g.game_timer_ticks,'scope':'Original computation from source-proven derived parent admission;not a captured live parent entry. Presentation countdown/ISR phase excluded.'}

def capture(research_root,directory=ROOT/'records/T38/oracle-r0001'):
    if oracle.common.git('status','--porcelain','--','tools/watched_parent.py'):raise ValueError('commit observer before original measurement')
    sources=anchors();proof_path=ROOT/'records/T38/admission-proof-r0001.json';proof=oracle.read_json(proof_path)
    if[(e['source_case'],e['source_sha256'])for e in proof['entries']]!=[(c['case'],sha)for c,sha in sources]:raise ValueError('parent proof/source anchors differ')
    # Prefer an actually observed current ball actor for the explicit mode tests.
    ball_actors=[pair for pair in sources if oracle.memory(pair[0]['entry'])[16*pair[0]['registers']['ds']+0xD09F]==pair[0]['scene']]
    representative=(ball_actors or sources)[-1]
    catalog=[(c,sha,5)for c,sha in sources]+[(representative[0],representative[1],mode)for mode in MODES if mode!=5]
    protocol={'schema':'original-parent-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':oracle.common.sha(Path(__file__).read_bytes()),'proof_sha256':oracle.common.sha(proof_path.read_bytes()),'cases':[{'source_case':c['case'],'source_sha256':sha,'mode':mode}for c,sha,mode in catalog],'clock':'isolated original match divisor0400;no-key services;source ISR runs,its presentation phase is excluded'}
    start=directory/'start.json'
    if start.exists():
        old=oracle.read_json(start)
        if any(old[k]!=protocol[k]for k in('observer_sha256','proof_sha256','cases','clock')):raise ValueError('parent capture resume source differs')
        protocol=old
    else:oracle.write_json(start,protocol)
    sys.path.insert(0,str(research_root.resolve()/'runtime/emul'));import match as original
    files=original.game_files();items=[]
    for number,(c,sha,mode)in enumerate(catalog):
        path=directory/f'case-{number:04d}.json'
        if not path.exists():
            try:value=run_one(research_root,c,sha,mode,number,files)
            except(Exception,KeyboardInterrupt)as error:
                i=len(list(directory.glob('aborted-*.json')))+1;oracle.write_json(directory/f'aborted-{i:04d}.json',{'status':'aborted','case':number,'source_case':c['case'],'mode':mode,'reason':f'{type(error).__name__}: {error}','producer':protocol['producer']});raise
            oracle.write_json(path,value)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':oracle.common.sha(path.read_bytes())})
        print(f'original parent{number+1}/{len(catalog)} source{c["case"]} mode{mode}',flush=True)
    codes={e['code']for item in items for e in oracle.read_json(ROOT/item['path'])['events']}
    if not codes.intersection({17,18,19,20}):raise ValueError('no original shot-launch event captured')
    out={'schema':'original-parent-corpus/1','status':'completed','producer':protocol['producer'],'protocol_sha256':oracle.common.sha(start.read_bytes()),'proof_sha256':protocol['proof_sha256'],'cases':items,'modes':sorted({v[2]for v in catalog}),'event_codes':sorted(codes),'scope':'32 source-proven unmodified parent admissions plus explicit action-mode interventions;not live parent-entry observations'}
    manifest=directory/'manifest.json'
    if manifest.exists():
        if oracle.read_json(manifest)!=out:raise ValueError('completed original parent manifest differs')
    else:oracle.write_json(manifest,out)
    print('original parent corpus completed:',len(items),'roots;codes',sorted(codes),flush=True)

def capture_shots(research_root,directory=ROOT/'records/T38/oracle-r0002'):
    if oracle.common.git('status','--porcelain','--','tools/watched_parent.py'):raise ValueError('commit observer before original measurement')
    sources=anchors();by_scene={}
    for pair in sources:by_scene.setdefault(pair[0]['scene'],pair)
    overlays=[];seen=set()
    for path in sorted((research_root/'runs/T40').glob('chunk-*.json')):
        value=oracle.read_json(path);payload=value['payload'];ticks={t['tick']:t['before']for t in payload['ticks']}
        for event in payload['actions']:
            if event['code']not in(17,18,19,20)or event['tick']in seen:continue
            before=ticks[event['tick']];actor=before['actor']
            if actor not in by_scene or before['scenes'][actor-1]['byte19']!=5:continue
            seen.add(event['tick']);overlays.append({'source':{'path':str(path.relative_to(research_root)),'sha256':oracle.common.sha(path.read_bytes()),'tick':event['tick'],'boundary':'before original watched tick','scope':'Input snapshot overlay,not an observed parent entry or a requested shot outcome'},'before':before})
        if len(overlays)>=8:break
    overlays=overlays[:8]
    if not overlays:raise ValueError('no original tick-entry input overlays available')
    inherited_dir=ROOT/'records/T38/oracle-r0001';inherited=[]
    for n in range(42):
        path=inherited_dir/f'case-{n:04d}.json';inherited.append({'path':str(path.relative_to(ROOT)),'sha256':oracle.common.sha(path.read_bytes()),'origin_protocol_sha256':oracle.common.sha((inherited_dir/'start.json').read_bytes())})
    proof_path=ROOT/'records/T38/admission-proof-r0001.json'
    protocol={'schema':'original-parent-supplement-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':oracle.common.sha(Path(__file__).read_bytes()),'proof_sha256':oracle.common.sha(proof_path.read_bytes()),'inherited':inherited,'input_overlays':overlays,'scope':'Additional initial player/ball/clock input overlays from original tick BEFORE observations;other facts retain the earlier source context. No actual parent-entry or exact historical-shot reproduction claim.'}
    start=directory/'start.json'
    if start.exists():
        old=oracle.read_json(start)
        if any(old[k]!=protocol[k]for k in('proof_sha256','inherited','input_overlays','scope')):raise ValueError('supplement resume admission differs')
        complete=all((directory/f'case-{i:04d}.json').exists()for i in range(42,42+len(overlays)))
        if not complete and old['observer_sha256']!=protocol['observer_sha256']:raise ValueError('incomplete supplement observer changed;resume its original source')
        protocol=old
    else:oracle.write_json(start,protocol)
    sys.path.insert(0,str(research_root.resolve()/'runtime/emul'));import match as original
    files=original.game_files();items=list(inherited)
    for i,overlay in enumerate(overlays,42):
        c,sha=by_scene[overlay['before']['actor']];path=directory/f'case-{i:04d}.json'
        if not path.exists():
            try:value=run_one(research_root,c,sha,5,i,files,overlay)
            except(Exception,KeyboardInterrupt)as error:
                n=len(list(directory.glob('aborted-*.json')))+1;oracle.write_json(directory/f'aborted-{n:04d}.json',{'status':'aborted','case':i,'source_case':c['case'],'reason':f'{type(error).__name__}: {error}','producer':protocol['producer']});raise
            oracle.write_json(path,value)
        items.append({'path':str(path.relative_to(ROOT)),'sha256':oracle.common.sha(path.read_bytes()),'origin_protocol_sha256':oracle.common.sha(start.read_bytes())})
        print(f'original shot-context parent{i+1} source{c["case"]} original-before-tick{overlay["source"]["tick"]}',flush=True)
    values=[oracle.read_json(ROOT/item['path'])for item in items];codes={e['code']for c in values for e in c['events']}
    if not codes.intersection({17,18,19,20}):raise ValueError('no computed original shot-launch event in supplemented corpus')
    result={'schema':'original-parent-corpus/1','status':'completed','producer':protocol['producer'],'protocol_sha256':oracle.common.sha(start.read_bytes()),'proof_sha256':protocol['proof_sha256'],'cases':items,'modes':sorted({c['mode']for c in values}),'event_codes':sorted(codes),'scope':'32 source-proven parent admissions,explicit modes and original-before-tick input overlays. Derived/intervened isolated calls,not observed live parent entries.'}
    out=directory/'manifest.json'
    if out.exists():
        if oracle.read_json(out)!=result:raise ValueError('completed parent manifest differs')
    else:oracle.write_json(out,result)
    print('original parent corpus completed:',len(items),'roots;codes',sorted(codes),flush=True)

def capture_margin(research_root):
    """[confirmed] 1998:087B restores/draws set-piece coordinates outside the old17rows.
    Reexecute original inputs;append only newly owned margin changes,never replace r1/r2.
    """
    if oracle.common.git('status','--porcelain','--','tools/watched_parent.py'):raise ValueError('commit observer before measurement')
    import replay_watched_actor as replay
    values,manifest=replay.originals();directory=ROOT/'records/T38/margin-r0001';directory.mkdir(parents=True,exist_ok=True)
    protocol={'schema':'original-parent-margin-protocol/1','producer':oracle.common.git('rev-parse','HEAD'),'observer_sha256':oracle.common.sha(Path(__file__).read_bytes()),'original_manifest_sha256':replay.CORPUS_SHA256,'scope':'Same original parent inputs and outcomes;extend logical pitch ownership from17to19 rows for set-piece occupancy. No previous observations changed.'}
    start=directory/'start.json'
    if start.exists():
        protocol=oracle.read_json(start)
        if protocol['observer_sha256']!=oracle.common.sha(Path(__file__).read_bytes()):raise ValueError('margin observer changed')
    else:oracle.write_json(start,protocol)
    sys.path.insert(0,str(research_root.resolve()/'runtime/emul'));import match as original
    files=original.game_files();items=[]
    for c,anchor in values:
        path=directory/f'case-{c["case"]:04d}.json'
        if not path.exists():
            source_sha=c['source_sha256'];got=run_one(research_root,anchor,source_sha,c['mode'],c['case'],files,c.get('input_overlay'),True)
            old={(d['area'],d['offset']):d for d in c['changes']};new=[]
            for d in got['changes']:
                key=(d['area'],d['offset'])
                if key in old:
                    if old.pop(key)!=d:raise ValueError('original prior change differs')
                else:
                    if d['area']not in('main','main_shown','background','background_shown')or facts.pitch(d['offset']):raise ValueError('unexpected margin change')
                    new.append(d)
            if old:raise ValueError('original lost prior change')
            for key in ('output_scene','seed_after','output_pointers','draws','events','exit_sp','exit_bp'):
                if got[key]!=c[key]:raise ValueError('original margin rerun outcome differs:'+key)
            oracle.write_json(path,{'schema':'original-parent-margin/1','case':c['case'],'original_sha256':manifest['cases'][c['case']]['sha256'],'changes':new,'prior_outcomes_equal':True})
        items.append({'path':str(path.relative_to(ROOT)),'sha256':oracle.common.sha(path.read_bytes())});print('original logical margin',c['case'],'verified',flush=True)
    result={'schema':'original-parent-margin-corpus/1','protocol_sha256':oracle.common.sha(start.read_bytes()),'cases':items}
    out=directory/'manifest.json'
    if not out.exists():oracle.write_json(out,result)
    elif oracle.read_json(out)!=result:raise ValueError('margin manifest differs')

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--margin',action='store_true');p.add_argument('--prove',action='store_true');p.add_argument('--capture',action='store_true');p.add_argument('--capture-shots',action='store_true');p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
    if a.margin:capture_margin(a.research_root)
    if a.prove:
        out=ROOT/'records/T38/admission-proof-r0001.json';oracle.write_json(out,prove(a.research_root));print('original parent admission proof:32 complete nested entries reproduced')
    if a.capture:capture(a.research_root)
    if a.capture_shots:capture_shots(a.research_root)
    if not(a.margin or a.prove or a.capture or a.capture_shots):p.error('choose --prove or --capture')
if __name__=='__main__':main()
