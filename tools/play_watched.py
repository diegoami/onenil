#!/usr/bin/env python3
"""Native watched reference preview: real computed actions, clock and pitch."""
import argparse
import hashlib
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INITIAL_SHA = '88274a14f94fb526c1f0777d06017355b73e41946aabaafc7d65233da5492311'

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
    ('toss', '{p} takes the coin toss.'),
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
    def __init__(self):
        import onenil_engine as native
        raw = (ROOT / 'engine/tests/fixtures/watched_match/initial.json').read_bytes()
        if hashlib.sha256(raw).hexdigest() != INITIAL_SHA:
            raise ValueError('watched initial admission digest differs')
        admission = json.loads(raw)
        request = {key: admission[key] for key in ('initial', 'no_pending_key')}
        result = json.loads(native.watched_match(json.dumps(request)))
        if 'ok' not in result:
            raise ValueError(result['error']['message'])
        output = result['ok']
        self.meta = json.loads((ROOT / 'demo/metadata.json').read_text())
        names = {int(key): name for key, name in self.meta['players'].items()}
        cells = {(f['area'], f['offset']): f['value'] for f in admission['initial']['cells']}
        self.clubs = [cells['ds', 0xD6E4 + 2 * side] + 256 * cells['ds', 0xD6E5 + 2 * side] for side in range(2)]
        player_clubs = {offset // 8: value for (area, offset), value in cells.items() if area == 'p8' and offset % 8 == 0}
        self.frames = []
        events = {}
        for index, event in enumerate(output['events']):
            events.setdefault(event['tick'], []).append(dict(describe(event, names), id=index))
        if len(output['tick_live_goals']) != len(output['ticks']):
            raise ValueError('missing native live-goal observations')

        def view(snapshot, goals):
            return dict(clock=snapshot['elapsed'], score=goals, ball=coordinate(snapshot['ball']['word0']),
                        players=[dict(scene=s['index'], player=s['word14'], club=player_clubs[s['word14']],
                                      focus=s['index'] == snapshot['actor'], **coordinate(s['word0']))
                                 for s in snapshot['scenes']])

        self.initial = view(output['ticks'][0]['before'], [0, 0])
        for tick, goals in zip(output['ticks'], output['tick_live_goals']):
            self.frames.append(dict(view=view(tick['after'], goals), events=events.pop(tick['tick'], [])))
        if events or not self.frames or self.frames[-1]['view']['score'] != output['live_goals']:
            raise ValueError('native action/live-score timeline differs')
        # [designed] Only this process's freshly computed native result is cached.
        # Restart rewinds its playback;no recorded original outcomes are opened.
        self.restart()

    def restart(self):
        self.cursor = 0
        self.phase = 'ready'
        self.events = []
        self.view = self.initial
        return self.state()

    def state(self):
        return dict(phase=self.phase, steps=self.cursor, total=len(self.frames),
                    view=self.view, events=self.events, meta=self.meta, clubs=self.clubs)

    def action(self, action):
        if action == 'restart':
            return self.restart()
        if action == 'start' and self.phase == 'ready':
            self.phase = 'first-half'
        elif action == 'continue' and self.phase == 'half-time':
            self.phase = 'second-half'
        elif action == 'step' and self.phase in ('first-half', 'second-half'):
            # [designed] Playback batch only;every native frame/event is retained.
            for _ in range(8):
                if self.cursor == len(self.frames):
                    if self.phase != 'full-time':
                        raise ValueError('native playback ended without fulltime event')
                    break
                frame = self.frames[self.cursor]
                self.cursor += 1
                self.view = frame['view']
                self.events.extend(frame['events'])
                if any(e['kind'] == 'full-time' for e in frame['events']):
                    self.phase = 'full-time'
                    break
                if any(e['kind'] == 'half-time' for e in frame['events']):
                    self.phase = 'half-time'
                    break
        else:
            raise ValueError('action unavailable in current match phase')
        return self.state()


def serve(port):
    match = Match()
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
            if not self.local():
                return self.reply(403, {'error': 'localhost only'})
            if self.path == '/api/state':
                return self.reply(200, match.state())
            if self.path in assets:
                name, mime = assets[self.path]
                return self.reply(200, (ROOT / 'web/watched' / name).read_bytes(), mime)
            self.reply(404, {'error': 'not found'})

        def do_POST(self):
            if not self.local() or self.headers.get('X-Onenil-Action') != '1':
                return self.reply(403, {'error': 'same-origin action header required'})
            if self.path not in ('/api/start', '/api/step', '/api/continue', '/api/restart') or self.headers.get('Content-Length', '0') != '0':
                return self.reply(400, {'error': 'unsupported request'})
            try:
                self.reply(200, match.action(self.path.removeprefix('/api/')))
            except ValueError as error:
                self.reply(409, {'error': str(error)})

        def log_message(self, *_):
            pass

    server = HTTPServer(('127.0.0.1', port), Handler)
    print(f'Native watched preview: http://127.0.0.1:{server.server_port}', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8041)
    serve(parser.parse_args().port)
