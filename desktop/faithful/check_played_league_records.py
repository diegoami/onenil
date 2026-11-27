#!/usr/bin/env python3
"""T120: authenticate the real-X11 played league table and results route."""
import argparse
import hashlib
import json
import re
from pathlib import Path

from capture_lock import capture_lock_digest

ROOT = Path(__file__).resolve().parents[2]
PLAYED = ROOT / 'desktop/faithful/fixtures/league-week28-played.json'
PRE = ROOT / 'desktop/faithful/fixtures/league-week28.json'
STEPS = ['played-main', 'played-tables', 'division-one', 'main-after-tables',
         'played-results', 'main-after-results', 'unplayed-main', 'unplayed-tables',
         'main-before-notice', 'unplayed-notice']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def check(path):
    record = json.loads(path.read_text())
    assert record['schema'] == 'played-league-window-check/1'
    assert record['status'] == 'passed' and record['error'] is None and not record['stderr']
    assert re.fullmatch(r'[0-9a-f]{40}', record['producer'])
    capture_lock_digest(record)
    assert sha(PLAYED.read_bytes()) == record['saves']['played']
    assert sha(PRE.read_bytes()) == record['saves']['pre']
    assert [step['name'] for step in record['steps']] == STEPS
    steps = {step['name']: step['state'] for step in record['steps']}

    main = steps['played-main']
    assert main['screen'] == 'Main'
    played = main['league_played']
    assert played is not None
    rows = played['rows']
    assert len(rows) == 16
    assert [row['position'] for row in rows] == list(range(1, 17))
    assert rows[0]['club'] == 'Newcastle Utd' and rows[0]['row']['points'] == 45
    blackburn = rows[1]
    assert blackburn['club'] == 'Blackburn R' and blackburn['position'] == 2
    row = blackburn['row']
    assert row['played'] == 28
    assert row['home'] == [8, 3, 3] and row['away'] == [9, 1, 4]
    assert row['goals_for'] == 60 and row['goals_against'] == 35 and row['points'] == 38
    results = played['results']
    assert len(results) == 8
    assert any(score['home'] == 'Newcastle Utd' and score['away'] == 'Blackburn R'
               and score['home_goals'] == 2 and score['away_goals'] == 0
               for score in results)

    assert steps['played-tables']['screen'] == 'LeagueTables'
    assert steps['played-tables']['league_division'] == 0
    assert steps['division-one']['screen'] == 'LeagueTables'
    assert steps['division-one']['league_division'] == 1
    assert steps['main-after-tables']['screen'] == 'Main'
    assert steps['played-results']['screen'] == 'LeagueResults'
    assert steps['main-after-results']['screen'] == 'Main'

    assert steps['unplayed-main']['screen'] == 'Main'
    assert steps['unplayed-main']['league_played'] is None
    assert steps['unplayed-tables']['screen'] == 'LeagueTables'
    assert steps['unplayed-tables']['league_division'] == 0
    assert steps['unplayed-notice']['screen'] == 'Main'
    assert steps['unplayed-notice']['notice'] is True
    assert 'No league results' in steps['unplayed-notice']['message']

    assert record['screenshots']
    for shot in record['screenshots']:
        image = ROOT / 'records/T120' / shot['path']
        assert image.exists() and sha(image.read_bytes()) == shot['sha256']
    print('T120 played league table (Blackburn 2nd, P28, 38) and the eight results: ready')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('record', type=Path)
    args = parser.parse_args()
    check(args.record)


if __name__ == '__main__':
    main()
