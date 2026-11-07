#!/usr/bin/env python3
"""T61: compare the continuing native caller with independently verified T67."""
import argparse
import json
import subprocess
from pathlib import Path

import fresh_watched_inputs as incoming
import fresh_watched_reference as reference
import replay_fresh_preparation as preparation
import watched_oracle

ROOT = Path(__file__).resolve().parents[1]


def request():
    admission, _ = preparation.admission()
    facts = incoming.admission()['support']
    return dict(preparation=admission['input'], support=facts, no_pending_key=True)


def compare(got, directory):
    """Only normalize source draw sites/event ordinals; compare every other field."""
    index = json.loads((directory / 'index.json').read_text())
    tick = draw = event = 0
    for item in index['chunks']:
        payload = json.loads((directory / item['path']).read_text())['payload']
        for expected in payload['ticks']:
            if got['ticks'][tick] != expected:
                raise ValueError(f'fresh native tick differs: {tick + 1}')
            tick += 1
        for expected in payload['draws']:
            if got['draws'][draw] != watched_oracle.normalized(expected):
                raise ValueError(f'fresh native ordered RNG differs: {draw}')
            draw += 1
        for expected in payload['actions']:
            expected = {key: value for key, value in expected.items() if key != 'event'}
            if got['events'][event] != expected:
                raise ValueError(f'fresh native player action differs: {event}')
            event += 1
    if (len(got['ticks']), len(got['draws']), len(got['events'])) != (tick, draw, event):
        raise ValueError('fresh native stream length differs')
    finish = json.loads((directory / index['finish']['path']).read_text())
    if got['final'] != finish['final'] or got['return'] != finish['boundary']:
        raise ValueError('fresh native final state/actual return differs')
    print(f'PASS fresh native {tick} ticks,{draw} draws,{event} actions,final and return')


def replay(directory, probe, python=False):
    value = request()
    if python:
        import onenil_engine as native
        session = native.NativePreparedWatchedSession.fresh(json.dumps(value))
        initial = session.snapshot(), session.observation()
        if initial != (session.snapshot(), session.observation()):
            raise ValueError('fresh read-only observations mutate the session')
        result = json.loads(session.finish())
        if 'ok' not in result:
            raise ValueError(result)
        got = result['ok']
        final = session.snapshot(), session.observation()
        if final != (session.snapshot(), session.observation()):
            raise ValueError('finished read-only observations mutate the session')
        if 'error' not in json.loads(session.finish()):
            raise ValueError('finished fresh session accepted another finish')
        try:
            session.step()
        except ValueError:
            pass
        else:
            raise ValueError('finished fresh session accepted another step')
    else:
        result = subprocess.run([str(probe)], input=json.dumps(dict(
            fresh=True, finish=True, input=value)) + '\n', capture_output=True,
            text=True, check=True)
        out = json.loads(result.stdout)
        if 'ok' not in out:
            raise ValueError(out.get('error', 'native fresh probe failed'))
        got = out['ok']['finished']
    compare(got, directory)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--import-from', type=Path)
    parser.add_argument('--check', action='store_true')
    parser.add_argument('--replay', action='store_true')
    parser.add_argument('--probe', type=Path, default=ROOT / 'engine/target/debug/examples/prepared_watched_probe')
    parser.add_argument('--python', action='store_true')
    args = parser.parse_args()
    if args.import_from:
        reference.import_sources(args.import_from.resolve())
    if args.check or args.replay:
        reference.sources()
        request()
        print('PASS exact original full stream,independent repeat and incoming facts')
    if args.replay:
        replay(reference.DIRECTORY, args.probe, args.python)


if __name__ == '__main__':
    main()
