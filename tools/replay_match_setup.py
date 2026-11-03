#!/usr/bin/env python3
"""One selected-squad/legacy admission;native preparation then the entire continuing match."""
import argparse,copy,json,sys
from datetime import datetime,timezone
from pathlib import Path
import replay_continuous_match as match
half=match.half
common=match.common
ROOT=common.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/setup'

def initial_data(points,inputs):
    old=copy.deepcopy(points[0]['state']);selected=points[2]['state']
    raw_selected=selected['caller']['pairing']['context']['rebuild']['outer']
    pool={p['id']:p for p in raw_selected['players']}
    old_pool={p['id']:p for p in old['caller']['pairing']['context']['rebuild']['outer']['players']}
    if set(pool)!=set(old_pool) or any(p[k]!=old_pool[i][k] for i,p in pool.items() for k in p if k!='p8_6'):
        raise ValueError('incoming selected-squad facts differ beyond supplied role/roster choices')
    clubs={c['id']:c['roster'] for c in raw_selected['selection']['clubs']}
    def project(value):
        # Structural linking of supplied P8 roles/rosters/presentation attributes.
        # Keep all old DS counters,clock,caches,seed and selector bytes as legacy.
        if isinstance(value,list):return [project(v) for v in value]
        if not isinstance(value,dict):return value
        out={k:project(v) for k,v in value.items()}
        identifier=out.get('id',out.get('player'))
        if identifier in pool:
            role=pool[identifier]['p8_6']
            if 'p8_6' in out:out['p8_6']=role
            if 'p8_position' in out:out['p8_position']=role
            if 'id' in out:
                for key in ('position','role'):
                    if key in out:out[key]=role
        if out.get('id') in clubs and 'roster' in out:out['roster']=clubs[out['id']][:len(out['roster'])]
        if out.get('club') in clubs and 'roster' in out:out['roster']=clubs[out['club']][:len(out['roster'])]
        if out.get('club') in clubs and 'players' in out and all(isinstance(i,int) for i in out['players']):out['players']=clubs[out['club']][:len(out['players'])]
        if 'directions' in out and isinstance(out['directions'],list) and len(out['directions'])==4:out['directions']=raw_selected['directions']
        return out
    old=project(old);a=old['initial'];a['goals']['rosters']=[clubs[i] for i in a['goals']['clubs']];c=old['caller'];raw=c['pairing']['context'];outer=raw['rebuild']['outer']
    core=match.rearrangement.core(raw)
    ratings=[];fields={f['club']:f for f in selected['setup_cells']['club_fields']}
    for side,club in enumerate(reversed(outer['selection']['clubs'])):
        roster=club['roster'][:11];bonus=next(b for b in a['ratings']['bonuses'] if b['club']==club['id'])
        ratings.append({'club':club['id'],'side':side,'flag':a['extra']['home_bonus'] if side==0 else 0,
                        'club_37':fields[club['id']]['byte55'],'roster':roster,
                        'players':[{'id':i,**{k:pool[i][k] for k in ('p8_4','p8_6','p8_7','p15_6','p15_8','p15_11')}} for i in sorted(set(roster)) if i],
                        'staff_ids':bonus['staff_ids'],'staff':bonus['staff'],'tables':a['ratings']['tables'],
                        'ratings':c['ratings'],'seed':c['seed']})
    # Only the pinned readonly chance witness is joined from the prior corpus.
    chance=json.loads((half.FIXTURES/'admission.json').read_text())['contest']['k']
    legacy={'contest':{'k':chance,'seed':c['seed'],'ratings':c['ratings'],'goals':[0,0]},
            'ratings':ratings,'players':core['players'],'scenes':outer['scenes'],'goal':a['goals'],
            'extra':{'state':a['extra'],'opponent_d0b':a['threshold']},'event':a['events'],'reset':a['reset'],
            'substitution':{'context':outer['selection'],'replacement':None},'neutral':a['neutral'],'clock':a['clock'],
            'history':a['history'],'config':a['config'],'selectors':[v['value'] for v in a['selectors']],
            'scorer_slots':a['scorer_slots'],'minute_limits':a['minute_limits'],'replacement_word':outer['globals']['position_default']}
    wanted=set(a['positions']['context']['criteria'][2:]);skills=[s for s in a['skills'] if s['criterion'] in wanted];ref=skills[0]['state']
    extra={'bonus':ref['bonus'],'rosters':ref['club_rosters'],'tables':sorted(match.table_rows(skills),key=lambda t:(t['criterion'],t['player']['id'])),'criterion_cells':[{'criterion':s['criterion'],'value':s['state']['criterion_cell']} for s in skills]}
    state={'core':core,'pairing':match.pairing.state(c['pairing']),'codes':match.codes.state(c['codes']),
           'cache':match.cache.state(a['positions']['context'],extra),
           'terminal':match.terminal.state({'context':raw},a['positions']['positions'],a['proximity'])}
    g=outer['globals']
    settings_path=ROOT/'records/T28/settings-r0001.json'
    settings_pin=json.loads((FIXTURES/'settings-index.json').read_text())
    if common.sha(settings_path.read_bytes())!=settings_pin['sha256']:raise ValueError('setup settings evidence digest differs')
    settings=json.loads(settings_path.read_text())
    return {'managed_selection_mode':settings['observed']['managed_selection_mode'],'legacy':legacy,'replacement':{'match_state':state,'display_mode':g['mode'],'position_sentinel':g['position_sentinel'],'speaker_gate':g['speaker_gate']},
            'gates':c['gates'],'skill_tables':match.table_rows([s for s in a['skills'] if s['criterion'] in (6,14)]),
            'setup_cells':selected['setup_cells'],'income':[r['input'] for r in inputs['incomes']],
            'supplement':inputs['fields']}

def load():
    pin=json.loads((FIXTURES/'provenance.json').read_text())
    if common.sha((match.FIXTURES/'provenance.json').read_bytes())!=pin['prior_match_pin_sha256']:raise ValueError('prior full-match source pin differs')
    for name,sha in pin['sources'].items():
        if common.sha((FIXTURES/name).read_bytes())!=sha:raise ValueError('setup original source digest differs')
    rows=[json.loads(l) for l in (FIXTURES/'T38-r0002.jsonl').read_text().splitlines()]
    points=[r for r in rows if r['schema']=='setup-boundary/1'];incomes=[r for r in rows if r['schema']=='setup-income/1']
    supplement=json.loads((FIXTURES/'T38-input-r0001.json').read_text());fields={r['name']:r['value'] for r in supplement['outputs']}['fields']
    data=initial_data(points,{'incomes':incomes,'fields':fields})
    if not common.same(data,json.loads((FIXTURES/'admission.json').read_text())):raise ValueError('setup initial-data projection differs')
    return data,points,pin

def check(data,points):
    import onenil_engine as native
    session=native.NativeExhibitionSession(common.encoded(data))
    # The whole original setup/match sequence will be asserted here;no later
    # expected snapshot is ever passed to this constructor or any step().
    return session

def capture_settings(research_root):
    """Observe the original once;never change guest settings or export private bytes."""
    root=research_root.resolve()
    sys.path.insert(0,str(root/'runtime/emul'))
    import match_setup as observer
    meta,base=next(iter(observer.selected_runs().values()))
    _,data=observer.match.find_state(base['start_state']['sha256'])
    week=observer.match.Week(data);observer.Recorder(week)
    captured=None
    class ReachedAdmission(Exception):pass
    def observe(g):
        nonlocal captured
        current=observer.state(week)
        expected=observer.contests.outputs(base)
        draws=[e for e in week.events if e['kind']=='draw']
        if not common.same(current,expected['points'][2]['state']):
            raise ValueError('settings capture reached a different pre86E9 state')
        if not common.same(draws,expected['draws'][:255]):
            raise ValueError('settings capture original255-draw prefix differs')
        captured={'initial_sha256':observer.initials.digest(current),
                  'draws_sha256':observer.initials.digest(draws),'draw_count':len(draws),
                  'managed_selection_mode':g.mem(g.far(0xD7D2)+0xF6,1)[0]}
        raise ReachedAdmission()
    if common.git('status','--porcelain','--','tools/replay_match_setup.py'):
        raise ValueError('commit observer before capture')
    week.g.code_hook('0AB9:86E9',observe)
    started=datetime.now(timezone.utc).isoformat()
    try:week.play(on_boundary=lambda w:print(f'settings:screen{w.screen_i},draws{w.draws}',flush=True))
    except ReachedAdmission:pass
    else:raise ValueError('settings capture did not reach admission')
    if week.g.unsupported or captured is None:raise ValueError('unsupported/incomplete settings capture')
    record={'schema':'setup-settings-observation/1','code_commit':common.git('rev-parse','HEAD'),
            'started':started,'ended':datetime.now(timezone.utc).isoformat(),
            'source':{'repository':'https://github.com/diegoami/onenil-archaeology',
                      'commit':observer.match.git('rev-parse','HEAD'),
                      'record':str(meta.relative_to(root)),'record_sha256':common.sha(meta.read_bytes()),
                      'static':{'static/1-0-exe/code/seg_0AB9_18.asm':common.sha((root/'static/1-0-exe/code/seg_0AB9_18.asm').read_bytes())}},
            'start_state_sha256':base['start_state']['sha256'],
            'game_files':{k:common.sha(v) for k,v in week.files_at_start.items()},'observed':captured,
            'scope':'Read-only original config+F6 at exact T38 pre86E9 state/255-draw prefix;intentional stop,not whole-match capture'}
    out=ROOT/'records/T28';out.mkdir(parents=True,exist_ok=True)
    n=max((int(p.stem.split('-r')[-1]) for p in out.glob('settings-r*.json')),default=0)+1
    path=out/f'settings-r{n:04d}.json'
    with path.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
    print(path, captured,flush=True)

def main():
    ap=argparse.ArgumentParser(description=__doc__);g=ap.add_mutually_exclusive_group(required=True);g.add_argument('--check',action='store_true');g.add_argument('--capture-settings',action='store_true');ap.add_argument('--research-root',type=Path);args=ap.parse_args()
    if args.capture_settings:
        if args.research_root is None:ap.error('--capture-settings requires --research-root')
        capture_settings(args.research_root);return
    data,points,_=load();check(data,points)
if __name__=='__main__':main()
