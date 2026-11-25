#!/usr/bin/env python3
"""T117: compare the native league action stream with the original's T123 record.

The archaeology T123 record lists every original action of the week-28 fixture
(code, player, secondary, side, club, minute, queue, draw interval, seed); this
tool runs the native league session, aligns the two streams and reports the
first divergence, or the full parity when there is none.

    python3 tools/compare_league_actions.py --actions <archaeology>/runs/T123/actions-r0001.json
"""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIELDS = ('code', 'player', 'secondary', 'side', 'club')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def native_actions():
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
    if record.get('schema') != 'league-action-stream/1':
        raise SystemExit('not a T123 action stream')
    if sha(json.dumps(record['actions'], sort_keys=True, separators=(',', ':')).encode()) != record['actions_sha256']:
        raise SystemExit('the action digest differs')
    return record


def compare(original, native):
    theirs = original['actions']
    ours = native['events']
    print('original actions', len(theirs), 'native events', len(ours))
    if original['score_changes'] and original['score_changes'][-1]['score'] != [2, 0]:
        raise SystemExit('the original record does not end 2-0')
    limit = min(len(theirs), len(ours))
    for i in range(limit):
        a, b = theirs[i], ours[i]
        diff = {k: (a.get(k), b.get(k)) for k in FIELDS if a.get(k) != b.get(k)}
        if diff:
            print('first divergence at action', i, diff)
            for j in range(max(0, i - 3), min(limit, i + 3)):
                print(' ', j, 'original', {k: theirs[j].get(k) for k in FIELDS},
                      'native', {k: ours[j].get(k) for k in FIELDS})
            return 1
    if len(theirs) != len(ours):
        print('action counts differ:', len(theirs), 'against', len(ours))
        return 1
    print('all', limit, 'actions agree on', FIELDS)
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--actions', type=Path, required=True)
    args = parser.parse_args()
    original = load_original(args.actions)
    native = native_actions()
    raise SystemExit(compare(original, native))


if __name__ == '__main__':
    main()
