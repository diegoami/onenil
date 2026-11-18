#!/usr/bin/env python3
"""[confirmed:archaeology 3d41a0db T89] Compile the witnessed Sevilla reference assets.

The generated fixtures pin the witnessed match as identity and typed facts; the
raw capture chunks never enter this repository. Generation is deterministic and
refuses any surprise in the record; --check regenerates and compares bytes.
The emulated-hardware PIT restoration belongs to the research capture only and
is deliberately absent here (task T91 hazard: no native timer concept).
"""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / 'engine/tests/fixtures'
IDENTITY_PATH = FIXTURES / 'sevilla_watched/identity.json'
PRESENTATION_PATH = FIXTURES / 'sevilla_watched/presentation-r0001.json'
CALLER_PATH = FIXTURES / 'sevilla_post_match/sevilla-result-r0001.json'
CAMPAIGN_PATH = ROOT / 'campaign/fixtures/sevilla-post-match.json'
COMMIT = '3d41a0dbf6b0a5ccd5cb7fe2d0076f5bce1ab58e'
CAPTURE = 'runs/T89/sevilla-r0001'
MANAGED_CLUB = 130
OPPONENT = 176
SCENE_KEYS = ('index', 'word14', 'word0', 'byte7', 'word4')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load(research, name):
    return json.loads((research / name).read_text())


def chunk_payloads(research):
    index = load(research, f'{CAPTURE}/index.json')
    payloads = []
    for leaf in index['chunks']:
        name = Path(leaf['path']).name
        data = (research / CAPTURE / name).read_bytes()
        if sha(data) != leaf['sha256']:
            raise ValueError(f'capture chunk digest differs: {name}')
        payload = json.loads(data)
        canonical = json.dumps(payload['payload'], sort_keys=True, separators=(',', ':')).encode()
        if payload['payload_sha256'] != sha(canonical):
            raise ValueError(f'capture chunk self digest differs: {name}')
        payloads.append(payload['payload'])
    return index, payloads


def stream(research):
    """The witnessed stream with structural chaining and event facts verified."""
    finish = load(research, f'{CAPTURE}/finish.json')
    if finish['status'] != 'completed' or finish['error'] is not None:
        raise ValueError('capture did not complete')
    result = finish['result']
    index, payloads = chunk_payloads(research)
    ticks, actions = [], []
    previous_draw_end = 0
    for position, payload in enumerate(payloads):
        if payload['index'] != position:
            raise ValueError('capture chunk index differs')
        if payload['draw_start'] != previous_draw_end:
            raise ValueError('witnessed draw chain differs')
        previous_draw_end = payload['draw_end']
        ticks += payload['ticks']
        actions += payload['actions']
    counts = {'ticks': result['ticks'], 'draws': result['draws'], 'actions': result['actions']}
    if (len(ticks), len(actions)) != (counts['ticks'], counts['actions']):
        raise ValueError('witnessed stream counts differ')
    if ticks[0]['tick'] != 1 or [t['tick'] for t in ticks] != list(range(1, len(ticks) + 1)):
        raise ValueError('witnessed tick chain differs')
    if ticks[-1]['tick'] != counts['ticks']:
        raise ValueError('witnessed tick tail differs')
    if result['entry_draws'] != 0:
        raise ValueError('the witnessed entry is not the stream origin')
    total_draws = sum(len(p['draws']) for p in payloads)
    if total_draws != counts['draws'] or previous_draw_end != counts['draws']:
        raise ValueError('witnessed draw count differs')
    events = {
        'half_time': [a for a in actions if a['code'] == 64],
        'full_time': [a for a in actions if a['code'] == 63],
        'goals': [a for a in actions if a['code'] in (38, 39)],
    }
    if len(events['half_time']) != 1 or len(events['full_time']) != 1:
        raise ValueError('witnessed half/full-time events differ')
    sides = {}
    for action in actions:
        pair = (action['side'], action['club'])
        if sides.setdefault(action['side'], action['club']) != action['club']:
            raise ValueError('witnessed action side/club mapping differs')
    if set(sides) != {0, 1} or sides[0] != MANAGED_CLUB or sides[1] != OPPONENT:
        raise ValueError('witnessed side/club identities differ')
    final_scores = ticks[-1]['after']['scores']
    frame = result['caller']['frame']
    if frame['scores'] != final_scores:
        raise ValueError('witnessed caller frame and final scores differ')
    for goal, expected in zip(
        events['goals'],
        [
            {'tick': 1140, 'minute': 27, 'club': MANAGED_CLUB, 'player': 2590, 'side': 0},
            {'tick': 2291, 'minute': 52, 'club': OPPONENT, 'player': 3510, 'side': 1},
        ],
    ):
        if any(goal[key] != value for key, value in expected.items()):
            raise ValueError('witnessed goal facts differ')
    return finish, index, ticks, actions, events, counts


def compact_snapshot(snapshot):
    """Only the fields the desktop witnessed adaptation consumes."""
    scenes = []
    for scene in snapshot['scenes']:
        for key in SCENE_KEYS:
            if key not in scene:
                raise ValueError(f'witnessed scene lacks {key}')
        scenes.append({k: scene[k] for k in SCENE_KEYS})
    return {
        'elapsed': snapshot['elapsed'],
        'actor': snapshot['actor'],
        'scores': snapshot['scores'],
        'ball': snapshot['ball']['word0'],
        'scenes': scenes,
    }


def presentation(research, ticks, actions, first_entry):
    rows = []
    for tick in ticks:
        row = compact_snapshot(tick['after'])
        row['tick'] = tick['tick']
        rows.append(row)
    events = []
    for action in actions:
        events.append({
            'tick': action['tick'],
            'code': action['code'],
            'player': action['player'],
            'secondary': action['secondary'],
            'club': action['club'],
            'side': action['side'],
            'minute': action['minute'],
        })
    if events and events[0]['tick'] != rows[0]['tick'] and events[0]['tick'] < rows[0]['tick']:
        raise ValueError('witnessed action precedes the first tick')
    return {
        'schema': 'sevilla-watched-presentation/1',
        'research_commit': COMMIT,
        'first_entry': compact_snapshot(first_entry),
        'ticks': rows,
        'actions': events,
    }


def sevilla_roster(research):
    """The witnessed Sevilla roster from the caller's club[176] initial fields."""
    finish = load(research, f'{CAPTURE}/finish.json')
    fields = finish['result']['caller']['initial']
    roster = {}
    for field in fields:
        name = field.get('field', '')
        if name.startswith('club[176]+') and field['size'] == 2 and field['value']:
            offset = int(name.split('+')[1])
            if offset % 2 == 0 and offset <= 36:
                roster[offset] = field['value']
    ordered = [roster[offset] for offset in sorted(roster)]
    if len(ordered) != 18 or len(set(ordered)) != 18:
        raise ValueError('witnessed Sevilla roster differs')
    return ordered


def caller_draws(research, start, end):
    """The witnessed caller RNG ledger, extracted from the digest-chained chunks."""
    _, payloads = chunk_payloads(research)
    rows = []
    for payload in payloads:
        for draw in payload['draws']:
            if start <= draw['i'] < end:
                rows.append({
                    'i': draw['i'],
                    'n': draw['n'],
                    'seed_before': draw['seed_before'],
                    'seed_after': draw['seed_after'],
                    'value': draw['value'],
                })
    if len(rows) != end - start:
        raise ValueError('witnessed caller draw ledger differs')
    return rows


def caller_fixture(research):
    finish = load(research, f'{CAPTURE}/finish.json')
    result = finish['result']
    caller = result['caller']
    for key in ('entry', 'returned', 'frame', 'initial', 'final', 'ops', 'regions',
                'draw_start', 'draw_end', 'census', 'preservation'):
        if key not in caller:
            raise ValueError(f'witnessed caller lacks {key}')
    census = caller['census']
    if census['reads'] != 2275 or census['writes'] != 409 or census['initial'] != 431:
        raise ValueError('witnessed caller census differs')
    if caller['preservation']['unexpected_count'] != 0:
        raise ValueError('witnessed caller preservation refuses')
    if caller['preservation']['changed_bytes'] != 169:
        raise ValueError('witnessed caller preservation count differs')
    condition = result['condition']
    if len(condition['calls']) != 36 or len(condition['contexts']) != 2:
        raise ValueError('witnessed condition service differs')
    calls = []
    for call in condition['calls']:
        calls.append({
            'index': call['index'],
            'context': call['context'],
            'slot': call['slot'],
            'return_site': call['return_site'],
            'input': call['input'],
            'status_byte': call['status_byte'],
            'dependencies': call['dependencies'],
            'writes': call['writes'],
            'output': call['output'],
            'draw_start': call['draw_start'],
            'draw_end': call['draw_end'],
        })
    contexts = []
    for context in condition['contexts']:
        contexts.append({
            'index': context['index'],
            'kind': context['kind'],
            'club': context['club'],
            'arguments': context['arguments'],
            'active': context['active'],
            'roster': context['roster'],
            'entry_seed': context['entry_seed'],
            'exit_seed': context['exit_seed'],
            'draw_start': context['draw_start'],
            'draw_end': context['draw_end'],
            'calls': sum(1 for call in calls if call['context'] == context['index']),
        })
    managed = [call for call in calls if call['context'] != 0]
    roster = [c for c in contexts if c['club'] == MANAGED_CLUB][0]['roster']
    if [call['input']['player'] for call in managed] != [pid for pid in roster if pid]:
        raise ValueError('witnessed managed-condition calls differ')
    return {
        'schema': 'sevilla-postmatch-result/1',
        'research_commit': COMMIT,
        'capture': CAPTURE,
        'entry_draws': result['entry_draws'],
        'ticks': result['ticks'],
        'draws': result['draws'],
        'actions': result['actions'],
        'match_return': {
            'site': result['match_return']['site'],
            'draw': result['match_return']['draw'],
            'seed': result['match_return']['seed'],
        },
        'caller': {
            'entry': caller['entry'],
            'returned': caller['returned'],
            'frame': caller['frame'],
            'initial': caller['initial'],
            'final': caller['final'],
            'ops': caller['ops'],
            'regions': caller['regions'],
            'draw_start': caller['draw_start'],
            'draw_end': caller['draw_end'],
            'census': caller['census'],
            'preservation': caller['preservation'],
        },
        'post_draws': caller_draws(research, caller['draw_start'], caller['draw_end']),
        'condition': {'calls': calls, 'contexts': contexts},
    }


def identity(research, index, events, counts, caller_digest, presentation_digest):
    start = load(research, f'{CAPTURE}/start.json')
    finish = load(research, f'{CAPTURE}/finish.json')
    boot = load(research, 'records/T89/boot-r0001.json')
    result = finish['result']
    if start['interventions'] or finish['keys_delivered'] is None:
        raise ValueError('witnessed capture control differs')
    return {
        'schema': 'sevilla-watched-identity/1',
        'research_commit': COMMIT,
        'source': {
            'capture': CAPTURE,
            'index_sha256': sha((research / CAPTURE / 'index.json').read_bytes()),
            'start_sha256': sha((research / CAPTURE / 'start.json').read_bytes()),
            'finish_sha256': sha((research / CAPTURE / 'finish.json').read_bytes()),
            'boot_record_sha256': sha((research / 'records/T89/boot-r0001.json').read_bytes()),
            'preparation_state_sha256': start['preparation_state']['sha256'],
        },
        'counts': dict(counts, entry_draws=result['entry_draws'], chunks=len(index['chunks'])),
        'chunk_sha256': [leaf['sha256'] for leaf in index['chunks']],
        'entry': {
            'seed': result['first_entry']['seed'],
            'elapsed': result['first_entry']['elapsed'],
            'remaining': result['first_entry']['remaining'],
            'live_week_index': start['live_context']['week_index'],
            'prepared_fixture': boot['context']['prepared_fixture'],
        },
        'match_return': {
            'site': result['match_return']['site'],
            'draw': result['match_return']['draw'],
            'seed': result['match_return']['seed'],
        },
        'clubs': {'managed': MANAGED_CLUB, 'opponent': OPPONENT, 'competition': 1},
        'frame': result['caller']['frame'],
        'boundaries': {
            'half_time_tick': events['half_time'][0]['tick'],
            'full_time_tick': events['full_time'][0]['tick'],
            'final_tick': counts['ticks'],
        },
        'goals': [
            {'tick': goal['tick'], 'minute': goal['minute'], 'club': goal['club'],
             'player': goal['player'], 'side': goal['side']}
            for goal in events['goals']
        ],
        'caller': {
            'fixture_sha256': caller_digest,
            'draw_start': result['caller']['draw_start'],
            'draw_end': result['caller']['draw_end'],
            'census': result['caller']['census'],
            'changed_bytes': result['caller']['preservation']['changed_bytes'],
            'condition_calls': len(result['condition']['calls']),
            'condition_contexts': len(result['condition']['contexts']),
        },
        'roster': {'opponent': sevilla_roster(research)},
        'presentation_sha256': presentation_digest,
    }


def campaign_fixture(research):
    """The campaign's witnessed post-caller facts: the managed roster's caller
    condition outcomes and the witnessed score/return, from the sealed record."""
    caller = caller_fixture(research)
    managed = [
        call
        for call in caller['condition']['calls']
        if call['context'] != 0
    ]
    conditions = [
        {
            'player': call['input']['player'],
            'condition_before': call['input']['condition'],
            'condition_after': call['output']['condition'],
        }
        for call in managed
    ]
    if len(conditions) != 18:
        raise ValueError('witnessed managed condition outcomes differ')
    return {
        'schema': 'sevilla-post-match/1',
        'research_commit': COMMIT,
        'caller_fixture_sha256': sha((json.dumps(caller, indent=1) + '\n').encode()),
        'managed_club': 130,
        'witnessed': {
            'score': [1, 1],
            'native_return': '0AB9:EBE2',
            'conditions': conditions,
        },
        'scope': 'The caller\'s own condition outcomes for the managed roster; weekly recovery '
        'remains unwitnessed and is not modeled. The week transaction after this caller '
        'refuses: the second week\'s cash movement was not witnessed.',
    }


def generate(research):
    """Generate into scratch; the caller compares against the committed assets."""
    finish, index, ticks, actions, events, counts = stream(research)
    caller = caller_fixture(research)
    caller_bytes = (json.dumps(caller, indent=1) + '\n').encode()
    first_entry = finish['result']['first_entry']
    presentation_doc = presentation(research, ticks, actions, first_entry)
    presentation_bytes = (json.dumps(presentation_doc, separators=(',', ':')) + '\n').encode()
    identity_doc = identity(
        research, index, events, counts, sha(caller_bytes), sha(presentation_bytes))
    if compact_snapshot(first_entry)['elapsed'] != 0 or compact_snapshot(first_entry)['scores'] != [0, 0]:
        raise ValueError('witnessed first entry differs')
    identity_bytes = (json.dumps(identity_doc, indent=1) + '\n').encode()
    campaign_bytes = (json.dumps(campaign_fixture(research), indent=1) + '\n').encode()
    return {
        IDENTITY_PATH: identity_bytes,
        PRESENTATION_PATH: presentation_bytes,
        CALLER_PATH: caller_bytes,
        CAMPAIGN_PATH: campaign_bytes,
    }


def verify_local(directory=None):
    """Authenticate the committed assets against each other, without the capture."""
    directory = FIXTURES if directory is None else Path(directory)
    identity_bytes = (directory / 'sevilla_watched/identity.json').read_bytes()
    presentation_bytes = (directory / 'sevilla_watched/presentation-r0001.json').read_bytes()
    caller_bytes = (directory / 'sevilla_post_match/sevilla-result-r0001.json').read_bytes()
    campaign_bytes = (directory.parents[2] / 'campaign/fixtures/sevilla-post-match.json').read_bytes()
    pinned = json.loads(identity_bytes)
    if pinned['schema'] != 'sevilla-watched-identity/1':
        raise ValueError('identity schema differs')
    if pinned['presentation_sha256'] != sha(presentation_bytes):
        raise ValueError('presentation digest differs')
    if pinned['caller']['fixture_sha256'] != sha(caller_bytes):
        raise ValueError('caller fixture digest differs')
    presentation = json.loads(presentation_bytes)
    caller = json.loads(caller_bytes)
    if len(presentation['ticks']) != pinned['counts']['ticks']:
        raise ValueError('presentation tick count differs')
    if len(presentation['actions']) != pinned['counts']['actions']:
        raise ValueError('presentation action count differs')
    if [row['tick'] for row in presentation['ticks']] != list(
        range(1, pinned['counts']['ticks'] + 1)
    ):
        raise ValueError('presentation tick chain differs')
    boundaries = pinned['boundaries']
    if presentation['ticks'][-1]['tick'] != boundaries['final_tick']:
        raise ValueError('presentation final tick differs')
    if presentation['ticks'][-1]['scores'] != pinned['frame']['scores']:
        raise ValueError('presentation final scores differ')
    by_tick = {row['tick']: row for row in presentation['ticks']}
    for goal in pinned['goals']:
        row = by_tick.get(goal['tick'])
        if row is None:
            raise ValueError('goal tick outside the presentation')
    if caller['caller']['draw_start'] != pinned['caller']['draw_start'] or caller['caller'][
        'draw_end'
    ] != pinned['caller']['draw_end']:
        raise ValueError('caller draw span differs')
    if caller['match_return']['site'] != pinned['match_return']['site']:
        raise ValueError('match return site differs')
    if len(caller['caller']['initial']) != pinned['caller']['census']['initial']:
        raise ValueError('caller initial census differs')
    if len(caller['condition']['calls']) != pinned['caller']['condition_calls']:
        raise ValueError('caller condition census differs')
    if pinned['counts']['ticks'] != caller['ticks'] or pinned['counts']['actions'] != caller[
        'actions'
    ]:
        raise ValueError('identity and caller counts differ')
    campaign = json.loads(campaign_bytes)
    witnessed = campaign['witnessed']['conditions']
    replayed = [
        (call['input']['player'], call['output']['condition'])
        for call in caller['condition']['calls']
        if call['context'] != 0
    ]
    if [(c['player'], c['condition_after']) for c in witnessed] != replayed:
        raise ValueError('campaign witnessed conditions differ from the caller record')
    if campaign['witnessed']['score'] != caller['caller']['frame']['scores']:
        raise ValueError('campaign witnessed score differs from the caller frame')
    if campaign['caller_fixture_sha256'] != sha(caller_bytes):
        raise ValueError('campaign caller digest differs')
    for name in ('pit', 'divisor', 'timer'):
        if name in identity_bytes.decode() or name in caller_bytes.decode():
            raise ValueError(f'native assets must not carry emulation timers: {name}')
    return pinned


def verify(research):
    generated = generate(research)
    for path, data in generated.items():
        if not path.exists():
            raise ValueError(f'committed Sevilla fixture is missing: {path.name}')
        if path.read_bytes() != data:
            raise ValueError(f'committed Sevilla fixture differs: {path.name}')
    pinned = verify_local()
    campaign = json.loads((CAMPAIGN_PATH).read_text())
    caller = json.loads((CALLER_PATH).read_text())
    witnessed = campaign['witnessed']['conditions']
    replayed = [
        (call['input']['player'], call['output']['condition'])
        for call in caller['condition']['calls']
        if call['context'] != 0
    ]
    if [(c['player'], c['condition_after']) for c in witnessed] != replayed:
        raise ValueError('campaign witnessed conditions differ from the caller record')
    if campaign['witnessed']['score'] != caller['caller']['frame']['scores']:
        raise ValueError('campaign witnessed score differs from the caller frame')
    if campaign['caller_fixture_sha256'] != sha((CALLER_PATH).read_bytes()):
        raise ValueError('campaign caller digest differs')
    if pinned['presentation_sha256'] != sha(generated[PRESENTATION_PATH]):
        raise ValueError('presentation digest differs')
    if pinned['caller']['fixture_sha256'] != sha(generated[CALLER_PATH]):
        raise ValueError('caller fixture digest differs')
    return {
        'research_commit': COMMIT,
        'fixtures': {str(path.relative_to(ROOT)): sha(data) for path, data in generated.items()},
        'counts': pinned['counts'],
        'scope': 'Witnessed identity, presentation, sealed caller and campaign facts; raw chunks never enter.',
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', required=True, type=Path)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    research = args.research_root.resolve()
    if args.check:
        print(json.dumps(verify(research), indent=2))
    else:
        for path, data in generate(research).items():
            path.parent.mkdir(parents=True, exist_ok=True)
            if path.exists():
                raise FileExistsError(f'retain the accepted fixture; use --check: {path}')
            path.write_bytes(data)
            print(f'{path}: {sha(data)}')


if __name__ == '__main__':
    main()
