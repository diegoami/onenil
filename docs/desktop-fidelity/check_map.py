#!/usr/bin/env python3
"""[designed] Check proposed desktop coverage against the pinned public manual."""
import argparse
import copy
import hashlib
import json
import re
import subprocess
from datetime import datetime, timezone
from functools import lru_cache
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PIN = '6aca98caeebc1b9b5410852139fa305462224a05'


@lru_cache(maxsize=None)
def read_at(research, name):
    return subprocess.check_output(['git', '-C', str(research), 'show', f'{PIN}:{name}'])


def source_tables(research):
    text = read_at(research, 'docs/manual/coverage-audit.md').decode()
    menu = []
    for line in text.splitlines():
        cells = [c.strip() for c in line.split('|')[1:-1]]
        if cells and re.fullmatch(r'\d+', cells[0]):
            menu.append(dict(slot=int(cells[0]), action=cells[1], evidence=cells[2], boundary=cells[3]))
    gaps = []
    for line in text.splitlines():
        cells = [c.strip() for c in line.split('|')[1:-1]]
        if cells and re.match(r'G\d\d ', cells[0]):
            gaps.append(dict(id=cells[0][:3], title=cells[0][4:], observed=cells[1],
                             unvisited=cells[2], next_scenario=cells[3]))
    return menu, gaps


def markdown(data):
    ref = f'https://github.com/diegoami/onenil-archaeology/blob/{PIN}/'
    lines = ['# Original-to-desktop screen map', '',
             '[designed] Proposed screen coverage, not implemented features or approved departures.',
             'Generated from [screen-map.json](screen-map.json) by [check_map.py](check_map.py).',
             'The original manual remains the exact procedure/key/record authority. Families below',
             'are compositions and variants, not interchangeable controls or ordered state machines.', '',
             f'[confirmed] Research source: `{PIN}`;83 workflows,18 menu slots and14 explicit gap groups.', '',
             '## Screen families', '']
    for family in data['families']:
        lines += [f"### {family['id']} · {family['name']}", '',
                  '[designed] '+family['composition'], '',
                  '**Variants:** '+ '; '.join(family['variants'])+'.', '',
                  '**Input/return contract:** '+family['input_return'], '',
                  '**State boundary:** '+family['state_boundary'], '',
                  '**Unknown groups:** '+', '.join(family['gaps'])+'.', '']
    lines += ['## Every documented workflow', '',
              '[derived] Each row preserves the catalog title and complete original screenshot/record',
              'set in the JSON. The source chapter supplies exact keys and order; a family is reused',
              'only for its presentation, never to generalize cancellation or persistent effects.', '',
              '| Workflow / original procedure | Desktop families | Variant anchors | Transaction requirement |',
              '| --- | --- | --- | --- |']
    for row in data['workflows']:
        lines.append(f"| [{row['id']} {row['title']}]({ref}{row['chapter']}) | {', '.join(row['families'])} | {'; '.join(row['variants'])} | {row['boundary_note']} |")
    lines += ['', '## All executable main-menu slots', '',
              '[derived from the accepted audit] Labels and observed/static boundaries retain their source;',
              'the desktop destination is proposed. Previous player is visible/static only.', '',
              '| Slot | Original action | Desktop destination | Original coverage boundary |',
              '| --- | --- | --- | --- |']
    for row in data['menus']:
        lines.append(f"| {row['slot']} | {row['action']} | {', '.join(row['families'])} | {row['boundary']} |")
    lines += ['', '## Unvisited and conditional branches', '',
              '[confirmed] These remain evidence gaps, not implemented/observed desktop features.', '',
              '| Group | Unvisited original boundary | Targeted follow-up |', '| --- | --- | --- |']
    for row in data['gaps']:
        lines.append(f"| {row['id']} {row['title']} | {row['unvisited']} | {row['next_scenario']} |")
    lines += ['', '## Next', '',
              '[designed] T51 creates original/proposed visual comparisons for the first families.',
              'Then a representative desktop feasibility task proves rendering, keys, save ownership',
              'and Rust event/decision/result delivery. Owner toolkit selection follows on #103.',
              'The map itself neither runs the game nor approves a visual or control departure.', '']
    return '\n'.join(lines)


def validate(data, research, check_markdown=True):
    assert data['schema'] == 'original-desktop-map/1'
    assert data['research_commit'] == PIN
    assert data['status'] == 'proposed-design'
    raw = read_at(research, 'runs/T62/catalog-r0001.json')
    assert hashlib.sha256(raw).hexdigest() == data['catalog_sha256']
    catalog = json.loads(raw)
    want = {row['id']: row for row in catalog['workflows']}
    rows = data['workflows']
    assert len(rows) == len(want) == 83
    assert {row['id'] for row in rows} == set(want)
    assert len({row['id'] for row in rows}) == len(rows), 'duplicate workflow'
    family_ids = {row['id'] for row in data['families']}
    assert len(family_ids) == len(data['families']), 'duplicate family'
    menus, gaps = source_tables(research)
    assert len(menus) == 18 and len(gaps) == 14
    assert {g['id'] for g in gaps} == {f'G{n:02}' for n in range(1, 15)}
    assert data['gaps'] == gaps
    for row in data['families']:
        for field in ('name', 'composition', 'variants', 'input_return', 'state_boundary', 'gaps'):
            assert row[field], (row['id'], field)
        assert set(row['gaps']) <= {g['id'] for g in gaps}
    images = set()
    for row in rows:
        original = want[row['id']]
        assert row['title'] == original['title'] and row['chapter'] == original['chapter']
        assert row['evidence'] == original['screenshots'], f"changed evidence for {row['id']}"
        assert row['families'] and set(row['families']) <= family_ids
        assert len(row['families']) == len(set(row['families']))
        assert row['boundary_note'] and row['variants']
        assert hashlib.sha256(read_at(research, row['chapter'])).hexdigest() == row['chapter_sha256']
        for item in row['evidence']:
            read_at(research, item['record'])
            if item['screenshot'] not in images:
                assert hashlib.sha256(read_at(research, item['screenshot'])).hexdigest() == item['screenshot_sha256']
                images.add(item['screenshot'])
    assert len(data['menus']) == len(menus)
    for original, row in zip(menus, data['menus']):
        assert all(row[key] == value for key, value in original.items())
        assert row['families'] and set(row['families']) <= family_ids
        expected = 'visible-static-only' if 'Visible/static only' in original['boundary'] else 'observed-with-limits'
        assert row['observation'] == expected
    used = {family for row in rows for family in row['families']}
    assert used == family_ids, 'unmapped/orphan screen family'
    if check_markdown:
        assert (HERE/'screen-map.md').read_text() == markdown(data), 'readable map drift'
    return dict(workflows=len(rows), menus=len(menus), families=len(family_ids),
                gaps=len(gaps), unique_original_images=len(images))


def negative_checks(data, research):
    cases = []
    for name, mutate in [
        ('missing workflow', lambda d: d['workflows'].pop()),
        ('duplicate workflow', lambda d: d['workflows'].__setitem__(1, copy.deepcopy(d['workflows'][0]))),
        ('unknown family', lambda d: d['workflows'][0]['families'].append('D999')),
        ('missing menu', lambda d: d['menus'].pop()),
        ('missing gap', lambda d: d['gaps'].pop()),
        ('changed evidence', lambda d: d['workflows'][0]['evidence'].pop()),
        ('static menu promoted to observed', lambda d: d['menus'][1].__setitem__('observation', 'observed-with-limits')),
    ]:
        altered = copy.deepcopy(data); mutate(altered)
        try:
            validate(altered, research, check_markdown=False)
        except AssertionError:
            cases.append(name)
        else:
            raise AssertionError(f'negative control accepted: {name}')
    return cases


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', type=Path, required=True)
    parser.add_argument('--write-markdown', action='store_true')
    parser.add_argument('--negative-checks', action='store_true')
    parser.add_argument('--record', type=Path)
    args = parser.parse_args()
    result = dict(schema='desktop-map-check/1', source=subprocess.check_output(
        ['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], text=True).strip(),
        checker_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        started=datetime.now(timezone.utc).isoformat(), status='failed')
    try:
        data = json.loads((HERE/'screen-map.json').read_text())
        result['checked'] = validate(data, args.research_root, not args.write_markdown)
        if args.write_markdown:
            (HERE/'screen-map.md').write_text(markdown(data))
        if args.negative_checks:
            result['rejected_negative_controls'] = negative_checks(data, args.research_root)
        result['manifest_sha256'] = hashlib.sha256((HERE/'screen-map.json').read_bytes()).hexdigest()
        result['markdown_sha256'] = hashlib.sha256((HERE/'screen-map.md').read_bytes()).hexdigest()
        result['status'] = 'passed'
    except Exception as error:
        result['error'] = f'{type(error).__name__}: {error}'
    result['finished'] = datetime.now(timezone.utc).isoformat()
    if args.record:
        args.record.parent.mkdir(parents=True, exist_ok=True)
        with args.record.open('x') as output:
            json.dump(result, output, indent=2); output.write('\n')
    print(json.dumps(result, indent=2))
    return 0 if result['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
