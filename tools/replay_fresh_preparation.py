#!/usr/bin/env python3
"""T60 source-backed fresh preparation; projections only, no Python engine model."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

import replay_match_setup as prior
import replay_watched_setup as watched
import watched_oracle

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT / 'records/T60/source-r0001'
FIXTURE = ROOT / 'engine/tests/fixtures/fresh_preparation/admission.json'
PINS = {
    'admission-r0001.json': '9639833b657ed2cf0c5684e7338843534f0d552e8323b55dd12e947a5b6e6331',
    'admission-r0002.json': '1740b20d867391823970d9feb0c02b3e0040f476e8d492282c38363f0015414f',
    'incoming-r0001.json': '550ac2590e78256ab3c442c61b2f7eaac1259872fb327a1a43b8775fea37d755',
}


def read(path):
    return json.loads(path.read_text())


def sha(data):
    return hashlib.sha256(data).hexdigest()


def import_sources(research):
    """Import unchanged decoded leaves only after the archaeology public checks."""
    import subprocess
    subprocess.run(['python3', str(research/'runtime/emul/campaign_admission.py'),
                    '--check', str(research/'runs/T66/admission-r0002.json'),
                    '--compare', str(research/'runs/T66/admission-r0001.json')], check=True)
    subprocess.run(['python3', str(research/'runtime/emul/campaign_admission.py'),
                    '--check', str(research/'runs/T66/incoming-r0001.json')], check=True)
    values = {}
    for name, digest in PINS.items():
        data = (research/'runs/T66'/name).read_bytes()
        if sha(data) != digest:
            raise ValueError('fresh original source pin differs: '+name)
        values[name] = data
    a, b = (json.loads(values[name]) for name in ('admission-r0001.json', 'admission-r0002.json'))
    if a['payload'] != b['payload']:
        raise ValueError('independent original preparation differs')
    # Retain one complete measured leaf. The repeat is authenticated above and
    # remains in archaeology; its independent identity and equal digest travel.
    DIRECTORY.mkdir(parents=True, exist_ok=True)
    for name in ('admission-r0001.json', 'incoming-r0001.json'):
        with (DIRECTORY/name).open('xb') as output:
            output.write(values[name])


def sources():
    records = {}
    for name in ('admission-r0001.json', 'incoming-r0001.json'):
        data = (DIRECTORY/name).read_bytes()
        if sha(data) != PINS[name]:
            raise ValueError('fresh original source digest differs: '+name)
        records[name] = json.loads(data)
    record = records['admission-r0001.json']
    incoming = records['incoming-r0001.json']
    payload = record['payload']
    if record['status'] != 'completed' or record['unsupported'] or payload['interventions']:
        raise ValueError('incomplete/intervened fresh original source')
    if incoming['payload']['draws'] != payload['draws'][:payload['points'][2]['draw']]:
        raise ValueError('fresh incoming RNG join differs')
    return payload, incoming['payload']['fields']


def admission(freeze=False):
    original, fields = sources()
    setup = prior.initial_data(
        original['points'],
        {'incomes': [{'input': row} for row in original['income_inputs']],
         'fields': original['supplement']},
        incoming_fields=fields,
        # [designed] Watched preparation does not execute quick_contests. Zero
        # is an unused API adapter value, never a claimed original chance count.
        contest_k=0,
    )
    value = {'schema': 'fresh-campaign-watched-preparation/1',
             'sources': copy.deepcopy(PINS),
             'input': {'setup': setup, 'incoming': original['incoming']}}
    if freeze:
        FIXTURE.parent.mkdir(parents=True, exist_ok=True)
        with FIXTURE.open('x') as output:
            json.dump(value, output, indent=2)
            output.write('\n')
    if read(FIXTURE) != value:
        raise ValueError('fresh admission differs from incoming source projection')
    return value, original


def replay(value, original):
    import onenil_engine as native
    prepared = native.NativeFreshWatchedPreparation(json.dumps(value['input']))
    expected = copy.deepcopy(original)
    expected['draws'] = [watched_oracle.normalized(draw) for draw in original['draws']]
    watched.compare_preparation(json.loads(prepared.preparation()), expected)
    prior.require_equal(json.loads(prepared.snapshot()), original['first_entry'], 'fresh first entry')
    before = prepared.snapshot(), prepared.preparation()
    if before != (prepared.snapshot(), prepared.preparation()):
        raise ValueError('fresh repeated observation mutates state')
    print('PASS fresh native eleven boundaries,258 draws,every first-entry field')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-from', type=Path)
    parser.add_argument('--freeze', action='store_true')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--replay', action='store_true')
    args = parser.parse_args()
    if args.import_from:
        import_sources(args.import_from.resolve())
    if args.freeze or args.check or args.replay:
        value, original = admission(args.freeze)
        print('PASS authenticated original source and fresh incoming projection')
        if args.replay:
            replay(value, original)


if __name__ == '__main__':
    main()
