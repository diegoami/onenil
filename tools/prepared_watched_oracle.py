#!/usr/bin/env python3
"""Decoded pre-seeding support probes for the native prepared-image bridge."""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]
SLOTS={'p8':0xD7BA,'p15':0xD7BE,'club':0xD7C2,'config':0xD7D2,
       'background':0xD092,'counts':0xDCC8,'results':0xD7CE,'fixtures':0xD7CA}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(research):
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    _,base,_=full.reference();_,data=full.match.find_state(base['start_state']['sha256'])
    week=full.match.Week(data)
    # Only the old logical field names are discovery probes. Their old captured
    # values are never used to construct support or a native state.
    prior=json.loads((ROOT/'engine/tests/fixtures/watched_match/initial.json').read_text())
    candidates=sorted({(f['area'],f['offset']) for f in prior['initial']['cells'] if f['area']=='ds' or f['area'] in SLOTS})
    candidates=sorted(set(candidates)|{('background',offset) for offset in range(4000)})
    output=dict(status='incomplete',fixture_request=None,fields=[],background_header_stable=None,
                background_unchanged=None,first_entry=None,setup_draws=None)
    incoming=None;headers=None
    def values(g):
        result={}
        bases={area:g.far(slot) for area,slot in SLOTS.items()}
        for area,base in bases.items():
            if base<16*g.reg('ds')+65536:
                raise ValueError(f'original {area} pool is not initialized before support observation')
        for area,offset in candidates:
            address=(16*g.reg('ds')+offset) if area=='ds' else (bases[area]+offset)
            if not 0<=address<0x110000:
                raise ValueError('initial support probe exceeds original memory')
            result[area,offset]=g.mem(address,1)[0]
        return result
    class Complete(Exception):pass
    def begin(g):
        nonlocal incoming,headers
        if incoming is not None:raise ValueError('duplicate pre-seeding support boundary')
        incoming=values(g);headers={area:g.far(slot) for area,slot in SLOTS.items()}
        bp=g.reg('bp');ss=16*g.reg('ss')
        output['fixture_request']=dict(week=g.word_at(ss+bp+4),club=g.word_at(ss+bp+6))
        g.put(g.far(0xD7D2)+0xF2,b'\x01')
        print('original initial support observed before19CA',flush=True)
    def entry(g):
        if incoming is None:raise ValueError('watched entry preceded incoming probes')
        final=values(g)
        output['fields']=[dict(area=area,offset=offset,before=incoming[area,offset],entry=final[area,offset]) for area,offset in candidates]
        output['background_header_stable']=headers['background']==g.far(SLOTS['background'])
        output['background_unchanged']=all(incoming['background',i]==final['background',i] for i in range(4000))
        output['first_entry']=full.state(g);output['setup_draws']=week.draws
        raise Complete()
    week.g.code_hook('0AB9:E24C',begin);week.g.code_hook('0AB9:7C67',entry)
    try:
        try:week.play()
        except Complete:pass
        else:raise ValueError('original ended before watched bridge entry')
        if week.g.unsupported:raise ValueError('unsupported original services')
        if week.draws!=258:raise ValueError('initial support run does not join setup258 draws')
        output['status']='completed'
    except (Exception,KeyboardInterrupt) as error:
        output['status']=f'aborted:{type(error).__name__}:{error}'
    return output


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root',type=Path,required=True)
    parser.add_argument('--run',action='store_true')
    parser.add_argument('--private',type=Path)
    args=parser.parse_args()
    if args.run==bool(args.private):parser.error('choose --run or --private RESULT')
    if args.private:
        if observe(args.research_root)!=json.loads(args.private.read_text()):raise ValueError('pre-seeding support observations differ')
        print('all initial support probes reproduced from original pre-seeding state',flush=True);return
    if subprocess.check_output(['git','status','--porcelain','--','tools/prepared_watched_oracle.py'],cwd=ROOT,text=True):raise ValueError('commit original support observer before measurement')
    root=ROOT/'records/T44';root.mkdir(parents=True,exist_ok=True)
    number=max((int(p.name.split('-r')[1]) for p in root.glob('original-r*') if p.is_dir()),default=0)+1
    directory=root/f'original-r{number:04d}';directory.mkdir()
    protocol=dict(schema='original-prepared-support/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                  observer_sha256=sha(Path(__file__)),research_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=args.research_root,text=True).strip(),
                  probe_names_source_sha256=sha(ROOT/'engine/tests/fixtures/watched_match/initial.json'),
                  preparation_admission_sha256=sha(ROOT/'engine/tests/fixtures/watched_setup/admission.json'),
                  scope='Candidate logical support values before19CA and first-entry assertions;explicit source ownership is still required before admitting any field. No RAM archive persisted.')
    (directory/'start.json').write_text(json.dumps(protocol,indent=2)+'\n')
    result=observe(args.research_root)
    (directory/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    manifest=dict(schema='original-prepared-support-manifest/1',files={name:sha(directory/name) for name in ('start.json','result.json')})
    (directory/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(directory.relative_to(ROOT),result['status'],flush=True)
    if result['status']!='completed':raise SystemExit(1)


if __name__=='__main__':main()
