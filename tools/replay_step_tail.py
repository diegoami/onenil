#!/usr/bin/env python3
"""Replay the original deterministic tail prefix and separate caller clock through Rust.

Expected results and helper observations are compared after native calls. No
clock/player/history/event formulas are implemented in this Python tool.
"""
import argparse
import hashlib
import json
import platform
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from importlib.metadata import version
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'engine/tests/fixtures/step_tail'
HEX64 = re.compile(r'[0-9a-f]{64}')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def same(a, b):
    return json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)


def encoded(value):
    return json.dumps(value, separators=(',', ':'))


def load_cases():
    from replay_placement import load_cases as placements
    old, _, old_index, _ = placements()
    old_map = {(c['week'], c['step']): c for c in old}
    old_records = {r['week']: r for r in old_index['records']}
    data = (FIXTURES / 'index.json').read_bytes()
    index = json.loads(data)
    pin = json.loads((FIXTURES / 'provenance.json').read_text())
    if (pin['repository'] != 'https://github.com/diegoami/onenil-archaeology'
            or not re.fullmatch(r'[0-9a-f]{40}', pin['commit'])
            or pin['index_sha256'] != sha(data)):
        raise ValueError('tail source/index pin differs')
    if (index['schema'] != 'step-tail-corpus/1'
            or tuple(index[k] for k in ('calls', 'clock_calls', 'scene_visits', 'clock_events', 'weeks', 'traces'))
            != (558, 558, 12276, 12, 6, 15)):
        raise ValueError('tail/clock original coverage differs')
    if (index['t13_index_sha256'] != sha((FIXTURES.parent / 'placement/index.json').read_bytes())
            or index['t12_index_sha256'] != old_index['t12_index_sha256']):
        raise ValueError('retained T12/T13 index provenance differs')
    files, weeks, cases, seen = set(), set(), [], set()
    for record in index['records']:
        name = record['cases']
        if (not isinstance(name, str) or Path(name).name != name or name in files or record['week'] in weeks):
            raise ValueError('duplicate/malformed tail case filename/week')
        raw = (FIXTURES / name).read_bytes()
        if not HEX64.fullmatch(record['cases_sha256']) or sha(raw) != record['cases_sha256']:
            raise ValueError('tail fixture digest differs')
        rows = [json.loads(line) for line in raw.splitlines()]
        if len(rows) != record['calls']:
            raise ValueError('tail week case count differs')
        for n, case in enumerate(rows):
            key = case['week'], case['step']
            if (case['schema'] != 'step-tail-case/1' or case['week'] != record['week']
                    or case['step'] != n or key in seen or key not in old_map
                    or not same(case['placement'], old_map[key])):
                raise ValueError('tail identity/literal T13 case differs')
            source, previous = case['source'], old_records[case['week']]
            if (source['t13_record'] != previous['record'] or source['t13_record_sha256'] != previous['record_sha256']
                    or source['t13_cases'] != previous['cases'] or source['t13_cases_sha256'] != previous['cases_sha256']
                    or source['t12_record'] != case['placement']['source']['t12_record']
                    or source['t12_record_sha256'] != case['placement']['source']['t12_record_sha256']
                    or Path(source['t12_cases']).name != Path(case['placement']['source']['t12_cases']).name
                    or source['t12_cases_sha256'] != case['placement']['source']['t12_cases_sha256']
                    or source['t08_trace'] != case['placement']['source']['t08_trace']
                    or source['t08_trace_sha256'] != case['placement']['source']['t08_sha256']):
                raise ValueError('tail retained source digests differ')
            if (case['prefix']['draw_start'] != case['prefix']['draw_end']
                    or case['clock']['draw_start'] != case['clock']['draw_end']):
                raise ValueError('deterministic prefix/clock unexpectedly drew RNG')
            seen.add(key)
        cases.extend(rows); files.add(name); weeks.add(record['week'])
    coverage = index['coverage']
    if (seen != set(old_map) or len(files) != 6 or len(coverage) != 15
            or len({r['trace'] for r in coverage}) != 15
            or {r['cases'] for r in coverage} != files
            or any(not HEX64.fullmatch(r['trace_sha256']) for r in coverage)
            or sum(len(c['prefix']['visits']) for c in cases) != 12276
            or sum(c['clock']['event'] is not None for c in cases) != 12):
        raise ValueError('incomplete source references/scene/numeric-event coverage')
    return cases, index, pin


def expected_prefix(case):
    return {k: case['prefix'][k] for k in ('output', 'visits', 'pending_calls')}


def check(cases):
    import onenil_engine
    for case in cases:
        prefix = json.loads(onenil_engine.step_tail_prefix(encoded(case['prefix']['input'])))
        clock = json.loads(onenil_engine.logical_clock(encoded(case['clock']['input'])))
        if not same(prefix, expected_prefix(case)):
            raise ValueError(f"{case['week']} step {case['step']}: prefix/scene observations differ")
        if not same(clock, {k: case['clock'][k] for k in ('output', 'result', 'event')}):
            raise ValueError(f"{case['week']} step {case['step']}: logical clock/event differs")
        composed = json.loads(onenil_engine.placement_tail_prefix(
            encoded(case['placement']['input']), encoded(case['prefix']['input'])))
        if (not same(composed['prefix'], expected_prefix(case))
                or any(not same(composed['placement'][key], case['placement'][key])
                       for key in ('output', 'draws', 'callbacks', 'minute_fills', 'attempts', 'mappings'))):
            raise ValueError(f"{case['week']} step {case['step']}: actual placement-to-prefix handoff differs")
    bad = json.loads(encoded(cases[0]['prefix']['input'])); bad['placement']['seed'] ^= 1
    try:
        onenil_engine.placement_tail_prefix(encoded(cases[0]['placement']['input']), encoded(bad))
    except ValueError:
        pass
    else:
        raise ValueError('native composed bridge accepted an incorrect seed handoff')
    print('Rust step tail: 558 prefixes / 558 clocks / 12276 scene visits / 12 numeric events / 15 T08 traces; actual placement handoffs preserved')


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], text=True).strip()


def benchmark(cases, pin, repetitions, output):
    from onenil_engine import onenil_engine as backend
    if git('status', '--porcelain', '--', 'engine', 'tools/replay_step_tail.py', 'pyproject.toml'):
        raise ValueError('commit code and fixtures before measuring')
    if backend.__debug_assertions__:
        raise ValueError('timing requires the release extension')
    prepared = [(encoded(c['prefix']['input']), encoded(c['clock']['input'])) for c in cases]
    started = datetime.now(timezone.utc).isoformat()
    begin, serialized_bytes = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for prefix, clock in prepared:
            serialized_bytes += len(backend.step_tail_prefix(prefix))
            serialized_bytes += len(backend.logical_clock(clock))
    elapsed = time.perf_counter_ns() - begin
    out = ROOT / output; out.mkdir(parents=True, exist_ok=True)
    number = 1 + max((int(p.stem.split('-r')[-1]) for p in out.glob('T05-r*.json')), default=0)
    path = out / f'T05-r{number:04d}.json'
    record = {'schema': 'routine-benchmark/1', 'run_id': path.stem, 'code_commit': git('rev-parse', 'HEAD'),
              'started': started, 'ended': datetime.now(timezone.utc).isoformat(), 'platform': platform.platform(),
              'python': platform.python_version(), 'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
              'cargo_lock_sha256': sha((ROOT / 'engine/Cargo.lock').read_bytes()),
              'build': {'crate_version': backend.__version__, 'debug_assertions': backend.__debug_assertions__,
                        'maturin': version('maturin'), 'extension_sha256': sha(Path(backend.__file__).read_bytes())},
              'archaeology': pin, 'routine': 'DD02 DF35..DFDB prefix and separate logical 7B7B clock',
              'distinct_prefixes': len(cases), 'distinct_clocks': len(cases), 'source_traces': 15,
              'repetitions': repetitions, 'prefix_calls': len(cases) * repetitions,
              'clock_calls': len(cases) * repetitions, 'calls': len(cases) * repetitions * 2,
              'rng_draws': 0, 'elapsed_ns': elapsed, 'serialized_output_bytes': serialized_bytes,
              'method': 'Pre-serialized independent prefix and clock inputs; bridge parsing, image setup, native logical updates, projection and serialization included; input preparation/output decoding excluded; after untimed full original replay',
              'limits': 'Separate routine calls across unresolved late-DD02 callees; no direct prefix-to-clock/full-DD02/match/week, timer/renderer parity or emulator speed comparison'}
    with path.open('x') as stream: stream.write(json.dumps(record, indent=2) + '\n')
    print(f"{path.relative_to(ROOT)}: {record['calls']} prefix/clock routine calls in {elapsed / 1e9:.6f} s")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true'); parser.add_argument('--benchmark', action='store_true')
    parser.add_argument('--out', default='records/T05'); parser.add_argument('--repetitions', type=int, default=20)
    args = parser.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        parser.error('choose --check or --benchmark; repetitions must be positive')
    cases, _, pin = load_cases(); check(cases)
    if args.benchmark: benchmark(cases, pin, args.repetitions, args.out)
    return 0


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError) as e:
        print(f'step-tail replay failed: {e}', file=sys.stderr); sys.exit(1)
