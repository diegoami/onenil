#!/usr/bin/env python3
"""[designed] Static desktop compositions from public references, not a game UI."""
import argparse
import base64
import copy
from collections import Counter
from datetime import datetime, timezone
from functools import lru_cache
import hashlib
import html
import io
import json
from pathlib import Path
import subprocess
import time
import urllib.error
import urllib.request
import zipfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PIN = '6aca98caeebc1b9b5410852139fa305462224a05'
FONT_SHA = 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
PACK_SHA = '21b3c0a3770ef0afc46564760613d7b078f4fcc9ed93db4b829a440b68822e08'
PACK_URL = 'https://int10h.org/oldschool-pc-fonts/download/oldschool_pc_font_pack_v2.2_FULL.zip'
FONT_MEMBER = 'ttf - Px (pixel outline)/PxPlus_IBM_VGA_8x16.ttf'
REQUIRED = {'main', 'skills', 'details', 'put-here', 'preparation', 'pitch',
            'history', 'bench', 'halftime', 'fulltime'}
BOX_ARMS = dict(zip('─│┌┐└┘├┤┬┴┼═║╔╗╚╝╠╣╦╩╬',
                    ['LR','UD','RD','LD','RU','LU','URD','ULD','LRD','LRU','ULRD']*2))
BLOCKS = {'▀':(0,0,9,8), '▄':(0,8,9,8), '█':(0,0,9,16),
          '▌':(0,0,4.5,16), '▐':(4.5,0,4.5,16)}
SIZES = [('desktop', 1440, 900, 1), ('compact', 960, 640, 1), ('hidpi', 720, 450, 2)]
sha = lambda data: hashlib.sha256(data).hexdigest()


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], text=True).strip()


@lru_cache(maxsize=None)
def original(research, name):
    return subprocess.check_output(['git', '-C', str(research), 'show', f'{PIN}:{name}'])


def font_bytes(path, fetch=False):
    if not path.exists() and fetch:
        # The 20MB vendor download flaked once in CI (a truncated or mid-transfer
        # response failed the checksum); retry, accepting only the pinned bytes.
        archive = None
        for attempt in range(3):
            request = urllib.request.Request(PACK_URL, headers={'User-Agent':'OneNil-Design-Study/1.0'})
            try:
                with urllib.request.urlopen(request, timeout=300) as response:
                    archive = response.read()
                if sha(archive) == PACK_SHA:
                    break
            except (urllib.error.URLError, OSError):
                pass
            archive = None
            time.sleep(5 * (attempt + 1))
        assert archive is not None and sha(archive) == PACK_SHA, 'font pack checksum differs'
        with zipfile.ZipFile(io.BytesIO(archive)) as pack:
            data = pack.read(FONT_MEMBER)
        assert sha(data) == FONT_SHA
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open('xb') as output:
            output.write(data)
    data = path.read_bytes()
    assert sha(data) == FONT_SHA, 'replacement font checksum differs'
    return data


def validate_plan(value):
    assert value['schema'] == 'faithful-desktop-study/1'
    assert value['research_commit'] == PIN
    assert value['status'] == 'proposed-static-design'
    assert len(value['screens']) == len(REQUIRED)
    assert {row['id'] for row in value['screens']} == REQUIRED
    departures = {row['id'] for row in value['departures']}
    assert departures
    for row in value['screens']:
        assert row['departures'] and set(row['departures']) <= departures
    return value


def plan():
    return validate_plan(json.loads((HERE/'design.json').read_text()))


def input_screen(row, research):
    from PIL import Image
    raw = original(research, row['record'])
    assert sha(raw) == row['record_sha256']
    value = json.loads(raw)
    if 'screens' in value:
        value = next(s for s in value['screens'] if s['id'] == row['step'])
    picture = original(research, row['reference'])
    assert sha(picture) == row['reference_sha256'] == value['screenshot_sha256']
    image = Image.open(io.BytesIO(picture)).convert('RGB')
    assert image.size == (720, 400)
    lines = value['text'].splitlines()
    assert len(lines) <= 25 and all(len(line) <= 80 for line in lines)
    lines = [line.ljust(80) for line in lines]
    lines += [' '*80] * (25-len(lines))
    return lines, image


def svg_for(row, research):
    lines, reference = input_screen(row, research)
    ground = Counter(reference.getdata()).most_common(1)[0][0]
    colour = lambda rgb: '#'+''.join(f'{n:02x}' for n in rgb)
    ground_css = colour(ground)
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 720 400" role="img" aria-labelledby="title desc">',
             f'<title id="title">Proposed desktop design: {html.escape(row["title"])}</title>',
             '<desc id="desc">Static original-led composition; independent font, provisional role glyphs and schematic pitch figures. Not an implemented game.</desc>',
             f'<rect width="720" height="400" fill="{ground_css}"/>',
             '<g font-family="OneNilStudy" font-size="16" dominant-baseline="text-before-edge">']
    counts = dict(text_cells=0, bar_cells=0, filled_bar_cells=0, figure_cells=0, provisional_role_cells=0)
    for y, line in enumerate(lines):
        separators = [i for i, ch in enumerate(line) if ch == '║']
        role = separators[-1]-1 if len(separators)>1 and separators[-1]-separators[-2]==2 else -1
        for x, ch in enumerate(line):
            px, py = x*9, y*16
            colors = Counter(reference.crop((px, py, px+9, py+16)).getdata())
            # [derived] Read displayed colours, never original palette/font bytes.
            # Most pixels supply a normal text-cell background; block/figure
            # primitives instead use the screen ground and their visible colour.
            primitive = row['pitch_layer'] and 5 <= y < 24 and ch in 'ôÆ£¥öò•'
            vector = ch in BOX_ARMS or ch in BLOCKS
            if vector:
                background = (0,0,0) if colors[(0,0,0)] > colors[ground] else ground
                candidates = [item for item in colors.most_common() if item[0] != background]
                foreground = candidates[0][0] if candidates else background
            elif primitive:
                background = ground
                candidates = [item for item in colors.most_common() if item[0] != ground]
                foreground = candidates[0][0] if candidates else ground
            else:
                background = colors.most_common(1)[0][0]
                candidates = [item for item in colors.most_common() if item[0] != background]
                foreground = candidates[0][0] if candidates else background
            if background != ground:
                parts.append(f'<rect x="{px}" y="{py}" width="9" height="16" fill="{colour(background)}"/>')
            if ch == ' ':
                continue
            fill = colour(foreground)
            if ch in BOX_ARMS:
                arms=BOX_ARMS[ch]
                edges={'L':(0,8),'R':(9,8),'U':(4.5,0),'D':(4.5,16)}
                # Independent thin vector rules keep continuous box joins; font
                # metrics and inverse skill-cell backgrounds do not define frames.
                double=ch in '═║╔╗╚╝╠╣╦╩╬'
                for arm in arms:
                    ex,ey=edges[arm]
                    for offset in (-.6,.6) if double else (0,):
                        dx,dy=(offset,0) if arm in 'UD' else (0,offset)
                        parts.append(f'<path data-kind="frame" d="M{px+4.5+dx} {py+8+dy}L{px+ex+dx} {py+ey+dy}" stroke="{fill}" stroke-width="1" fill="none"/>')
            elif ch in BLOCKS:
                bx,by,bw,bh=BLOCKS[ch]
                parts.append(f'<rect data-kind="block" x="{px+bx}" y="{py+by}" width="{bw}" height="{bh}" fill="{fill}"/>')
            elif ch == '■':
                # [derived] Filled original bars invert the cell: light band
                # around a dark slot. Empty cells have dark ground/olive slot.
                # Preserve that displayed state rather than turning both into
                # coloured foreground squares. These are new rectangle primitives.
                filled = background != ground
                parts.append(f'<rect data-kind="bar" data-filled="{str(filled).lower()}" x="{px+2}" y="{py+4}" width="5" height="7" fill="{fill}"/>')
                counts['bar_cells'] += 1
                counts['filled_bar_cells'] += int(filled)
            elif primitive:
                if ch == '•':
                    parts.append(f'<circle data-kind="ball" cx="{px+4.5}" cy="{py+8}" r="1.7" fill="{fill}"/>')
                else:
                    parts.append(f'<g data-kind="figure" fill="{fill}" stroke="{fill}" stroke-width="1"><circle cx="{px+4.5}" cy="{py+4}" r="1.3"/><path d="M{px+4.5} {py+6}v4m0-3l-2 2m2-2l2 2m-2 1l-2 3m2-3l2 3"/></g>')
                counts['figure_cells'] += 1
            else:
                provisional = x == role
                parts.append(f'<text data-cell="{x},{y}" data-provisional-role="{str(provisional).lower()}" x="{px}" y="{py}" textLength="9" lengthAdjust="spacingAndGlyphs" fill="{fill}">{html.escape(ch)}</text>')
                counts['text_cells'] += 1
                counts['provisional_role_cells'] += int(provisional)
    parts += ['</g>', '</svg>', '']
    svg = '\n'.join(parts)
    assert '<image' not in svg and 'data:font' not in svg
    return svg, counts


def generated(data, research, write=False):
    result = {}
    for row in data['screens']:
        svg, counts = svg_for(row, research)
        path = HERE/'pages'/f'{row["id"]}.svg'
        if write:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(svg)
        else:
            assert path.read_text() == svg, f'generated page drift: {row["id"]}'
        result[row['id']] = dict(svg_sha256=sha(svg.encode()), **counts)
    return result


def negative_checks(data, research):
    result = []
    for name, mutate in [
        ('missing required variant', lambda d: d['screens'].pop()),
        ('duplicate variant', lambda d: d['screens'].__setitem__(1, copy.deepcopy(d['screens'][0]))),
        ('unknown departure', lambda d: d['screens'][0]['departures'].append('P99')),
    ]:
        changed=copy.deepcopy(data);mutate(changed)
        try: validate_plan(changed)
        except AssertionError: result.append(name)
        else: raise AssertionError(f'negative control accepted: {name}')
    for field in ('record_sha256', 'reference_sha256'):
        changed=copy.deepcopy(data['screens'][0]);changed[field]='0'*64
        try: input_screen(changed, research)
        except AssertionError: result.append('changed '+field)
        else: raise AssertionError(f'negative control accepted: {field}')
    return result


def render(data, pages, font, out, stem):
    from playwright.sync_api import sync_playwright
    import importlib.metadata
    assert not git('status', '--porcelain', '--', 'docs/desktop-fidelity/mockups'), 'commit producer/input/pages before rendering'
    errors, shots = [], []
    with sync_playwright() as pw:
        browser = pw.chromium.launch(headless=True)
        version = browser.version
        try:
            for label, width, height, dpr in SIZES:
                context = browser.new_context(viewport=dict(width=width, height=height), device_scale_factor=dpr, offline=True)
                page = context.new_page()
                page.on('pageerror', lambda error: errors.append(str(error)))
                for row in data['screens']:
                    svg = (HERE/'pages'/f'{row["id"]}.svg').read_text()
                    content = '<!doctype html><meta charset="utf-8"><style>@font-face{font-family:OneNilStudy;src:url(data:font/ttf;base64,'+base64.b64encode(font).decode()+')}*{box-sizing:border-box}html,body{margin:0;width:100%;height:100%;background:#111;overflow:hidden}body{display:flex;align-items:center;justify-content:center}svg{width:min(100vw,calc(100vh * 1.8));height:auto;display:block}</style>'+svg
                    page.set_content(content)
                    page.evaluate('document.fonts.ready')
                    geometry = page.evaluate('''() => {
                      const svg=document.querySelector('svg'),r=svg.getBoundingClientRect();
                      const clipped=[...svg.querySelectorAll('text')].filter(n=>{
                        const b=n.getBBox();return b.x < -.5 || b.y < -.5 || b.x+b.width>720.5 || b.y+b.height>400.5;
                      }).map(n=>n.dataset.cell);
                      return {font:document.fonts.check('16px OneNilStudy'),x:r.x,y:r.y,width:r.width,height:r.height,clipped};
                    }''')
                    assert geometry['font'] and not geometry['clipped'], (row['id'], geometry)
                    assert geometry['x'] >= -.01 and geometry['y'] >= -.01
                    assert geometry['x']+geometry['width'] <= width+.01
                    assert geometry['y']+geometry['height'] <= height+.01
                    path = out/f'{stem}-{row["id"]}-{label}.png'
                    assert not path.exists()
                    page.screenshot(path=str(path))
                    shots.append(dict(screen=row['id'], size=label, viewport=[width,height], device_scale_factor=dpr,
                                      file=path.name, sha256=sha(path.read_bytes()), geometry=geometry,
                                      source_record=row['record'], original_image=row['reference']))
                context.close()
        finally:
            browser.close()
    assert not errors, errors
    return dict(screenshots=shots, chromium=version, playwright=importlib.metadata.version('playwright'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root', type=Path, required=True)
    parser.add_argument('--font', type=Path, default=ROOT/'.cache/T51-font/PxPlus_IBM_VGA_8x16.ttf')
    parser.add_argument('--fetch-font', action='store_true')
    parser.add_argument('--build', action='store_true')
    parser.add_argument('--render', action='store_true')
    parser.add_argument('--negative-checks', action='store_true')
    parser.add_argument('--out', type=Path, default=ROOT/'records/T51')
    parser.add_argument('--record', type=Path)
    args = parser.parse_args()
    result = dict(schema='faithful-desktop-study-check/1', source=git('rev-parse','HEAD'),
                  started=datetime.now(timezone.utc).isoformat(), status='failed', research_commit=PIN,
                  scope='Static design only; no GUI toolkit, game execution or campaign-state parity')
    target = args.record
    try:
        data = plan()
        pages = generated(data, args.research_root, args.build)
        result['pages'] = pages
        if args.negative_checks:
            result['rejected_negative_controls'] = negative_checks(data, args.research_root)
        font = font_bytes(args.font, args.fetch_font)
        result['replacement_font_sha256'] = sha(font)
        import PIL
        result['pillow'] = PIL.__version__
        paths = git('ls-files', 'docs/desktop-fidelity/mockups').splitlines()
        result['producer_files'] = {p:sha((ROOT/p).read_bytes()) for p in paths}
        if args.render:
            args.out.mkdir(parents=True, exist_ok=True)
            n = 1
            while (args.out/f'render-r{n:04}.json').exists(): n += 1
            stem = f'render-r{n:04}'
            target = args.out/(stem+'.json')
            result.update(render(data, pages, font, args.out, stem))
        result['status'] = 'passed'
    except Exception as error:
        result['error'] = f'{type(error).__name__}: {error}'
    result['finished'] = datetime.now(timezone.utc).isoformat()
    if target:
        target.parent.mkdir(parents=True, exist_ok=True)
        with target.open('x') as output:
            json.dump(result, output, indent=2); output.write('\n')
    print(json.dumps({k:v for k,v in result.items() if k not in ('screenshots','producer_files','pages')}, indent=2))
    if result.get('screenshots'):
        print(f'{len(result["screenshots"])} screenshots with verified unclipped layout')
    return 0 if result['status'] == 'passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
