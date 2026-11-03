#!/usr/bin/env python3
"""One selected-squad/legacy admission;native preparation then the entire continuing match."""
import argparse,copy,json
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
    old=project(old);a=old['initial'];c=old['caller'];raw=c['pairing']['context'];outer=raw['rebuild']['outer']
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
    return {'legacy':legacy,'replacement':{'match_state':state,'display_mode':g['mode'],'position_sentinel':g['position_sentinel'],'speaker_gate':g['speaker_gate']},
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

def main():
    ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--check',action='store_true',required=True);ap.parse_args()
    data,points,_=load();check(data,points)
if __name__=='__main__':main()
