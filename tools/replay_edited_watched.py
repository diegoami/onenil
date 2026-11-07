#!/usr/bin/env python3
"""T68 candidate comparison; accepted independent-source import is separately gated."""
import argparse,json
from pathlib import Path
import edited_watched_inputs as incoming
import watched_oracle


def compare(got,directory):
    index=json.loads((directory/'index.json').read_text());tick=draw=action=0
    for item in index['chunks']:
        raw=(directory/item['path']).read_bytes()
        if incoming.preparation.sha(raw)!=item['sha256']:raise ValueError('edited candidate chunk digest differs')
        chunk=json.loads(raw)
        if incoming.preparation.sha(incoming.preparation.encoded(chunk['payload']))!=chunk['payload_sha256']:raise ValueError('edited candidate payload differs')
        payload=chunk['payload']
        for expected in payload['ticks']:
            if got['ticks'][tick]!=expected:raise ValueError(f'edited native tick differs: {tick+1}')
            tick+=1
        for expected in payload['draws']:
            if got['draws'][draw]!=watched_oracle.normalized(expected):raise ValueError(f'edited ordered RNG differs: {draw}')
            draw+=1
        for expected in payload['actions']:
            expected={k:v for k,v in expected.items() if k!='event'}
            if got['events'][action]!=expected:raise ValueError(f'edited player action differs: {action}')
            action+=1
    if (len(got['ticks']),len(got['draws']),len(got['events']))!=(tick,draw,action):raise ValueError('edited stream census differs')
    finish_raw=(directory/index['finish']['path']).read_bytes()
    if incoming.preparation.sha(finish_raw)!=index['finish']['sha256']:raise ValueError('edited candidate finish digest differs')
    finish=json.loads(finish_raw)
    if got['final']!=finish['final'] or got['return']!=finish['boundary']:raise ValueError('edited final state/actual return differs')
    return dict(ticks=tick,draws=draw,actions=action,actual_return=got['return'])

def candidate(directory):
    import onenil_engine as native
    session=native.NativePreparedWatchedSession.edited(json.dumps(incoming.request()))
    first=session.snapshot(),session.observation()
    if first!=(session.snapshot(),session.observation()):raise ValueError('edited initial inspection mutates state')
    value=json.loads(session.finish())
    if 'ok' not in value:
        folder=incoming.ROOT/'.cache/T68-candidate';folder.mkdir(parents=True,exist_ok=True)
        number=len(list(folder.glob('failure-r*.json')))+1
        path=folder/f'failure-r{number:04d}.json'
        with path.open('x') as output:json.dump(value,output)
        error=value['error']
        summary={k:error[k] for k in ('message','missing') if k in error}
        raise ValueError(f'edited native failure: {summary}; full local diagnostic {path}')
    result=compare(value['ok'],directory)
    final=session.snapshot(),session.observation()
    if final!=(session.snapshot(),session.observation()):raise ValueError('edited final inspection mutates state')
    if 'error' not in json.loads(session.finish()):raise ValueError('ended edited session accepted second finish')
    try:session.step()
    except ValueError:pass
    else:raise ValueError('ended edited session accepted another step')
    return dict(status='provisional-first-pass-parity',independent_source_acceptance=False,**result)

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--candidate-directory',type=Path,required=True);p.add_argument('--local-record',type=Path);a=p.parse_args()
    # This diagnostic deliberately has no accepted-proof/merge claim.
    result=candidate(a.candidate_directory.resolve())
    if a.local_record:
        with a.local_record.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
    print('PROVISIONAL first-pass comparison only:',json.dumps(result))
