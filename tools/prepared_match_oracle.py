#!/usr/bin/env python3
"""Original full-lifecycle numeric field-initializer preservation proof;no RAM export."""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def encoded(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()
def observe(research,directory):
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    import watched_oracle as normalize
    import replay_watched_match as reference
    ticks,draws,events,finish=reference.sources()
    expected_draws=[normalize.normalized(d)for d in draws]
    _,base,_=full.reference();_,data=full.match.find_state(base['start_state']['sha256'])
    week=full.match.Week(data);tick=0;pending=None;event_pending=None
    tick_digest=hashlib.sha256();event_digest=hashlib.sha256()
    result=dict(status='incomplete',projection=None,ticks=0,actions=0,draws=0,caller_clocks=[],final=None)
    class Complete(Exception):pass
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def initialize(g):
        if result['projection']is not None:raise ValueError('duplicate derived field initializer')
        address=g.far(0xD092);before=g.mem(0,0x110000);page=bytearray(g.mem(address,4000))
        for x in range(-8,9):
            for y in range(-37,38):
                at=0x90E+160*x+2*y;page[at]=9 if(x,y)==(0,0)else 32;page[at+1]=7
        g.put(address,page);after=g.mem(0,0x110000)
        same=before[:address]==after[:address]and before[address+4000:]==after[address+4000:]
        result['projection']=dict(site='0AB9:E303',field='background gameplay rectangle',cells=1275,
                                  encoding='space/attribute7;centre marker9',outside_background_unchanged=same)
        if not same:raise ValueError('field initializer changes memory outside background')
    def enter(g):
        nonlocal tick,pending
        if pending is not None or tick>=len(ticks):raise ValueError('nested/excess original watched tick')
        tick+=1;pending=dict(tick=tick,draw_start=week.draws,before=full.state(g))
    def leave(g):
        nonlocal pending
        if pending is None:raise ValueError('original tick returned without entry')
        pending.update(draw_end=week.draws,after=full.state(g))
        if pending!=ticks[tick-1]:raise ValueError(f'field initializer changes original tick{tick}')
        tick_digest.update(encoded(pending)+b'\n');result['ticks']=tick;pending=None
        if tick%250==0:
            with (directory/'progress.jsonl').open('a')as output:
                output.write(json.dumps(dict(tick=tick,draws=week.draws,actions=result['actions']))+'\n')
            print('preserved original tick',tick,'draws',week.draws,flush=True)
    def event_enter(g):
        nonlocal event_pending
        if not tick:return
        if event_pending is not None:raise ValueError('nested original commentary')
        args=full.prefix.arguments(g);side=args['side'];club=g.rd(0xD6E4+2*side)
        def player(slot):return 0 if slot==0 else int.from_bytes(g.mem(g.far(0xD7C2)+62*(club-1)+2*(slot-1),2),'little')
        event_pending=dict(args,club=club,player=player(args['actor_slot']),secondary=player(args['secondary_slot']),minute=g.rd(0xD0B2),
            tick=tick,draw_start=week.draws,seed_before=g.rd(0xA51C),head_before=g.rd(0x380),caller=g.image_address(g.reg('cs'),g.stack_word(0)))
    def event_leave(g):
        nonlocal event_pending
        event=event_pending;event_pending=None
        if event is not None and g.rd(0x380)!=event['head_before']:
            index=result['actions'];event.update(event=index,draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g))
            if index>=len(events)or event!=events[index]:raise ValueError(f'field initializer changes original action{index}')
            event_digest.update(encoded(event)+b'\n');result['actions']+=1
    def clock(g):
        if pending is not None:raise ValueError('caller clock returned inside watched tick')
        fields={'active':(0xD0B8,1),'counter':(0xD0B0,2),'remaining':(0xD0AE,2),'stoppage':(0xD0B4,2),'elapsed':(0xD0B2,2),'phase':(0xD0AC,1),'hold':(0xD0B9,1)}
        result['caller_clocks'].append(dict(tick=tick,clock={k:g.rd(o,w)for k,(o,w)in fields.items()}))
    def end(g):
        if not tick:return
        if pending is not None or event_pending is not None:raise ValueError('unfinished original observation at EBE2')
        result['final']=full.state(g);raise Complete()
    for site,hook in [('0AB9:E24C',enable),('0AB9:E303',initialize),('0AB9:7A02',enter),('0AB9:7B79',leave),
                      ('0AB9:0F94',event_enter),('0AB9:0FFC',event_leave),('0AB9:7C94',clock),(full.match.MATCH_RETURN,end)]:
        week.g.code_hook(site,hook)
    try:
        try:week.play(max_run_on=500000)
        except Complete:pass
        else:raise ValueError('original ended without full watched EBE2 return')
        actual_draws=[normalize.normalized(d)for d in week.events if d['kind']=='draw'];result['draws']=len(actual_draws)
        if week.g.unsupported or result['ticks']!=len(ticks)or result['actions']!=len(events)or actual_draws!=expected_draws or result['final']!=finish['final']:
            raise ValueError('original full-lifecycle field preservation differs')
        import replay_watched_caller as caller
        (prefix,_),_=caller.sources()
        if result['caller_clocks'][:171]!=[dict(tick=i+1,clock=row['output']['clock'])for i,row in enumerate(prefix)]:
            raise ValueError('derived clock observations differ from the existing original prefix')
        indices=[row['tick']for row in result['caller_clocks']]
        if any(a>=b for a,b in zip(indices,indices[1:])):raise ValueError('original caller clocks lack unique increasing tick identities')
        result.update(status='completed',boundary=full.match.MATCH_RETURN,draws_sha256=hashlib.sha256(encoded(actual_draws)).hexdigest())
    except (Exception,KeyboardInterrupt)as error:
        result['status']=f'aborted:{type(error).__name__}:{error}';result['draws']=week.draws
    result.update(ticks_sha256=tick_digest.hexdigest(),actions_sha256=event_digest.hexdigest())
    return result

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
    if subprocess.check_output(['git','status','--porcelain','--','tools/prepared_match_oracle.py'],cwd=ROOT,text=True):raise ValueError('commit observer before original measurement')
    import replay_watched_match as reference
    directory=ROOT/'records/T45';directory.mkdir(exist_ok=True)
    number=max((int(x.name.split('-r')[1])for x in directory.glob('original-r*')if x.is_dir()),default=0)+1
    directory=directory/f'original-r{number:04d}';directory.mkdir()
    protocol=dict(schema='original-prepared-full-lifecycle/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
        observer_sha256=sha(Path(__file__)),research_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=a.research_root,text=True).strip(),
        reference_index_sha256=reference.INDEX_SHA,prepared_admission_sha256=sha(ROOT/'engine/tests/fixtures/prepared_watched/admission.json'),
        scope='Derived original field initializer;all4199 ticks/182811draws/831actions and actual final return compared to the unchanged reference. Actual caller clocks decoded separately. No RAM archive or native runtime backend.')
    (directory/'start.json').write_text(json.dumps(protocol,indent=2)+'\n')
    result=observe(a.research_root,directory);(directory/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    files={x.name:sha(x)for x in sorted(directory.iterdir())if x.is_file()}
    (directory/'manifest.json').write_text(json.dumps(dict(schema='original-prepared-full-lifecycle-manifest/1',files=files),indent=2)+'\n')
    print(directory.relative_to(ROOT),result['status'],flush=True)
    if result['status']!='completed':raise SystemExit(1)
if __name__=='__main__':main()
