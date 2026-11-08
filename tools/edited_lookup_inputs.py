"""T68: authenticate T73's exact independent bitmap witnesses before projection."""
import argparse,json,subprocess,sys
from pathlib import Path
import replay_edited_preparation as preparation
DIRECTORY=preparation.ROOT/'records/T68/lookup-source-r0001'
PINS={f'lookup-r{i:04d}.json':'55b9a3e238732b99a810a8837d1a74714f5578cf2c50ac38c098847c3984689b' for i in (1,2)}

def import_sources(research):
    subprocess.run([sys.executable,str(research/'runtime/emul/edited_lookup_support.py'),'--check',str(research/'runs/T73/lookup-r0002.json'),'--compare',str(research/'runs/T73/lookup-r0001.json')],check=True)
    pending={name:(research/'runs/T73'/name).read_bytes() for name in PINS}
    for name,data in pending.items():
        if preparation.sha(data)!=PINS[name]:raise ValueError('edited lookup import pin differs')
    if DIRECTORY.exists():raise ValueError('immutable lookup import already exists')
    DIRECTORY.mkdir(parents=True)
    for name,data in pending.items():
        with (DIRECTORY/name).open('xb') as out:out.write(data)
    sources()

def sources():
    if set(p.name for p in DIRECTORY.iterdir())!=set(PINS):raise ValueError('lookup source census differs')
    original,_=preparation.sources();records=[]
    joins=[preparation.sha(preparation.encoded(original['points'][i]['state'])) for i in (0,7)]+[preparation.sha(preparation.encoded(original['first_entry']))]
    for name,digest in PINS.items():
        raw=(DIRECTORY/name).read_bytes()
        if preparation.sha(raw)!=digest:raise ValueError('edited lookup source pin differs')
        v=json.loads(raw);records.append(v)
        if v['schema']!='original-edited-lookup/1' or v['status']!='completed' or v['error'] is not None or v['unsupported'] or v['interventions']:raise ValueError('lookup source incomplete')
        if v['source']['sha256']!=preparation.PINS['admission-r0002.json'][1] or v['ui']!=original['ui']:raise ValueError('lookup edited case identity differs')
        p=v['payload']
        if [x['site'] for x in p['points']]!=['0AB9:E24C','0AB9:E008','0AB9:7C67'] or [x['join_sha256'] for x in p['points']]!=joins:raise ValueError('lookup source state join differs')
        if p['draw_count']!=258 or p['draws_sha256']!=preparation.sha(preparation.encoded(original['draws'])):raise ValueError('lookup source RNG join differs')
    if any(records[0][k]!=records[1][k] for k in ('payload','request','ui','keys_delivered','source')):raise ValueError('independent lookup differs')
    rows=records[0]['payload']['fields'];seen=set();result=[]
    for row in rows:
        key=row['area'],row['offset']
        if key in seen or key[0]!='ds' or type(key[1]) is not int or not 0xD36<=key[1]<0xD56:raise ValueError('foreign/duplicate lookup field')
        seen.add(key)
        if row['incoming'] is not True or not row['before']==row['precall']==row['entry'] or type(row['before']) is not int or not 0<=row['before']<=255:raise ValueError('lookup input ownership differs')
        result.append(dict(area='ds',offset=key[1],value=row['before']))
    if seen!={('ds',o) for o in range(0xD36,0xD56)}:raise ValueError('lookup32-byte census incomplete')
    return result

if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--import-from',type=Path);p.add_argument('--check',action='store_true');a=p.parse_args()
    if a.import_from:import_sources(a.import_from.resolve())
    if a.check:print('PASS pinned edited lookup witnesses',len(sources()))
