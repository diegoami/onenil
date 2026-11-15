#!/usr/bin/env python3
"""T84 authenticate accepted early original leaves; no engine model or late inputs."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
ROOT=Path(__file__).resolve().parents[1]
DIRECTORY=ROOT/'engine/tests/fixtures/post_match_incoming'
COMMIT='ef8dad0ea8a7a20cdfa86ea048cf319a02b3a163'
PINS={'incoming-r0001.json':'d301ca8d2cb507522b8f2ab8882520e86047bb43671a957714ede0f5138d435a',
      'incoming-r0002.json':'e84b5c800cf6bdfd8ac6bc6873514accef954df24bf237b86d68dd6281bdd4f0'}
def verify_local(directory=None):
    directory=DIRECTORY if directory is None else directory
    if {p.name for p in directory.glob('*.json')}!=set(PINS):raise ValueError('early leaf census differs')
    payloads=[]
    for name,digest in PINS.items():
        data=(directory/name).read_bytes()
        if hashlib.sha256(data).hexdigest()!=digest:raise ValueError('early original fixture digest differs: '+name)
        payloads.append(json.loads(data)['payload'])
    if payloads[0]!=payloads[1]:raise ValueError('early original payloads differ')

def verify(research):
    verify_local()
    recorder='runtime/emul/postmatch_incoming.py'
    accepted=subprocess.check_output(['git','-C',str(research),'show',COMMIT+':'+recorder])
    if (research/recorder).read_bytes()!=accepted:raise ValueError('accepted source checker differs')
    subprocess.run([sys.executable,str(research/recorder),'--check'],check=True)
    for name,digest in PINS.items():
        native=(DIRECTORY/name).read_bytes()
        original=subprocess.check_output(['git','-C',str(research),'show',COMMIT+':records/T87/'+name])
        if hashlib.sha256(native).hexdigest()!=digest or native!=original:raise ValueError('early original fixture digest differs: '+name)
    return dict(commit=COMMIT,source_sha256=PINS,scope='Early values only; known/selected fields and canonical ABI bindings retain native ownership.')
def main():
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--research-root',required=True,type=Path);parser.add_argument('--check',action='store_true',required=True);a=parser.parse_args()
    print(json.dumps(verify(a.research_root.resolve()),indent=2))
if __name__=='__main__':main()
