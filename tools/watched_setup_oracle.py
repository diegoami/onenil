#!/usr/bin/env python3
"""Original watched preparation observer;decoded incoming inputs and assertions only."""
import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SITES = ('0AB9:E24C','0AB9:E24F','0AB9:86E9','0AB9:E2D4','0AB9:E2D9',
         '0AB9:E2EF','0AB9:E303','0AB9:E008','0AB9:7D86','0AB9:7DC8','0AB9:7C67')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(research):
    sys.path.insert(0,str(research.resolve()/'runtime/emul'))
    import watched_complete as full
    import match_setup as setup
    import watched_oracle as normalize
    _,base,_ = full.reference()
    _,data = full.match.find_state(base['start_state']['sha256'])
    week = full.match.Week(data)
    output = dict(status='incomplete', incoming=None, intervention=None, points=[], draws=[], first_entry=None)
    class Complete(Exception):
        pass

    def point(g, site):
        if site == SITES[0]:
            if output['incoming'] is not None:
                raise ValueError('duplicate original watched preparation')
            # [confirmed] 09E9 leaves these fields alone;read before19CA seeding.
            output['incoming'] = dict(ball_byte6=g.rd(0xCF4C,1),ball_word7=g.rd(0xCF4D),
                                      ball_word9=g.rd(0xCF4F),scores=[g.rd(0xD086,1),g.rd(0xD087,1)])
            at = g.far(0xD7D2)+0xF2
            output['intervention'] = dict(site=site,field='config+F2',before=g.mem(at,1)[0],after=1)
            g.put(at,b'\x01')
        if output['incoming'] is None:
            raise ValueError('original boundary preceded incoming observation')
        output['points'].append(dict(site=site,draw=week.draws,state=setup.state(week),income_total=g.rd(0xD6E8,4)))
        print(f'original watched preparation {site} draw{week.draws}',flush=True)
        if site == SITES[-1]:
            output['first_entry'] = full.state(g)
            raise Complete()

    for site in SITES:
        week.g.code_hook(site,lambda g,site=site:point(g,site))
    try:
        try:
            week.play()
        except Complete:
            pass
        else:
            raise ValueError('original ended before first watched entry')
        if week.g.unsupported:
            raise ValueError('unsupported original services')
        if [p['site'] for p in output['points']] != list(SITES):
            raise ValueError('original preparation boundary order differs')
        output['draws'] = [normalize.normalized(d) for d in week.events if d['kind']=='draw']
        if len(output['draws']) != 258 or week.draws != 258:
            raise ValueError('original watched preparation draw count differs')
        legacy = output['incoming'];ball = output['first_entry']['ball']
        if (legacy['ball_byte6'],legacy['ball_word7'],legacy['ball_word9'],legacy['scores']) != (ball['byte6'],ball['word7'],ball['word9'],output['first_entry']['scores']):
            raise ValueError('claimed incoming ball/score fields were modified by preparation')
        output['status'] = 'completed'
    except (Exception,KeyboardInterrupt) as error:
        output['status'] = f'aborted:{type(error).__name__}:{error}'
        output['draws'] = [normalize.normalized(d) for d in week.events if d['kind']=='draw']
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--research-root',type=Path,required=True)
    parser.add_argument('--run',action='store_true')
    parser.add_argument('--private',type=Path)
    args = parser.parse_args()
    if args.run == bool(args.private):
        parser.error('choose --run or --private RESULT')
    if args.private:
        expected=json.loads(args.private.read_text())
        if observe(args.research_root) != expected:
            raise ValueError('original watched preparation does not reproduce decoded inputs/assertions')
        print('all incoming values and watched preparation assertions reproduced by original',flush=True)
        return
    dirty = subprocess.check_output(['git','status','--porcelain','--','tools/watched_setup_oracle.py'],cwd=ROOT,text=True)
    if dirty:
        raise ValueError('commit original observer before measurement')
    parent=ROOT/'records/T43';parent.mkdir(parents=True,exist_ok=True)
    number=max((int(p.name.split('-r')[1]) for p in parent.glob('original-r*') if p.is_dir()),default=0)+1
    directory=parent/f'original-r{number:04d}';directory.mkdir()
    protocol=dict(schema='original-watched-preparation/1',producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                  observer_sha256=sha(Path(__file__)),research_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=args.research_root,text=True).strip(),
                  sites=SITES,setup_admission_sha256=sha(ROOT/'engine/tests/fixtures/setup/admission.json'),
                  full_reference_index_sha256=sha(ROOT/'records/T41/reference/index.json'),
                  scope='Incoming supplement observed before19CA;later setup/first-entry states are assertions only. No new RAM archive persisted.')
    (directory/'start.json').write_text(json.dumps(protocol,indent=2)+'\n')
    result=observe(args.research_root)
    (directory/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    manifest=dict(schema='original-watched-preparation-manifest/1',files={name:sha(directory/name) for name in ('start.json','result.json')})
    (directory/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    print(directory.relative_to(ROOT),result['status'],flush=True)
    if result['status']!='completed':
        raise SystemExit(1)


if __name__=='__main__':
    main()
