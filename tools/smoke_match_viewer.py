#!/usr/bin/env python3
"""Drive the actual local viewer through a complete native match in Chromium."""
import argparse
import hashlib
import json
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]


def smoke(out):
    from playwright.sync_api import sync_playwright
    from onenil_engine import onenil_engine as native
    def git(*args):
        return subprocess.check_output(['git', '-C', str(ROOT), *args], text=True).strip()
    if git('status', '--porcelain', '--', 'engine/src', 'tools/play_match.py', 'tools/smoke_match_viewer.py', 'web', 'demo'):
        raise ValueError('commit viewer source before recording smoke')
    out.mkdir(parents=True, exist_ok=True)
    num = max((int(p.stem.split('-r')[-1]) for p in out.glob('T27-r*.json')), default=0) + 1
    stem = f'T27-r{num:04d}'
    process = subprocess.Popen([sys.executable, str(ROOT/'tools/play_match.py'), '--port', '0'],
                               cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    started = datetime.now(timezone.utc).isoformat()
    try:
        first = process.stdout.readline().strip()
        if not first.startswith('Native match preview: '):
            raise RuntimeError(first + process.stderr.read())
        url = first.split(': ', 1)[1]
        # Missing same-origin action header must not mutate a local session.
        try: urlopen(Request(url+'/api/start', method='POST'))
        except HTTPError as error: assert error.code == 403
        else: raise AssertionError('unprotected action accepted')
        errors = []
        with sync_playwright() as pw:
            browser = pw.chromium.launch(headless=True)
            page = browser.new_page(viewport={'width':1440,'height':1100}, device_scale_factor=1)
            page.on('pageerror', lambda error: errors.append(str(error)))
            page.on('console', lambda message: errors.append(message.text) if message.type == 'error' else None)
            page.goto(url)
            page.get_by_role('button', name='Kick off', exact=True).wait_for()
            page.wait_for_function("document.querySelectorAll('#players circle').length === 22")
            assert page.locator('.player-name').count() == 22
            page.get_by_role('button', name='Kick off', exact=True).click()
            page.wait_for_selector('.event.clock')
            page.get_by_role('button', name='Pause', exact=True).click()
            paused = json.loads(page.request.get(url+'/api/state').text())['steps']
            page.wait_for_timeout(450)
            assert json.loads(page.request.get(url+'/api/state').text())['steps'] == paused
            page.locator('#speed').select_option('30')
            page.get_by_role('button', name='Resume', exact=True).click()
            page.get_by_role('button', name='Continue second half', exact=True).wait_for(timeout=30000)
            half = json.loads(page.request.get(url+'/api/state').text())
            page.wait_for_timeout(250)
            assert json.loads(page.request.get(url+'/api/state').text())['steps'] == half['steps']
            desktop = out/f'{stem}-desktop.png'
            page.screenshot(path=str(desktop), full_page=True)
            page.get_by_role('button', name='Continue second half', exact=True).click()
            page.wait_for_selector('.event.full-time', timeout=30000)
            assert page.locator('.event.goal').count() == 4
            assert page.locator('.event.substitution').count() == 2
            done = json.loads(page.request.get(url+'/api/state').text())
            assert done['phase'] == 'full-time' and done['steps'] == 95
            assert done['view']['score_side'] == [1,3]  # assertion only
            assert page.locator('#score').inner_text() == '1 : 3'
            assert page.get_by_role('button', name='Match finished', exact=True).is_disabled()
            page.set_viewport_size({'width':390,'height':844})
            mobile = out/f'{stem}-mobile.png'
            page.screenshot(path=str(mobile), full_page=True)
            assert page.evaluate('document.documentElement.scrollWidth <= innerWidth')
            assert page.locator('#pitch').is_visible()
            page.get_by_role('button', name='Restart', exact=True).click()
            page.get_by_role('button', name='Kick off', exact=True).wait_for()
            assert page.locator('#score').inner_text() == '0 : 0'
            assert page.locator('.event').count() == 0
            assert page.locator('.player-name').count() == 22
            assert not errors, errors
            browser_version = browser.version
            browser.close()
        receipt = {'schema':'viewer-smoke/1','run_id':stem,'code_commit':git('rev-parse','HEAD'),
                   'started':started,'ended':datetime.now(timezone.utc).isoformat(),'browser':browser_version,
                   'extension_sha256':hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),
                   'checks':['22 live formation players','kickoff','pause retains step count','pace change','explicit halftime stop/continue','95 native steps','4 goals/2 substitutions','native final score','desktop/mobile render','no horizontal mobile overflow','restart resets score/events','no console errors','action header guard'],
                   'steps':done['steps'],'result':done['view']['score_side'],
                   'screenshots':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [desktop,mobile]},
                   'limits':'One prepared reference after setup;formation projection only,no watched-engine/ball movement or formal playable acceptance'}
        with (out/f'{stem}.json').open('x') as stream:
            json.dump(receipt,stream,indent=2);stream.write('\n')
        print(f'Browser full-match/restart smoke passed: {out/stem}.json')
    finally:
        process.terminate()
        try:process.wait(timeout=5)
        except subprocess.TimeoutExpired:process.kill();process.wait()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,default=ROOT/'records/T27')
    smoke(parser.parse_args().out)
