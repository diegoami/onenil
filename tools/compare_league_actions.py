#!/usr/bin/env python3
"""Compare the native watched league match with the original's T123 record.

The original's T123 dispatcher actions and the native watched event stream are
different vocabularies: the original records seven dispatcher actions (kickoff,
goals, half time, substitutions, full time), while the native watched session
emits the per-action gameplay stream. Comparing them positionally always
reports a mismatch at action 0 and hides the real difference.

This tool aligns the comparable facts instead:

* the original's ``score_changes`` against the native ``tick_live_goals``;
* the original's goal and substitution actions against the native goal and
  substitution records;
* the dispatcher census (kickoff, goal, half time, substitution, full time).

It reports the first real divergence. ``--check`` asserts the divergence that
``docs/campaign-state.md`` documents, so the watched path has a regression gate
instead of a printout.

    python3 tools/compare_league_actions.py
    python3 tools/compare_league_actions.py --check
    python3 tools/compare_league_actions.py --actions <archaeology>/runs/T123/actions-r0001.json
"""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DEFAULT_ORIGINAL = ROOT / 'engine/tests/fixtures/league_watched/t123-actions.json'
ACTIONS_SHA256 = '3d1d772f689344ef5c3f99270882c048ab5195ffe3fb1dabdfe178b0e127bda3'
DISPATCH = {59: 'kickoff', 38: 'goal', 64: 'half time', 0: 'substitution', 63: 'full time'}

# The documented divergence (docs/campaign-state.md, "The watched league
# engine's divergence"), pinned so a change on either side fails --check.
EXPECTED_ORIGINAL_DISPATCH = {59: 1, 38: 2, 64: 1, 0: 2, 63: 1}
EXPECTED_NATIVE_DISPATCH = {59: 0, 38: 3, 64: 1, 0: 0, 63: 1}
EXPECTED_ORIGINAL_SCORES = [[1, 2], [1, 1], [2, 0]]
EXPECTED_ORIGINAL_SCORE_MINUTES = [29, 33, 46]
EXPECTED_ORIGINAL_FINAL = [2, 0]
EXPECTED_NATIVE_SCORES = [[1, 0], [1, 1], [1, 2]]
EXPECTED_NATIVE_SCORE_MINUTES = [56, 79, 83]
EXPECTED_NATIVE_FINAL = [2, 1]
EXPECTED_NATIVE_LIVE_GOALS = [1, 2]
EXPECTED_NATIVE_RETURN = '0AB9:EBE2'
EXPECTED_NATIVE_TICKS = 3880
EXPECTED_NATIVE_EVENTS = 839
EXPECTED_ORIGINAL_GOALS = [(29, 2429, 122, 1), (33, 2426, 122, 1)]
EXPECTED_NATIVE_GOALS = [(56, 2584, 130, 0), (79, 2426, 122, 1), (83, 2436, 122, 1)]
EXPECTED_ORIGINAL_SUBSTITUTIONS = [(57, 2434, 2427, 122, 1), (57, 2423, 2435, 122, 1)]
EXPECTED_NATIVE_SUBSTITUTIONS = []
EXPECTED_HALF_TIME = 46
EXPECTED_FULL_TIME = 91


def sha(data):
    return hashlib.sha256(data).hexdigest()


def native_match():
    from onenil_engine import NativePreparedWatchedSession
    prep = json.loads((ROOT / 'engine/tests/fixtures/league_preparation/admission.json').read_text())
    incoming = json.loads((ROOT / 'engine/tests/fixtures/league_watched/incoming.json').read_text())
    request = json.dumps({'preparation': prep['input'], 'support': incoming['support'],
                          'no_pending_key': True}, sort_keys=True, separators=(',', ':'))
    session = NativePreparedWatchedSession.league(request)
    out = json.loads(session.finish())
    if 'ok' not in out:
        raise SystemExit('native league session failed: ' + out.get('error', {}).get('message', '?'))
    return out['ok']


def load_original(path):
    record = json.loads(path.read_text())
    if record.get('schema') not in ('league-action-stream/1', 'league-watched-actions/1'):
        raise SystemExit('not a T123 action stream')
    if sha(json.dumps(record['actions'], sort_keys=True, separators=(',', ':')).encode()) != record['actions_sha256']:
        raise SystemExit('the action digest differs')
    if sha(json.dumps(record['actions'], sort_keys=True, separators=(',', ':')).encode()) != ACTIONS_SHA256:
        raise SystemExit('the action stream is not the pinned T123 record')
    if record['action_count'] != len(record['actions']):
        raise SystemExit('the action census differs')
    if not record['score_changes'] or record['score_changes'][-1]['score'] != EXPECTED_ORIGINAL_FINAL:
        raise SystemExit('the original record does not end 2-0')
    return record


def census(actions):
    counts = {}
    for action in actions:
        counts[action['code']] = counts.get(action['code'], 0) + 1
    return counts


def dispatcher_census(actions):
    counts = census(actions)
    return {code: counts.get(code, 0) for code in DISPATCH}


def goal_records(actions):
    return [(a['minute'], a['player'], a['club'], a['side']) for a in actions if a['code'] == 38]


def substitution_records(actions):
    return [(a['minute'], a['player'], a['secondary'], a['club'], a['side'])
            for a in actions if a['code'] == 0]


def first_event(actions, code):
    for action in actions:
        if action['code'] == code:
            return action
    return None


def score_sequence(native):
    """The native score at every change, with the tick minute it happened."""
    changes = []
    previous = [0, 0]
    for tick, score in enumerate(native['tick_live_goals']):
        if score != previous:
            changes.append({'minute': native['ticks'][tick]['after']['elapsed'], 'score': score})
            previous = score
    return changes


def describe(record):
    rows = ' / '.join(f"{change['minute']}' {change['score']}" for change in record)
    return rows


def first_difference(theirs, ours):
    for i in range(min(len(theirs), len(ours))):
        if theirs[i] != ours[i]:
            return i, theirs[i], ours[i]
    if len(theirs) != len(ours):
        index = min(len(theirs), len(ours))
        return index, (theirs[index:] if index < len(theirs) else None), (
            ours[index:] if index < len(ours) else None)
    return None


def compare(original, native):
    theirs = original['actions']
    ours = native['events']
    original_scores = original['score_changes']
    native_scores = score_sequence(native)
    original_goals = goal_records(theirs)
    native_goals = goal_records(ours)
    original_subs = substitution_records(theirs)
    native_subs = substitution_records(ours)

    print('dispatcher census: original', len(theirs), 'actions, native', len(ours), 'events')
    counts = census(theirs)
    native_counts = census(ours)
    for code, name in DISPATCH.items():
        print(f'  code {code:2d} {name:12s} original {counts.get(code, 0):2d} native {native_counts.get(code, 0)}')

    print('\nscore sequence (tick_live_goals against score_changes)')
    print('  opening state: both [0, 0]')
    print('  original', describe(original_scores))
    print('  native  ', describe(native_scores))
    print('  final scores: original', original_scores[-1]['score'],
          'native', native['final']['scores'],
          f"(live_goals {native['live_goals']}, tick_live_goals ends {native_scores[-1]['score']})")
    score_difference = first_difference([c['score'] for c in original_scores],
                                        [c['score'] for c in native_scores])
    if score_difference is None:
        print('  the score sequences agree')
    else:
        index, their_score, our_score = score_difference
        their_minute = original_scores[index]['minute'] if index < len(original_scores) else 'none'
        our_minute = native_scores[index]['minute'] if index < len(native_scores) else 'none'
        print(f'first real divergence after the shared opening: score change {index}: '
              f"original {their_score} at {their_minute}' against native {our_score} at {our_minute}'")

    print('\ngoal records')
    print('  original', original_goals or 'none')
    print('  native  ', native_goals or 'none')
    goal_difference = first_difference(original_goals, native_goals)
    if goal_difference is None:
        print('  the goal records agree')
    else:
        index, theirs_row, ours_row = goal_difference
        print(f'  first goal difference: goal {index}: original {theirs_row} against native {ours_row}')

    print('\nsubstitution records')
    print('  original', original_subs or 'none')
    print('  native  ', native_subs or 'none')
    sub_difference = first_difference(original_subs, native_subs)
    if sub_difference is None:
        print('  the substitution records agree')
    else:
        index, theirs_row, ours_row = sub_difference
        if theirs_row is None:
            print(f'  native has {len(native_subs)} substitution records the original does not')
        elif ours_row is None:
            print(f'  original has {len(original_subs)} substitution records the native does not')
        else:
            print(f'  first substitution difference: substitution {index}: original {theirs_row} '
                  f'against native {ours_row}')

    print('\nphase records')
    for code, label in ((64, 'half time'), (63, 'full time')):
        theirs_row = first_event(theirs, code)
        ours_row = first_event(ours, code)
        print(f'  {label}: original {theirs_row["minute"] if theirs_row else "none"}\' '
              f'native {ours_row["minute"] if ours_row else "none"}\'')

    print('\nnative identity: return', native['return'], 'ticks', len(native['ticks']),
          'events', len(ours))
    diverged = (score_difference is not None or goal_difference is not None
                or sub_difference is not None
                or native['final']['scores'] != EXPECTED_ORIGINAL_FINAL)
    return 1 if diverged else 0


def check(original, native):
    original_scores = original['score_changes']
    native_scores = score_sequence(native)
    counts = dispatcher_census(original['actions'])
    native_counts = dispatcher_census(native['events'])
    original_goals = goal_records(original['actions'])
    native_goals = goal_records(native['events'])
    original_subs = substitution_records(original['actions'])
    native_subs = substitution_records(native['events'])
    original_half = first_event(original['actions'], 64)
    native_half = first_event(native['events'], 64)
    original_full = first_event(original['actions'], 63)
    native_full = first_event(native['events'], 63)

    checks = [
        ('original dispatcher census', counts, EXPECTED_ORIGINAL_DISPATCH),
        ('native dispatcher census', native_counts, EXPECTED_NATIVE_DISPATCH),
        ('original score sequence', [c['score'] for c in original_scores], EXPECTED_ORIGINAL_SCORES),
        ('original score minutes', [c['minute'] for c in original_scores], EXPECTED_ORIGINAL_SCORE_MINUTES),
        ('original final score', original_scores[-1]['score'], EXPECTED_ORIGINAL_FINAL),
        ('native score sequence', [c['score'] for c in native_scores], EXPECTED_NATIVE_SCORES),
        ('native score minutes', [c['minute'] for c in native_scores], EXPECTED_NATIVE_SCORE_MINUTES),
        ('native final score', native['final']['scores'], EXPECTED_NATIVE_FINAL),
        ('native live goals', native['live_goals'], EXPECTED_NATIVE_LIVE_GOALS),
        ('native return', native['return'], EXPECTED_NATIVE_RETURN),
        ('native ticks', len(native['ticks']), EXPECTED_NATIVE_TICKS),
        ('native events', len(native['events']), EXPECTED_NATIVE_EVENTS),
        ('original goal records', original_goals, EXPECTED_ORIGINAL_GOALS),
        ('native goal records', native_goals, EXPECTED_NATIVE_GOALS),
        ('original substitution records', original_subs, EXPECTED_ORIGINAL_SUBSTITUTIONS),
        ('native substitution records', native_subs, EXPECTED_NATIVE_SUBSTITUTIONS),
        ('original half time', original_half['minute'] if original_half else None, EXPECTED_HALF_TIME),
        ('native half time', native_half['minute'] if native_half else None, EXPECTED_HALF_TIME),
        ('original full time', original_full['minute'] if original_full else None, EXPECTED_FULL_TIME),
        ('native full time', native_full['minute'] if native_full else None, EXPECTED_FULL_TIME),
    ]
    for label, got, want in checks:
        if got != want:
            raise SystemExit(f'{label}: {got} against the documented {want}')
    print('PASS: the watched league divergence is the documented one: '
          f'original {EXPECTED_ORIGINAL_FINAL[0]}-{EXPECTED_ORIGINAL_FINAL[1]} '
          f'against native {EXPECTED_NATIVE_FINAL[0]}-{EXPECTED_NATIVE_FINAL[1]}, '
          f'first at score change 0 ({EXPECTED_ORIGINAL_SCORES[0]} at '
          f"{EXPECTED_ORIGINAL_SCORE_MINUTES[0]}' against {EXPECTED_NATIVE_SCORES[0]} at "
          f"{EXPECTED_NATIVE_SCORE_MINUTES[0]}')")


def raw_events(original, native):
    theirs = original['actions']
    ours = native['events']
    print('raw-event diagnostic (not the alignment): original', len(theirs),
          'dispatcher actions against native', len(ours), 'action events')
    limit = min(len(theirs), len(ours))
    for i in range(limit):
        if theirs[i].get('code') != ours[i].get('code'):
            print('  first raw difference at position', i,
                  'original code', theirs[i].get('code'), 'native code', ours[i].get('code'))
            return
    print('  no raw position agrees before one stream ends')


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--actions', type=Path, default=DEFAULT_ORIGINAL,
                        help='the T123 action record (defaults to the committed fixture)')
    parser.add_argument('--check', action='store_true',
                        help='assert the documented divergence instead of reporting it')
    parser.add_argument('--raw-events', action='store_true',
                        help='also print the separate positional event diagnostic')
    args = parser.parse_args()
    original = load_original(args.actions)
    native = native_match()
    if args.check:
        check(original, native)
        return
    status = compare(original, native)
    if args.raw_events:
        raw_events(original, native)
    raise SystemExit(status)


if __name__ == '__main__':
    main()
