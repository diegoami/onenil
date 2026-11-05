#!/usr/bin/env python3
"""Supported initial choices through one continuing native prepared watched match."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path
from play_watched import ROOT, prepared_input, compute_prepared, validate_choices, describe

CHOICES = [dict(lineup=lineup, seed=seed) for lineup in ('reference', 'reserve') for seed in ('64', '122')]
def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':')).encode()

def check_inputs():
    import replay_prepared_match as reference
    if prepared_input(CHOICES[0]) != reference.admission():
        raise ValueError('reference initial admission changed')
    invalid = [None, {}, {'lineup':'unknown','seed':'64'}, {'lineup':'reference','seed':64},
               {'lineup':[],'seed':'64'}, {'lineup':'reference','seed':'64','expected':{}},
               {'lineup':'reserve','seed':'0'}, {'lineup':True,'seed':'122'}]
    for candidate in invalid:
        try: validate_choices(candidate)
        except ValueError: pass
        else: raise AssertionError('invalid supported choice accepted')
    print('Pinned reference initial input unchanged;8 invalid choices rejected', flush=True)

def validate_output(choices, output, initial, setup_draws):
    import replay_prepared_match as reference
    ticks = output['ticks'];events = output['events'];clocks = output['caller_clocks']
    assert output['return'] == '0AB9:EBE2' and ticks
    assert [t['tick'] for t in ticks] == list(range(1, len(ticks)+1))
    assert len(output['tick_live_goals']) == len(ticks)
    assert output['tick_live_goals'][-1] == output['live_goals'] == list(reversed(output['final']['scores']))
    assert all(len(t['after']['scenes']) == 22 for t in ticks)
    ids = [r['tick'] for r in clocks]
    assert ids and ids == sorted(set(ids)) and ids[0] == 1 and ids[-1] <= len(ticks)
    for row in clocks:
        assert set(row['clock']) == set(reference.CLOCK_FIELDS)
        assert all(type(row['clock'][k]) is int and 0 <= row['clock'][k] <= limit for k,limit in reference.CLOCK_FIELDS.items())
    assert [e['tick'] for e in events] == sorted(e['tick'] for e in events)
    whistles = [e for e in events if e['code'] == 63]
    assert len(whistles) == 1 and ids[-1] == whistles[0]['tick']
    assert len([e for e in events if e['code'] == 64]) == 1
    names = {int(k):v for k,v in json.loads((ROOT/'demo/metadata.json').read_text())['players'].items()}
    described = [describe(e, names) for e in output['setup_events']+events]
    assert all('Play continues' not in e['text'] for e in described)
    starter = 2590 if choices['lineup'] == 'reference' else 2593
    assert starter in [s['word14'] for s in initial['scenes']]
    assert setup_draws[0]['seed_before'] == int(choices['seed'])
    assert output['draws'][:len(setup_draws)] == setup_draws
    if choices == CHOICES[0]:
        reference.compare(output)
    return dict(choices=choices, actual_starting_players=[p['word14'] for p in initial['scenes']],
                initial_seed=setup_draws[0]['seed_before'], setup_draws=len(setup_draws),
                ticks=len(ticks), draws=len(output['draws']), actions=len(events), clock_rows=len(clocks),
                score=output['live_goals'], actual_return=output['return'],
                input_sha256=hashlib.sha256(canonical(prepared_input(choices))).hexdigest(),
                output_sha256=hashlib.sha256(canonical({'ok':output})).hexdigest(),
                clock_sha256=hashlib.sha256(canonical(clocks)).hexdigest())

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true');parser.add_argument('--replay', action='store_true')
    parser.add_argument('--out', type=Path)
    args=parser.parse_args()
    if not (args.check or args.replay):parser.error('choose --check and/or --replay')
    if args.check:check_inputs()
    summaries=[]
    if args.replay:
        for choices in CHOICES:
            output,initial,draws=compute_prepared(choices)
            summary=validate_output(choices,output,initial,draws);summaries.append(summary)
            print(json.dumps(summary, sort_keys=True), flush=True)
            del output,initial,draws
    if args.out:
        record=dict(scope='Four supported native completions;only reference64 claims original-reference parity.',
                    code_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),
                    product_source_sha256=hashlib.sha256((ROOT/'tools/play_watched.py').read_bytes()).hexdigest(),
                    verifier_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),sessions=summaries)
        with args.out.open('x') as out:json.dump(record,out,indent=2);out.write('\n')
if __name__=='__main__':main()
