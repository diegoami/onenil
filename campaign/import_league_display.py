#!/usr/bin/env python3
"""Pin the captured unplayed division tables and empty-results notice from the original evidence."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
COMMIT = '16029c0db43156c138cc115af0291c0251e479a9'
RECORDS = [
    ('runs/T49/session-r0002/step-0023.json', '05bdd59b542ab3c916ee8e65646187550adee53a1f4183df5c051594d03c76db'),
    ('runs/T49/session-r0002/step-0024.json', 'a6cf1686181ea917e04f7ba5456d983b4781819cacf6579bf0d5a9221e734ece'),
    ('runs/T49/session-r0002/step-0028.json', '92634f83fee48b4fea9cfb570555f2e02e3ade7463ef2d80d5a37ad55fc46afe'),
]
COLUMNS = ['P', 'HW', 'HD', 'HL', 'AW', 'AD', 'AL', 'F', 'A', 'PTS']
EMPTY = 'No league results to see : hit any key'
OUT = ROOT / 'campaign/fixtures/league-display.json'


def read(research, path):
    """The record's bytes at the pinned commit, checked against the pinned digest."""
    raw = subprocess.check_output(['git', 'show', f'{COMMIT}:{path}'], cwd=research)
    digest = dict(RECORDS)[path]
    assert hashlib.sha256(raw).hexdigest() == digest, path
    return json.loads(raw)


def parse_table(text):
    """Division title and the ordered club cells of a captured table page."""
    divisions = []
    for line in text.splitlines():
        stripped = line.strip()
        if stripped in ('PREMIER LEAGUE', 'DIVISION ONE'):
            continue
        if stripped.startswith('│') or '│' in line:
            cells = [cell.strip() for cell in line.split('│')]
            if len(cells) < 4 or not cells[1].isdigit():
                continue
            club = cells[2].rstrip('-').strip()
            if len(cells[1]) and 1 <= int(cells[1]) <= 16:
                divisions.append((int(cells[1]), club))
    return [club for _, club in sorted(divisions)]


def parse_division(text):
    title = next(line.strip() for line in text.splitlines()
                 if line.strip() in ('PREMIER LEAGUE', 'DIVISION ONE'))
    clubs = parse_table(text)
    assert len(clubs) == 16 and len(set(clubs)) == 16, (title, len(clubs))
    return dict(name=title, clubs=clubs)


def build(research):
    premier = read(research, RECORDS[0][0])
    division = read(research, RECORDS[1][0])
    empty = read(research, RECORDS[2][0])
    assert 'PREMIER LEAGUE' in premier['text'] and 'DIVISION ONE' in division['text']
    assert EMPTY in empty['text'], 'captured empty-results notice changed'
    divisions = [parse_division(premier['text']), parse_division(division['text'])]
    assert divisions[0]['clubs'][0] == 'Blackburn R'
    assert divisions[1]['clubs'][0] == 'Manchester City'
    value = dict(
        schema='league-display/1',
        source=dict(
            research_repository='https://github.com/diegoami/onenil-archaeology',
            research_commit=COMMIT,
            records=[dict(path=path, sha256=digest) for path, digest in RECORDS],
        ),
        columns=COLUMNS,
        divisions=divisions,
        empty_results=EMPTY,
    )
    raw = (json.dumps(value, indent=2) + '\n').encode()
    return raw


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--research-root', type=Path, required=True)
    parser.add_argument('--build', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    raw = build(args.research_root)
    if args.build:
        with OUT.open('xb') as handle:
            handle.write(raw)
    else:
        assert OUT.read_bytes() == raw, 'fixture differs from the captured divisions'
    print(f'PASS pinned unplayed divisions and empty-results notice at {COMMIT[:8]}')


if __name__ == '__main__':
    main()
