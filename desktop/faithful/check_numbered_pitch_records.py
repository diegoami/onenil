#!/usr/bin/env python3
"""[designed:T72] Authenticate original-style numbers in actual native window pixels."""
import argparse
import hashlib
import json
import struct
import zlib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
RECORD_SHA='cde91e4c33dfb0ce488a4adfc2850573cc4fa7d7fc592d3cd39f9adc47dd1702'
PRODUCER='46115a6cad8c452a39a67c5f93af4fdb833da92b'
BINARY_SHA='cb77a7af16d0c1f7a98f0ea4d6c1db73734234ce9b7dc72cdd9536318e0cfba0'
FONT_SHA='c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
def digest(raw): return hashlib.sha256(raw).hexdigest()

def native_rgb(data):
    """Recover native RGB8 pixels at either supported window size."""
    assert data[:8] == b'\x89PNG\r\n\x1a\n'
    pos, compressed = 8, bytearray()
    header = None
    while pos < len(data):
        length = struct.unpack('>I', data[pos:pos+4])[0]
        kind = data[pos+4:pos+8]
        payload = data[pos+8:pos+8+length]
        assert len(payload) == length
        crc = struct.unpack('>I', data[pos+8+length:pos+12+length])[0]
        assert zlib.crc32(kind+payload) == crc
        if kind == b'IHDR': header = struct.unpack('>IIBBBBB', payload)
        if kind == b'IDAT': compressed.extend(payload)
        pos += length + 12
        if kind == b'IEND': break
    assert header is not None and header[2:]==(8,2,0,0,0) and pos==len(data)
    width,height=header[:2]
    assert (width,height) in ((1440,900),(1280,800))
    scale=min(width/720,height/400)
    w,h=int(720*scale),int(400*scale)
    ox,oy=(width-w)//2,(height-h)//2
    xs=[ox+(x*w+719)//720 for x in range(720)]
    ys={oy+(y*h+399)//400 for y in range(400)}
    stride = width*3
    raw = zlib.decompress(compressed)
    assert len(raw) == (stride+1)*height
    previous = bytearray(stride)
    pixels = bytearray()
    for y in range(height):
        start = y*(stride+1)
        mode = raw[start]
        assert 0 <= mode <= 4
        row = bytearray(raw[start+1:start+1+stride])
        for i in range(stride):
            left = row[i-3] if i >= 3 else 0
            up = previous[i]
            corner = previous[i-3] if i >= 3 else 0
            if mode == 1: predictor = left
            elif mode == 2: predictor = up
            elif mode == 3: predictor = (left+up)//2
            elif mode == 4:
                p = left+up-corner
                distances = (abs(p-left), abs(p-up), abs(p-corner))
                predictor = (left, up, corner)[distances.index(min(distances))]
            else: predictor = 0
            row[i] = (row[i]+predictor) & 255
        if y in ys:
            for x in xs: pixels.extend(row[x*3:x*3+3])
        previous = row
    assert len(pixels) == 720*400*3
    return pixels


def verify(path):
    raw=path.read_bytes();assert digest(raw)==RECORD_SHA
    r=json.loads(raw)
    assert r['schema']=='native-numbered-pitch-window/1'
    assert r['status']=='passed' and r['error'] is None and not r['x11_errors']
    assert r['producer']==PRODUCER and r['binary_sha256']==BINARY_SHA and r['font_sha256']==FONT_SHA
    assert r['source_research']=='011a341464dfbf4f419cd0eefad8553a37002dca'
    assert r['marker_contract_sha256']==digest((ROOT/'engine/tests/fixtures/scene_codes/provenance.json').read_bytes())
    assert r['runtime']['window_backend']=='software/minifb'
    assert r['runtime']['cargo_lock_sha256']==digest((ROOT/'desktop/faithful/Cargo.lock').read_bytes())
    assert r['fresh_process_count']==1 and r['cancelled_and_closed_workers_reaped'] is True
    steps={s['name']:s['state'] for s in r['steps']}
    assert list(steps)==['main','preparation','worker','pitch','slow-numbered-pitch','commentary']
    initial=steps['main']['campaign']
    assert all(s['campaign']==initial and s['exhibition'] is False for s in steps.values())
    assert steps['slow-numbered-pitch']['screen']=='Pitch' and steps['slow-numbered-pitch']['speed']==1
    assert steps['commentary']['screen']=='History' and steps['commentary']['recent_actions']
    seen=set()
    for shot in r['screenshots']:
        name=shot['name'];assert name not in seen;seen.add(name)
        assert Path(shot['path']).name==shot['path']
        data=(path.parent/shot['path']).read_bytes();assert digest(data)==shot['sha256']
        state=shot['state']
        assert digest(native_rgb(data))==state['raster_sha256'],'actual window pixels do not match computed view'
        assert state['campaign']==initial and state['players']==22
        assert state['screen']==('Pitch' if name.startswith('pitch-') else 'History')
        markers=state['pitch_markers'];assert len(markers)==22
        assert {m['selector'] for m in markers}==set(range(1,23))
        for m in markers:
            selector=m['selector']
            expected='G' if selector in (1,12) else chr(((selector-1)&65535)%11+47)
            assert m['label']==expected and 0<=m['attribute']<=255
            assert set(m['position'])=={'x','y'}
        assert sum(m['label']=='G' for m in markers)==2
        assert sorted(m['label'] for m in markers if m['label']!='G')==sorted('0123456789'*2)
    assert seen=={'pitch-default','pitch-resized','commentary-default','commentary-resized'}
    print('PASS original numbered pitch: 22 computed selectors/attributes,2 keepers,4 actual raster bindings,2 sizes,named commentary,no campaign mutation')
if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('record',type=Path);verify(parser.parse_args().record)
