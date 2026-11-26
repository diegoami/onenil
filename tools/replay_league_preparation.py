#!/usr/bin/env python3
"""T117: the league fixture's native preparation, imported from archaeology T119.

Archaeology T119 decodes the T100 preparation state (the match-#28 PLAY GAME
machine) without running a guest instruction. This tool pins that record, builds
the native setup input from its full decode, and checks the resulting clubs,
rosters and managed positions against the record's own admission projection.

    python3 tools/replay_league_preparation.py --import-from <archaeology checkout>
    python3 tools/replay_league_preparation.py --freeze
    python3 tools/replay_league_preparation.py --check
    python3 tools/replay_league_preparation.py --replay
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
SUPPORT_NAME = 'support-r0001.json'
SUPPORT_SHA = '395b4de94dcdd9a2e023394e29a069977199702d659ae8e06755991829473c41'
SUPPORT_FIXTURE = ROOT / 'engine/tests/fixtures/league_watched/incoming.json'


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
    support = load_support()
    incoming = dict(schema='league-watched-incoming/1',
                    sources={SUPPORT_NAME: SUPPORT_SHA}, support=support['facts'])
    return dict(schema='league-campaign-watched-preparation/1', research_commit=RESEARCH_COMMIT,
                sources={SOURCE_NAME: SOURCE_SHA, SUPPORT_NAME: SUPPORT_SHA},
                input=dict(setup=value, incoming=payload['incoming']),
                support=incoming, entry=dict(site=support['entry']['site'],
                                             screen_sha256=support['entry']['screen_sha256']))


def validate_support(record):
    if record['schema'] != 'league-incoming-support/1' or record['status'] != 'completed':
        raise ValueError('incomplete league support source')
    if record['entry']['site'] != '0AB9:7C67':
        raise ValueError('league support entry site differs')
    facts = record['facts']
    if not facts:
        raise ValueError('league support facts are empty')
    seen = set()
    for fact in facts:
        if set(fact) != {'area', 'offset', 'value'}:
            raise ValueError('league support fact shape differs')
        if fact['area'] not in ('ds', 'config', 'counts') or fact['offset'] < 0:
            raise ValueError('league support fact area differs')
        if not isinstance(fact['value'], int) or not 0 <= fact['value'] <= 255:
            raise ValueError('league support fact value differs')
        key = fact['area'], fact['offset']
        if key in seen:
            raise ValueError('duplicate league support fact')
        seen.add(key)


def load_support():
    raw = (SOURCE_DIR / SUPPORT_NAME).read_bytes()
    if sha(raw) != SUPPORT_SHA:
        raise ValueError('league support source digest differs')
    record = json.loads(raw)
    validate_support(record)
    return record


def import_sources(research):
    sources = [(research / 'runs/T119/admission-r0004.json', SOURCE_SHA, SOURCE_NAME, validate_source),
               (research / 'runs/T121/support-r0001.json', SUPPORT_SHA, SUPPORT_NAME, validate_support)]
    if SOURCE_DIR.exists():
        raise ValueError('refusing to overwrite the imported source evidence')
    SOURCE_DIR.mkdir(parents=True)
    for path, digest, name, validate in sources:
        data = path.read_bytes()
        if sha(data) != digest:
            raise ValueError('the archaeology record digest differs: ' + name)
        validate(json.loads(data))
        (SOURCE_DIR / name).write_bytes(data)
    print('imported', SOURCE_DIR.relative_to(ROOT), SOURCE_SHA[:12], SUPPORT_SHA[:12])


def replay():
    value = build()
    legacy = value['input']['setup']['legacy']
    context = legacy['substitution']['context']
    print('preparation: clubs', [club['id'] for club in context['clubs']],
          'managed roster', context['clubs'][0]['roster'],
          'players', len(legacy['players']))
    import onenil_engine
    request = json.dumps({'preparation': value['input'],
                          'support': value['support']['support'],
                          'no_pending_key': True}, sort_keys=True, separators=(',', ':'))
    out = json.loads(onenil_engine.NativePreparedWatchedSession.league(request).finish())
    if 'ok' not in out:
        raise ValueError('the native watched league session failed: '
                         + out.get('error', {}).get('message', '?'))
    ok = out['ok']
    if ok['return'] != '0AB9:EBE2':
        raise ValueError('the native watched return differs')
    final = ok['final']
    print('watched completion:', final['scores'], 'return', ok['return'],
          'ticks', len(ok['ticks']), 'draws', len(ok['draws']), 'events', len(ok['events']))
    print('the T102 2-0 quick result is checked by tools/replay_quick_match.py (T118)')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-from', type=Path)
    parser.add_argument('--freeze', action='store_true')
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--replay', action='store_true')
    args = parser.parse_args()
    if args.import_from:
        import_sources(args.import_from.resolve())
    if args.replay:
        replay()
        return
    if args.freeze or args.check:
        value = build()
        documents = [(FIXTURE, dict(schema=value['schema'], research_commit=value['research_commit'],
                                    sources=value['sources'], input=value['input'])),
                     (SUPPORT_FIXTURE, value['support'])]
        if args.freeze:
            for path, document in documents:
                path.parent.mkdir(parents=True, exist_ok=True)
                with path.open('x') as handle:
                    json.dump(document, handle, indent=2)
                    handle.write('\n')
        for path, document in documents:
            if path.read_bytes() != (json.dumps(document, indent=2) + '\n').encode():
                raise ValueError('the league preparation fixture differs from its source')
        count = len(value['input']['setup']['legacy']['players'])
        print('PASS league preparation fixture:', count, 'players, clubs 130/122')


if __name__ == '__main__':
    main()
