#!/usr/bin/env python3
"""T66: pinned edited preparation inputs and original parity; no Python engine model."""
import argparse
import copy
import hashlib
import json
import subprocess
import sys
from pathlib import Path
import replay_match_setup as prior
import replay_watched_setup as watched
import watched_oracle

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT/'records/T66/source-r0001'
FIXTURE = ROOT/'engine/tests/fixtures/edited_preparation/admission.json'
RESEARCH_COMMIT = 'f646ac2a403ac7e8a3ca8ece485c614927dcd6ac'
REPEAT_SHA = 'a4bdfe03398b7c9de21ca8c6800b7c15cda7873218d413806b643fa31ce7b4af'
PINS = {
 'admission-r0002.json': ('runs/T69/admission-r0002.json', '45ca77eda64349d90700eb3983f6c725b5308e0dcf87c8a788bb081fde3ab627'),
 'support-r0001.json': ('runs/T72/support-r0001.json', 'f5c1bf9e4d22c327b36cfaf7b38151f5dc58cbee06db33382cb7f938652dddd6'),
 'support-r0002.json': ('runs/T72/support-r0002.json', '78186b820c55909b2b181a614bfb4e639494882522f1ffa1c9f8fe96f4081abd'),
 'support-r0003.json': ('runs/T72/support-r0003.json', '75fd970f22015e7ea5b98fa35a947fd9cdf642460ce96700965456d9796e90f6'),
 'support-r0004.json': ('runs/T72/support-r0004.json', '75fd970f22015e7ea5b98fa35a947fd9cdf642460ce96700965456d9796e90f6'),
 'verification-r0001.json': ('runs/T72/verification-r0001.json', '204ba000f0296dbf7eb9200f06edecd289730515918bf6f714f6b70dc154657d'),
}

def read(path):return json.loads(path.read_text())
def sha(data):return hashlib.sha256(data).hexdigest()
def encoded(value):return json.dumps(value,sort_keys=True,separators=(',',':')).encode()

def import_sources(research):
    subprocess.run(['git','merge-base','--is-ancestor',RESEARCH_COMMIT,'HEAD'],cwd=research,check=True)
    subprocess.run([sys.executable,str(research/'runtime/emul/edited_admission.py'),'--check',str(research/'runs/T69/admission-r0003.json'),'--compare',str(research/'runs/T69/admission-r0002.json')],check=True)
    for a,b in ((1,2),(3,4)):
        subprocess.run([sys.executable,str(research/'runtime/emul/edited_support.py'),'--check',str(research/f'runs/T72/support-r{b:04d}.json'),'--compare',str(research/f'runs/T72/support-r{a:04d}.json')],check=True)
    if sha((research/'runs/T69/admission-r0003.json').read_bytes())!=REPEAT_SHA:
        raise ValueError('edited preparation repeat pin differs')
    pending={name:(research/path).read_bytes() for name,(path,_) in PINS.items()}
    for name,data in pending.items():
        if sha(data)!=PINS[name][1]:raise ValueError('edited source pin differs: '+name)
    if DIRECTORY.exists():raise ValueError('refusing to overwrite edited source evidence')
    DIRECTORY.mkdir(parents=True)
    for name,data in pending.items():
        with (DIRECTORY/name).open('xb') as output:output.write(data)

def sources():
    if set(p.name for p in DIRECTORY.iterdir())!=set(PINS):raise ValueError('edited source census differs')
    records={}
    for name,(_,digest) in PINS.items():
        data=(DIRECTORY/name).read_bytes()
        if sha(data)!=digest:raise ValueError('edited original source digest differs: '+name)
        records[name]=json.loads(data)
    original=records['admission-r0002.json']
    if original['status']!='completed' or original['unsupported'] or original['payload']['interventions']:
        raise ValueError('incomplete edited original preparation')
    reference=None;combined={}
    for first in (1,3):
        a,b=(records[f'support-r{i:04d}.json'] for i in (first,first+1))
        if a['payload']!=b['payload'] or a['request']!=b['request']:raise ValueError('edited independent witnesses differ')
        for record in (a,b):
            if record['status']!='completed' or record['unsupported'] or record['interventions'] or record['error'] is not None:
                raise ValueError('incomplete edited input witness')
            if record['source']['sha256']!=PINS['admission-r0002.json'][1]:raise ValueError('edited input source join differs')
        prep=a['payload']['preparation']
        if reference is None:reference=prep
        elif reference!=prep:raise ValueError('edited cross-group preparation differs')
        for row in a['payload']['fields']:
            if row['incoming']:
                if not row['before']==row['precall']==row['entry']:raise ValueError('edited incoming ownership differs')
                key=row['area'],row['offset']
                if key in combined and combined[key]!=row['before']:raise ValueError('edited incoming conflict')
                combined[key]=row['before']
    expected_fields={'managed_selection_mode','legacy_income_total','legacy_queue_zero'}
    before=reference['points'][0]['fields']
    if set(before)!=expected_fields or any(reference['eligible'][k] is not True or before[k]!=reference['points'][1]['fields'][k] for k in expected_fields):
        raise ValueError('edited preparation input ownership differs')
    catalog=dict(preparation_inputs=before,incoming=[dict(area=a,offset=o,value=v) for (a,o),v in sorted(combined.items())])
    manifest=records['verification-r0001.json']
    if sha(encoded(catalog))!=manifest['canonical_catalog_sha256'] or before!=manifest['preparation_inputs'] or len(combined)!=manifest['incoming_support_bytes']:
        raise ValueError('edited catalog manifest differs')
    return original['payload'],copy.deepcopy(before)

def admission(freeze=False):
    original,fields=sources()
    setup=prior.initial_data(original['points'],{'incomes':[{'input':row} for row in original['income_inputs']],'fields':original['supplement']},incoming_fields=fields,contest_k=0)
    value=dict(schema='edited-campaign-watched-preparation/1',research_commit=RESEARCH_COMMIT,sources={k:v[1] for k,v in PINS.items()},repeat_sha256=REPEAT_SHA,input=dict(setup=setup,incoming=original['incoming']))
    if freeze:
        FIXTURE.parent.mkdir(parents=True,exist_ok=True)
        with FIXTURE.open('x') as output:json.dump(value,output,indent=2);output.write('\n')
    if read(FIXTURE)!=value:raise ValueError('edited admission differs from incoming source projection')
    return value,original

def replay(value,original):
    import onenil_engine as native
    prepared=native.NativeEditedWatchedPreparation(json.dumps(value['input']))
    expected=copy.deepcopy(original);expected['draws']=[watched_oracle.normalized(d) for d in original['draws']]
    watched.compare_preparation(json.loads(prepared.preparation()),expected)
    prior.require_equal(json.loads(prepared.snapshot()),original['first_entry'],'edited first entry')
    before=prepared.preparation(),prepared.snapshot()
    if before!=(prepared.preparation(),prepared.snapshot()):raise ValueError('edited observation mutates state')
    print('PASS edited native eleven boundaries,258 ordered draws,every first-entry field')

def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--import-from',type=Path);parser.add_argument('--freeze',action='store_true');parser.add_argument('--check',action='store_true');parser.add_argument('--replay',action='store_true');args=parser.parse_args()
    if args.import_from:import_sources(args.import_from.resolve())
    if args.freeze or args.check or args.replay:
        value,original=admission(args.freeze);print('PASS pinned edited source and before-input projection')
        if args.replay:replay(value,original)
if __name__=='__main__':main()
