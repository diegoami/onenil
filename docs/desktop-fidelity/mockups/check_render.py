#!/usr/bin/env python3
"""[designed] Verify published design sheets without a browser or private assets."""
import argparse
import copy
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[3]
PREFIX = 'docs/desktop-fidelity/mockups/'
FONT = 'c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f'
SIZES = {'desktop': (1440,900,1), 'compact': (960,640,1), 'hidpi': (720,450,2)}
sha = lambda data: hashlib.sha256(data).hexdigest()


@lru_cache(maxsize=None)
def at(source, name):
    return subprocess.check_output(['git','-C',str(ROOT),'show',f'{source}:{name}'])


def verify(record, directory):
    assert record['schema'] == 'faithful-desktop-study-check/1' and record['status'] == 'passed'
    assert re.fullmatch('[0-9a-f]{40}',record['source'])
    assert record['replacement_font_sha256'] == FONT
    sources=record['producer_files']
    assert PREFIX+'build.py' in sources and PREFIX+'design.json' in sources
    for name,digest in sources.items():
        assert name.startswith(PREFIX) and '..' not in Path(name).parts
        assert sha(at(record['source'],name)) == digest, ('producer identity',name)
        assert sha((ROOT/name).read_bytes()) == digest, ('current producer drift',name)
    design=json.loads(at(record['source'],PREFIX+'design.json'))
    expected={row['id']:row for row in design['screens']}
    assert len(expected)==10 and set(record['pages'])==set(expected)
    for name,page in record['pages'].items():
        assert sha((ROOT/PREFIX/'pages'/f'{name}.svg').read_bytes())==page['svg_sha256']
    assert len(record['screenshots'])==len(expected)*len(SIZES)
    seen=set();names=set()
    for shot in record['screenshots']:
        key=(shot['screen'],shot['size']);assert key not in seen;seen.add(key)
        assert shot['screen'] in expected and shot['size'] in SIZES
        assert shot['file']==Path(shot['file']).name and shot['file'] not in names;names.add(shot['file'])
        width,height,dpr=SIZES[shot['size']]
        assert shot['viewport']==[width,height] and shot['device_scale_factor']==dpr
        data=(directory/shot['file']).read_bytes();assert sha(data)==shot['sha256']
        assert data[:8]==b'\x89PNG\r\n\x1a\n' and data[12:16]==b'IHDR'
        assert struct.unpack('>II',data[16:24])==(width*dpr,height*dpr)
        g=shot['geometry'];assert g['font'] and not g['clipped']
        assert g['x']>=-.01 and g['y']>=-.01 and g['width']>0 and g['height']>0
        assert g['x']+g['width']<=width+.01 and g['y']+g['height']<=height+.01
        assert shot['source_record']==expected[shot['screen']]['record']
        assert shot['original_image']==expected[shot['screen']]['reference']
    assert seen=={(name,size) for name in expected for size in SIZES}
    return dict(variants=len(expected),screenshots=len(names),producer_files=len(sources),sizes=len(SIZES))


def negatives(record,directory):
    cases=[]
    for name,mutate in [
        ('missing screenshot',lambda r:r['screenshots'].pop()),
        ('duplicate screenshot',lambda r:r['screenshots'].__setitem__(1,copy.deepcopy(r['screenshots'][0]))),
        ('wrong PNG digest',lambda r:r['screenshots'][0].__setitem__('sha256','0'*64)),
        ('clipped geometry',lambda r:r['screenshots'][0]['geometry'].__setitem__('clipped',['0,0'])),
        ('wrong font identity',lambda r:r.__setitem__('replacement_font_sha256','0'*64)),
        ('wrong producer identity',lambda r:r['producer_files'].__setitem__(PREFIX+'build.py','0'*64)),
    ]:
        changed=copy.deepcopy(record);mutate(changed)
        try:verify(changed,directory)
        except AssertionError:cases.append(name)
        else:raise AssertionError('negative control accepted: '+name)
    return cases


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('render_record',type=Path)
    parser.add_argument('--negative-checks',action='store_true')
    parser.add_argument('--record',type=Path)
    args=parser.parse_args()
    result=dict(schema='desktop-study-artifact-verification/1',status='failed',
                verifier_sha256=sha(Path(__file__).read_bytes()),render_record=str(args.render_record),
                source=subprocess.check_output(['git','-C',str(ROOT),'rev-parse','HEAD'],text=True).strip())
    try:
        raw=args.render_record.read_bytes();record=json.loads(raw)
        result['render_record_sha256']=sha(raw)
        result['checked']=verify(record,args.render_record.parent)
        if args.negative_checks:result['rejected_negative_controls']=negatives(record,args.render_record.parent)
        result['status']='passed'
    except Exception as error:
        result['error']=f'{type(error).__name__}: {error}'
    if args.record:
        with args.record.open('x') as output:json.dump(result,output,indent=2);output.write('\n')
    print(json.dumps(result,indent=2))
    return 0 if result['status']=='passed' else 1


if __name__=='__main__':
    raise SystemExit(main())
