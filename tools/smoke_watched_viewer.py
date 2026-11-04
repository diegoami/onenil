#!/usr/bin/env python3
"""[designed] Full native watched browser acceptance;recorded outcomes are assertions only."""
import argparse
import hashlib
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]


def smoke(directory):
    from playwright.sync_api import sync_playwright
    from onenil_engine import onenil_engine as native
    def git(*args):
        return subprocess.check_output(['git', '-C', str(ROOT), *args], text=True).strip()
    if git('status', '--porcelain', '--', 'engine', 'tools/play_watched.py', 'tools/test_play_watched.py', 'tools/smoke_watched_viewer.py', 'web/watched'):
        raise ValueError('commit watched viewer source before recording browser evidence')
    directory.mkdir(parents=True, exist_ok=True)
    number = max((int(re.search(r'T42-r(\d+)', p.name)[1]) for p in directory.glob('T42-r*.json')), default=0) + 1
    stem = f'T42-r{number:04d}'
    record = directory / f'{stem}.json'
    if record.exists():
        raise ValueError('immutable browser record exists')
    began = datetime.now(timezone.utc).isoformat()
    process = subprocess.Popen([sys.executable, str(ROOT / 'tools/play_watched.py'), '--port', '0'], cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    try:
        line = process.stdout.readline().strip()
        if not line.startswith('Native watched preview: '):
            raise RuntimeError(line + process.stderr.read())
        url = line.split(': ', 1)[1]
        for request, code in [
            (Request(url + '/api/start', method='POST'), 403),
            (Request(url + '/api/state', headers={'Host': 'example.invalid'}), 403),
            (Request(url + '/api/start', data=b'x', headers={'X-Onenil-Action':'1'}, method='POST'), 400),
        ]:
            try:
                urlopen(request)
            except HTTPError as error:
                assert error.code == code
            else:
                raise AssertionError('invalid local request accepted')
        assert json.load(urlopen(url + '/api/state'))['phase'] == 'ready'
        errors = []
        with sync_playwright() as pw:
            browser = pw.chromium.launch(headless=True)
            page = browser.new_page(viewport={'width':1440,'height':1100})
            page.on('pageerror', lambda error: errors.append(str(error)))
            page.on('console', lambda message: errors.append(message.text) if message.type == 'error' else None)
            page.goto(url)
            page.wait_for_function("document.querySelectorAll('#players circle').length === 22")
            assert page.locator('#ball').is_visible()
            initial = page.request.get(url + '/api/state').json()
            initial_dots = page.locator('#players').inner_html()
            page.get_by_role('button', name='Kick off', exact=True).click()
            page.wait_for_selector('.event.pass', timeout=30000)
            page.get_by_role('button', name='Pause', exact=True).click()
            paused = page.request.get(url + '/api/state').json()['steps']
            page.wait_for_timeout(350)
            assert page.request.get(url + '/api/state').json()['steps'] == paused
            assert page.locator('#players').inner_html() != initial_dots
            assert 'Play continues' not in page.locator('#commentary').inner_text()
            page.locator('#speed').select_option('8')
            page.get_by_role('button', name='Resume', exact=True).click()
            page.get_by_role('button', name='Continue second half', exact=True).wait_for(timeout=60000)
            half = page.request.get(url + '/api/state').json()
            assert half['phase'] == 'half-time'
            page.wait_for_timeout(250)
            assert page.request.get(url + '/api/state').json()['steps'] == half['steps']
            desktop = directory / f'{stem}-desktop.png'
            page.screenshot(path=str(desktop), full_page=True)
            page.get_by_role('button', name='Continue second half', exact=True).click()
            page.get_by_role('button', name='Match finished', exact=True).wait_for(timeout=60000)
            done = page.request.get(url + '/api/state').json()
            assert done['phase'] == 'full-time' and done['steps'] == done['total'] == 4199
            assert done['view']['score'] == [2,1] and done['view']['clock'] == 93
            assert len(done['events']) == page.locator('.event').count() == 831
            assert page.locator('.event.goal').count() == 3
            for kind in ('pass','tackle','shot','save','substitution'):
                assert page.locator(f'.event.{kind}').count() > 0
            assert page.locator('#score').inner_text() == '2 : 1'
            assert page.locator('#clock').inner_text() == '90+3′'
            assert page.get_by_role('button', name='Match finished', exact=True).is_disabled()
            assert len({e['text'] for e in done['events']}) > 100
            page.set_viewport_size({'width':390,'height':844})
            page.locator('#commentary').evaluate('(e) => { e.scrollTop = e.scrollHeight; }')
            mobile = directory / f'{stem}-mobile.png'
            page.screenshot(path=str(mobile), full_page=True)
            assert page.evaluate('document.documentElement.scrollWidth <= innerWidth')
            assert page.locator('#score').bounding_box()['height'] < 60
            assert page.locator('#pitch').is_visible()
            page.get_by_role('button', name='Restart', exact=True).click()
            page.get_by_role('button', name='Kick off', exact=True).wait_for()
            assert page.request.get(url + '/api/state').json() == initial
            assert page.locator('.event').count() == 0
            assert not errors, errors
            version = browser.version
            browser.close()
        outcome = dict(schema='watched-viewer-browser/1', started=began, code_commit=git('rev-parse','HEAD'),
                       browser=version, debug_assertions=native.__debug_assertions__,
                       extension_sha256=hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),
                       ticks=done['steps'], accepted_actions=len(done['events']), score=done['view']['score'],
                       minute=done['view']['clock'], halftime_tick=half['steps'],
                       checks=['all native actions','named passes/tackles/shots/saves/goals','22 moving players and ball',
                               'pause','halftime pause/continue','complete post-whistle run','live score','restart',
                               'desktop/mobile','local-host and action-header guards','no console errors'],
                       screenshots={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (desktop,mobile)},
                       scope='One freshly computed native post-setup reference match;lineup/seed setup remains required.')
        record.write_text(json.dumps(outcome,indent=2)+'\n')
        print(record.relative_to(ROOT), flush=True)
    finally:
        process.terminate()
        process.wait(timeout=10)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,default=ROOT/'records/T42')
    smoke(parser.parse_args().out)
