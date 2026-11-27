#!/usr/bin/env python3
"""T119: the real-X11 league quick route — configuration, quick boards and week.

    python3 desktop/faithful/quick_window_check.py --binary <release binary>
        --font <pxplus ttf> --record records/T119/window-r0001.json
"""
import argparse, ctypes as C, hashlib, json, os, shutil, subprocess, tempfile, time
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
SAVE_FIXTURE = ROOT / 'desktop/faithful/fixtures/league-week28.json'
SAVE_SHA = '8f97f0ad04f6e1943483f0f93b0e63fb28f4c2124d8f315645e6d6060db213e3'

p = argparse.ArgumentParser()
p.add_argument('--binary', type=Path, required=True)
p.add_argument('--font', type=Path, required=True)
p.add_argument('--record', type=Path, required=True)
args = p.parse_args()
assert not args.record.exists()

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

window = None; steps = []; shots = []; process = None; process_ids = []
with tempfile.TemporaryDirectory(prefix='onenil-t119-') as directory:
    directory = Path(directory)
    binary = directory / 'onenil-desktop'; shutil.copy2(args.binary, binary)
    statepath = directory / 'state.json'; save = directory / 'games' / 'league.1-0'; save.parent.mkdir()
    save.write_bytes(SAVE_FIXTURE.read_bytes()); stderr = directory / 'stderr.txt'
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
        meta = C.cast(image, C.POINTER(XImage)).contents
        assert meta.bits_per_pixel == 32, f'bits={meta.bits_per_pixel}'
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
    try:
        with stderr.open('a') as err:
            process = subprocess.Popen([str(binary), '--root', str(ROOT), '--font', str(args.font.resolve()),
                '--save', str(save), '--observe', str(statepath), '--load'], stderr=err, stdout=subprocess.DEVNULL)
        process_ids.append(process.pid)
        end = time.monotonic() + 20
        while window is None and time.monotonic() < end:
            window = next((w for w in windows() if b'One-Nil | saved club prototype' in title(w)), None)
            time.sleep(.05)
        assert window, 'native window missing'
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
        step('league-main', lambda s: s.get('screen') == 'Main' and s.get('campaign', {}).get('week_index') == 54)
        shot('league-main')
        key('Up'); key('Up'); key('Return')
        step('configuration', lambda s: s.get('screen') == 'Configuration')
        shot('configuration-watch-on')
        key('Return')
        step('watch-off', lambda s: s.get('watch_match') is False)
        shot('configuration-watch-off')
        key('Escape')
        step('main-after-configuration', lambda s: s.get('screen') == 'Main')
        key('Down'); key('Down'); key('Return')
        step('preparation', lambda s: s.get('screen') == 'Preparation')
        shot('preparation')
        key('Return')
        step('half-time', lambda s: s.get('screen') == 'HalfTime' and (s.get('quick') or {}).get('stage') == 'HALF TIME')
        shot('half-time')
        key('space')
        step('full-time', lambda s: s.get('screen') == 'FullTime' and (s.get('quick') or {}).get('stage') == 'FULL TIME')
        shot('full-time')
        key('Return')
        step('cash-flow', lambda s: s.get('screen') == 'CashFlow' and s.get('campaign', {}).get('cash') == 1691393)
        shot('cash-flow')
        key('space')
        step('transfer-news', lambda s: s.get('screen') == 'TransferNews')
        shot('transfer-news')
        close_window()
        process.wait(timeout=10)
        status = 'passed' if not stderr.read_text() else 'stderr'
    except Exception as error:
        status = 'failed'; error_text = repr(error)
        try: close_window()
        except Exception: pass
        if process: process.kill()
    record = dict(schema='league-quick-window-check/1', runtime=dict(cargo_lock_sha256=hashlib.sha256((ROOT/'desktop/faithful/Cargo.lock').read_bytes()).hexdigest()), producer=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip(),
        binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), font_sha256=hashlib.sha256(args.font.read_bytes()).hexdigest(),
        save_sha256=SAVE_SHA, display=os.environ.get('DISPLAY'), elapsed_seconds=round(time.monotonic(), 3),
        steps=steps, screenshots=shots, process_ids=process_ids, status=status,
        error=error_text if status == 'failed' else None, stderr=stderr.read_text()[:4000])
    args.record.parent.mkdir(parents=True, exist_ok=True)
    args.record.write_text(json.dumps(record, indent=1) + '\n')
    print(status, len(steps), 'steps', len(shots), 'screenshots')
    if status != 'passed': raise SystemExit(1)
