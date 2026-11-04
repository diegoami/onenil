"""[designed] One native admitted image;original results are assertions only."""
import argparse,copy,hashlib,json,subprocess,time,platform
from datetime import datetime,timezone
from importlib.metadata import version
from pathlib import Path
import watched_tick_oracle as ticks
import watched_oracle as oracle
import watched_action_export as facts
ROOT=oracle.ROOT
FIXTURES=ROOT/'engine/tests/fixtures/watched_tick'
ACTUAL_SHA='61321a752ec7f0637a2cf54b534c9fa39618961615a8fe2b886d34ab5da249ef'
NEUTRAL_SHA='d11d0102eadf31eb0242b37467c90756ef47ed10a1c55ecd171884f699d186da'
FIXTURE_SHA='769006f19ebf35f5ed39403edbd1c42c4bb4715d3ed0fc41b2be72d233903b59'
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
    if (FIXTURES/'manifest.json').exists():return check()
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

def values_at_entry(state):
    out={(f['area'],f['offset']):f['value']for f in state['cells']if f['area']!='frame'}
    for key,value in state['scene'].items():
        if key=='index':continue
        width=2 if key.startswith('word')else 1;offset=int(key[4:])
        for i,b in enumerate(value.to_bytes(width,'little')):out['ds',0xCCEF+25*state['scene']['index']+offset+i]=b
    for i,b in enumerate(state['seed'].to_bytes(2,'little')):out['ds',0xA51C+i]=b
    for pointer in state['pointers']:
        slot=0xD0C0 if pointer['kind']=='style'else facts.SLOTS[pointer['kind']]
        for i,b in enumerate(pointer['offset'].to_bytes(2,'little')+pointer['segment'].to_bytes(2,'little')):out['ds',slot+i]=b
    return out

def check():
    path=ROOT/'records/T39/neutral-input/run-0001.json'
    if sha(path)!=NEUTRAL_SHA:raise ValueError('original no-key proof pin differs')
    proof=read(path)
    if proof['count']!=176 or len(proof['calls'])!=176 or any(c!={'tick':i//22+1,'AL':0,'site':'0AB9:791B'}for i,c in enumerate(proof['calls'])):raise ValueError('original no-key branch proof differs')
    (actual,am),(repeat,rm)=sources();m=read(FIXTURES/'manifest.json')
    if sha(FIXTURES/'manifest.json')!=FIXTURE_SHA:raise ValueError('native tick fixture manifest pin differs')
    if m['actual_manifest_sha256']!=ACTUAL_SHA or m['repeat_manifest_sha256']!=REPEAT_SHA or len(m['cases'])!=9:raise ValueError('decoded tick corpus provenance differs')
    values=[]
    for i,item in enumerate(m['cases']):
        path=ROOT/item['path']
        if sha(path)!=item['sha256']:raise ValueError('decoded tick fixture digest differs')
        value=read(path);c=actual[i]if i<8 else actual[0];outputs=[c]if i<8 else repeat
        if value['case']!=i or value['steps']!=len(outputs)or value['no_pending_key']is not True:raise ValueError('native tick session identity differs')
        if value['source_sha256']!=(am['cases'][i]['sha256']if i<8 else REPEAT_SHA):raise ValueError('native tick session source differs')
        state=value['initial']
        for key in c['context']:
            if key!='cells'and state[key]!=c['context'][key]:raise ValueError('native tick ENTRY context differs')
        facts_at=values_at_entry(state)
        if len(value['expected'])!=len(outputs):raise ValueError('decoded tick output count differs')
        for source,want in zip(outputs,value['expected'],strict=True):
            for change in source['changes']:
                if set(change)!={'area','offset','before','after'}:raise ValueError('undecoded original gameplay change')
                key=change['area'],change['offset']
                if key not in facts_at or facts_at[key]!=change['before']:raise ValueError('original changed undeclared/inconsistent gameplay baseline')
                facts_at[key]=change['after']
            expected_cells=[{**f,'value':facts_at[f['area'],f['offset']]}for f in state['cells']if f['area']!='frame']
            if want['cells']!=expected_cells:raise ValueError('decoded tick gameplay cells differ from original changes')
            original_events=copy.deepcopy(source['events'])
            if 'tick'in source:
                for event in original_events:
                    for field in('draw_start','draw_end'):event[field]-=source['draw_start']
            if any(want[key]!=source['output'][key]for key in('scenes','seed','pointers'))or want['draws']!=source['draws']or want['events']!=original_events:raise ValueError('decoded tick numeric outputs differ')
        values.append(value)
    return values

def verify_private(values):
    (actual,_),(repeat,_)=sources()
    for i,value in enumerate(values):
        c=actual[i]if i<8 else actual[0];state=value['initial'];raw=oracle.memory(c['entry']);seen=set()
        for fact in state['cells']:
            at=address(state,fact['area'],fact['offset'])
            if at in seen or ticks.named(state,at)!=(fact['area'],fact['offset'])or raw[at]!=fact['value']:raise ValueError('native tick fact is not a unique original initial ENTRY value')
            seen.add(at)
        outputs=[c]if i<8 else repeat
        for source,want in zip(outputs,value['expected'],strict=True):
            if expected(source,state)!=want:raise ValueError('private original exit/whole-gameplay ownership differs')
    print('original initial ENTRY facts,exit expectations and ownership verified for9sessions/16ticks',flush=True)

def replay(values):
    import onenil_engine as native
    for value in values:
        got=json.loads(native.watched_tick(json.dumps({key:value[key]for key in('initial','steps','no_pending_key')})))
        if got.get('ok')!=value['expected']:raise ValueError('native watched tick session differs:'+str(value['case'])+' '+str(got.get('error')))
    print('native persistent watched ticks:9sessions/16original transitions matched',flush=True)

def check_research(research):
    (actual,_),_=sources();chunk=read(research/'runs/T40/chunk-000000.json')['payload'];protocol=read(ROOT/'records/T39/oracle-r0001/start.json')
    if sha(research/'runs/T40/chunk-000000.json')!=protocol['reference_chunk_sha256']:raise ValueError('original complete-reference digest differs')
    for c,reference in zip(actual,chunk['ticks'][:8],strict=True):
        for field,side in [('input','before'),('output','after')]:
            if c[field]['seed']!=reference[side]['seed']:raise ValueError('original tick/reference seed differs')
            def decoded(scene):
                raw=bytearray(25)
                for key,value in scene.items():
                    if key!='index':
                        width=2 if key.startswith('word')else 1;offset=int(key[4:]);raw[offset:offset+width]=value.to_bytes(width,'little')
                return raw
            if [decoded(s)for s in c[field]['scenes']]!=[decoded(s)for s in reference[side]['scenes']]:raise ValueError('original tick/reference scenes differ')
        if(c['draw_start'],c['draw_end'])!=(reference['draw_start'],reference['draw_end']):raise ValueError('original tick/reference draw cursors differ')
        draws=[oracle.normalized(d)for d in chunk['draws'][c['draw_start']:c['draw_end']]]
        if c['draws']!=draws:raise ValueError('original tick/reference ordered draws differ')
        events=[{k:v for k,v in event.items()if k not in('tick','event')}for event in chunk['actions']if event['tick']==c['tick']and c['draw_start']<=event['draw_start']and event['draw_end']<=c['draw_end']]
        # Tick-labelled caller events after the root return are not root events.
        if c['events']!=events:raise ValueError('original root/reference action interval differs')
    print('8actual original tick entries/returns join complete watched reference',flush=True)

def benchmark(values,repetitions):
    from onenil_engine import onenil_engine as native
    if oracle.common.git('status','--porcelain','--','engine','tools','pyproject.toml'):raise ValueError('commit native benchmark sources first')
    if native.__debug_assertions__:raise ValueError('release extension required')
    inputs=[json.dumps({k:v[k]for k in('initial','steps','no_pending_key')},sort_keys=True,separators=(',',':'))for v in values]
    began=datetime.now(timezone.utc).isoformat();start=time.perf_counter_ns();size=0
    for _ in range(repetitions):
        for value in inputs:size+=len(native.watched_tick(value))
    elapsed=time.perf_counter_ns()-start;directory=ROOT/'records/T39';n=len(list(directory.glob('T39-r*.json')))+1;path=directory/f'T39-r{n:04d}.json'
    oracle.write_json(path,{'schema':'routine-benchmark/1','code_commit':oracle.common.git('rev-parse','HEAD'),'started':began,'elapsed_ns':elapsed,'sessions':len(values)*repetitions,'ticks':sum(v['steps']for v in values)*repetitions,'repetitions':repetitions,'serialized_output_bytes':size,'platform':platform.platform(),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),'extension_sha256':sha(Path(native.__file__)),'cargo_lock_sha256':sha(ROOT/'engine/Cargo.lock'),'debug_assertions':native.__debug_assertions__,'maturin':version('maturin'),'pins':{'actual':ACTUAL_SHA,'repeated':REPEAT_SHA,'fixtures':FIXTURE_SHA},'method':'Pre-serialized initial admissions;native parsing,one image per session,all computed persistent advances and serialization included','limits':'Watched7A02 root only;no outer clock/caller,complete match or emulator-speed comparison'})
    print(path.relative_to(ROOT),flush=True)

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--export',action='store_true');p.add_argument('--probe',type=Path);p.add_argument('--check',action='store_true');p.add_argument('--check-private',action='store_true');p.add_argument('--check-research',type=Path);p.add_argument('--replay',action='store_true');p.add_argument('--benchmark',action='store_true');p.add_argument('--repetitions',type=int,default=5);a=p.parse_args()
    if a.repetitions<1:p.error('positive repetitions required')
    if a.export:
        if a.probe is None:p.error('--export requires --probe')
        export(a.probe)
    if a.check_research:check_research(a.check_research)
    if a.check or a.check_private or a.replay or a.benchmark:
        values=check();print('decoded watched tick corpus:9sessions/16outputs verified',flush=True)
        if a.check_private:verify_private(values)
        if a.replay or a.benchmark:replay(values)
        if a.benchmark:benchmark(values,a.repetitions)
    if not(a.export or a.check or a.check_private or a.check_research or a.replay or a.benchmark):p.error('choose a capture validation or replay operation')
if __name__=='__main__':main()
