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

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--prove',action='store_true',required=True);p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
    out=ROOT/'records/T38/admission-proof-r0001.json';oracle.write_json(out,prove(a.research_root));print('original parent admission proof:32 complete nested entries reproduced')
if __name__=='__main__':main()
