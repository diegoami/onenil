#!/usr/bin/env python3
"""Pinned original watched preparation;Python checks observations,never models setup."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path
import replay_match_setup as prior
import replay_watched_match as reference
import watched_oracle

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT/'records/T43/original-r0001'
MANIFEST_SHA = 'b5b054d1c95feaadc17ccab18bda9f4175cb8507b240a17c9d46e81d278c7e50'
FIXTURE = ROOT/'engine/tests/fixtures/watched_setup/admission.json'


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sources():
    if sha(DIRECTORY/'manifest.json') != MANIFEST_SHA:
        raise ValueError('watched preparation manifest root differs')
    manifest = read(DIRECTORY/'manifest.json')
    if set(manifest['files']) != {'start.json','result.json'}:
        raise ValueError('watched preparation source census differs')
    for name, digest in manifest['files'].items():
        if sha(DIRECTORY/name) != digest:
            raise ValueError('watched preparation original leaf differs')
    start = read(DIRECTORY/'start.json'); result = read(DIRECTORY/'result.json')
    if result['status'] != 'completed' or start['setup_admission_sha256'] != sha(prior.FIXTURES/'admission.json'):
        raise ValueError('watched preparation admission source differs')
    if start['full_reference_index_sha256'] != reference.INDEX_SHA or start['research_commit'] != '60179ee16009e175591b79d9230ff59027ccf6a5':
        raise ValueError('watched preparation evidence source differs')
    if start['observer_sha256'] != sha(ROOT/'tools/watched_setup_oracle.py'):
        raise ValueError('original observer source differs')
    from watched_setup_oracle import SITES
    if start['sites'] != list(SITES) or [p['site'] for p in result['points']] != list(SITES):
        raise ValueError('watched preparation boundaries differ')
    if [p['draw'] for p in result['points']] != [0,255,255,256,258,258,258,258,258,258,258]:
        raise ValueError('watched preparation draw cursor differs')
    ticks,draws,_,_ = reference.sources()
    if result['first_entry'] != ticks[0]['before'] or result['draws'] != [watched_oracle.normalized(d) for d in draws[:258]]:
        raise ValueError('watched preparation does not join complete original reference')
    incoming = result['incoming'];ball=result['first_entry']['ball']
    if set(incoming) != {'ball_byte6','ball_word7','ball_word9','scores'}:
        raise ValueError('incoming watched supplement fields differ')
    if (incoming['ball_byte6'],incoming['ball_word7'],incoming['ball_word9'],incoming['scores']) != (ball['byte6'],ball['word7'],ball['word9'],result['first_entry']['scores']):
        raise ValueError('incoming preservation witness differs')
    return start,result


def admission(freeze=False):
    _,original = sources()
    selected,_,_ = prior.load()
    incoming=original['incoming']
    if selected['legacy']['contest']['goals'] != incoming['scores']:
        raise ValueError('incoming scratch-score alias differs from selected legacy input')
    expected=dict(schema='selected-squad-watched-preparation/1',source_manifest_sha256=MANIFEST_SHA,
                  input=dict(setup=selected,incoming=incoming))
    if freeze:
        if FIXTURE.exists():
            raise ValueError('immutable watched preparation fixture exists')
        FIXTURE.parent.mkdir(parents=True,exist_ok=True)
        FIXTURE.write_text(json.dumps(expected,indent=2)+'\n')
    value=read(FIXTURE)
    if value != expected:
        raise ValueError('watched preparation input is not the initial selected data and pre-seeding supplement')
    return value,original


def compare_preparation(preparation, original):
    prior.require_equal(preparation['draws'],original['draws'],'watched setup258 ordered draws')
    points=original['points']
    witnesses={p['id']:p for p in points[2]['state']['caller']['pairing']['context']['rebuild']['outer']['selection']['players']}
    if len(preparation['points']) != len(points):
        raise ValueError('watched preparation point count differs')
    for index,(got,want) in enumerate(zip(preparation['points'],points)):
        initial=want['state']['initial'];caller=want['state']['caller']
        expected=dict(site=want['site'],draw=want['draw'],config=initial['config'],
                      selectors=[v['value'] for v in initial['selectors']],selector_head=initial['events']['selectors']['head'])
        if index<2:
            # T28's supplied career squad/colour boundary is unchanged. Compare
            # seeding ownership before pre86E9,then every named setup projection.
            expected['seed']=caller['seed']
            actual={k:got[k] for k in expected if k!='seed'};actual['seed']=got['caller']['seed']
        else:
            expected.update(income_total=want['income_total'],caller=prior.expected_caller(caller,witnesses),
                            **{k:initial[v] for k,v in [('goal','goals'),('event','events'),('extra','extra'),('reset','reset'),('clock','clock')]})
            actual=got
        prior.require_equal(actual,expected,f'watched preparation boundary{index} {want["site"]}')


def replay(value, original):
    import onenil_engine as native
    session=native.NativeWatchedPreparation(json.dumps(value['input']))
    preparation=json.loads(session.preparation())
    compare_preparation(preparation,original)
    snapshot=json.loads(session.snapshot())
    prior.require_equal(snapshot,original['first_entry'],'native first watched ENTRY')
    prior.require_equal(json.loads(session.snapshot()),snapshot,'read-only prepared inspection')
    prior.require_equal(json.loads(session.preparation()),preparation,'inspection preserves prepared state/draws')
    print('native selected-squad watched preparation:258draws/11boundaries and complete first ENTRY match',flush=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--freeze',action='store_true')
    parser.add_argument('--check',action='store_true')
    parser.add_argument('--replay',action='store_true')
    args=parser.parse_args()
    if not(args.freeze or args.check or args.replay):
        parser.error('choose --freeze,--check or --replay')
    value,original=admission(args.freeze)
    print('pinned incoming supplement and watched setup reference verified',flush=True)
    if args.replay:
        replay(value,original)


if __name__=='__main__':
    main()
