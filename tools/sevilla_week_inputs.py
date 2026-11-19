"""[confirmed:archaeology 788b833 T93] Export only the witnessed Sevilla week.

Source files and the exported receipt have independent fixed identities. Private
save bytes and checkpoint paths never enter this native fixture.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / 'campaign/fixtures/sevilla-week.json'
COMMIT = '788b8332db4a9aab8304f5d9abef80c091dde3ee'
FULL = 'runs/T93/sevilla-week-r0003/finish.json'
REPEAT = 'runs/T93/week-suffix-r0003/finish.json'
BOOT = 'records/T89/boot-r0001.json'
SOURCES = {
    FULL: '7d95a5e941c1d1268779626589315e741fa6854626a7cee9b3b12e1abe55edc9',
    REPEAT: 'a3e5b0269493a20f45bc01aecef4118af8bd57fae22fdfa7df7f882457e3d1b4',
    BOOT: '4fee511149ba1e1b08a28e60c24b2408ecf16a07da1445cf378c8f44cda109ba',
}
RECEIPT_SHA = 'fe51b0a36de636ca3ccb4bcd645395ce87a2183171fcdaddd5c7e9897d261990'

def encoded(value):
    return (json.dumps(value, indent=1, ensure_ascii=True) + '\n').encode()

def read_record(root, name):
    raw = (root / name).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCES[name]:
        raise ValueError(f'source identity differs: {name}')
    return json.loads(raw)

def verify_receipt(receipt):
    if hashlib.sha256(encoded(receipt)).hexdigest() != RECEIPT_SHA:
        raise ValueError('Sevilla week receipt identity differs')

def generate(root):
    full, repeat, boot = (read_record(root, name) for name in (FULL, REPEAT, BOOT))
    for record in (full, repeat):
        if record['status'] != 'completed' or record['error'] or record['unsupported']:
            raise ValueError('incomplete original route')
    result, restored = full['result'], repeat['result']
    week = result['week']
    if week != restored['week'] or result['stages'] != restored['stages']:
        raise ValueError('independent restored week differs')
    if week['conditions_before'] != week['conditions_after']:
        raise ValueError('unexpected post-caller refresh')
    preparation = {str(p['id']): p['availability']['condition_signed'] for p in boot['projected']['players']}
    if len(preparation) != 18 or sum(preparation[k] != v for k, v in week['conditions_after'].items()) != 10:
        raise ValueError('preparation-to-menu conditions differ')
    if boot['projected']['week_index'] != 2 or result['caller_boundary']['projection']['week_index'] != 2:
        raise ValueError('original preparation must already be week 2')
    stages = result['stages']
    cash_text = next(s['text'] for s in stages if 'CASH FLOW FOR' in s['text'])
    # Each screen row can contain a left and a right labelled field. Stop at
    # the next letter, stripping only the box-drawing cells around digits.
    def amount(label):
        tail = cash_text.split(label, 1)[1].split('\n', 1)[0]
        field = re.split('[A-Za-z]', tail, 1)[0]
        digits = ''.join(re.findall('[0-9]', field))
        if not digits:
            raise ValueError(f'missing cash field {label}')
        return int(digits)
    labels = {
        'gate_receipts': 'Gate receipts', 'prize_money': 'Prize money',
        'tv_coverage': 'TV coverage', 'radio_coverage': 'Radio coverage',
        'players_sold': 'Players sold', 'sponsorship': 'Sponsorship',
        'directors_handout': 'Directors hand-out', 'total_in': 'TOTAL IN',
        'player_salaries': 'Player salaries', 'win_bonuses': 'Win bonuses',
        'staff_salaries': 'Staff salaries', 'ground_maintenance': 'Ground maintenance',
        'players_bought': 'Players bought', 'contracts': 'Contracts',
        'interest': 'Interest', 'sundries': 'Sundries', 'total_out': 'TOTAL OUT',
        'after': 'Club funds', 'overdraft_facility': 'Overdraft facility',
    }
    cash = {'before': week['cash_before'], **{k: amount(v) for k, v in labels.items()}}
    if (cash['before'], cash['after'], cash['total_in'], cash['total_out']) != (421491, 421631, 69410, 69270):
        raise ValueError('cash movement differs')
    news = next(s['text'] for s in stages if 'TRANSFER' in s['text'] and 'NEWS' in s['text'])
    rows = []
    for line in news.splitlines():
        if '│' not in line:
            continue
        columns = re.split(r'\s{2,}', line.split('│', 1)[1].rsplit('│', 1)[0].strip())
        if len(columns) == 3 and any(c.isalpha() for c in columns[0]):
            rows.append(dict(zip(('player', 'from', 'to'), (c.rstrip('.-') for c in columns))))
    if len(rows) != 5:
        raise ValueError('transfer rows differ')
    fixture = week['next_fixture']
    if fixture != week['boundary_next_fixture'] or fixture != week['projection']['next_fixture']:
        raise ValueError('fixture projection differs')
    return {
        'schema': 'sevilla-week-receipt/1', 'research_commit': COMMIT, 'sources': SOURCES,
        'presentation': {
            'schema': 'friendly-week-receipt/1',
            'source_research': {'session': FULL, 'session_sha256': SOURCES[FULL],
                                'fixtures': REPEAT, 'fixtures_sha256': SOURCES[REPEAT]},
            'club': 130, 'manager': 1, 'week': {'index_before': 1, 'index_after': 2},
            'cash': cash, 'transfer_rows': rows,
            # The record decodes Celtic's club identity; it does not decode its manager.
            'following_fixture': {'competition': fixture['competition_code'],
                'opponent': fixture['opponent'], 'flags': fixture['flags'],
                'name': fixture['opponent_name'], 'manager': ''},
            'roster_unchanged': boot['projected']['club'] == week['projection']['club'],
            'interpretation': 'One witnessed week only; transfer rows are output, not executed rules.',
        },
        'following_fixture': fixture, 'conditions': week['conditions_after'],
        'original_save': next(f for f in week['files_written'] if f['name'] == 'SAVE1.1-0'),
    }

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', type=Path, required=True)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    receipt = generate(args.research_root)
    verify_receipt(receipt)
    if args.check:
        if OUTPUT.read_bytes() != encoded(receipt):
            raise ValueError('generated receipt differs')
        print('Sevilla week receipt authenticated')
    else:
        OUTPUT.write_bytes(encoded(receipt))

if __name__ == '__main__':
    main()
