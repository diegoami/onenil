#!/usr/bin/env python3
"""Bounded display names from accepted T65 save/league decoders; no game bytes."""
import argparse
import hashlib
import importlib.util
import json
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / 'engine/tests/fixtures/fresh_preparation/admission.json'
OUTPUT = ROOT / 'desktop/faithful/assets/campaign-match-names.json'
SAVE_SHA = '6ef61bcf78af17a78cbbdc50b68839be169552464b4f2ee9eec5ea9b3740bc36'
LEAGUE_SHA = 'fc4bfa045b71ddede9692f40d6a4c0e3b29cc7334c1ec6e910c275f63f714352'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def export(research):
    import sys
    seed_raw = (ROOT / 'campaign/fixtures/research-seed.json').read_bytes()
    if sha(seed_raw) != 'ebafc57db43544adcf13d3c002f372e587d8cc201e4a1ae8aa1a74dd37e31483':
        raise ValueError('accepted T65 name-source identity differs')
    seed = json.loads(seed_raw)
    decoders = ('tools/data/campaign_seed.py', 'tools/data/save.py', 'tools/data/league.py')
    historical = {}
    for name in decoders:
        data = subprocess.check_output(['git', 'show', seed['producer'] + ':' + name], cwd=research)
        if sha(data) != seed['producer_files'][name]:
            raise ValueError('accepted original name decoder differs: ' + name)
        historical[Path(name).name] = data
    data = (research / '.cache/One-Nil96/SAVE1.1-0').read_bytes()
    league = (research / '.cache/One-Nil96/ENGL-95.DAT').read_bytes()
    if sha(data) != SAVE_SHA or sha(league) != LEAGUE_SHA:
        raise ValueError('foreign campaign display-name namespace')
    # Execute precisely the accepted decoder versions, irrespective of later
    # archaeology tooling edits. Only public Python source goes in this temp dir.
    with tempfile.TemporaryDirectory(prefix='T62-decoders-') as temporary:
        directory = Path(temporary)
        for name, content in historical.items():
            (directory / name).write_bytes(content)
        sys.path.insert(0, str(directory))
        spec = importlib.util.spec_from_file_location('source_campaign_seed', directory / 'campaign_seed.py')
        source = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(source)
        tables = source.names(league)
        blocks = source.blocks(data)
        initial = source.project(data, tables)
    legacy = json.loads(FIXTURE.read_text())['input']['setup']['legacy']
    clubs = [row['id'] for row in legacy['substitution']['context']['clubs']]
    if sorted(clubs) != sorted([initial['club']['id'], initial['next_fixture']['opponent']]):
        raise ValueError('fresh preparation clubs differ from original campaign')
    players, memberships = {}, {}
    for player in legacy['players']:
        identifier = player['id']
        if identifier == 0:
            continue
        club = player['p8'][0]
        if club not in clubs or blocks['[DS:D7BA]'][8 * identifier] != club:
            raise ValueError('mixed original player/club namespace')
        name = (chr(blocks['[DS:D7AA]'][identifier]) + ' ' + tables['players'][identifier - 1]).strip()
        if not name:
            raise ValueError('missing original display name')
        players[str(identifier)] = name
        memberships[str(identifier)] = club
    return dict(schema='campaign-match-display-names/1',
                producer=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
                sources=dict(save_sha256=SAVE_SHA, league_sha256=LEAGUE_SHA,
                             preparation_sha256=sha(FIXTURE.read_bytes()),
                             research_commit=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=research, text=True).strip(),
                             decoder_producer=seed['producer'],
                             decoders={name: seed['producer_files'][name] for name in decoders}),
                clubs={str(club): tables['clubs'][club - 1] for club in clubs},
                players=players, player_clubs=memberships)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', type=Path, required=True)
    args = parser.parse_args()
    if subprocess.check_output(['git', 'status', '--porcelain', '--', 'desktop/faithful/export_match_names.py'], cwd=ROOT):
        raise ValueError('commit display-name exporter before measuring')
    value = export(args.research_root.resolve())
    with OUTPUT.open('x') as output:
        json.dump(value, output, indent=2)
        output.write('\n')
    print('PASS source-names only:', len(value['players']), 'players')


if __name__ == '__main__':
    main()
