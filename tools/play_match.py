#!/usr/bin/env python3
"""Local native match preview. Python presents data;Rust advances the match."""
import argparse
import hashlib
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


class Match:
    def __init__(self):
        from onenil_engine import NativeExhibitionSession
        self.factory = NativeExhibitionSession
        self.preset = (ROOT / 'demo/exhibition.json').read_text()
        self.meta = json.loads((ROOT / 'demo/metadata.json').read_text())
        if hashlib.sha256(self.preset.encode()).hexdigest() != self.meta['exhibition_sha256']:
            raise ValueError('selected-squad admission digest differs')
        self.choices = {"lineup":"reference", "seed":"64"}
        self.restart()

    def initial_data(self, choices):
        data = json.loads(self.preset)
        if choices['lineup'] == 'reserve':
            # Structural aliases of the supplied Blackburn roster: reserve2593
            # replaces starter2590. Rust computes all subsequent scene/role work.
            original = data['legacy']['substitution']['context']['clubs'][0]['roster']
            roster = original.copy()
            roster[9], roster[12] = roster[12], roster[9]
            def link(value):
                if isinstance(value, list):
                    if len(value) >= 11 and value == original[:len(value)]:
                        return roster[:len(value)]
                    return [link(item) for item in value]
                if isinstance(value, dict):
                    return {key:link(item) for key,item in value.items()}
                return value
            data = link(data)
            # Ratings require raw player witnesses for the selected eleven.
            pool = {p['id']:p for p in data['legacy']['players']}
            for rating in data['legacy']['ratings']:
                rating['players'] = [dict(id=identifier,
                    **{f'p8_{k}':pool[identifier]['p8'][k] for k in (4,6,7)},
                    **{f'p15_{k}':pool[identifier]['p15'][k] for k in (6,8,11)})
                    for identifier in sorted(set(rating['roster'])) if identifier]
        data['legacy']['config'][1] = {'64':1, '122':2}[choices['seed']]
        return json.dumps(data, separators=(',', ':'))

    def configure(self, choices):
        if self.started:
            raise ValueError('restart before changing the lineup or seed')
        if (not isinstance(choices,dict) or set(choices) != {'lineup','seed'}
                or choices['lineup'] not in ('reference','reserve')
                or choices['seed'] not in ('64','122')):
            raise ValueError('choose a supported lineup and seed')
        # Construct first;an unsupported native path cannot replace a live session.
        session = self.factory(self.initial_data(choices))
        self.session, self.choices = session, choices.copy()
        return self.state()

    def restart(self):
        self.session = self.factory(self.initial_data(self.choices))
        self.started = False
        self.events = []
        self.steps = 0
        return self.state()

    def name(self, player):
        return self.meta['players'].get(str(player), f'Player {player}')

    def club(self, club):
        return self.meta['clubs'].get(str(club), f'Club {club}')

    def add(self, kind, minute, text, **extra):
        self.events.append(dict(id=len(self.events), kind=kind, minute=minute, text=text, **extra))

    def state(self):
        view = json.loads(self.session.view_state())
        return dict(view=view, phase=view['phase'] if self.started else 'ready',
                    events=self.events.copy(), meta=self.meta, steps=self.steps, choices=self.choices.copy())

    def action(self, action):
        if action == 'restart':
            return self.restart()
        if action == 'start':
            if self.started:
                raise ValueError('match has already started')
            self.started = True
            self.add('kickoff', 0, 'Kickoff. The first half is under way.')
            return self.state()
        if not self.started:
            raise ValueError('start the match first')
        before = json.loads(self.session.view_state())
        if action == 'continue':
            self.session.continue_without_changes()
            self.add('kickoff', before['clock']['elapsed'], 'Second half. Both teams continue without lineup changes.')
        elif action == 'step':
            # No state,seed,expected result or fixture enters this call.
            transition = json.loads(self.session.step())
            self.steps += 1
            after = json.loads(self.session.view_state())
            minute = before['clock']['elapsed']
            event_count = len(self.events)
            for side, logs in enumerate(after['scorer_logs']):
                for goal in logs[len(before['scorer_logs'][side]):]:
                    # Scorer arrays use header order,opposite the DD02 score order.
                    club = after['clubs'][1 - side]
                    self.add('goal', goal['minute_byte'], f"GOAL! {self.name(goal['player'])} scores for {self.club(club)}.",
                             player=goal['player'], club=club)
            for call in transition['ai']:
                for helper in call['helpers']:
                    if helper['kind'] != 'replace':
                        continue
                    scene = helper['arguments'][1]
                    old = next(s['player'] for s in helper['input']['scenes'] if s['index'] == scene)
                    new = next(s['player'] for s in helper['output']['scenes'] if s['index'] == scene)
                    side = helper['arguments'][2]
                    club = helper['output']['clubs'][side]['id']
                    self.add('substitution', minute,
                             f'{self.club(club)} change: {self.name(new)} replaces {self.name(old)}.',
                             incoming=new, outgoing=old, club=club)
            phase = after['phase']
            if phase == 'half-time':
                self.add('half-time', after['clock']['elapsed'], 'Half-time. Continue when ready;the lineups will stay unchanged.')
            elif phase == 'full-time':
                score = after['score_side']
                self.add('full-time', after['clock']['elapsed'], f'Full-time. {self.club(after["clubs"][0])} {score[0]}–{score[1]} {self.club(after["clubs"][1])}.')
            elif event_count == len(self.events):
                self.add('clock', after['clock']['elapsed'], 'Play continues. The score is unchanged.')
        else:
            raise ValueError('unknown action')
        return self.state()


def serve(port=8040):
    match = Match()
    assets = {'/': ('web/index.html', 'text/html; charset=utf-8'),
              '/app.js': ('web/app.js', 'text/javascript; charset=utf-8'),
              '/style.css': ('web/style.css', 'text/css; charset=utf-8')}

    class Handler(BaseHTTPRequestHandler):
        def reply(self, status, data, content_type='application/json'):
            payload = json.dumps(data).encode() if content_type == 'application/json' else data
            self.send_response(status)
            self.send_header('Content-Type', content_type)
            self.send_header('Content-Length', str(len(payload)))
            self.send_header('Cache-Control', 'no-store')
            self.send_header('X-Content-Type-Options', 'nosniff')
            self.end_headers()
            self.wfile.write(payload)

        def local_host(self):
            return self.headers.get('Host') in (f'127.0.0.1:{self.server.server_port}', f'localhost:{self.server.server_port}')

        def do_GET(self):
            if not self.local_host():
                return self.reply(403, {'error': 'localhost only'})
            if self.path == '/api/state':
                return self.reply(200, match.state())
            if self.path in assets:
                name, mime = assets[self.path]
                return self.reply(200, (ROOT / name).read_bytes(), mime)
            self.reply(404, {'error': 'not found'})

        def do_POST(self):
            if not self.local_host() or self.headers.get('X-Onenil-Action') != '1':
                return self.reply(403, {'error': 'same-origin action header required'})
            if self.path == '/api/configure':
                try:
                    length = int(self.headers.get('Content-Length', '0'))
                    if not 0 < length <= 1024:
                        return self.reply(400, {'error':'unsupported configuration size'})
                    if self.headers.get('Content-Type') != 'application/json':
                        return self.reply(400, {'error':'JSON configuration required'})
                    choices = json.loads(self.rfile.read(length))
                    result = match.configure(choices)
                except (ValueError, UnicodeError) as error:
                    return self.reply(409, {'error':str(error)})
            else:
                if self.path not in ('/api/start', '/api/step', '/api/continue', '/api/restart') or self.headers.get('Content-Length', '0') != '0':
                    return self.reply(400, {'error': 'unsupported request'})
                try:
                    result = match.action(self.path.removeprefix('/api/'))
                except ValueError as error:
                    return self.reply(409, {'error': str(error)})
            self.reply(200, result)

        def log_message(self, *_):
            pass

    server = HTTPServer(('127.0.0.1', port), Handler)
    print(f'Native match preview: http://127.0.0.1:{server.server_port}', flush=True)
    print('Choose a supported lineup and seed;native setup runs on restart. Ctrl-C stops the local server.', flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8040)
    serve(parser.parse_args().port)
