"""[designed] One native admitted image;original results are assertions only."""
import argparse,copy,hashlib,json,subprocess
from pathlib import Path
import watched_tick_oracle as ticks
import watched_oracle as oracle
import watched_action_export as facts
ROOT=oracle.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/watched_tick'
ACTUAL_SHA='61321a752ec7f0637a2cf54b534c9fa39618961615a8fe2b886d34ab5da249ef'
REPEAT_SHA='9bd16e75ab9e32f2afa5933a19f280db1712cb00a259e0c14b977656e1ee42e4'
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def read(path):return json.loads(path.read_text())
def sources():
    out=[]
    for directory,pin in [(ROOT/'records/T39/oracle-r0001',ACTUAL_SHA),(ROOT/'records/T39/repeat-r0001',REPEAT_SHA)]:
        if sha(directory/'manifest.json')!=pin:raise ValueError('original tick manifest pin differs')
        m=read(directory/'manifest.json')
        if m['status']!='completed'or m['count']!=8 or len(m['cases'])!=8:raise ValueError('incomplete original tick corpus')
        if sha(directory/'start.json')!=m['protocol_sha256']:raise ValueError('original tick protocol differs')
        values=[]
        for i,item in enumerate(m['cases'],1):
            path=ROOT/item['path']
            if sha(path)!=item['sha256']:raise ValueError('original tick observation digest differs')
            c=read(path)
            if c.get('tick',c.get('step'))!=i:raise ValueError('original tick order differs')
            values.append(c)
        out.append((values,m))
    return out

def address(state,area,offset):
    if area=='style':
        p=state['pointers'][-1]
        if offset!=0 or p['kind']!='style':raise ValueError('bad style fact')
        return 16*p['segment']+p['offset']
    return facts.address(state,area,offset)
class Probe:
    def __init__(self,path):self.p=subprocess.Popen([str(path)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True)
    def run(self,initial,steps):
        self.p.stdin.write(json.dumps({'initial':initial,'steps':steps,'no_pending_key':True})+'\n');self.p.stdin.flush();line=self.p.stdout.readline()
        if not line:raise ValueError('native tick probe stopped')
        return json.loads(line)
    def close(self):self.p.stdin.close();assert self.p.wait()==0

def expected(c,state):
    raw=oracle.memory(c['exit']);out=copy.deepcopy(c['output']);out['cells']=[]
    for f in state['cells']:
        if f['area']!='frame':out['cells'].append({**f,'value':raw[address(state,f['area'],f['offset'])]})
    out['draws']=c['draws'];out['events']=copy.deepcopy(c['events'])
    if 'tick'in c:
        for e in out['events']:
            for key in('draw_start','draw_end'):e[key]-=c['draw_start']
    known={address(state,f['area'],f['offset'])for f in state['cells']};ds=16*state['regs'][11]
    known.update(range(ds+0xCCEF+25,ds+0xCCEF+50));known.update(range(ds+0xA51C,ds+0xA51E))
    for slot in [*facts.SLOTS.values(),0xD0C0]:known.update(range(ds+slot,ds+slot+4))
    for d in c['changes']:
        if address(state,d['area'],d['offset'])not in known:raise ValueError('original changed undeclared gameplay field:'+str(d))
    return out

def export(probe):
    (actual,am),(repeat,rm)=sources();native=Probe(probe);items=[];FIXTURES.mkdir(parents=True,exist_ok=True)
    catalog=[([c],c['entry'],c['context'],am['cases'][i]['sha256'],'observed-tick')for i,c in enumerate(actual)]+[(repeat,actual[0]['entry'],actual[0]['context'],REPEAT_SHA,'isolated-persistent-sequence')]
    for number,(outputs,entry,context,source_sha,kind)in enumerate(catalog):
        path=FIXTURES/f'case-{number:02d}.json'
        if not path.exists():
            raw=oracle.memory(entry);state=copy.deepcopy(context);state['cells']=[];seen=set()
            # All22 scene outputs belong to this tick;seed their unmodified ENTRY bytes.
            ds=16*state['regs'][11]
            for i in range(2,23):
                for k in range(25):
                    o=0xCCEF+25*i+k;state['cells'].append({'area':'ds','offset':o,'value':raw[ds+o]});seen.add(ds+o)
            for attempt in range(20000):
                got=native.run(state,len(outputs))
                if 'ok'in got:break
                error=got['error'];missing=error['missing']
                if missing is None:raise ValueError(f'tick session{number} unsupported: '+error['message'])
                try:area,offset=ticks.named(state,missing['address'])
                except ValueError as e:raise ValueError(f'tick session{number} unclassified: '+str(error))from e
                at=address(state,area,offset)
                if at in seen:raise ValueError('native tick repeated missing ENTRY fact:'+str(error))
                seen.add(at);state['cells'].append({'area':area,'offset':offset,'value':raw[at]})
            else:raise ValueError('native initial-entry selection did not converge')
            want=[expected(c,state)for c in outputs]
            if got['ok']!=want:
                for step,(a,b)in enumerate(zip(got['ok'],want,strict=True),1):
                    for key in b:
                        if a[key]!=b[key]:
                            debug=ROOT/'.cache/T39/parity-mismatch.json';debug.write_text(json.dumps({'session':number,'step':step,'field':key,'actual':a[key],'expected':b[key]},indent=2));raise ValueError(f'tick session{number} step{step} {key} differs;private development diagnostic at {debug}')
            oracle.write_json(path,{'case':number,'kind':kind,'source_sha256':source_sha,'initial':state,'no_pending_key':True,'steps':len(outputs),'expected':want})
        items.append({'path':str(path.relative_to(ROOT)),'sha256':sha(path)});print('native watched tick session',number,'matched',len(outputs),'original outputs',flush=True)
    native.close();out={'schema':'decoded-native-watched-ticks/1','producer':oracle.common.git('rev-parse','HEAD'),'actual_manifest_sha256':ACTUAL_SHA,'repeat_manifest_sha256':REPEAT_SHA,'cases':items,'scope':'Single native ENTRY admission per session;no intermediate inputs. Repeated-root sequence omits outer clock/caller.'};path=FIXTURES/'manifest.json'
    if not path.exists():oracle.write_json(path,out)
    elif read(path)!=out:raise ValueError('decoded tick manifest differs')

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path,required=True);a=p.parse_args()
    if a.export:export(a.probe)
    else:p.error('choose export')
if __name__=='__main__':main()
