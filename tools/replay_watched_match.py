"""[designed] Full original watched reference assertions;never a gameplay model."""
import argparse,hashlib,json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
REFERENCE=ROOT/'records/T41/reference'
INDEX_SHA='30a4fe8003f4677c2e2af0966d5fe81055c728a1b41c998a9d03766637414886'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def read(p):return json.loads(p.read_text())
def sources():
 if sha(REFERENCE/'index.json')!=INDEX_SHA:raise ValueError('full original reference index differs')
 index=read(REFERENCE/'index.json');ticks=[];draws=[];events=[]
 for item in[index['start'],*index['chunks'],index['finish']]:
  path=REFERENCE/Path(item['path']).name
  if sha(path)!=item['sha256']:raise ValueError('full original reference source differs')
  value=read(path)
  if 'payload'in value:
   payload=value['payload']
   if hashlib.sha256(json.dumps(payload,sort_keys=True,separators=(',',':')).encode()).hexdigest()!=value['payload_sha256']:raise ValueError('full original payload differs')
   if payload['draw_start']!=len(draws)or payload['action_start']!=len(events)or payload['tick_start']!=len(ticks)+1:raise ValueError('full original chunk continuity differs')
   ticks+=payload['ticks'];draws+=payload['draws'];events+=payload['actions']
 finish=read(REFERENCE/Path(index['finish']['path']).name)
 if(len(ticks),len(draws),len(events))!=(4199,182811,831)or finish['status']!='completed'or finish['boundary']!='0AB9:EBE2':raise ValueError('full original coverage/return differs')
 if[entry['tick']for entry in ticks]!=list(range(1,4200))or[entry['event']for entry in events]!=list(range(831)):raise ValueError('full original observation ordering differs')
 if not any(e['code']==64 for e in events)or not any(e['code']==63 for e in events):raise ValueError('full original phase boundaries missing')
 print('pinned original4199ticks/182811draws/831actions and actual fulltime return verified',flush=True)
 return ticks,draws,events,finish

PREFIX_SOURCE=None
def compare_prefix(got):
 global PREFIX_SOURCE
 if PREFIX_SOURCE is None:PREFIX_SOURCE=sources()
 ticks,draws,events,_=PREFIX_SOURCE;base=258
 for index,actual in enumerate(got['ticks']):
  source=ticks[index];expected=dict(source,draw_start=source['draw_start']-base,draw_end=source['draw_end']-base)
  if actual!=expected:
   path=ROOT/'.cache/T41/tick-mismatch.json';path.write_text(json.dumps({'tick':index+1,'actual':actual,'expected':expected},indent=2));raise ValueError(f'native completed tick{index+1} differs before missing input;private diagnostic retained')
 import watched_oracle as oracle
 expected=[oracle.normalized(d)for d in draws[base:base+len(got['draws'])]]
 if got['draws']!=expected:raise ValueError('native ordered RNG prefix differs before missing input')
 expected=[]
 for source in events[:len(got['events'])]:
  event={k:v for k,v in source.items()if k!='event'}
  for key in('draw_start','draw_end'):event[key]-=base
  expected.append(event)
 if got['events']!=expected:raise ValueError('native accepted-action prefix differs before missing input')
 return len(got['ticks']),len(got['draws'])

def compare(got):
 import watched_oracle as oracle
 ticks,draws,events,finish=sources();base=258
 expected_ticks=[]
 for original in ticks:
  tick=dict(original,draw_start=original['draw_start']-base,draw_end=original['draw_end']-base);expected_ticks.append(tick)
 if got['ticks']!=expected_ticks:
  for index,(actual,expected)in enumerate(zip(got['ticks'],expected_ticks),1):
   if actual!=expected:
    path=ROOT/'.cache/T41/tick-mismatch.json';path.write_text(json.dumps({'tick':index,'actual':actual,'expected':expected},indent=2));raise ValueError(f'full native tick{index} differs;private diagnostic retained')
  raise ValueError('full native watched tick count differs')
 expected_draws=[oracle.normalized(d)for d in draws[base:]]
 if got['draws']!=expected_draws:raise ValueError('full native ordered RNG differs')
 expected_events=[]
 for entry in events:
  event={k:v for k,v in entry.items()if k!='event'}
  for key in('draw_start','draw_end'):event[key]-=base
  expected_events.append(event)
 if got['events']!=expected_events:raise ValueError('full native ordered accepted actions differ')
 if got['final']!=finish['final']or got['return']!=finish['boundary']:raise ValueError('full native final snapshot/return differs')
 print('native4199watched ticks,182553computed draws plus258setup context,831actions and fulltime return match',flush=True)

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--check',action='store_true');a=p.parse_args()
 if a.check:sources()
 else:p.error('choose --check')
if __name__=='__main__':main()
