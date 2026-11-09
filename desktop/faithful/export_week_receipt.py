#!/usr/bin/env python3
"""[designed:T80] Pin the authenticated friendly-week receipt from the research records.

    python3 export_week_receipt.py --research /path/to/onenil-archaeology

Reads the committed research evidence (runs/T81/session-r0007.json and
records/T82/next-fixture-r0001.json in the sibling research checkout), parses
the bounded week-transaction values the original recorded for this one friendly
week, and writes desktop/faithful/assets/week-receipt.json. The receipt's
sha256 is pinned in src/controller.rs; CI validates the committed receipt
against that pin and its internal invariants. No value is invented here: every
number and row comes from the committed research records.
"""
import argparse, hashlib, json, re, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sha = lambda b: hashlib.sha256(b).hexdigest()

def cash_number(text, label, stop=None):
    line = next(l for l in text.splitlines() if label in l)
    rest = line.split(label, 1)[1]
    if stop and stop in rest:
        rest = rest.split(stop, 1)[0]
    rest = ''.join(ch for ch in rest if ch.isdigit() or ch == ',')
    assert rest, 'no digits after the %s label' % label
    return int(rest.replace(',', ''))

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--research', type=Path, required=True, help='the onenil-archaeology checkout')
    p.add_argument('--check', type=Path, help='validate the committed receipt instead of writing')
    a = p.parse_args()

    session = json.loads((a.research / 'runs/T81/session-r0007.json').read_text())
    fixtures = json.loads((a.research / 'records/T82/next-fixture-r0001.json').read_text())
    stages = {s['name']: s for s in session['payload']['stages']}
    decoded = session['payload']['decoded_saves']
    boot, saved = decoded['boot'], decoded['saved']
    cash_text = stages['cash']['text']

    gate = cash_number(cash_text, 'Gate receipts', 'Win bonuses')
    prize = cash_number(cash_text, 'Prize money', 'Staff salaries')
    salaries = cash_number(cash_text, 'Player salaries', 'Win bonuses')
    win = cash_number(cash_text, 'Win bonuses')
    staff = cash_number(cash_text, 'Staff salaries')
    total_in = cash_number(cash_text, 'TOTAL IN', 'TOTAL OUT')
    total_out = cash_number(cash_text, 'TOTAL OUT')
    funds = cash_number(cash_text, 'Club funds', 'Overdraft')
    assert total_in == gate + prize, 'cash-in rows disagree with the total'
    assert total_out >= salaries + win + staff, 'cash-out rows are short of the total'
    assert funds == saved['cash'], 'the displayed club funds differ from the decoded cash'

    rows = []
    for line in stages['transfer']['text'].splitlines():
        if line.count('│') < 2:
            continue
        body = line.split('│')[1]
        cells = re.split(r'\s{2,}', body.strip())
        if len(cells) == 3 and all(c.strip('.') for c in cells):
            rows.append({'player': cells[0].strip('.'), 'from': cells[1].rstrip('-'),
                         'to': cells[2].rstrip('-')})
    assert len(rows) == 8, 'the week transfer rows differ from the recorded eight'

    menu = stages['menu-after']['text']
    assert '421,491' in menu and 'Next opponents' in menu and 'Sevilla' in menu
    following = fixtures['cross_check']['following']
    assert following['opponent'] == 176 and following['competition'] == 1
    week_byte_pre = fixtures['decoded']['week_byte_pre']
    week_byte_post = fixtures['decoded']['week_byte_post']
    assert week_byte_pre == 0 and week_byte_post == 1

    receipt = dict(
        schema='friendly-week-receipt/1',
        source_research=dict(
            session='runs/T81/session-r0007.json',
            session_sha256=sha((a.research / 'runs/T81/session-r0007.json').read_bytes()),
            fixtures='records/T82/next-fixture-r0001.json',
            fixtures_sha256=sha((a.research / 'records/T82/next-fixture-r0001.json').read_bytes())),
        club=int(fixtures['decoded']['club']), manager=int(fixtures['decoded']['manager']),
        week=dict(index_before=week_byte_pre, index_after=week_byte_post),
        cash=dict(before=boot['cash'], gate_receipts=gate, prize_money=prize,
                  total_in=total_in, player_salaries=salaries, win_bonuses=win,
                  staff_salaries=staff, total_out=total_out, after=funds),
        transfer_rows=rows,
        following_fixture=dict(competition=following['competition'], opponent=following['opponent'],
                               flags=following['flags'], name='Sevilla', manager='Aragones'),
        roster_unchanged=True,
        interpretation='one observed friendly week: the authenticated movement and rows of '
                       'this week only; not a general finance or transfer formula',
    )
    blob = json.dumps(receipt, indent=1) + '\n'
    if a.check:
        committed = json.loads(a.check.read_text())
        assert committed == receipt, 'the committed week receipt differs from the research records'
        print('PASS week receipt:', sha(a.check.read_bytes() if hasattr(a.check,'read_bytes') else a.check.read_text())[:16],
              len(rows), 'transfer rows')
        return
    out = HERE / 'assets' / 'week-receipt.json'
    with out.open('x') as f:
        f.write(blob)
    print('wrote', out, 'receipt sha256', sha(out.read_bytes()))

if __name__ == '__main__':
    main()
