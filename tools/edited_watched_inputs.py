"""T68: exact edited-case support from T66's authenticated T72 witnesses."""
import argparse,json
from pathlib import Path
import replay_edited_preparation as preparation
ROOT=preparation.ROOT
FIXTURE=ROOT/'engine/tests/fixtures/edited_watched/incoming.json'

def admission(freeze=False):
    preparation.sources()  # All source leaves and canonical catalog authenticate first.
    facts={}
    for i in (1,3):
        record=preparation.read(preparation.DIRECTORY/f'support-r{i:04d}.json')
        for row in record['payload']['fields']:
            if not row['incoming']:continue
            if not row['before']==row['precall']==row['entry']:raise ValueError('edited ownership differs')
            key=row['area'],row['offset']
            if key in facts and facts[key]!=row['before']:raise ValueError('edited support conflict')
            facts[key]=row['before']
    value=dict(schema='edited-watched-incoming/1',sources={k:v[1] for k,v in preparation.PINS.items()},support=[dict(area=a,offset=o,value=v) for (a,o),v in sorted(facts.items())])
    if freeze:
        FIXTURE.parent.mkdir(parents=True,exist_ok=True)
        with FIXTURE.open('x') as f:json.dump(value,f,indent=2);f.write('\n')
    if preparation.read(FIXTURE)!=value:raise ValueError('edited incoming fixture differs from original before values')
    return value

def request():
    value,_=preparation.admission()
    return dict(preparation=value['input'],support=admission()['support'],no_pending_key=True)

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--freeze',action='store_true');p.add_argument('--check',action='store_true');a=p.parse_args()
    if a.freeze or a.check:print('PASS edited input support',len(admission(a.freeze)['support']))
