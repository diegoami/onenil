"""[designed] Mutation checks for independent original frame-witness joins;no engine model."""
import hashlib,json,subprocess,sys,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tools'))
import replay_watched_selection as replay
SOURCE=ROOT/'engine/tests/fixtures/watched_selection'
def mutate_draw(rows,capture):
    row=next(r for r in rows if r['output']['draws'])
    row['output']['draws'][0]['seed_after']^=1
def mutate_source(rows,capture):capture['source']['prefix_record_sha256']='0'*64
scenarios=[
    ('valid',None,None),
    ('entry scene',lambda rows,c:rows[0]['input']['scene'].__setitem__('word8',rows[0]['input']['scene']['word8']^1),'state join'),
    ('exit scene',lambda rows,c:rows[0]['output']['output']['scene'].__setitem__('word8',rows[0]['output']['output']['scene']['word8']^1),'state join'),
    ('ordered draw',mutate_draw,'draw/fact join'),
    ('boundary',lambda rows,c:rows[0].__setitem__('tick',rows[0]['tick']+1),'boundary join'),
    ('source prefix',mutate_source,'source differs'),
    ('boolean frame',lambda rows,c:rows[0]['input'].__setitem__('frame_byte5',True),'invalid raw entry'),
    ('missing case',lambda rows,c:rows.pop(),'count differs'),
    ('helper field',lambda rows,c:rows[0]['input'].__setitem__('helper_result',1),'fields differ'),
]
results=[]
for name,mutate,expected in scenarios:
    with tempfile.TemporaryDirectory(prefix='onenil-frame-join-')as directory:
        root=Path(directory);fixture=root/'fixtures';fixture.mkdir()
        for p in SOURCE.iterdir():
            if p.is_file():(fixture/p.name).write_bytes(p.read_bytes())
        for name_pin in ['provenance.json','frames-provenance.json']:
            pin=json.loads((fixture/name_pin).read_text());target=root/pin['capture'];target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes((ROOT/pin['capture']).read_bytes())
        if mutate:
            pin=json.loads((fixture/'frames-provenance.json').read_text());target=root/pin['capture'];capture=json.loads(target.read_text())
            rows=[json.loads(line)for line in(fixture/'frames.jsonl').read_text().splitlines()];mutate(rows,capture);capture['cases']=rows
            raw=''.join(replay.common.encoded(row)+'\n'for row in rows).encode();(fixture/'frames.jsonl').write_bytes(raw)
            target.write_text(json.dumps(capture));pin.update(source=capture['source'],cases_sha256=replay.common.sha(raw),capture_sha256=replay.common.sha(target.read_bytes()))
            (fixture/'frames-provenance.json').write_text(json.dumps(pin))
        replay.ROOT=root;replay.FIXTURES=fixture
        try:
            cases,pins=replay.load()
            if expected:raise AssertionError('mutation accepted:'+name)
            assert len(cases)==64 and len(pins)==2
            results.append({'case':name,'result':'accepted'})
        except ValueError as error:
            if expected is None or expected not in str(error):raise
            results.append({'case':name,'result':'rejected','reason':str(error)})
print(json.dumps({'schema':'frame-join-validation/1','code_commit':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip(),'script_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'results':results,'status':'passed','scope':'Copies only;digests/capture exports updated coherently before mutation checks;original records untouched'},indent=2))
