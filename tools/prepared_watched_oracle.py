#!/usr/bin/env python3
"""Decoded pre-seeding support probes for the native prepared-image bridge."""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
SLOTS={'p8':0xD7BA,'p15':0xD7BE,'club':0xD7C2,'config':0xD7D2,
       'background':0xD092,'counts':0xDCC8,'results':0xD7CE,'fixtures':0xD7CA}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(research):
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    _,base,_=full.reference();_,data=full.match.find_state(base['start_state']['sha256'])
    week=full.match.Week(data)
    # Only the old logical field names are discovery probes. Their old captured
    # values are never used to construct support or a native state.
    prior=json.loads((ROOT/'engine/tests/fixtures/watched_match/initial.json').read_text())
    candidates=sorted({(f['area'],f['offset']) for f in prior['initial']['cells'] if f['area']=='ds' or f['area'] in SLOTS})
    candidates=sorted(set(candidates)|{('background',offset) for offset in range(4000)})
    output=dict(status='incomplete',fixture_request=None,fields=[],background_header_stable=None,
                background_unchanged=None,first_entry=None,setup_draws=None)
    incoming=None;headers=None
    def values(g):
        result={}
        bases={area:g.far(slot) for area,slot in SLOTS.items()}
        for area,base in bases.items():
            if base<16*g.reg('ds')+65536:
                raise ValueError(f'original {area} pool is not initialized before support observation')
        for area,offset in candidates:
            address=(16*g.reg('ds')+offset) if area=='ds' else (bases[area]+offset)
            if not 0<=address<0x110000:
                raise ValueError('initial support probe exceeds original memory')
            result[area,offset]=g.mem(address,1)[0]
        return result
    class Complete(Exception):pass
    def begin(g):
        nonlocal incoming,headers
        if incoming is not None:raise ValueError('duplicate pre-seeding support boundary')
        incoming=values(g);headers={area:g.far(slot) for area,slot in SLOTS.items()}
        bp=g.reg('bp');ss=16*g.reg('ss')
        output['fixture_request']=dict(week=g.word_at(ss+bp+4),club=g.word_at(ss+bp+6))
        g.put(g.far(0xD7D2)+0xF2,b'\x01')
        print('original initial support observed before19CA',flush=True)
    def entry(g):
        if incoming is None:raise ValueError('watched entry preceded incoming probes')
        final=values(g)
        output['fields']=[dict(area=area,offset=offset,before=incoming[area,offset],entry=final[area,offset]) for area,offset in candidates]
        output['background_header_stable']=headers['background']==g.far(SLOTS['background'])
        output['background_unchanged']=all(incoming['background',i]==final['background',i] for i in range(4000))
        output['first_entry']=full.state(g);output['setup_draws']=week.draws
        raise Complete()
    week.g.code_hook('0AB9:E24C',begin);week.g.code_hook('0AB9:7C67',entry)
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before watched bridge entry')
        if week.g.unsupported:raise ValueError('unsupported original services')
        if week.draws!=258:raise ValueError('initial support run does not join setup258 draws')
        output['status']='completed'
    except (Exception,KeyboardInterrupt) as error:
        output['status']=f'aborted:{type(error).__name__}:{error}'
    return output


def headless_prefix(research):
    """[designed] Derived original presentation initializer;171 exact caller steps."""
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    import replay_watched_match as reference
    import watched_oracle as normalize
    _,base,_=full.reference();_,data=full.match.find_state(base['start_state']['sha256'])
    week=full.match.Week(data);tick=0;pending=None;event_pending=None
    result=dict(status='incomplete',projection=None,ticks=[],events=[],draws=[],clock_end=None)
    class Complete(Exception):pass
    def enable(g):g.put(g.far(0xD7D2)+0xF2,b'\x01')
    def initialize(g):
        if result['projection'] is not None:raise ValueError('duplicate derived field initializer')
        address=g.far(0xD092);before=g.mem(0,0x110000);page=bytearray(g.mem(address,4000))
        # [designed] Only the gameplay coordinate rectangle changes. Keep the
        # original HUD outside it so the original harness still recognizes UI.
        for x in range(-8,9):
            for y in range(-37,38):
                at=0x90E+160*x+2*y;page[at]=9 if(x,y)==(0,0)else 32;page[at+1]=7
        g.put(address,page);after=g.mem(0,0x110000)
        same=before[:address]==after[:address]and before[address+4000:]==after[address+4000:]
        result['projection']=dict(site='0AB9:E303',field='background gameplay rectangle',cells=17*75,
                                  encoding='space/attribute7;centre marker9',outside_background_unchanged=same)
        if not same:raise ValueError('derived field initializer changed memory outside background')
    def enter(g):
        nonlocal tick,pending
        if pending is not None:raise ValueError('nested original derived watched tick')
        tick+=1;pending=dict(tick=tick,draw_start=week.draws,before=full.state(g))
    def leave(g):
        nonlocal pending
        if pending is None:raise ValueError('original derived tick returned without entry')
        pending.update(draw_end=week.draws,after=full.state(g));result['ticks'].append(pending);pending=None
    def event_enter(g):
        nonlocal event_pending
        if not tick:return
        if event_pending is not None:raise ValueError('nested original commentary')
        args=full.prefix.arguments(g);side=args['side'];club=g.rd(0xD6E4+2*side)
        def player(slot):return 0 if slot==0 else int.from_bytes(g.mem(g.far(0xD7C2)+62*(club-1)+2*(slot-1),2),'little')
        event_pending=dict(args,club=club,player=player(args['actor_slot']),secondary=player(args['secondary_slot']),
                           minute=g.rd(0xD0B2),tick=tick,draw_start=week.draws,seed_before=g.rd(0xA51C),
                           head_before=g.rd(0x380),caller=g.image_address(g.reg('cs'),g.stack_word(0)))
    def event_leave(g):
        nonlocal event_pending
        event=event_pending;event_pending=None
        if event is not None and g.rd(0x380)!=event['head_before']:
            event.update(event=len(result['events']),draw_end=week.draws,seed_after=g.rd(0xA51C),queue=full.prefix.row(g))
            result['events'].append(event)
    def caller_end(g):
        if tick%25==0:print(f'derived original caller{tick}/171 draws{week.draws}',flush=True)
        if tick==171:
            if pending is not None:raise ValueError('derived caller ended within tick')
            result['clock_end']=full.state(g);raise Complete()
    for site,hook in [('0AB9:E24C',enable),('0AB9:E303',initialize),('0AB9:7A02',enter),
                      ('0AB9:7B79',leave),('0AB9:0F94',event_enter),('0AB9:0FFC',event_leave),('0AB9:7C94',caller_end)]:
        week.g.code_hook(site,hook)
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before171 derived caller iterations')
        if week.g.unsupported:raise ValueError('unsupported original services')
        result['draws']=[normalize.normalized(d)for d in week.events if d['kind']=='draw']
        ticks,draws,events,_=reference.sources();end=ticks[171]['draw_start']
        if result['ticks']!=ticks[:171]:raise ValueError('derived original field changes watched snapshots')
        if result['clock_end']!=ticks[171]['before']:raise ValueError('derived original field changes caller clock/end state')
        if result['draws']!=[normalize.normalized(d)for d in draws[:end]]:raise ValueError('derived original field changes RNG prefix')
        if result['events']!=[e for e in events if e['tick']<=171]:raise ValueError('derived original field changes accepted actions')
        result['status']='completed'
    except (Exception,KeyboardInterrupt)as error:
        result['status']=f'aborted:{type(error).__name__}:{error}'
        result['draws']=[normalize.normalized(d)for d in week.events if d['kind']=='draw']
    return result


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root',type=Path,required=True)
    parser.add_argument('--run',action='store_true')
    parser.add_argument('--private',type=Path)
    parser.add_argument('--headless-prefix',action='store_true')
    args=parser.parse_args()
    if sum((args.run,bool(args.private),args.headless_prefix))!=1:parser.error('choose --run,--headless-prefix or --private RESULT')
    if args.private:
        if observe(args.research_root)!=json.loads(args.private.read_text()):raise ValueError('pre-seeding support observations differ')
        print('all initial support probes reproduced from original pre-seeding state',flush=True);return
    if subprocess.check_output(['git','status','--porcelain','--','tools/prepared_watched_oracle.py'],cwd=ROOT,text=True):raise ValueError('commit original support observer before measurement')
    root=ROOT/'records/T44';root.mkdir(parents=True,exist_ok=True)
    prefix='headless-original'if args.headless_prefix else'original'
    number=max((int(p.name.split('-r')[1]) for p in root.glob(prefix+'-r*') if p.is_dir()),default=0)+1
    directory=root/f'{prefix}-r{number:04d}';directory.mkdir()
    protocol=dict(schema='original-prepared-support/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                  observer_sha256=sha(Path(__file__)),research_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=args.research_root,text=True).strip(),
                  probe_names_source_sha256=sha(ROOT/'engine/tests/fixtures/watched_match/initial.json'),
                  preparation_admission_sha256=sha(ROOT/'engine/tests/fixtures/watched_setup/admission.json'),
                  scope='Candidate logical support values before19CA and first-entry assertions;explicit source ownership is still required before admitting any field. No RAM archive persisted.')
    if args.headless_prefix:
        protocol['scope']='Derived original gameplay-rectangle initializer atE303;171 caller iterations compared to original unmodified reference. No full-lifecycle/other-session preservation claim.'
    # Write the complete protocol before starting the original.
    with (directory/'start.json').open('x')as output:json.dump(protocol,output,indent=2);output.write('\n')
    result=headless_prefix(args.research_root)if args.headless_prefix else observe(args.research_root)
    (directory/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    manifest=dict(schema='original-prepared-support-manifest/1',files={name:sha(directory/name) for name in ('start.json','result.json')})
    (directory/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(directory.relative_to(ROOT),result['status'],flush=True)
    if result['status']!='completed':raise SystemExit(1)


if __name__=='__main__':main()
