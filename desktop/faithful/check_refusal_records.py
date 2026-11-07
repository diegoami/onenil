#!/usr/bin/env python3
"""[designed] Authenticate T63's complete refusal and actual window pixels (stdlib only)."""
import argparse
import copy
import json
import struct
import zlib
from pathlib import Path
from check_match_records import check, digest

MESSAGE = 'Only the unchanged initial campaign can play. Reload the original save.'
RECORD_SHA = 'PENDING_CAPTURE'


def native_rgb(data):
    """Decode the bounded RGB8 screenshot and undo the window's exact 2x scaling."""
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
    assert header == (1440, 900, 8, 2, 0, 0, 0) and pos == len(data)
    stride = 1440*3
    raw = zlib.decompress(compressed)
    assert len(raw) == (stride+1)*900
    previous = bytearray(stride)
    pixels = bytearray()
    for y in range(900):
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
        if 50 <= y < 850 and y % 2 == 0:
            for x in range(0, 1440, 2): pixels.extend(row[x*3:x*3+3])
        previous = row
    assert len(pixels) == 720*400*3
    return pixels


def check_refusal(record):
    check(record)
    state = next(s['state'] for s in record['steps'] if s['name'] == 'edited-admission-refuses')
    assert state['message'] == MESSAGE and len(state['message']) <= 80
    assert state['refused_worker_absent'] is True
    return state


def verify(path):
    raw = path.read_bytes()
    assert digest(raw) == RECORD_SHA, 'immutable T63 capture differs'
    record = json.loads(raw)
    state = check_refusal(record)
    seen = set()
    for shot in record['screenshots']:
        assert Path(shot['path']).name == shot['path'] and shot['name'] not in seen
        seen.add(shot['name'])
        data = (path.parent / shot['path']).read_bytes()
        assert digest(data) == shot['sha256']
        assert list(struct.unpack('>II', data[16:24])) == shot['size'] == [1440, 900]
        if shot['name'] == 'refused-edit':
            rgb = native_rgb(data)
            assert digest(rgb) == state['raster_sha256'], 'actual screenshot differs from refused controller raster'
            damaged = bytearray(rgb)
            damaged[-1] ^= 1
            assert digest(damaged) != state['raster_sha256']
    assert seen == {'main', 'preparation', 'pitch-actions', 'commentary', 'bench', 'half-time', 'full-time', 'refused-edit'}
    for damage in ('truncated', 'altered', 'worker', 'campaign'):
        bad = copy.deepcopy(record)
        refused = next(s['state'] for s in bad['steps'] if s['name'] == 'edited-admission-refuses')
        if damage == 'truncated': refused['message'] = MESSAGE[:-6]
        elif damage == 'altered': refused['message'] = MESSAGE.replace('Reload', 'Ignore')
        elif damage == 'worker': refused['refused_worker_absent'] = False
        else: refused['campaign']['revision'] += 1
        try: check_refusal(bad)
        except AssertionError: pass
        else: raise AssertionError('corrupt refusal accepted: '+damage)
    print('PASS complete refusal/reload instruction; 20 steps, 8 screenshots, actual raster, 5 corruptions')


if __name__ == '__main__':
    p = argparse.ArgumentParser()
    p.add_argument('record', type=Path)
    verify(p.parse_args().record)
