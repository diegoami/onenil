"""T68: transport the complete T71 edited stream only after source acceptance."""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT / 'records/T68/reference-r0001'
RESEARCH_ACCEPTANCE = None  # Set only after the pinned T71 research merge.
PINS = {
    'index.json':'2a5a3d84b9c5e8c8419db23dfd8dbaa1c335a3e3318c01abb426e60b1216fba9',
    'start.json':'124fe90298175e0f29c369580ecd5b663fec5cc0830c9da540db98beca627258',
    'attempt-0003-start.json':'62982211d9c1f93c85f6fdb1dfc9e205fa2b909512986d2f5f64c5d4d5b0b884',
    'attempt-0004-start.json':'58ca59fbacbe39300f26fc45a3e04145831fb82abbdec64ce216f256e8f92d26',
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()


def pinned(directory, name, digest):
    data = (directory / name).read_bytes()
    if sha(data) != digest:
        raise ValueError('fresh watched original source digest differs: ' + name)
    return json.loads(data)


def sources(directory=DIRECTORY):
    index = pinned(directory, 'index.json', PINS['index.json'])
    for name, digest in PINS.items():
        pinned(directory, name, digest)
    names = set(PINS)
    names.add(index['finish']['path'])
    finish = pinned(directory, index['finish']['path'], index['finish']['sha256'])
    producer = json.loads((directory / 'attempt-0004-start.json').read_text())['producer']
    repeat_name = 'repeat-0004.json'
    repeat_finish_name = 'attempt-0004-finish.json'
    names.update((repeat_name, repeat_finish_name))
    repeat = json.loads((directory / repeat_name).read_text())
    repeat_finish = json.loads((directory / repeat_finish_name).read_text())
    expected_repeat = dict(producer=producer, result=finish['result'],
                           chunk_payloads=finish['result']['chunk_payloads'],
                           payload_sha256=sha(encoded(finish['result'])))
    if repeat != expected_repeat:
        raise ValueError('independent original full-match repeat differs')
    expected_finish = dict(finish, producer=producer)
    if repeat_finish != expected_finish:
        raise ValueError('independent original actual-return record differs')
    ticks = draws = actions = 0
    for number, item in enumerate(index['chunks']):
        name = item['path']
        names.add(name)
        chunk = pinned(directory, name, item['sha256'])
        payload = chunk['payload']
        receipt_name = f'attempt-0004-verified-{number:06d}.json'
        names.add(receipt_name)
        receipt = json.loads((directory / receipt_name).read_text())
        if receipt != dict(producer=producer, chunk=name,
                           payload_sha256=chunk['payload_sha256'],
                           tick=payload['tick_end'], draws=payload['draw_end']):
            raise ValueError('independent original chunk receipt differs')
        if sha(encoded(payload)) != chunk['payload_sha256']:
            raise ValueError('fresh original chunk payload differs')
        ticks += len(payload['ticks'])
        draws += len(payload['draws'])
        actions += len(payload['actions'])
    if (ticks, draws, actions) != (finish['ticks'], finish['draws'], finish['actions']):
        raise ValueError('fresh original stream totals differ')
    if set(p.name for p in directory.iterdir()) != names:
        raise ValueError('fresh original source leaf census differs')
    return index, finish


def import_sources(research):
    if RESEARCH_ACCEPTANCE is None:
        raise ValueError('accepted T71 research merge required before original import')
    subprocess.run(['git','merge-base','--is-ancestor',RESEARCH_ACCEPTANCE,'HEAD'],cwd=research,check=True)
    directory = research / 'runs/T70/reference-r0001'
    # This fails until every independent chunk and actual return exists.
    subprocess.run([sys.executable, str(research / 'runtime/emul/edited_watched.py'),
                    '--check', '--directory', str(directory)], check=True)
    index = pinned(directory, 'index.json', PINS['index.json'])
    names = set(PINS) | {index['finish']['path'], 'repeat-0004.json', 'attempt-0004-finish.json'}
    names.update(item['path'] for item in index['chunks'])
    names.update(f'attempt-0004-verified-{i:06d}.json' for i in range(len(index['chunks'])))
    pending = {name: (directory / name).read_bytes() for name in names}
    if DIRECTORY.exists():
        raise ValueError('refusing to overwrite imported original watched evidence')
    DIRECTORY.mkdir(parents=True)
    for name, data in pending.items():
        with (DIRECTORY / name).open('xb') as output:
            output.write(data)
    sources()
