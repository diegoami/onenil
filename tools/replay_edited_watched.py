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

def execute(directory):
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
    return result


def require_acceptance():
    import edited_watched_reference as reference
    import edited_lookup_inputs as lookup
    if reference.RESEARCH_ACCEPTANCE is None or lookup.RESEARCH_ACCEPTANCE is None:
        raise ValueError('accepted T71 and T73 research merges required for official verification')
    return reference,lookup


def accepted_replay(record=None):
    reference,lookup=require_acceptance()
    reference.sources()
    incoming.request()
    producer=None
    if record:
        import subprocess
        if subprocess.check_output(['git','status','--porcelain'],cwd=incoming.ROOT,text=True).strip():
            raise ValueError('commit accepted native producer before retaining official verification')
        producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=incoming.ROOT,text=True).strip()
        record.mkdir(parents=True,exist_ok=False)
        start=dict(schema='edited-watched-native-verification/1',producer=producer,
                   original_research=reference.RESEARCH_ACCEPTANCE,lookup_research=lookup.RESEARCH_ACCEPTANCE,
                   original_index_sha256=reference.PINS['index.json'],
                   incoming_sha256=incoming.preparation.sha(incoming.FIXTURE.read_bytes()))
        with (record/'start.json').open('x') as f:json.dump(start,f,indent=2);f.write('\n')
    try:
        result=dict(status='passed',independent_source_acceptance=True,**execute(reference.DIRECTORY))
    except (Exception,KeyboardInterrupt) as error:
        if record:
            with (record/'result.json').open('x') as f:json.dump(dict(status='failed',error=f'{type(error).__name__}: {error}'),f,indent=2);f.write('\n')
        raise
    if record:
        with (record/'result.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
        with (record/'manifest.json').open('x') as f:json.dump(dict(files={name:incoming.preparation.sha((record/name).read_bytes()) for name in ('start.json','result.json')}),f,indent=2);f.write('\n')
    return result


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--candidate-directory',type=Path)
    p.add_argument('--local-record',type=Path)
    p.add_argument('--import-from',type=Path)
    p.add_argument('--check',action='store_true')
    p.add_argument('--replay',action='store_true')
    p.add_argument('--record',type=Path)
    a=p.parse_args()
    if a.candidate_directory:
        if a.check or a.replay or a.record or a.import_from:p.error('candidate comparison is separate from official verification')
        result=dict(status='provisional-first-pass-parity',independent_source_acceptance=False,**execute(a.candidate_directory.resolve()))
        if a.local_record:
            with a.local_record.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
        print('PROVISIONAL first-pass comparison only:',json.dumps(result))
    else:
        reference,_=require_acceptance()
        if a.local_record:p.error('--local-record requires candidate mode')
        if a.record and not a.replay:p.error('--record requires --replay')
        if a.import_from:reference.import_sources(a.import_from.resolve())
        if a.check or a.replay:
            reference.sources();incoming.request()
            print('PASS accepted complete original/repeat reference and exact input projection')
        if a.replay:print('PASS accepted edited native full parity:',json.dumps(accepted_replay(a.record)))
