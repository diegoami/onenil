#!/usr/bin/env python3
"""Check the native quick match against the committed original observations.

The expected facts are the original's, not this engine's: T49's Watch-off
friendly board (0-2, 27/45), T102's league result (2-0, 29/33), T123's league
action stream (scorer and substitution player records) and the T12/T13 step
corpus (92 steps, 5,595 DD02 draws, the step-58 player set). This tool runs the
native session through the Python bridge and compares those facts.

    python3 tools/replay_quick_match.py --check
    python3 tools/replay_quick_match.py --replay
"""
import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'engine/tests/fixtures'
EXPECTED = FIXTURES / 'quick_match/expected.json'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load_expected():
    data = EXPECTED.read_bytes()
    expected = json.loads(data)
    if expected.get('schema') != 'quick-match-corpus/1':
        raise ValueError('not a quick-match corpus')
    for name, digest in expected['sources'].items():
        if sha((ROOT / name).read_bytes()) != digest:
            raise ValueError(f'{name}: source digest differs')
    return expected


def request(row):
    preparation = json.loads((FIXTURES / row['preparation']).read_text())
    support = json.loads((FIXTURES / row['support']).read_text())
    return json.dumps({'preparation': preparation['input'], 'support': support['support'],
                       'no_pending_key': True}, sort_keys=True, separators=(',', ':'))


def play(profile, row):
    import onenil_engine
    return json.loads(onenil_engine.NativeQuickMatch.play(profile, request(row)))


def compare(profile, row, report):
    label = f'{profile} quick match'
    if report['steps'] != row['steps']:
        raise ValueError(f'{label}: steps {report["steps"]} != {row["steps"]}')
    if report['setup_draws'] != row['setup_draws']:
        raise ValueError(f'{label}: setup draws differ')
    if report['step_draws'] != row['step_draws']:
        raise ValueError(f'{label}: step draws {report["step_draws"]} != {row["step_draws"]}')
    if report['setup_draws'] + report['step_draws'] != row['total_draws']:
        raise ValueError(f'{label}: total draws differ')
    if report['half_time'] != row['half_time']:
        raise ValueError(f'{label}: halftime step differs')
    if report['score'] != row['score']:
        raise ValueError(f'{label}: score {report["score"]} != {row["score"]}')
    goals = row['goals']
    if len(report['goals']) != len(goals):
        raise ValueError(f'{label}: goal count {len(report["goals"])} != {len(goals)}')
    logs = report['snapshot']['scorer_logs']
    for got, want in zip(report['goals'], goals):
        event = got['event']
        for field in ('minute', 'actor', 'side_flag'):
            if event[field] != want[field]:
                raise ValueError(f'{label}: goal {field} {event[field]} != {want[field]}')
        scorer = next((entry for entry in logs[want['side_flag']]
                       if entry['minute_byte'] == want['minute']), None)
        if scorer is None or scorer['player'] != want['player']:
            raise ValueError(f'{label}: scorer record at {want["minute"]} differs')
    for substitution in row['substitutions']:
        elapsed = substitution['elapsed']
        got = [event['arguments'] for step in report['timeline']
               if step['elapsed'] == elapsed
               for event in step['events'] if event['kind'] == 'replacement-log']
        if got != substitution['signatures']:
            raise ValueError(f'{label}: substitutions at minute {elapsed} differ')
    players = [row['player'] for row in report['snapshot']['players']]
    if players != row['scene_players']:
        raise ValueError(f'{label}: final scene players differ')
    return report


def replay(profile, row, report):
    print(f'{profile}: {report["score"]["goals"][0]}-{report["score"]["goals"][1]} '
          f'{report["score"]["clubs"]}, {report["steps"]} steps, '
          f'{report["setup_draws"]}+{report["step_draws"]} draws')
    logs = report['snapshot']['scorer_logs']
    for goal in report['goals']:
        event = goal['event']
        scorer = next((entry for entry in logs[event['side_flag']]
                       if entry['minute_byte'] == event['minute']), None)
        print(f'  goal {event["minute"]}\' side {event["side_flag"]} '
              f'player {scorer["player"] if scorer else "?"}')
    for substitution in row['substitutions']:
        print(f'  substitutions at {substitution["elapsed"]}\': {substitution["signatures"]}')
    print(f'  final scene players: {[row["player"] for row in report["snapshot"]["players"]]}')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--check', action='store_true')
    group.add_argument('--replay', action='store_true')
    args = parser.parse_args(argv)
    expected = load_expected()
    for profile in ('fresh', 'league'):
        row = expected['matches'][profile]
        report = compare(profile, row, play(profile, row))
        if args.replay:
            replay(profile, row, report)
    if args.check:
        print('PASS: the native quick match reproduces T49, T102 and T123')
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError) as error:
        print(f'quick-match replay failed: {error}', file=sys.stderr)
        sys.exit(1)
