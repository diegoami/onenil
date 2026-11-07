#!/usr/bin/env python3
"""Authenticate the actual campaign window, named actions and unmodified campaign."""
import argparse
import copy
import hashlib
import json
import struct
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def check(record):
    assert record['schema'] == 'native-campaign-match-window/1'
    assert record['status'] == 'passed' and not record['error'] and not record['x11_errors']
    assert record['cancelled_and_closed_workers_reaped'] is True
    assert record['font_sha256'] == 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
    assert record['names_sha256'] == digest((ROOT / 'desktop/faithful/assets/campaign-match-names.json').read_bytes())
    assert record['runtime']['window_backend'] == 'software/minifb'
    source = json.loads((ROOT / 'campaign/fixtures/research-seed.json').read_text())['initial']
    steps = {step['name']: step['state'] for step in record['steps']}
    assert len(steps) == len(record['steps']) == 20
    initial = steps['initial-campaign']['campaign']
    projected = copy.deepcopy(initial)
    for name in ('origin', 'version', 'revision'):
        projected.pop(name)
    projected['schema'] = 'original-club-seed/1'
    for name in ('first_team', 'reserves'):
        projected['club'][name] = [identifier or 0 for identifier in projected['club'][name]]
    assert projected == source and initial['revision'] == 0
    edited = copy.deepcopy(initial)
    edited['club']['first_team'][:2] = [2582, 2581]
    edited['revision'] = 1
    for step in record['steps']:
        expected = edited if step['name'] in ('changed-xi', 'edited-main', 'edited-admission-refuses') else initial
        assert step['state']['campaign'] == expected, step['name']
    assert steps['cancel-preserves-campaign']['screen'] == 'Preparation'
    assert steps['fresh-match-pitch']['screen'] == 'Pitch'
    assert steps['fresh-match-pitch']['players'] == 22
    assert steps['fresh-match-pitch']['match_clubs'] == ['Blackburn R', 'P St Germain']
    assert not steps['fresh-match-pitch']['exhibition']
    assert steps['half-time']['phase'] == 'HalfTime'
    assert steps['half-time-bench']['screen'] == 'Bench'
    final = steps['full-time']
    assert final['screen'] == 'FullTime' and final['phase'] == 'FullTime'
    assert (final['frames'], final['events'], final['score']) == (3825, 749, [1, 4])
    assert 'not been saved' in steps['result-not-posted']['message']
    assert steps['edited-admission-refuses']['screen'] == 'Preparation'
    assert steps['edited-admission-refuses']['message']
    names = json.loads((ROOT / 'desktop/faithful/assets/campaign-match-names.json').read_text())['players']
    messages = json.loads((ROOT / 'desktop/faithful/messages.json').read_text())
    actions = steps['detailed-commentary']['recent_actions']
    assert len(actions) == 8
    for action in actions:
        event = action['native']
        text = messages[event['code']][1]
        for marker, field in (('{p}', 'player'), ('{s}', 'secondary')):
            if marker in text:
                text = text.replace(marker, names[str(event[field])])
        assert action['text'] == text and 'Play continues' not in text


def verify(path):
    record = json.loads(path.read_text())
    check(record)
    produced_lock = subprocess.check_output(['git', 'show', record['producer'] + ':desktop/faithful/Cargo.lock'], cwd=ROOT)
    assert digest(produced_lock) == record['runtime']['cargo_lock_sha256']
    seen = set()
    for shot in record['screenshots']:
        assert Path(shot['path']).name == shot['path'] and shot['name'] not in seen
        seen.add(shot['name'])
        data = (path.parent / shot['path']).read_bytes()
        assert data[:8] == b'\x89PNG\r\n\x1a\n'
        assert list(struct.unpack('>II', data[16:24])) == shot['size'] == [1440, 900]
        assert digest(data) == shot['sha256']
    assert seen == {'main', 'preparation', 'pitch-actions', 'commentary', 'bench', 'half-time', 'full-time', 'refused-edit'}
    for damage in ('score', 'posted', 'clubs', 'action', 'namespace', 'worker'):
        bad = copy.deepcopy(record)
        steps = {step['name']: step['state'] for step in bad['steps']}
        if damage == 'score': steps['full-time']['score'] = [4, 1]
        elif damage == 'posted': steps['completed-save']['campaign']['week_index'] = 1
        elif damage == 'clubs': steps['fresh-match-pitch']['match_clubs'][1] = 'Sheffield Wed'
        elif damage == 'action': steps['detailed-commentary']['recent_actions'][0]['text'] = 'Play continues'
        elif damage == 'namespace': bad['names_sha256'] = '0' * 64
        else: bad['cancelled_and_closed_workers_reaped'] = False
        try:
            check(bad)
        except AssertionError:
            pass
        else:
            raise AssertionError('corrupt campaign match record accepted: ' + damage)
    print('PASS 20 native keyboard steps,8 screenshots,6 corruptions;actual clubs/actions and no result posting')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('record', type=Path)
    verify(parser.parse_args().record)
