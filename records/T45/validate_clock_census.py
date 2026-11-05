from pathlib import Path
import json,hashlib,tempfile,sys
from unittest.mock import patch
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools'))
import replay_prepared_match as replay
# Synthetic validator-unit fixture only;never an original capture or accepted proof.
with tempfile.TemporaryDirectory(prefix='onenil-clock-census-unit-')as temporary:
 directory=Path(temporary);start=json.loads((replay.ROOT/'records/T45/original-r0003/start.json').read_text())
 empty=hashlib.sha256(b'').hexdigest();array=hashlib.sha256(replay.canonical([])).hexdigest()
 result=dict(status='completed',ticks=4199,draws=182811,actions=831,boundary='0AB9:EBE2',projection=dict(site='0AB9:E303',field='background gameplay rectangle',cells=1275,encoding='space/attribute7;centre marker9',outside_background_unchanged=True),ticks_sha256=empty,actions_sha256=empty,draws_sha256=array,final={},caller_clocks=[])
 for name,value in [('start.json',start),('result.json',result)]: (directory/name).write_text(json.dumps(value)+'\n')
 manifest={'files':{name:replay.sha(directory/name)for name in ('start.json','result.json')}};(directory/'manifest.json').write_text(json.dumps(manifest)+'\n')
 with patch.object(replay,'FIELD_DIRECTORY',directory),patch.object(replay,'FIELD_MANIFEST_SHA',replay.sha(directory/'manifest.json')),patch.object(replay.reference,'sources',return_value=([],[],[],{'final':{}})):
  try:replay.field_preservation()
  except ValueError as error:
   assert 'clock'in str(error),error
   print('Missing clock stream rejected by standalone proof checker:',error)
  else:raise AssertionError('Standalone proof checker accepted a synthetic complete metadata record with zero actual clock rows')
