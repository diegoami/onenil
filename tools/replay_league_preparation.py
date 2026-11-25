#!/usr/bin/env python3
"""T117: the league fixture's native preparation, imported from archaeology T119.

Archaeology T119 decodes the T100 preparation state (the match-#28 PLAY GAME
machine) without running a guest instruction. This tool pins that record, builds
the native setup input from its full decode, and checks the resulting clubs,
rosters and managed positions against the record's own admission projection.

    python3 tools/replay_league_preparation.py --import-from <archaeology checkout>
    python3 tools/replay_league_preparation.py --freeze
    python3 tools/replay_league_preparation.py --check
"""
import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import replay_match_setup as setup  # noqa: E402

RESEARCH_COMMIT = '04a4996369e17084ccefb634a830c31ad2de7973'
SOURCE_NAME = 'admission-r0004.json'
SOURCE_SHA = 'bacaf4c5adba07499a4d7086091ab1edf59e1a442bf77c24007b2921d1f7770b'
SOURCE_DIR = ROOT / 'records/T117/source-r0001'
FIXTURE = ROOT / 'engine/tests/fixtures/league_preparation/admission.json'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def validate_source(record):
    if record['schema'] != 'original-league-admission/2' or record['status'] != 'completed':
        raise ValueError('incomplete league admission source')
    if record['source']['state_sha256'] != '3abfd6b00f8024900b93a20d1ff1eec60cfd2d570e927a00eca495d7f7b1d1c8':
        raise ValueError('source savestate identity differs')
    payload = record['payload']
    if set(payload) != {'campaign', 'watch_setting', 'admission', 'state', 'setup_state',
                        'supplement', 'income_inputs', 'incoming', 'incoming_fields'}:
        raise ValueError('source payload shape differs')
    if payload['campaign']['week_index'] != 54 or payload['campaign']['club']['id'] != 130:
        raise ValueError('source is not the week-54 Blackburn league machine')
    if {club['id'] for club in payload['admission']['clubs']} != {130, 122}:
        raise ValueError('source admission clubs differ')


def load_source():
    raw = (SOURCE_DIR / SOURCE_NAME).read_bytes()
    if sha(raw) != SOURCE_SHA:
        raise ValueError('league admission source digest differs')
    record = json.loads(raw)
    validate_source(record)
    return record


def build():
    record = load_source()
    payload = record['payload']
    state = payload['setup_state']
    points = [{'state': state, 'income_total': payload['incoming_fields']['legacy_income_total']}] * 3
    inputs = {'incomes': [{'input': row} for row in payload['income_inputs']],
              'fields': payload['supplement']}
    value = setup.initial_data(points, inputs, incoming_fields=payload['incoming_fields'],
                               contest_k=0)
    context = value['legacy']['substitution']['context']
    if [club['id'] for club in context['clubs']] != [130, 122]:
        raise ValueError('native setup clubs differ from Blackburn R/Newcastle Utd')
    projected = context['clubs'][0]['roster']
    admitted = payload['admission']['clubs'][0]['roster']
    if projected != admitted:
        raise ValueError('native setup roster differs from the recorded admission')
    if value['managed_selection_mode'] != payload['incoming_fields']['managed_selection_mode']:
        raise ValueError('native selection mode differs from the recorded field')
    return dict(schema='league-campaign-watched-preparation/1', research_commit=RESEARCH_COMMIT,
                sources={SOURCE_NAME: SOURCE_SHA},
                input=dict(setup=value, incoming=payload['incoming']))


def import_sources(research):
    source = research / 'runs/T119/admission-r0004.json'
    data = source.read_bytes()
    if sha(data) != SOURCE_SHA:
        raise ValueError('the archaeology record digest differs')
    validate_source(json.loads(data))
    if SOURCE_DIR.exists():
        raise ValueError('refusing to overwrite the imported source evidence')
    SOURCE_DIR.mkdir(parents=True)
    (SOURCE_DIR / SOURCE_NAME).write_bytes(data)
    print('imported', SOURCE_DIR.relative_to(ROOT), SOURCE_SHA[:12])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-from', type=Path)
    parser.add_argument('--freeze', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    if args.import_from:
        import_sources(args.import_from.resolve())
    if args.freeze or args.check:
        value = build()
        if args.freeze:
            FIXTURE.parent.mkdir(parents=True, exist_ok=True)
            with FIXTURE.open('x') as handle:
                json.dump(value, handle, indent=2)
                handle.write('\n')
        if FIXTURE.read_bytes() != (json.dumps(value, indent=2) + '\n').encode():
            raise ValueError('the league preparation fixture differs from its source')
        count = len(value['input']['setup']['legacy']['players'])
        print('PASS league preparation fixture:', count, 'players, clubs 130/122')


if __name__ == '__main__':
    main()
