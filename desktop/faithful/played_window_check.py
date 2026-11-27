#!/usr/bin/env python3
"""T120: the real-X11 played league table and results screens.

    python3 desktop/faithful/played_window_check.py --binary <release binary>
        --font <pxplus ttf> --record records/T120/window-r0001.json
"""
import argparse, ctypes as C, hashlib, json, os, shutil, subprocess, tempfile, time
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
PLAYED_SAVE = ROOT / 'desktop/faithful/fixtures/league-week28-played.json'
PRE_SAVE = ROOT / 'desktop/faithful/fixtures/league-week28.json'

p = argparse.ArgumentParser()
p.add_argument('--binary', type=Path, required=True)
p.add_argument('--font', type=Path, required=True)
p.add_argument('--record', type=Path, required=True)
args = p.parse_args()
assert not args.record.exists()
args.record.parent.mkdir(parents=True, exist_ok=True)

x = C.CDLL('libX11.so.6'); P = C.c_void_p; U = C.c_ulong; I = C.c_int
for name, restype, argtypes in [('XOpenDisplay', P, [C.c_char_p]),
    ('XDefaultRootWindow', U, [P]), ('XQueryTree', I, [P, U, C.POINTER(U), C.POINTER(U), C.POINTER(C.POINTER(U)), C.POINTER(C.c_uint)]),
    ('XFetchName', I, [P, U, C.POINTER(C.c_char_p)]), ('XFree', I, [P]),
    ('XStringToKeysym', U, [C.c_char_p]), ('XKeysymToKeycode', C.c_ubyte, [P, U]),
    ('XSendEvent', I, [P, U, I, C.c_long, P]), ('XFlush', I, [P]),
    ('XSetInputFocus', I, [P, U, I, U]), ('XGetGeometry', I, [P, U, C.POINTER(U), C.POINTER(I), C.POINTER(I), C.POINTER(C.c_uint), C.POINTER(C.c_uint), C.POINTER(C.c_uint), C.POINTER(C.c_uint)]),
    ('XGetImage', P, [P, U, I, I, C.c_uint, C.c_uint, U, I]), ('XDestroyImage', I, [P]),
    ('XInternAtom', U, [P, C.c_char_p, I])]:
    f = getattr(x, name); f.restype = restype; f.argtypes = argtypes
class Attributes(C.Structure):
    _fields_ = [('x', I), ('y', I), ('width', I), ('height', I), ('border', I), ('depth', I), ('visual', P), ('root', U), ('klass', I), ('bit_gravity', I), ('win_gravity', I), ('backing_store', I), ('backing_planes', U), ('backing_pixel', U), ('save_under', I), ('colormap', U), ('map_installed', I), ('map_state', I), ('all_events', C.c_long), ('your_events', C.c_long), ('dont_propagate', C.c_long), ('override_redirect', I), ('screen', P)]
x.XGetWindowAttributes.argtypes = [P, U, C.POINTER(Attributes)]; x.XGetWindowAttributes.restype = I
class Key(C.Structure):
    _fields_ = [('type', I), ('serial', U), ('send', I), ('display', P), ('window', U), ('root', U), ('subwindow', U), ('time', U), ('x', I), ('y', I), ('x_root', I), ('y_root', I), ('state', C.c_uint), ('keycode', C.c_uint), ('same_screen', I)]
class Client(C.Structure):
    _fields_ = [('type', I), ('serial', U), ('send', I), ('display', P), ('window', U), ('message_type', U), ('format', I), ('data', C.c_long * 5)]
class Event(C.Union):
    _fields_ = [('key', Key), ('client', Client), ('pad', C.c_long * 24)]
class XImage(C.Structure):
    _fields_ = [('width', I), ('height', I), ('xoffset', I), ('format', I), ('data', P), ('byte_order', I), ('bitmap_unit', I), ('bitmap_bit_order', I), ('bitmap_pad', I), ('depth', I), ('bytes_per_line', I), ('bits_per_pixel', I)]

xerrors = []
ERROR_HANDLER = C.CFUNCTYPE(I, P, P)
@ERROR_HANDLER
def xerror(display, event):
    xerrors.append('X11 request error')
    return 0
x.XSetErrorHandler.argtypes = [ERROR_HANDLER]
x.XSetErrorHandler(xerror)
display = x.XOpenDisplay(None); assert display, 'X11 display unavailable'
root = x.XDefaultRootWindow(display)
def windows():
    r = U(); parent = U(); children = C.POINTER(U)(); count = C.c_uint()
    x.XQueryTree(display, root, C.byref(r), C.byref(parent), C.byref(children), C.byref(count))
    ids = [children[i] for i in range(count.value)]; x.XFree(children); return ids
def title(w):
    name = C.c_char_p(); x.XFetchName(display, w, C.byref(name)); s = name.value or b''
    if name: x.XFree(name)
    return s

window = None; steps = []; shots = []; process = None; process_ids = []; error_text = None
with tempfile.TemporaryDirectory(prefix='onenil-t120-') as directory:
    directory = Path(directory)
    binary = directory / 'onenil-desktop'; shutil.copy2(args.binary, binary)
    statepath = directory / 'state.json'; save = directory / 'games' / 'league.1-0'; save.parent.mkdir()
    stderr = directory / 'stderr.txt'
    def state():
        if statepath.exists(): return json.loads(statepath.read_text())
        return {}
    def wait(check, timeout=20):
        end = time.monotonic() + timeout
        while time.monotonic() < end:
            s = state()
            if check(s): return s
            if process.poll() is not None: raise AssertionError('app exited: ' + stderr.read_text())
            time.sleep(.02)
        raise AssertionError('condition timeout: ' + json.dumps(state())[:400])
    def key(name):
        code = x.XKeysymToKeycode(display, x.XStringToKeysym(name.encode())); assert code
        for typ, mask in [(2, 1), (3, 2)]:
            event = Event(); event.key = Key(typ, 0, 1, display, window, root, 0, 0, 1, 1, 1, 1, 0, code, 1)
            assert x.XSendEvent(display, window, 0, mask, C.byref(event)); x.XFlush(display); time.sleep(.08)
    def shot(name):
        time.sleep(.12)
        r = U(); xx = I(); yy = I(); w = C.c_uint(); h = C.c_uint(); border = C.c_uint(); depth = C.c_uint()
        assert x.XGetGeometry(display, window, C.byref(r), C.byref(xx), C.byref(yy), C.byref(w), C.byref(h), C.byref(border), C.byref(depth))
        image = x.XGetImage(display, window, 0, 0, w, h, U(-1), 2)
        assert image, f'XGetImage failed w={w.value} h={h.value} errors={xerrors}'
        meta = C.cast(image, C.POINTER(XImage)).contents; assert meta.bits_per_pixel == 32
        data = C.string_at(meta.data, meta.bytes_per_line * h.value)
        out = args.record.with_name(args.record.stem + '-' + name + '.png'); assert not out.exists()
        Image.frombytes('RGB', (w.value, h.value), data, 'raw', 'BGRX', meta.bytes_per_line, 1).save(out)
        x.XDestroyImage(image)
        shots.append(dict(name=name, path=out.name, size=[w.value, h.value], sha256=hashlib.sha256(out.read_bytes()).hexdigest()))
    def close_window():
        event = Event(); event.client = Client(33, 0, 1, display, window, x.XInternAtom(display, b'WM_PROTOCOLS', 0), 32, (C.c_long * 5)(x.XInternAtom(display, b'WM_DELETE_WINDOW', 0), 0, 0, 0, 0))
        x.XSendEvent(display, window, 0, 0, C.byref(event)); x.XFlush(display)
    def step(name, check, timeout=20):
        s = wait(check, timeout); steps.append(dict(name=name, state=s)); return s
    def launch(source, label):
        global process, window
        if process is not None:
            try:
                close_window(); process.wait(timeout=10)
            except Exception:
                process.kill()
        window = None
        save.write_bytes(source.read_bytes())
        with stderr.open('a') as err:
            process = subprocess.Popen([str(binary), '--root', str(ROOT), '--font', str(args.font.resolve()),
                '--save', str(save), '--observe', str(statepath), '--load'], stderr=err, stdout=subprocess.DEVNULL)
        process_ids.append(process.pid)
        end = time.monotonic() + 20
        while window is None and time.monotonic() < end:
            window = next((w for w in windows() if b'One-Nil | saved club prototype' in title(w)), None)
            time.sleep(.05)
        assert window, 'native window missing: ' + label
        end = time.monotonic() + 20
        while time.monotonic() < end:
            attributes = Attributes()
            if x.XGetWindowAttributes(display, window, C.byref(attributes)) and attributes.map_state == 2:
                break
            if process.poll() is not None:
                raise AssertionError('app exited before visible: ' + stderr.read_text())
            time.sleep(.05)
        else:
            raise AssertionError('native window did not become viewable')
        x.XSetInputFocus(display, window, 1, 0); x.XFlush(display)
    try:
        launch(PLAYED_SAVE, 'played')
        step('played-main', lambda s: s.get('screen') == 'Main' and s.get('league_played') is not None)
        for _ in range(6): key('Down')
        key('Return')
        step('played-tables', lambda s: s.get('screen') == 'LeagueTables' and s.get('league_division') == 0)
        shot('played-tables')
        key('Down')
        step('division-one', lambda s: s.get('league_division') == 1)
        shot('division-one')
        key('Escape')
        step('main-after-tables', lambda s: s.get('screen') == 'Main')
        key('Down'); key('Return')
        step('played-results', lambda s: s.get('screen') == 'LeagueResults')
        shot('played-results')
        key('Escape')
        step('main-after-results', lambda s: s.get('screen') == 'Main')
        launch(PRE_SAVE, 'unplayed')
        step('unplayed-main', lambda s: s.get('screen') == 'Main' and s.get('league_played') is None)
        for _ in range(6): key('Down')
        key('Return')
        step('unplayed-tables', lambda s: s.get('screen') == 'LeagueTables' and s.get('league_division') == 0)
        shot('unplayed-tables')
        key('Escape')
        step('main-before-notice', lambda s: s.get('screen') == 'Main')
        key('Down'); key('Return')
        step('unplayed-notice', lambda s: s.get('notice') is True and 'No league results' in s.get('message', ''))
        shot('unplayed-notice')
        close_window(); process.wait(timeout=10)
        status = 'passed' if not stderr.read_text() else 'stderr'
    except Exception as error:
        status = 'failed'; error_text = repr(error)
        try: close_window()
        except Exception: pass
        if process: process.kill()
    record = dict(schema='played-league-window-check/1',
        runtime=dict(cargo_lock_sha256=hashlib.sha256((ROOT / 'desktop/faithful/Cargo.lock').read_bytes()).hexdigest()),
        producer=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
        binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
        font_sha256=hashlib.sha256(args.font.read_bytes()).hexdigest(),
        saves={'played': hashlib.sha256(PLAYED_SAVE.read_bytes()).hexdigest(), 'pre': hashlib.sha256(PRE_SAVE.read_bytes()).hexdigest()},
        display=os.environ.get('DISPLAY'), steps=steps, screenshots=shots, process_ids=process_ids,
        status=status, error=error_text if status == 'failed' else None, stderr=stderr.read_text()[:4000])
    args.record.parent.mkdir(parents=True, exist_ok=True)
    args.record.write_text(json.dumps(record, indent=1) + '\n')
    print(status, len(steps), 'steps', len(shots), 'screenshots')
    if status != 'passed': raise SystemExit(1)
