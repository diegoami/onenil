#!/usr/bin/env python3
"""Replay raw DBC6/helper inputs through Rust; 69B8 remains an external request.

Python checks literal observations/provenance only, never engine formulas.
"""
import argparse
import json
import platform
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from importlib.metadata import version
from pathlib import Path

import replay_step_tail as tail

ROOT = tail.ROOT
FIXTURES = ROOT / 'engine/tests/fixtures/extra_event'
SITES = {'0AB9:DBD1', '0AB9:DC38', '0AB9:DC98', '0AB9:DCBF', '0AB9:DCD4'}
sha, same, encoded = tail.sha, tail.same, tail.encoded


def pinned_index(directory, schema):
    raw = (directory / 'index.json').read_bytes()
    index, pin = json.loads(raw), json.loads((directory / 'provenance.json').read_text())
    if (pin['repository'] != 'https://github.com/diegoami/onenil-archaeology'
            or not re.fullmatch(r'[0-9a-f]{40}', pin['commit']) or pin['index_sha256'] != sha(raw)
            or index['schema'] != schema):
        raise ValueError('extra-event source/index pin differs')
    return index, pin


def read_rows(directory, name, digest):
    if not isinstance(name, str) or Path(name).name != name:
        raise ValueError('malformed fixture path')
    raw = (directory / name).read_bytes()
    if not tail.HEX64.fullmatch(digest) or sha(raw) != digest:
        raise ValueError('extra-event fixture digest differs')
    return [json.loads(line) for line in raw.splitlines()]


def load_cases():
    old, old_index, _ = tail.load_cases()
    tails = {(c['week'], c['step']): c for c in old}
    index, pin = pinned_index(FIXTURES, 'extra-event-corpus/1')
    sup, sup_pin = pinned_index(FIXTURES / 'inputs', 'extra-event-inputs/1')
    if (tuple(index[k] for k in ('calls', 'decision_draws', 'helpers', 'dispatches', 'weeks', 'traces')) != (184, 499, 909, 22, 6, 15)
            or (sup['weeks'], sup['cells']) != (2, 184)
            or sup['t17_index_sha256'] != pin['index_sha256']
            or index['t16_index_sha256'] != sha((FIXTURES.parent / 'step_tail/index.json').read_bytes())):
        raise ValueError('extra-event coverage or retained index differs')
    supplemental = {}
    for record in sup['records']:
        rows = read_rows(FIXTURES / 'inputs', record['cases'], record['cases_sha256'])
        if len(rows) != record['cells']:
            raise ValueError('supplementary week incomplete')
        for row in rows:
            key = record['week'], row['step']
            if (key in supplemental or row['schema'] != 'extra-event-input/1' or row['offset'] != 0xD0B
                    or type(row['value']) is not int or not 0 <= row['value'] <= 255
                    or row['source']['t17_record'] != record['t17_record']
                    or row['source']['t17_record_sha256'] != record['t17_record_sha256']):
                raise ValueError('supplementary cell/source identity differs')
            supplemental[key] = row
    cases, files, weeks, seen = [], set(), set(), set()
    for record in index['records']:
        name = record['cases']
        if name in files or record['week'] in weeks:
            raise ValueError('duplicate original filename/week')
        rows = read_rows(FIXTURES, name, record['cases_sha256'])
        if len(rows) != record['calls']:
            raise ValueError('DBC6 week incomplete')
        for c in rows:
            key = c['week'], c['step']
            cell = supplemental.pop(key)
            if c['schema'] != 'extra-event-case/1' or c['week'] != record['week'] or key in seen:
                raise ValueError('DBC6 case identity differs')
            if (cell['seed'] != c['input']['seed'] or cell['draw_start'] != c['draw_start']
                    or any(cell['source'][k] != record[v] for k, v in (
                        ('t17_record', 'record'), ('t17_record_sha256', 'record_sha256'),
                        ('t17_cases', 'cases'), ('t17_cases_sha256', 'cases_sha256')))):
                raise ValueError('DBC6 cell-to-original source differs')
            old_case = tails[key]
            expected_source = next(r for r in old_index['records'] if r['week'] == c['week'])
            if (any(c['source'][k] != expected_source[v] for k, v in (
                    ('t16_record', 'record'), ('t16_record_sha256', 'record_sha256'),
                    ('t16_cases', 'cases'), ('t16_cases_sha256', 'cases_sha256')))
                    or c['source']['t08_trace'] != old_case['source']['t08_trace']
                    or c['source']['t08_trace_sha256'] != old_case['source']['t08_trace_sha256']
                    or c['draw_start'] != old_case['prefix']['draw_end']
                    or c['input']['seed'] != old_case['prefix']['output']['seed']):
                raise ValueError('DBC6 retained prefix/trace source differs')
            cases.append((c, {'state': c['input'], 'opponent_d0b': cell['value']}, old_case))
            seen.add(key)
        files.add(name); weeks.add(record['week'])
    requested = {key for key, c in tails.items() if any(r['routine'] == '0AB9:DBC6' for r in c['prefix']['pending_calls'])}
    coverage = index['coverage']; prior = {r['trace']: r for r in old_index['coverage']}
    new_week = {r['cases']: r['week'] for r in index['records']}
    old_week = {r['cases']: r['week'] for r in old_index['records']}
    for row in coverage:
        previous = prior[row['trace']]
        if row['trace_sha256'] != previous['trace_sha256'] or new_week[row['cases']] != old_week[previous['cases']]:
            raise ValueError('DBC6 T08 reference differs')
    if (supplemental or seen != requested or len(files) != 6 or len(cases) != 184
            or len(coverage) != 15 or len({r['trace'] for r in coverage}) != 15
            or {r['cases'] for r in coverage} != files):
        raise ValueError('DBC6 request/cell/source coverage incomplete')
    return cases, {'decisions': pin, 'supplementary_inputs': sup_pin}


def expected(case):
    begin = case['draw_start']
    end = case['dispatch']['draw_start'] if case['dispatch'] else case['draw_end']
    draws = []
    for d in case['draws'][:end - begin]:
        sites = SITES.intersection(d['callers'])
        if len(sites) != 1:
            raise ValueError('unattributed original decision draw')
        draws.append({'site': sites.pop(), **{k: d[k] for k in ('n', 'value', 'seed_before', 'seed_after')}})
    helpers = [{'caller': h['caller'], 'kind': h['kind'], 'result': h['result'],
                'input': {k: v for k, v in h['input'].items() if k not in ('offset', 'segment')},
                'draw_position': h['draw_start'] - begin} for h in case['helpers']]
    request = {k: case['dispatch'][k] for k in ('routine', 'input')} if case['dispatch'] else None
    return {'output': request['input'] if request else case['output'],
            'draws': draws, 'helpers': helpers, 'dispatch': request}


def check(cases):
    import onenil_engine as native
    draws, helpers, requests = 0, 0, 0
    for case, input_data, previous in cases:
        request = encoded(input_data)
        got = json.loads(native.extra_event_decision(request))
        if not same(got, expected(case)):
            raise ValueError(f"{case['week']} step {case['step']}: decision/helper/draw mismatch")
        composed = json.loads(native.placement_prefix_extra_event(
            encoded(previous['placement']['input']), encoded(previous['prefix']['input']), request))
        if (not same(composed['decision'], got)
                or not same(composed['prefix']['prefix'], tail.expected_prefix(previous))
                or any(not same(composed['prefix']['placement'][k], previous['placement'][k])
                       for k in ('output', 'draws', 'callbacks', 'minute_fills', 'attempts', 'mappings'))):
            raise ValueError('actual placement/prefix-to-decision handoff differs')
        for h in case['helpers']:
            argument = h['input']['player'] if h['kind'] == 'threshold' else h['input']['scene']
            answer = native.extra_event_helper(request, h['kind'], argument, h['input'].get('mode', 0), h.get('stack_word_before'))
            if type(answer) is not int or answer != h['result']:
                raise ValueError('standalone raw helper differs')
        draws += len(got['draws']); helpers += len(got['helpers']); requests += got['dispatch'] is not None
    if (draws, helpers, requests) != (499, 909, 22):
        raise ValueError('native decision coverage differs')
    for key in ('seed', 'actor'):
        _, input_data, previous = cases[0]
        bad = json.loads(encoded(input_data)); bad['state'][key] ^= 1
        try:
            native.placement_prefix_extra_event(encoded(previous['placement']['input']), encoded(previous['prefix']['input']), encoded(bad))
        except ValueError:
            pass
        else:
            raise ValueError('bridge accepted invalid actual handoff')
    bad = dict(cases[0][1], expected_helpers=[])
    try:
        native.extra_event_decision(encoded(bad))
    except ValueError:
        pass
    else:
        raise ValueError('bridge accepted expected helper answers as input')
    print('Rust DBC6: 184 decisions / 499 draws / 909 helpers / 22 external requests / 15 T08 references; actual placement/prefix handoffs preserved')


def benchmark(cases, pin, repetitions, output):
    from onenil_engine import onenil_engine as native
    if tail.git('status', '--porcelain', '--', 'engine', 'tools/replay_extra_event.py', 'pyproject.toml'):
        raise ValueError('commit code and fixtures before measuring')
    if native.__debug_assertions__:
        raise ValueError('timing requires a release extension')
    prepared = [encoded(input_data) for _, input_data, _ in cases]
    started = datetime.now(timezone.utc).isoformat()
    begin, serialized = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for input_data in prepared:
            serialized += len(native.extra_event_decision(input_data))
    elapsed = time.perf_counter_ns() - begin
    directory = ROOT / output; directory.mkdir(parents=True, exist_ok=True)
    number = 1 + max((int(p.stem.split('-r')[-1]) for p in directory.glob('T06-r*.json')), default=0)
    path = directory / f'T06-r{number:04d}.json'
    rec = {'schema': 'routine-benchmark/1', 'run_id': path.stem, 'code_commit': tail.git('rev-parse', 'HEAD'),
           'started': started, 'ended': datetime.now(timezone.utc).isoformat(), 'platform': platform.platform(),
           'python': platform.python_version(), 'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
           'cargo_lock_sha256': sha((ROOT / 'engine/Cargo.lock').read_bytes()),
           'build': {'crate_version': native.__version__, 'debug_assertions': native.__debug_assertions__,
                     'maturin': version('maturin'), 'extension_sha256': sha(Path(native.__file__).read_bytes())},
           'archaeology': pin, 'routine': 'DBC6 decision through optional external 69B8 request',
           'distinct_decisions': len(cases), 'source_weeks': 6, 'source_traces': 15,
           'repetitions': repetitions, 'calls': len(cases) * repetitions, 'rng_draws': 499 * repetitions,
           'elapsed_ns': elapsed, 'serialized_output_bytes': serialized,
           'method': 'Pre-serialized raw original inputs; release bridge parsing, image setup, native decisions/helpers/RNG and serialization included; input preparation/output decoding excluded; after untimed full original and composed replay',
           'limits': 'Routine decision timing only. 69B8 is external; no full DD02/match/week/scratch/timer/render parity or emulator speed comparison'}
    with path.open('x') as stream: stream.write(json.dumps(rec, indent=2) + '\n')
    print(f'{path.relative_to(ROOT)}: {rec["calls"]} decision calls / {rec["rng_draws"]} draws in {elapsed / 1e9:.6f} s')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true'); parser.add_argument('--benchmark', action='store_true')
    parser.add_argument('--out', default='records/T06'); parser.add_argument('--repetitions', type=int, default=20)
    args = parser.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        parser.error('choose --check or --benchmark; repetitions must be positive')
    cases, pin = load_cases(); check(cases)
    if args.benchmark: benchmark(cases, pin, args.repetitions, args.out)
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError, StopIteration) as exc:
        print(f'extra-event replay failed: {exc}', file=sys.stderr); sys.exit(1)
