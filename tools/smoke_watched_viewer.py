#!/usr/bin/env python3
"""[designed] Prepared browser acceptance;reference records are assertions only."""
import argparse
import hashlib
import json
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

ROOT=Path(__file__).resolve().parents[1]

def next_stem(directory, task='T42'):
    numbers=[]
    for path in directory.glob(task+'-r*'):
        match=re.fullmatch(re.escape(task)+r'-r(\d+)(?:\.json|-.*\.png)',path.name)
        if match:numbers.append(int(match[1]))
    return f'{task}-r{max(numbers,default=0)+1:04d}'

def smoke(directory):
    from playwright.sync_api import sync_playwright
    from onenil_engine import onenil_engine as native
    def git(*args):
        return subprocess.check_output(['git','-C',str(ROOT),*args],text=True).strip()
    if git('status','--porcelain','--','engine','tools','web/watched'):
        raise ValueError('commit prepared viewer sources before browser evidence')
    directory=directory.resolve();directory.mkdir(parents=True,exist_ok=True)
    stem=next_stem(directory,'T46');record=directory/f'{stem}.json'
    assert not record.exists()
    expected=json.loads((ROOT/'records/T46/native-completions-r0004.json').read_text())['sessions']
    process=subprocess.Popen([sys.executable,str(ROOT/'tools/play_watched.py'),'--port','0'],cwd=ROOT,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    screenshots={};cases=[];errors=[];preparations=[];began=datetime.now(timezone.utc).isoformat()
    def api(url):
        return json.load(urlopen(url+'/api/state'))
    try:
        line=process.stdout.readline().strip()
        if not line.startswith('Native watched preview: '):raise RuntimeError(line+process.stderr.read())
        url=line.split(': ',1)[1]
        initial=api(url)
        assert initial['phase'] in ('preparing','ready')
        for request,code in [
            (Request(url+'/api/start',method='POST'),403),
            (Request(url+'/api/state',headers={'Host':'example.invalid'}),403),
            (Request(url+'/api/start',data=b'x',headers={'X-Onenil-Action':'1'},method='POST'),400),
            (Request(url+'/api/configure',data=b'{}',headers={'X-Onenil-Action':'1','Content-Type':'application/json'},method='POST'),400),
        ]:
            try:urlopen(request)
            except HTTPError as error:assert error.code==code
            else:raise AssertionError('invalid local request accepted')
        with sync_playwright() as pw:
            browser=pw.chromium.launch(headless=True)
            page=browser.new_page(viewport={'width':1440,'height':1100})
            page.on('pageerror',lambda e:errors.append(str(e)))
            page.on('console',lambda m:errors.append(m.text) if m.type=='error' else None)
            page.goto(url)
            page.get_by_role('button',name='Kick off',exact=True).wait_for(timeout=60000)
            assert page.evaluate('[point({x:0,y:0}),point({x:-8,y:-37}),point({x:8,y:37})]')==[dict(x=500,y=300),dict(x=56,y=68),dict(x=944,y=532)]
            for viewport,width,height in [('desktop',1440,1100),('mobile',390,844)]:
                page.set_viewport_size({'width':width,'height':height})
                for want in expected:
                    choices=want['choices'];tag=f"{choices['lineup']}-{choices['seed']}-{viewport}"
                    current=api(url)
                    if current['phase']!='ready':
                        page.get_by_role('button',name='Restart',exact=True).click()
                        page.get_by_role('button',name='Kick off',exact=True).wait_for()
                    current=api(url)
                    if current['choices']!=choices:
                        page.locator('#lineup').select_option(choices['lineup'])
                        page.locator('#seed').select_option(choices['seed'])
                        assert page.get_by_role('button',name='Kick off',exact=True).is_disabled()
                        page.get_by_role('button',name='Prepare match',exact=True).click()
                        if viewport=='desktop':
                            page.wait_for_function("document.getElementById('phase').textContent==='Preparing match…'",timeout=3000)
                            page.wait_for_timeout(200)
                            start=time.monotonic();pending=api(url);elapsed=time.monotonic()-start
                            assert pending['phase']=='preparing' and pending['view'] is None
                            assert elapsed<2, f'preparation blocked HTTP for{elapsed}s'
                            assert page.locator('#players circle').count()==0
                            assert page.get_by_role('button',name='Preparing…',exact=True).is_disabled()
                            preparations.append(dict(choices=choices,response_seconds=elapsed))
                        page.get_by_role('button',name='Kick off',exact=True).wait_for(timeout=60000)
                    ready=api(url)
                    assert ready['choices']==choices and ready['phase']=='ready'
                    assert ready['selection']['seed']==int(choices['seed'])
                    assert ready['selection']['players']==want['actual_starting_players']
                    starter=2590 if choices['lineup']=='reference' else 2593
                    assert ready['selection']['starter']==starter
                    assert str(starter) in page.locator('#players circle').evaluate_all('(nodes)=>nodes.map(n=>n.dataset.player)')
                    assert page.locator('#players circle').count()==22 and page.locator('#ball').is_visible()
                    assert ready['meta']['players'][str(starter)] in page.locator('#selected').inner_text()
                    invalid=page.request.post(url+'/api/configure',data=json.dumps(dict(lineup='invalid',seed='64')),headers={'X-Onenil-Action':'1','Content-Type':'application/json'})
                    assert invalid.status==400 and api(url)==ready
                    positions=page.locator('#players circle').evaluate_all("nodes=>nodes.map(n=>[n.getAttribute('cx'),n.getAttribute('cy')])")
                    page.locator('#speed').select_option('80')
                    held=[]
                    def hold_step(route):held.append(route)
                    force_inflight=viewport=='desktop' and choices==expected[0]['choices']
                    if force_inflight:page.route('**/api/step',hold_step)
                    page.get_by_role('button',name='Kick off',exact=True).click()
                    if force_inflight:
                        deadline=time.monotonic()+10
                        while not held and time.monotonic()<deadline:page.wait_for_timeout(20)
                        assert len(held)==1
                        assert page.get_by_role('button',name='Pause',exact=True).is_enabled()
                        page.get_by_role('button',name='Pause',exact=True).click()
                        assert page.get_by_role('button',name='Pausing…',exact=True).is_disabled()
                        held[0].continue_();page.unroute('**/api/step',hold_step)
                        page.get_by_role('button',name='Resume',exact=True).wait_for()
                        settled=api(url)['steps'];page.wait_for_timeout(350)
                        assert api(url)['steps']==settled
                        page.get_by_role('button',name='Resume',exact=True).click()
                    page.wait_for_selector('.event.pass',timeout=30000)
                    page.wait_for_function("before=>JSON.stringify([...document.querySelectorAll('#players circle')].map(n=>[n.getAttribute('cx'),n.getAttribute('cy')]))!==JSON.stringify(before)",arg=positions,timeout=30000)
                    page.get_by_role('button',name='Pause',exact=True).click()
                    page.get_by_role('button',name='Resume',exact=True).wait_for()
                    paused=api(url);page.wait_for_timeout(350)
                    assert api(url)['steps']==paused['steps']
                    assert page.locator('#lineup').is_disabled() and page.locator('#seed').is_disabled()
                    unavailable=page.request.post(url+'/api/configure',data=json.dumps(choices),headers={'X-Onenil-Action':'1','Content-Type':'application/json'})
                    assert unavailable.status==409 and api(url)==paused
                    assert 'Play continues' not in page.locator('#commentary').inner_text()
                    page.locator('#speed').select_option('8')
                    page.get_by_role('button',name='Resume',exact=True).click()
                    page.get_by_role('button',name='Continue second half',exact=True).wait_for(timeout=60000)
                    half=api(url);page.wait_for_timeout(250)
                    assert half['phase']=='half-time' and api(url)['steps']==half['steps']
                    assert half['view']['clock']>=half['events'][-1]['minute']
                    half_image=directory/f'{stem}-{tag}-half.png';assert not half_image.exists()
                    page.screenshot(path=str(half_image),full_page=True)
                    screenshots[half_image.name]=hashlib.sha256(half_image.read_bytes()).hexdigest()
                    page.get_by_role('button',name='Continue second half',exact=True).click()
                    page.get_by_role('button',name='Match finished',exact=True).wait_for(timeout=60000)
                    page.wait_for_function("document.getElementById('live-label').textContent==='FULL-TIME'")
                    done=api(url);watched=[e for e in done['events'] if e['tick']>0]
                    assert done['phase']=='full-time' and done['steps']==done['total']==want['ticks']
                    assert done['view']['score']==want['score'] and len(watched)==want['actions']
                    assert page.locator('.event').count()==len(done['events'])
                    for kind in {e['kind'] for e in done['events']}:
                        assert page.locator(f'.event.{kind}').count()==sum(e['kind']==kind for e in done['events'])
                    assert len({e['text'] for e in watched})>100
                    assert page.locator('#score').inner_text()==' : '.join(map(str,want['score']))
                    whistle=next(e for e in watched if e['code']==63)
                    assert done['view']['clock']==whistle['minute']
                    clock=f"90+{whistle['minute']-90}′" if whistle['minute']>90 else f"{whistle['minute']}′"
                    assert page.locator('#clock').inner_text()==clock
                    assert page.get_by_role('button',name='Match finished',exact=True).is_disabled()
                    assert page.evaluate('document.documentElement.scrollWidth<=innerWidth')
                    assert page.locator('#score').bounding_box()['height']<70
                    assert page.locator('#pitch').is_visible()
                    page.locator('#commentary').evaluate('e=>{e.scrollTop=e.scrollHeight;}')
                    final_image=directory/f'{stem}-{tag}-full.png';assert not final_image.exists()
                    page.screenshot(path=str(final_image),full_page=True)
                    screenshots[final_image.name]=hashlib.sha256(final_image.read_bytes()).hexdigest()
                    page.get_by_role('button',name='Restart',exact=True).click()
                    page.get_by_role('button',name='Kick off',exact=True).wait_for()
                    assert api(url)==ready and page.locator('.event').count()==0
                    cases.append(dict(choices=choices,viewport=viewport,ticks=done['steps'],watched_actions=len(watched),score=done['view']['score'],minute=done['view']['clock'],halftime_tick=half['steps'],initial_selection=ready['selection']))
            assert len(cases)==8 and len(preparations)==3 and not errors,errors
            browser_version=browser.version;browser.close()
        outcome=dict(schema='prepared-watched-viewer-browser/1',started=began,code_commit=git('rev-parse','HEAD'),
                     engine_tree=git('rev-parse','HEAD:engine'),browser=browser_version,debug_assertions=native.__debug_assertions__,
                     extension_sha256=hashlib.sha256(Path(native.__file__).read_bytes()).hexdigest(),cases=cases,
                     visible_native_preparations=preparations,screenshots=screenshots,
                     checks=['all4choices ondesktop+mobile','actualstarter/initialseed','liveactualnativeclock/score',
                             'allnamednativeactions','22movingnativeplayers+ball','sharedcoordinates',
                             'responsive visiblepreparation','invalidchoice atomicity','inflightpause/settle','pause/resume','halftime/continue',
                             'actualpostwhistle completion','restart','localrequestguards','nohorizontaloverflow','noconsoleerrors'],
                     scope='Fresh application-computed native prepared sessions;onlyreference64 hasoriginal parity.No captured postsetup/runtime outcome backend.')
        with record.open('x')as out:json.dump(outcome,out,indent=2);out.write('\n')
        print(record.relative_to(ROOT),flush=True)
    finally:
        process.terminate();process.wait(timeout=10)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out',type=Path,default=ROOT/'records/T46')
    smoke(parser.parse_args().out)
