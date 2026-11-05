#!/usr/bin/env python3
"""Native selected-squad watched preview: real computed actions, clock and pitch."""
import argparse
import hashlib
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PREPARED_SHA = '842f1453cae4ba9ab626d7e57316494cfe7e70f64ea80e746c3834de0e12989f'
STATIC_INPUT = ROOT/'engine/tests/fixtures/prepared_choices/initial_static-r0002.json'
STATIC_INPUT_SHA = 'a135cc41ee516bf07e518036323ba62638ed7d71a21376ab4c0653328c4e87eb'


def validate_choices(choices):
    if (type(choices) is not dict or set(choices) != {'lineup', 'seed'}
            or choices['lineup'] not in ('reference', 'reserve')
            or choices['seed'] not in ('64', '122')):
        raise ValueError('choose a supported lineup and seed')
    return choices.copy()


def prepared_input(choices):
    choices = validate_choices(choices)
    raw = (ROOT/'engine/tests/fixtures/prepared_match/admission.json').read_bytes()
    if hashlib.sha256(raw).hexdigest() != PREPARED_SHA:
        raise ValueError('prepared initial admission digest differs')
    value = json.loads(raw)['input']
    setup = value['preparation']['setup']
    if choices['lineup'] == 'reserve':
        # [designed] Same supported initial roster choice as T28/T29. Only
        # structural aliases/raw witnesses change;Rust owns all subsequent work.
        original = setup['legacy']['substitution']['context']['clubs'][0]['roster']
        roster = original.copy()
        roster[9], roster[12] = roster[12], roster[9]

        def link(item):
            if isinstance(item, list):
                if len(item) >= 11 and item == original[:len(item)]:
                    return roster[:len(item)]
                return [link(v) for v in item]
            if isinstance(item, dict):
                return {k:link(v) for k,v in item.items()}
            return item
        setup = link(setup)
        pool = {p['id']:p for p in setup['legacy']['players']}
        for rating in setup['legacy']['ratings']:
            rating['players'] = [dict(id=identifier,
                **{f'p8_{k}':pool[identifier]['p8'][k] for k in (4,6,7)},
                **{f'p15_{k}':pool[identifier]['p15'][k] for k in (6,8,11)})
                for identifier in sorted(set(rating['roster'])) if identifier]
        value['preparation']['setup'] = setup
    assert setup['legacy']['config'][0] == 6
    setup['legacy']['config'][1] = {'64':1, '122':2}[choices['seed']]
    if choices != dict(lineup='reference', seed='64'):
        # Initial source-static numeric mappings,not captured match results.
        raw = STATIC_INPUT.read_bytes()
        if hashlib.sha256(raw).hexdigest() != STATIC_INPUT_SHA:
            raise ValueError('initial numeric table digest differs')
        tables = json.loads(raw)
        if tables['schema'] != 'initial-source-numeric-tables/1' or tables['ranges'] != [[0x0D36,0x0D56],[0x935B,0x9A2A]]:
            raise ValueError('initial numeric table shape differs')
        known = {(f['area'],f['offset']):f['value'] for f in value['support']}
        for fact in tables['facts']:
            key = fact['area'], fact['offset']
            if key in known:
                if known[key] != fact['value']:
                    raise ValueError('initial numeric table alias differs')
            else:
                value['support'].append(fact)
                known[key] = fact['value']
    return value


def compute_prepared(choices):
    from onenil_engine import NativePreparedWatchedSession
    value = prepared_input(choices)
    session = NativePreparedWatchedSession(json.dumps(value, separators=(',', ':')))
    initial = json.loads(session.snapshot())
    setup_draws = json.loads(session.observation())['draws']
    if not setup_draws or setup_draws[0]['seed_before'] != int(choices['seed']):
        raise ValueError('computed initial seed handoff differs')
    starter = 2590 if choices['lineup'] == 'reference' else 2593
    if starter not in [p['word14'] for p in initial['scenes']]:
        raise ValueError('selected starter missing from native first entry')
    packet = json.loads(session.finish())
    if set(packet) != {'ok'}:
        fault = packet.get('error', {})
        raise ValueError('native prepared match failed: '+str(fault.get('message'))
                         +';missing='+str(fault.get('missing')))
    output = packet['ok']
    if output['return'] != '0AB9:EBE2' or output['draws'][:len(setup_draws)] != setup_draws:
        raise ValueError('prepared native continuation/return differs')
    return output, initial, setup_draws

# [derived] Original09E8/09EB indexes DS0382+21*code;09A06..0AD2 uses
# marker1=primary,marker2=secondary. Staticd9e20a4/data/ds_2864_01.hex.
# [designed] New English wording;no original template/name asset is loaded.
MESSAGES = [
    ('substitution', '{p} replaces {s}.'),
    ('pass', '{p} sends a pass.'),
    ('pass', '{p} plays a pass.'),
    ('throw', '{p} throws the ball out.'),
    ('kick', '{p} kicks it away.'),
    ('kick', '{p} sends a long kick upfield.'),
    ('run', '{p} carries the ball forward.'),
    ('touch', '{p} takes a loose touch.'),
    ('receive', 'A good ball is played towards {s}.'),
    ('receive', 'A long pass is aimed at {s}.'),
    ('receive', 'A high ball is sent towards {s}.'),
    ('tackle', '{p} goes in for a tackle.'),
    ('evade', '{p} gets away from the challenge.'),
    ('dive', '{p} goes into a dive.'),
    ('win', '{p} takes possession.'),
    ('cross', '{p} delivers a cross.'),
    ('cross', '{p} crosses the ball in.'),
    ('shot', '{p} shoots at goal.'),
    ('shot', '{p} strikes a volley.'),
    ('shot', '{p} gets a quick shot away.'),
    ('shot', '{p} heads towards goal.'),
    ('clearance', '{p} clears the danger.'),
    ('clearance', '{p} clears it with a header.'),
    ('clearance', '{p} punts the ball away.'),
    ('clearance', '{p} heads the ball away.'),
    ('possession', '{p} has possession.'),
    ('post', '{p} strikes the post!'),
    ('bar', '{p} strikes the crossbar!'),
    ('possession', '{p} holds the ball.'),
    ('save', '{p} makes the save.'),
    ('block', '{p} gets in the way and blocks it.'),
    ('deflection', 'The ball takes a deflection off {p}.'),
    ('header', '{p} nods the ball on.'),
    ('touch', '{p} nudges the ball on.'),
    ('interception', '{p} cuts out the ball.'),
    ('collection', '{p} gathers the ball.'),
    ('beaten', 'The play gets past {p}.'),
    ('beaten', '{p} cannot reach it.'),
    ('goal', 'Goal! {p} scores.'),
    ('own-goal', 'Own goal by {p}!'),
    ('miss', 'The attempt clears the crossbar.'),
    ('miss', 'The attempt goes just outside the post.'),
    ('throw', '{p} prepares to take the throw.'),
    ('throw', 'A throw-in is awarded.'),
    ('corner', '{p} prepares to take the corner.'),
    ('corner', 'A corner is awarded.'),
    ('goal-kick', '{p} prepares to take the goal kick.'),
    ('penalty', '{p} steps up for the penalty.'),
    ('free-kick', '{p} prepares to take the free kick.'),
    ('kickoff', '{p} gets ready for kickoff.'),
    ('booking', 'The referee cautions {p}.'),
    ('dismissal', 'The referee dismisses {p}.'),
    ('injury', '{p} needs attention for an injury.'),
    ('foul', '{p} commits a foul.'),
    ('offside', '{p} is caught offside.'),
    ('fouled', '{p} is brought to the ground.'),
    ('goal-kick', 'A goal kick is awarded.'),
    ('penalty', 'The referee awards a penalty!'),
    ('free-kick', 'A free kick is awarded.'),
    ('toss', '{p} wins the coin toss.'),
    ('penalty', '{p} prepares to shoot.'),
    ('penalties', 'The match goes to a penalty shootout.'),
    ('extra-time', 'Extra time begins.'),
    ('full-time', 'The referee ends the match.'),
    ('half-time', 'The referee ends the first half.'),
    ('challenge', '{p} challenges for the ball.'),
    ('advantage', 'The referee allows the advantage.'),
    ('flag', 'The assistant referee raises the flag.'),
]


def describe(event, names):
    code = event['code']
    if type(code) is not int or not 0 <= code < len(MESSAGES):
        raise ValueError(f'unsupported native commentary code {code}')
    kind, template = MESSAGES[code]
    values = {}
    for marker, field in [('p', 'player'), ('s', 'secondary')]:
        if '{' + marker + '}' in template:
            identifier = event[field]
            if not identifier or identifier not in names:
                raise ValueError(f'missing commentary identity {identifier}')
            values[marker] = names[identifier]
    return dict(event, kind=kind, text=template.format(**values))


def coordinate(word):
    def signed(n):
        return n - 256 if n >= 128 else n
    return dict(x=signed(word & 255), y=signed((word >> 8) & 255))


class Match:
    def __init__(self, choices=None, background=False):
        from threading import RLock
        self._lock = RLock()
        self.background = background
        self._cache = {}
        self.meta = json.loads((ROOT/'demo/metadata.json').read_text())
        choices = dict(lineup='reference', seed='64') if choices is None else validate_choices(choices)
        initial = prepared_input(choices)
        self.clubs = [c['id'] for c in initial['preparation']['setup']['legacy']['substitution']['context']['clubs']]
        self.phase = 'ready'
        self.choices = None
        self.configure(choices)

    def _build(self, choices):
        output, initial, draws = compute_prepared(choices)
        setup = prepared_input(choices)['preparation']['setup']
        player_clubs = {p['id']:p['p8'][0] for p in setup['legacy']['players']}
        names = {int(k):v for k,v in self.meta['players'].items()}
        def view(snapshot, goals, clock):
            players = []
            for s in snapshot['scenes']:
                player = s['word14']
                if player not in names or player_clubs.get(player) not in self.clubs:
                    raise ValueError(f'missing native player/club identity {player}')
                players.append(dict(scene=s['index'], player=player, club=player_clubs[player],
                                    focus=s['index'] == snapshot['actor'], **coordinate(s['word0'])))
            if len(players) != 22:
                raise ValueError('missing native player positions')
            return dict(clock=clock, score=goals, ball=coordinate(snapshot['ball']['word0']), players=players)
        setup_events = [dict(describe(e, names), id=i) for i,e in enumerate(output['setup_events'])]
        events = {}
        for i,event in enumerate(output['events'], start=len(setup_events)):
            events.setdefault(event['tick'], []).append(dict(describe(event, names), id=i))
        if len(output['tick_live_goals']) != len(output['ticks']):
            raise ValueError('missing native live-goal observations')
        clocks = {r['tick']:r['clock']['elapsed'] for r in output['caller_clocks']}
        frames = []
        for tick, goals in zip(output['ticks'], output['tick_live_goals']):
            actions = events.pop(tick['tick'], [])
            clock = clocks.get(tick['tick'], tick['after']['elapsed'])
            current = view(tick['after'], goals, clock)
            if any(e['minute'] > clock for e in actions):
                raise ValueError('native event is newer than its observed caller clock')
            frames.append(dict(view=current, events=actions))
        if events or not frames or frames[-1]['view']['score'] != output['live_goals']:
            raise ValueError('native action/live-score timeline differs')
        return dict(frames=frames, initial=view(initial, initial['scores'], initial['elapsed']),
                    setup_events=setup_events, selection=dict(starter=2590 if choices['lineup']=='reference' else 2593,
                    seed=draws[0]['seed_before'], players=[s['word14'] for s in initial['scenes']], setup_draws=len(draws)),
                    native_return=output['return'])

    def _install(self, data):
        self._data = data
        self.frames = data['frames']
        self.initial = data['initial']
        self.selection = data['selection']
        self.cursor = 0
        self.phase = 'ready'
        self.events = []
        self.full_whistle_seen = False
        self.view = self.initial
        self.error = None

    def _prepare(self, choices, key):
        try:
            data = self._build(choices)
            with self._lock:
                self._cache[key] = data
                self._install(data)
        except Exception as error:
            with self._lock:
                self.phase = 'error'
                self.error = str(error)

    def configure(self, choices):
        choices = validate_choices(choices)
        with self._lock:
            if self.phase not in ('ready', 'full-time', 'error'):
                raise ValueError('restart before changing the lineup or seed')
            key = choices['lineup'], choices['seed']
            self.choices = choices
            self.error = None
            if key in self._cache:
                self._install(self._cache[key])
            else:
                self.phase = 'preparing'
                self.frames = []
                self.cursor = 0
                self.events = []
                self.view = None
                self.selection = None
                if self.background:
                    from threading import Thread
                    self._worker = Thread(target=self._prepare, args=(choices,key), daemon=True)
                    self._worker.start()
                else:
                    self._prepare(choices, key)
            return self.state()

    def restart(self):
        with self._lock:
            if self.phase == 'preparing':
                raise ValueError('wait for match preparation')
            if self.phase == 'error':
                return self.configure(self.choices)
            self._install(self._data)
            return self.state()

    def state(self):
        with self._lock:
            return dict(phase=self.phase, steps=self.cursor, total=len(self.frames), view=self.view,
                        events=self.events, meta=self.meta, clubs=self.clubs, choices=self.choices.copy(),
                        selection=self.selection, error=self.error)

    def action(self, action):
        with self._lock:
            if action == 'restart':
                return self.restart()
            if action == 'start' and self.phase == 'ready':
                self.phase = 'first-half'
                self.events.extend(self._data['setup_events'])
            elif action == 'continue' and self.phase == 'half-time':
                self.phase = 'second-half'
            elif action == 'step' and self.phase in ('first-half', 'second-half'):
                for _ in range(8):
                    if self.cursor == len(self.frames):
                        raise ValueError('native playback ended without fulltime event')
                    frame = self.frames[self.cursor]
                    self.cursor += 1
                    self.view = frame['view']
                    self.events.extend(frame['events'])
                    if any(e['kind']=='full-time' for e in frame['events']):
                        self.full_whistle_seen = True
                    if self.cursor == len(self.frames):
                        if not self.full_whistle_seen:
                            raise ValueError('native playback ended without fulltime event')
                        self.phase = 'full-time'
                        break
                    if any(e['kind']=='half-time' for e in frame['events']):
                        self.phase = 'half-time'
                        break
            else:
                raise ValueError('action unavailable in current match phase')
            return self.state()


def serve(port):
    match = Match(background=True)
    assets = {'/': ('index.html', 'text/html; charset=utf-8'),
              '/app.js': ('app.js', 'text/javascript; charset=utf-8'),
              '/style.css': ('style.css', 'text/css; charset=utf-8')}
    class Handler(BaseHTTPRequestHandler):
        def reply(self, status, data, mime='application/json'):
            payload = json.dumps(data).encode() if mime == 'application/json' else data
            self.send_response(status)
            self.send_header('Content-Type', mime)
            self.send_header('Content-Length', str(len(payload)))
            self.send_header('Cache-Control', 'no-store')
            self.send_header('X-Content-Type-Options', 'nosniff')
            self.end_headers()
            self.wfile.write(payload)
        def local(self):
            return self.headers.get('Host') in (f'127.0.0.1:{self.server.server_port}', f'localhost:{self.server.server_port}')
        def do_GET(self):
            if not self.local():return self.reply(403, {'error':'localhost only'})
            if self.path == '/api/state':return self.reply(200, match.state())
            if self.path in assets:
                name,mime=assets[self.path]
                return self.reply(200, (ROOT/'web/watched'/name).read_bytes(), mime)
            self.reply(404, {'error':'not found'})
        def do_POST(self):
            if not self.local() or self.headers.get('X-Onenil-Action') != '1':
                return self.reply(403, {'error':'same-origin action header required'})
            try:length=int(self.headers.get('Content-Length','0'))
            except ValueError:return self.reply(400, {'error':'invalid request length'})
            if self.path == '/api/configure':
                if not 0 < length <= 256 or self.headers.get('Content-Type') != 'application/json':
                    return self.reply(400, {'error':'choose a supported lineup and seed'})
                try:choices=validate_choices(json.loads(self.rfile.read(length)))
                except (ValueError,UnicodeError):return self.reply(400, {'error':'choose a supported lineup and seed'})
                try:return self.reply(200, match.configure(choices))
                except ValueError as error:return self.reply(409, {'error':str(error)})
            if self.path not in ('/api/start','/api/step','/api/continue','/api/restart') or length != 0:
                return self.reply(400, {'error':'unsupported request'})
            try:self.reply(200, match.action(self.path.removeprefix('/api/')))
            except ValueError as error:self.reply(409, {'error':str(error)})
        def log_message(self,*_):pass
    server=HTTPServer(('127.0.0.1',port),Handler)
    print(f'Native watched preview: http://127.0.0.1:{server.server_port}',flush=True)
    try:server.serve_forever()
    except KeyboardInterrupt:pass
    finally:server.server_close()


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port',type=int,default=8041)
    serve(parser.parse_args().port)
