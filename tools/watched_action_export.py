"""[designed] Decode ENTRY facts selected by native guards;never feed expected outcomes."""
import json
from pathlib import Path
import watched_oracle as oracle
ROOT=oracle.ROOT
SLOTS={'p8':0xD7BA,'p15':0xD7BE,'club':0xD7C2,'config':0xD7D2,'main':0xD08A,'main_shown':0xD08E,'background':0xD092,'background_shown':0xD096,'commentary':0xD09A,'seed':0xDE84}
LENGTHS={'p8':31368,'p15':58815,'club':10850,'config':256,'main':4000,'main_shown':4000,'background':4000,'background_shown':4000}

def word(raw,at):return int.from_bytes(raw[at:at+2],'little')
def context(case,raw):
    ds=16*case['registers']['ds']
    pointers=[{'kind':kind,'offset':word(raw,ds+slot),'segment':word(raw,ds+slot+2)}for kind,slot in SLOTS.items()]
    return {'kind':case['kind'],'scene':case['input_scene'],'seed':case['seed_before'],'regs':[case['registers'][r]for r in oracle.REGS], 'flags':case['flags'],'stack_words':case['stack_words'],'pointers':pointers,'cells':[]}

def pitch(offset):return any(0x90E+160*row-74<=offset<0x90E+160*row+76 for row in range(-8,9))
def address(state,area,offset):
    if area=='ds':return 16*state['regs'][11]+offset
    if area=='frame':return 16*state['regs'][10]+state['regs'][4]+offset
    p=next(p for p in state['pointers']if p['kind']==area);return p['offset']+16*p['segment']+offset

def named(state,at):
    ds=16*state['regs'][11];stack=16*state['regs'][10]+state['regs'][4]
    if stack-4096<=at<stack:return 'frame',at-stack
    offset=at-ds
    if 0x380<=offset<0x382 or 0x900<=offset<0xC00 or 0xD00<=offset<0xD80 or 0x957D<=offset<=0xAD9A or 0xCCEF<=offset<0xD7BA:return 'ds',offset
    for area,n in LENGTHS.items():
        start=address(state,area,0);offset=at-start
        if 0<=offset<n and(area in('p8','p15','club','config')or pitch(offset)):return area,offset
    raise ValueError(f'unclassified original entry fact {at:05X}')

def export_one(case,native,prior=()):
    raw=oracle.memory(case['entry']);state=context(case,raw);seen=set();selected=[]
    def add(area,offset):
        at=address(state,area,offset)
        # Typed facts already supply root scene,seed,pointer slots,and return ABI.
        ds=16*state['regs'][11];scene=ds+0xCCEF+25*state['scene']['index']
        reserved=scene<=at<scene+25 or ds+0xA51C<=at<ds+0xA51E or any(ds+s<=at<ds+s+4 for s in SLOTS.values())
        if reserved or at in seen:return
        # Recheck names against physical aliases;cached facts are entry facts too.
        canonical=named(state,at)
        if canonical!=(area,offset):return
        seen.add(at);selected.append((area,offset));state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
    for area,offset in prior:add(area,offset)
    for attempt in range(20000):
        result=json.loads(native.watched_action(json.dumps(state)))
        if 'ok'in result:return state,result['ok'],selected,attempt
        fault=result['error'];missing=fault['missing']
        if missing is None:raise ValueError(f'case{case["case"]}: {fault["message"]}')
        area,offset=named(state,missing['address'])
        before=len(seen);add(area,offset)
        if len(seen)==before:raise ValueError('native repeated admitted missing fact:'+str(fault))
    raise ValueError('native fact admission did not converge')

def expected(case,state):
    raw=oracle.memory(case['exit']);out={'scene':case['output_scene'],'seed':case['seed_after'],'pointers':[],'cells':[],'draws':case['draws'],'events':[]}
    ds=16*state['regs'][11]
    for kind,slot in SLOTS.items():out['pointers'].append({'kind':kind,'offset':word(raw,ds+slot),'segment':word(raw,ds+slot+2)})
    for f in state['cells']:
        if f['area']!='frame':out['cells'].append({**f,'value':raw[address(state,f['area'],f['offset'])]})
    for event in case['events']:
        e={k:v for k,v in event.items()if k not in('tick','event')}
        for key in('draw_start','draw_end'):e[key]-=case['draw_start']
        out['events'].append(e)
    return out

def validate_ownership(case,state):
    entry=oracle.memory(case['entry']);exit=oracle.memory(case['exit']);ds=16*state['regs'][11]
    known={address(state,f['area'],f['offset'])for f in state['cells']}
    known.update(range(ds+0xCCEF+25*state['scene']['index'],ds+0xCCEF+25*state['scene']['index']+25));known.update(range(ds+0xA51C,ds+0xA51E))
    for slot in SLOTS.values():known.update(range(ds+slot,ds+slot+4))
    spans=[(ds+lo,ds+hi)for lo,hi in [(0x380,0x382),(0x900,0xC00),(0xD00,0xD80),(0x957D,0xAD9B),(0xCCEF,0xD7BA)]]
    for area,n in LENGTHS.items():
        start=address(state,area,0)
        if area in('p8','p15','club','config'):spans.append((start,start+n))
        else:spans.extend((start+0x90E+160*r-74,start+0x90E+160*r+76)for r in range(-8,9))
    for lo,hi in spans:
        for a in range(lo,hi):
            if entry[a]!=exit[a]and a not in known:raise ValueError(f'original changed undeclared gameplay fact {a:05X} case{case["case"]}')

def main():
    import argparse,onenil_engine
    p=argparse.ArgumentParser();p.add_argument('--limit',type=int,default=1);p.add_argument('--compare',action='store_true');a=p.parse_args()
    cases,_=oracle.validate(ROOT/'records/T37/oracle-r0001');prior={}
    directory=ROOT/'.cache/T37/export-dev';directory.mkdir(parents=True,exist_ok=True)
    for case in cases[:a.limit]:
        state,out,selected,attempts=export_one(case,onenil_engine,prior.get(case['kind'],()))
        prior[case['kind']]=selected
        if a.compare:
            want=expected(case,state)
            for key in want:
                if want[key]!=out[key]:raise ValueError(f'case{case["case"]} {key} differs: '+str({'expected':want[key],'actual':out[key]}))
            validate_ownership(case,state)
        (directory/f'case-{case["case"]:05d}.json').write_text(json.dumps({'input':state,'actual':out},indent=2)+'\n')
        print('native case',case['case'],'kind',case['kind'],'facts',len(selected),'restarts',attempts,'draws',len(out['draws']),flush=True)
if __name__=='__main__':main()
