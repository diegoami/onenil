#!/usr/bin/env python3
"""[derived] Review126 F1/F3 observations from existing accepted captures and listings."""
import argparse,hashlib,json,re,subprocess
from pathlib import Path
PIN='68cd3396ec3621d55df8c9377cd5c4b7b74c1314'
p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);p.add_argument('--record',type=Path);a=p.parse_args()
sha=lambda b:hashlib.sha256(b).hexdigest()
def original(name):
 data=subprocess.check_output(['git','-C',str(a.research_root),'show',PIN+':'+name]);assert data==(a.research_root/name).read_bytes();return data
sources={}
for name,addresses in {
 'static/1-0-exe/code/seg_0AB9_14.asm':['8B0B','8B47','8B58'],
 'static/1-0-exe/code/seg_0AB9_17.asm':['B2A8','B2EB','B2F4'],
}.items():
 data=original(name)
 for address in addresses:assert any(line.startswith(address+' ') for line in data.decode().splitlines()),address
 sources[name]=dict(sha256=sha(data),addresses=addresses)
observations=[]
for step,position,positioning,passing in [(15,'goalkeeper','come out of box P','throw the ball'),(44,'striker','stay back P','short passes'),(45,'goalkeeper','stay in box P','throw the ball'),(46,'sweeper','stay back P','short passes')]:
 name=f'runs/T46/session-r0001/step-{step:04d}.json';raw=original(name);record=json.loads(raw);picture=original(name.replace('.json','.png'));assert sha(picture)==record['screenshot_sha256']
 lines=record['text'].splitlines();rows={}
 for label in ['Positioning','Possession','Passing','Tackling','Mark']:
  indices=[i for i,line in enumerate(lines) if re.match(r'^│ '+label+r'\s',line)];assert len(indices)==1
  i=indices[0];value=re.split('[│├┤]',lines[i].split(label,1)[1],maxsplit=1)[0];rows[label]=dict(row=i,value=' '.join(value.split()))
 assert [r['row'] for r in rows.values()]==[13,14,15,16,17]
 assert rows['Positioning']['value']==positioning and rows['Passing']['value']==passing
 position_line=next(line for line in lines if 'Pos.' in line and '1st pos' in line)
 current=position_line.split('Pos.',1)[1].split('1st pos',1)[0].strip();club=position_line.split('1st pos',1)[1].split('│',1)[0].strip()
 assert current==position and club==('goalkeeper' if step==15 else 'centre back')
 if step in [15,45]:assert rows['Mark']['value']==''
 observations.append(dict(step=step,record=name,record_sha256=sha(raw),screenshot_sha256=sha(picture),current_position=current,club_position=club,rows=rows))
result=dict(schema='review126-source/1',research_commit=PIN,sources=sources,observations=observations,conclusions=['Five displayed rows, including blank Mark for the captured goalkeeper','Repositioned Pearce already displays goalkeeper alternatives in Details despite his club role','Current-role table substitution is shared by the Details/instruction presenter'],limits=['No new original run','No broader outfield instruction-action claim','Chooser edge clamping is native design, not an original observation'])
if a.record:
 with a.record.open('x') as f:json.dump(result,f,indent=2);f.write('\n')
print('PASS 4 original capture/image identities,5 displayed rows and Pearce current-role label observations;2 listing identities')
