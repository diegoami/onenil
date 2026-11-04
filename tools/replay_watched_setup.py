#!/usr/bin/env python3
"""Pinned original watched preparation;Python checks observations,never models setup."""
import argparse
import hashlib
import json
import subprocess
import time
import platform
from datetime import datetime, timezone
from importlib.metadata import version
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


def benchmark(value, repetitions):
    import onenil_engine as api
    from onenil_engine import onenil_engine as native
    if subprocess.check_output(['git','status','--porcelain','--','engine','tools'],cwd=ROOT,text=True):
        raise ValueError('commit watched preparation sources before measuring')
    if native.__debug_assertions__:
        raise ValueError('release extension required')
    request=json.dumps(value['input'],sort_keys=True,separators=(',',':'))
    began=datetime.now(timezone.utc).isoformat();size=0;start=time.perf_counter_ns()
    for _ in range(repetitions):
        session=api.NativeWatchedPreparation(request)
        size+=len(session.preparation())+len(session.snapshot())
        del session
    elapsed=time.perf_counter_ns()-start
    directory=ROOT/'records/T43'
    number=max((int(p.stem.split('-r')[1]) for p in directory.glob('T43-r*.json')),default=0)+1
    path=directory/f'T43-r{number:04d}.json'
    record=dict(schema='routine-benchmark/1',code_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                started=began,elapsed_ns=elapsed,repetitions=repetitions,setup_draws=258*repetitions,
                setup_boundaries=11*repetitions,serialized_output_bytes=size,debug_assertions=native.__debug_assertions__,
                extension_sha256=sha(Path(native.__file__)),cargo_lock_sha256=sha(ROOT/'engine/Cargo.lock'),
                admission_sha256=sha(FIXTURE),original_manifest_sha256=MANIFEST_SHA,
                platform=platform.platform(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),maturin=version('maturin'),
                method='Pre-serialized selected-squad and pre-seeding incoming input;release parse,owned native preparation,all boundary/first-entry serialization and drop.',
                scope='Preparation to first watched ENTRY only;full continuing watched lifecycle composition and supported choices remain required. No emulator-speed comparison.')
    with path.open('x') as output:
        json.dump(record,output,indent=2);output.write('\n')
    print(path.relative_to(ROOT),flush=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--freeze',action='store_true')
    parser.add_argument('--check',action='store_true')
    parser.add_argument('--replay',action='store_true')
    parser.add_argument('--benchmark',action='store_true')
    parser.add_argument('--repetitions',type=int,default=3)
    args=parser.parse_args()
    if not(args.freeze or args.check or args.replay or args.benchmark):
        parser.error('choose --freeze,--check,--replay or --benchmark')
    if args.repetitions<1:
        parser.error('repetitions must be positive')
    value,original=admission(args.freeze)
    print('pinned incoming supplement and watched setup reference verified',flush=True)
    if args.replay or args.benchmark:
        replay(value,original)
    if args.benchmark:
        benchmark(value,args.repetitions)


if __name__=='__main__':
    main()
