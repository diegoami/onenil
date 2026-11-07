#!/usr/bin/env python3
"""T61: authenticate unchanged original incoming facts; no engine model."""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT / 'records/T61/incoming-r0001'
FIXTURE = ROOT / 'engine/tests/fixtures/fresh_watched/incoming.json'
# Exact immutable original leaves; independent pairs cover requests3..7.
PINS = {'support-r0005.json': '5a1d9959102eeac9bc7e0eafd59f4f9c5748745df6db572fa83c87950b8ebacc', 'support-r0006.json': '5a1d9959102eeac9bc7e0eafd59f4f9c5748745df6db572fa83c87950b8ebacc', 'support-r0007.json': '56303640871b045934c05123c990ac2ba7d6a4246ee75b05645bdd8e32ec15b6', 'support-r0008.json': '56303640871b045934c05123c990ac2ba7d6a4246ee75b05645bdd8e32ec15b6', 'support-r0009.json': 'd04076b19e96e135006db4112d26c1e74e58d25724eeec93cdb4037c8aa7e50a', 'support-r0010.json': 'd04076b19e96e135006db4112d26c1e74e58d25724eeec93cdb4037c8aa7e50a', 'support-r0011.json': 'e38b815830cc6f0878e2ff1bb1b071f91fb9cd79e36ff64ca38876e46657f11f', 'support-r0012.json': 'e38b815830cc6f0878e2ff1bb1b071f91fb9cd79e36ff64ca38876e46657f11f', 'support-r0013.json': '5ebf2a13a989a28bfa93fc8537b39559e203b5437fee488a771b7f3867a933ad', 'support-r0014.json': '5ebf2a13a989a28bfa93fc8537b39559e203b5437fee488a771b7f3867a933ad'}
BASE_SHA = '9639833b657ed2cf0c5684e7338843534f0d552e8323b55dd12e947a5b6e6331'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_text())


def import_sources(research):
    """The archaeology checker authenticates historical producers and source joins."""
    pending = {}
    if len(PINS) != 10:
        raise ValueError('missing incoming source census')
    for first in range(5, 15, 2):
        names = [f'support-r{i:04d}.json' for i in (first, first + 1)]
        subprocess.run([sys.executable, str(research / 'runtime/emul/fresh_support.py'),
                        '--check', str(research / 'runs/T68' / names[1]),
                        '--compare', str(research / 'runs/T68' / names[0])], check=True)
        for name in names:
            data = (research / 'runs/T68' / name).read_bytes()
            if sha(data) != PINS[name]:
                raise ValueError('incoming original source pin differs: ' + name)
            pending[name] = data
    if DIRECTORY.exists():
        raise ValueError('refusing to overwrite imported incoming evidence')
    DIRECTORY.mkdir(parents=True)
    for name, data in pending.items():
        with (DIRECTORY / name).open('xb') as output:
            output.write(data)


def projection():
    """Public byte pins preserve the already checked producer/source proof."""
    if len(PINS) != 10 or set(p.name for p in DIRECTORY.iterdir()) != set(PINS):
        raise ValueError('incoming original leaf census differs')
    records = {}
    for name, digest in PINS.items():
        data = (DIRECTORY / name).read_bytes()
        if sha(data) != digest:
            raise ValueError('incoming original source digest differs: ' + name)
        record = json.loads(data)
        if (record['schema'] != 'original-fresh-support/1'
                or record['status'] != 'completed' or record['error'] is not None
                or record['unsupported'] or record['interventions']
                or record['source']['sha256'] != BASE_SHA):
            raise ValueError('incoming source is incomplete or foreign')
        records[name] = record
    lookup = {}
    for first in range(5, 15, 2):
        a, b = (records[f'support-r{i:04d}.json'] for i in (first, first + 1))
        if a['payload'] != b['payload'] or a['request'] != b['request']:
            raise ValueError('independent original incoming witnesses differ')
        for field in a['payload']['fields']:
            if not field['incoming']:
                # Output/computed values never become admission support.
                continue
            if not field['before'] == field['precall'] == field['entry']:
                raise ValueError('incoming ownership alias differs')
            key = field['area'], field['offset']
            if key in lookup and lookup[key] != field['before']:
                raise ValueError('overlapping incoming requests conflict')
            lookup[key] = field['before']
    return dict(schema='fresh-watched-incoming/1', sources=PINS,
                support=[dict(area=area, offset=offset, value=value)
                         for (area, offset), value in sorted(lookup.items())])


def admission(freeze=False):
    value = projection()
    if freeze:
        FIXTURE.parent.mkdir(parents=True, exist_ok=True)
        with FIXTURE.open('x') as output:
            json.dump(value, output, indent=2)
            output.write('\n')
    if read(FIXTURE) != value:
        raise ValueError('fresh incoming admission differs from original witnesses')
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-from', type=Path)
    parser.add_argument('--freeze', action='store_true')
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    if args.import_from:
        import_sources(args.import_from.resolve())
    if args.freeze or args.check:
        value = admission(args.freeze)
        print('PASS independently sourced fresh incoming facts:', len(value['support']))


if __name__ == '__main__':
    main()
