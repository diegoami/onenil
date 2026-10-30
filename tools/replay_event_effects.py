#!/usr/bin/env python3
"""Replay the logical 69B8 projection through Rust; Python checks observations only."""
import argparse
import json
import platform
import subprocess
import sys
import time
from datetime import datetime, timezone
from importlib.metadata import version
from pathlib import Path

import replay_extra_event as previous
import replay_step_tail as tail

ROOT = tail.ROOT
FIXTURES = ROOT / 'engine/tests/fixtures/event_effects'
sha, same, encoded = tail.sha, tail.same, tail.encoded


def load_cases():
    original, _ = previous.load_cases()
    prior = {(c['week'], c['step']): (c, inp, old) for c, inp, old in original}
    wanted = {k for k, (c, _, _) in prior.items() if c['dispatch']}
    index, pin = previous.pinned_index(FIXTURES, 'event-effects-corpus/1')
    if tuple(index[k] for k in ('weeks', 'calls', 'draws', 'selectors', 'actions', 'helpers', 'events')) != (2, 22, 17, 44, 6, 23, 6):
        raise ValueError('logical effect coverage differs')
    for k, filename in [('t17_index_sha256', '../extra_event/index.json'), ('t18_index_sha256', '../extra_event/inputs/index.json')]:
        if index[k] != sha((FIXTURES / filename).read_bytes()):
            raise ValueError('logical effect retained index differs')
    old_index = json.loads((FIXTURES.parent / 'extra_event/index.json').read_text())
    old_meta = {r['week']: r for r in old_index['records']}
    lookup_index = json.loads((FIXTURES.parent / 'extra_event/inputs/index.json').read_text())
    lookups = {}
    for r in lookup_index['records']:
        for cell in previous.read_rows(FIXTURES.parent / 'extra_event/inputs', r['cases'], r['cases_sha256']):
            lookups[r['week'], cell['step']] = cell
    cases, seen, files, weeks = [], set(), set(), set()
    for r in index['records']:
        if r['cases'] in files or r['week'] in weeks:
            raise ValueError('duplicate logical effect file/week')
        rows = previous.read_rows(FIXTURES, r['cases'], r['cases_sha256'])
        if len(rows) != r['calls']:
            raise ValueError('incomplete logical effect week')
        for c in rows:
            key = c['week'], c['step']
            if c['schema'] != 'event-effects-case/1' or c['week'] != r['week'] or key in seen or key not in wanted:
                raise ValueError('logical effect identity differs')
            old, inp, prefix = prior[key]
            if (not same(c['decision'], old) or not same(c['lookup'], lookups[key])
                    or not same(c['input']['handoff'], old['dispatch']['input'])
                    or not same(c['output']['handoff'], old['output'])
                    or c['draw_start'] != old['dispatch']['draw_start'] or c['draw_end'] != old['draw_end']
                    or not same(c['draws'], [d for d in old['draws'] if d['i'] >= c['draw_start']])):
                raise ValueError('effect inputs/returns/draws do not link literally to original DBC6')
            if any(c['source'][k] != old_meta[c['week']][v] for k, v in (
                    ('t17_record', 'record'), ('t17_record_sha256', 'record_sha256'),
                    ('t17_cases', 'cases'), ('t17_cases_sha256', 'cases_sha256'))):
                raise ValueError('effect source metadata differs')
            cases.append((c, inp, prefix)); seen.add(key)
        files.add(r['cases']); weeks.add(r['week'])
    if seen != wanted or len(cases) != 22 or len(files) != 2:
        raise ValueError('missing logical effect dispatches')
    return cases, pin


def expected(case):
    draws = []
    for d in case['draws']:
        sites = {'0AB9:6A15', '1998:13EE'}.intersection(d['callers'])
        if len(sites) != 1: raise ValueError('unattributed effect RNG site')
        draws.append({'site': sites.pop(), **{k: d[k] for k in ('n', 'value', 'seed_before', 'seed_after')}})
    return {'draws': draws, **{k: case[k] for k in ('output', 'selectors', 'actions', 'helpers', 'events', 'external_calls')}}


def check(cases):
    import onenil_engine as native
    counts = [0, 0, 0, 0]
    for c, decision_input, old in cases:
        got = json.loads(native.logical_event_effects(encoded(c['input'])))
        if not same(got, expected(c)):
            fields = [k for k in got if not same(got[k], expected(c)[k])]
            raise ValueError(f"{c['week']} step {c['step']}: logical fields differ: {fields}")
        composed = json.loads(native.placement_prefix_event_effects(encoded(old['placement']['input']),
            encoded(old['prefix']['input']), encoded(decision_input), encoded(c['input'])))
        if (not same(composed['effects'], got)
                or not same(composed['predecessor']['decision'], previous.expected(c['decision']))
                or not same(composed['predecessor']['prefix']['prefix'], tail.expected_prefix(old))):
            raise ValueError('computed decision request/effect handoff differs')
        for i, name in enumerate(('draws', 'selectors', 'helpers', 'events')): counts[i] += len(got[name])
    if counts != [17, 44, 23, 6]: raise ValueError('native logical effect coverage differs')
    c, decision_input, old = cases[0]
    for name in ('seed', 'actor', 'secondary'):
        bad = json.loads(encoded(c['input'])); bad['handoff'][name] ^= 1
        try:
            native.placement_prefix_event_effects(encoded(old['placement']['input']), encoded(old['prefix']['input']), encoded(decision_input), encoded(bad))
        except ValueError:
            pass
        else:
            raise ValueError('bridge accepted an inconsistent computed handoff')
    bad = dict(c['input'], expected_events=c['events'])
    try:
        native.logical_event_effects(encoded(bad))
    except ValueError:
        pass
    else:
        raise ValueError('bridge accepted expected events as inputs')
    print('Rust event effects: 22 handlers / 17 LCG draws / 44 selectors / 23 helpers / 6 numeric events; computed DBC6 requests preserved')


def benchmark(cases, pin, repetitions, output):
    from onenil_engine import onenil_engine as native
    if tail.git('status', '--porcelain', '--', 'engine', 'tools/replay_event_effects.py', 'pyproject.toml'):
        raise ValueError('commit code and fixtures before measuring')
    if native.__debug_assertions__: raise ValueError('timing requires a release extension')
    prepared = [encoded(c['input']) for c, _, _ in cases]
    started = datetime.now(timezone.utc).isoformat(); begin = time.perf_counter_ns(); serialized = 0
    for _ in range(repetitions):
        for data in prepared: serialized += len(native.logical_event_effects(data))
    elapsed = time.perf_counter_ns() - begin
    directory = ROOT / output; directory.mkdir(parents=True, exist_ok=True)
    n = 1 + max((int(p.stem.split('-r')[-1]) for p in directory.glob('T07-r*.json')), default=0)
    path = directory / f'T07-r{n:04d}.json'
    rec = {'schema': 'routine-benchmark/1', 'run_id': path.stem, 'code_commit': tail.git('rev-parse', 'HEAD'),
           'started': started, 'ended': datetime.now(timezone.utc).isoformat(), 'platform': platform.platform(),
           'python': platform.python_version(), 'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(),
           'cargo_lock_sha256': sha((ROOT / 'engine/Cargo.lock').read_bytes()),
           'build': {'crate_version': native.__version__, 'debug_assertions': native.__debug_assertions__,
                     'maturin': version('maturin'), 'extension_sha256': sha(Path(native.__file__).read_bytes())},
           'archaeology': pin, 'routine': '69B8 bounded quick logical effects', 'distinct_handlers': len(cases),
           'source_weeks': 2, 'repetitions': repetitions, 'calls': len(cases) * repetitions,
           'rng_draws': 17 * repetitions, 'selector_calls': 44 * repetitions, 'elapsed_ns': elapsed,
           'serialized_output_bytes': serialized,
           'method': 'Pre-serialized original raw logical inputs; bridge parsing, image setup, native logical effects/selectors/RNG and serialization included; preparation/output decoding excluded; after untimed original/composed replay',
           'limits': 'Bounded quick logical handlers; AI rearrangements diagnose. No full DD02/match/week/shared-stream setup, physical/scratch/timer/render parity or emulator speed comparison'}
    with path.open('x') as stream: stream.write(json.dumps(rec, indent=2) + '\n')
    print(f'{path.relative_to(ROOT)}: {rec["calls"]} handlers / {rec["rng_draws"]} draws / {rec["selector_calls"]} selectors in {elapsed/1e9:.6f} s')


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--check', action='store_true'); ap.add_argument('--benchmark', action='store_true')
    ap.add_argument('--out', default='records/T07'); ap.add_argument('--repetitions', type=int, default=20)
    args = ap.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1: ap.error('choose --check or --benchmark; positive repetitions required')
    cases, pin = load_cases(); check(cases)
    if args.benchmark: benchmark(cases, pin, args.repetitions, args.out)
    return 0


if __name__ == '__main__':
    try: sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError, StopIteration) as exc:
        print(f'event-effects replay failed: {exc}', file=sys.stderr); sys.exit(1)
