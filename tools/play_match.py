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
        from onenil_engine import NativeMatchSession
        self.factory = NativeMatchSession
        self.preset = (ROOT / 'demo/reference.json').read_text()
        self.meta = json.loads((ROOT / 'demo/metadata.json').read_text())
        if hashlib.sha256(self.preset.encode()).hexdigest() != self.meta['admission_sha256']:
            raise ValueError('prepared admission digest differs')
        self.restart()

    def restart(self):
        self.session = self.factory(self.preset)
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
                    events=self.events.copy(), meta=self.meta, steps=self.steps)

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
    print('One prepared session after setup;Ctrl-C stops the local server.', flush=True)
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
