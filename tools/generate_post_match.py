#!/usr/bin/env python3
"""[derived: archaeology ba499b9/T86] Compile the E679 post-match closure.

This is a translation check, not an original-record execution parity check.
The condition boundary deliberately refuses pending a live-image adapter.
"""
import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'engine/src/watched_compiled'
GRAPH = BASE / 'post_match_graph.rs'
MANIFEST = ROOT / 'records/T82/graph.json'
RESEARCH = 'ba499b9a678254d6a14497153581f85f93ee4e18'
EVIDENCE = f'archaeology {RESEARCH}/T86,E679 static closure'
REUSED = ['action_graph', 'match_graph']


def generate(research_root):
    """Generate into scratch space; never mutate an accepted graph or record."""
    with tempfile.TemporaryDirectory() as directory:
        output = Path(directory) / 'graph.rs'
        command = [sys.executable, str(ROOT / 'tools/compile_watched.py'),
                   '--research-root', str(research_root), '--out', str(output),
                   '--root', '0AB9:E679', '--source-addresses',
                   '--defer', '1998:14A5', '--evidence', EVIDENCE]
        for module in REUSED:
            command += ['--reuse-existing', str(BASE / (module + '.rs'))]
        result = subprocess.run(command, check=True, capture_output=True, text=True)
        metadata = json.loads(result.stdout)
        metadata['research_commit'] = RESEARCH
        metadata['execution_validated'] = False
        return output.read_bytes(), metadata


def verify(source, metadata, research_root):
    """Verify the compiled artifact, source pins and explicit boundary contract."""
    if metadata['research_commit'] != RESEARCH or metadata['execution_validated'] is not False:
        raise ValueError('invalid evidence/execution claim')
    if metadata['roots'] != ['0AB9:E679'] or metadata['source_addresses'] is not True:
        raise ValueError('invalid translation root/address contract')
    if metadata.get('boundaries') != {'1998:14A5': {
            'kind': 'unported', 'reason': 'explicit unported boundary'}}:
        raise ValueError('unexpected translation boundary')
    if hashlib.sha256(source).hexdigest() != metadata['generated_sha256']:
        raise ValueError('generated graph digest differs')
    for name, digest in metadata['listing_digests'].items():
        if hashlib.sha256((research_root / name).read_bytes()).hexdigest() != digest:
            raise ValueError('listing digest differs: ' + name)
    if [entry['module'] for entry in metadata['reused']] != REUSED:
        raise ValueError('reused module census differs')
    for entry in metadata['reused']:
        data = (BASE / (entry['module'] + '.rs')).read_bytes()
        if hashlib.sha256(data).hexdigest() != entry['sha256']:
            raise ValueError('reused module digest differs: ' + entry['module'])
    actual, actual_metadata = generate(research_root)
    if source != actual or metadata != actual_metadata:
        raise ValueError('post-match regeneration differs')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', required=True, type=Path)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    if args.check:
        source, metadata = GRAPH.read_bytes(), json.loads(MANIFEST.read_text())
        verify(source, metadata, args.research_root)
    else:
        if GRAPH.exists() or MANIFEST.exists():
            raise FileExistsError('retain the accepted graph/record; use --check')
        source, metadata = generate(args.research_root)
        GRAPH.write_bytes(source)
        MANIFEST.parent.mkdir(parents=True, exist_ok=True)
        MANIFEST.write_text(json.dumps(metadata, indent=2) + '\n')
    print(json.dumps(metadata, indent=2))


if __name__ == '__main__':
    main()
