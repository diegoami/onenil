"""Synthetic provenance-graph checks; no match simulation or original bytes."""
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import fresh_watched_reference as subject


def write(directory, name, value):
    data = json.dumps(value).encode()
    (directory / name).write_bytes(data)
    return subject.sha(data)


def graph(directory):
    pins = {}
    for name in subject.PINS:
        if name != 'index.json':
            pins[name] = write(directory, name, dict(producer='repeat-producer'))
    payload = dict(ticks=[], draws=[], actions=[], tick_end=0, draw_end=0)
    chunk = dict(payload=payload, payload_sha256=subject.sha(subject.encoded(payload)))
    digest = write(directory, 'chunk-000000.json', chunk)
    result = dict(final={'named': 1}, chunk_payloads=[chunk['payload_sha256']])
    finish = dict(producer='first-producer', result=result, ticks=0, draws=0, actions=0)
    finish_digest = write(directory, 'attempt-0002-finish.json', finish)
    index = dict(chunks=[dict(path='chunk-000000.json', sha256=digest)],
                 finish=dict(path='attempt-0002-finish.json', sha256=finish_digest))
    pins['index.json'] = write(directory, 'index.json', index)
    write(directory, 'attempt-0003-finish.json', dict(finish, producer='repeat-producer'))
    write(directory, 'repeat-0003.json', dict(producer='repeat-producer', result=result,
          chunk_payloads=result['chunk_payloads'], payload_sha256=subject.sha(subject.encoded(result))))
    write(directory, 'attempt-0003-verified-000000.json', dict(producer='repeat-producer',
          chunk='chunk-000000.json', payload_sha256=chunk['payload_sha256'], tick=0, draws=0))
    return pins


class FreshReferenceTests(unittest.TestCase):
    def test_complete_graph_and_corrupted_repeat_refusals(self):
        for damage in (None, 'chunk', 'result', 'producer', 'return', 'receipt', 'missing', 'extra'):
            with self.subTest(damage=damage), tempfile.TemporaryDirectory() as temp:
                directory = Path(temp)
                pins = graph(directory)
                if damage == 'chunk':
                    write(directory, 'chunk-000000.json', {})
                elif damage in ('result', 'producer'):
                    path = directory / 'repeat-0003.json'
                    value = json.loads(path.read_text())
                    if damage == 'result':
                        value['result']['final']['named'] = 2
                        value['payload_sha256'] = subject.sha(subject.encoded(value['result']))
                    else:
                        value['producer'] = 'foreign'
                    write(directory, path.name, value)
                elif damage == 'return':
                    path = directory / 'attempt-0003-finish.json'
                    value = json.loads(path.read_text())
                    value['result']['final']['named'] = 2
                    write(directory, path.name, value)
                elif damage == 'receipt':
                    path = directory / 'attempt-0003-verified-000000.json'
                    value = json.loads(path.read_text())
                    value['tick'] = 1
                    write(directory, path.name, value)
                elif damage == 'missing':
                    (directory / 'repeat-0003.json').unlink()
                elif damage == 'extra':
                    write(directory, 'unrecognized.json', {})
                with patch.object(subject, 'PINS', pins):
                    if damage is None:
                        subject.sources(directory)
                    else:
                        with self.assertRaises((ValueError, FileNotFoundError)):
                            subject.sources(directory)


if __name__ == '__main__':
    unittest.main()
