#!/usr/bin/env python3
"""T119: authenticate the real-X11 league quick route and its week."""
import argparse
import hashlib
import json
import re
from pathlib import Path

from capture_lock import capture_lock_digest

ROOT = Path(__file__).resolve().parents[2]
SAVE = ROOT / 'desktop/faithful/fixtures/league-week28.json'
SAVE_SHA = '8f97f0ad04f6e1943483f0f93b0e63fb28f4c2124d8f315645e6d6060db213e3'
STEPS = ['league-main', 'configuration', 'watch-off', 'main-after-configuration',
         'preparation', 'half-time', 'full-time', 'cash-flow', 'transfer-news',
         'management-after-week']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def check(path):
    record = json.loads(path.read_text())
    assert record['schema'] == 'league-quick-window-check/1'
    assert record['status'] == 'passed' and record['error'] is None and not record['stderr']
    assert re.fullmatch(r'[0-9a-f]{40}', record['producer'])
    capture_lock_digest(record)
    assert re.fullmatch(r'[0-9a-f]{64}', record['binary_sha256'])
    assert re.fullmatch(r'[0-9a-f]{64}', record['font_sha256'])
    assert sha(SAVE.read_bytes()) == SAVE_SHA == record['save_sha256']
    assert [step['name'] for step in record['steps']] == STEPS
    steps = {step['name']: step['state'] for step in record['steps']}

    main = steps['league-main']
    assert main['screen'] == 'Main' and main['watch_match'] is True
    campaign = main['campaign']
    assert campaign['week_index'] == 54 and campaign['cash'] == 1_045_060
    assert campaign['next_fixture']['slot'] == 55
    assert campaign['next_fixture']['opponent_name'] == 'Chelsea'
    assert campaign.get('league_played') is None

    assert steps['configuration']['screen'] == 'Configuration'
    assert steps['configuration']['watch_match'] is True
    assert steps['watch-off']['watch_match'] is False
    assert steps['main-after-configuration']['screen'] == 'Main'
    assert steps['preparation']['screen'] == 'Preparation'
    assert steps['preparation']['watch_match'] is False

    half = steps['half-time']
    assert half['screen'] == 'HalfTime'
    quick = half['quick']
    assert quick['stage'] == 'HALF TIME' and quick['score'] == [2, 0]
    assert quick['clubs'] == [122, 130] and quick['attendance'] == 36_986
    assert quick['lines'] == ['**** GOAL **** by R Fox', '**** GOAL **** by S Howey',
                              '---- HALF TIME ----']
    assert [(s['name'], s['minute']) for s in quick['scorers']] == [('R Fox', 29), ('S Howey', 33)]

    full = steps['full-time']
    assert full['screen'] == 'FullTime'
    quick = full['quick']
    assert quick['stage'] == 'FULL TIME' and quick['score'] == [2, 0]
    assert quick['lines'] == ['**** GOAL **** by R Fox', '**** GOAL **** by S Howey',
                              'Clark on for Venison', 'Watson on for Albert',
                              '---- FULL TIME ----']

    cash = steps['cash-flow']
    assert cash['screen'] == 'CashFlow'
    campaign = cash['campaign']
    assert campaign['cash'] == 1_691_393 and campaign['week_index'] == 54
    assert campaign['next_fixture']['slot'] == 55
    assert campaign['next_fixture']['opponent_name'] == 'Chelsea'
    played = campaign['league_played']
    assert played['score'] == [0, 2] and played['prepared']['name'] == 'Newcastle Utd'
    assert steps['transfer-news']['screen'] == 'TransferNews'
    assert steps['transfer-news']['campaign'] == campaign
    end = steps['management-after-week']
    assert end['screen'] == 'Main' and end['campaign'] == campaign
    assert 'Week 54 complete' in end['message'] and 'Chelsea' in end['message']

    assert record['screenshots']
    for shot in record['screenshots']:
        image = ROOT / 'records/T119' / shot['path']
        assert image.exists() and sha(image.read_bytes()) == shot['sha256']
        assert shot['size'][0] > 0 and shot['size'][1] > 0
    print('T119 league quick route: configuration, halftime 2-0, full time, cash 1,691,393: ready')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('record', type=Path)
    args = parser.parse_args()
    check(args.record)


if __name__ == '__main__':
    main()
