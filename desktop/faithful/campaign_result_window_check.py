#!/usr/bin/env python3
"""[designed] Actual computed result, explicit save and fresh-process X11 recovery."""
import argparse,ctypes as C,hashlib,json,os,subprocess,tempfile,time,traceback,shutil,sys
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--binary',type=Path,required=True);p.add_argument('--font',type=Path,required=True);p.add_argument('--record',type=Path,required=True);p.add_argument('--focus-only',action='store_true');args=p.parse_args();assert not args.record.exists()
x=C.CDLL('libX11.so.6');P=C.c_void_p;U=C.c_ulong;I=C.c_int
for name,restype,argtypes in [('XOpenDisplay',P,[C.c_char_p]),('XDefaultRootWindow',U,[P]),('XQueryTree',I,[P,U,C.POINTER(U),C.POINTER(U),C.POINTER(C.POINTER(U)),C.POINTER(C.c_uint)]),('XFetchName',I,[P,U,C.POINTER(C.c_char_p)]),('XFree',I,[P]),('XStringToKeysym',U,[C.c_char_p]),('XKeysymToKeycode',C.c_ubyte,[P,U]),('XSendEvent',I,[P,U,I,C.c_long,P]),('XFlush',I,[P]),('XResizeWindow',I,[P,U,C.c_uint,C.c_uint]),('XSetInputFocus',I,[P,U,I,U]),('XGetGeometry',I,[P,U,C.POINTER(U),C.POINTER(I),C.POINTER(I),C.POINTER(C.c_uint),C.POINTER(C.c_uint),C.POINTER(C.c_uint),C.POINTER(C.c_uint)]),('XGetImage',P,[P,U,I,I,C.c_uint,C.c_uint,U,I]),('XDestroyImage',I,[P]),('XInternAtom',U,[P,C.c_char_p,I])]:
 f=getattr(x,name);f.restype=restype;f.argtypes=argtypes
class Attributes(C.Structure):_fields_=[('x',I),('y',I),('width',I),('height',I),('border',I),('depth',I),('visual',P),('root',U),('klass',I),('bit_gravity',I),('win_gravity',I),('backing_store',I),('backing_planes',U),('backing_pixel',U),('save_under',I),('colormap',U),('map_installed',I),('map_state',I),('all_events',C.c_long),('your_events',C.c_long),('dont_propagate',C.c_long),('override_redirect',I),('screen',P)]
x.XGetWindowAttributes.argtypes=[P,U,C.POINTER(Attributes)];x.XGetWindowAttributes.restype=I
xerrors=[]
ERROR_HANDLER=C.CFUNCTYPE(I,P,P)
@ERROR_HANDLER
def xerror(display,event):
 xerrors.append('X11 request error');return 0
x.XSetErrorHandler.argtypes=[ERROR_HANDLER];x.XSetErrorHandler(xerror)
class Key(C.Structure):_fields_=[('type',I),('serial',U),('send',I),('display',P),('window',U),('root',U),('subwindow',U),('time',U),('x',I),('y',I),('x_root',I),('y_root',I),('state',C.c_uint),('keycode',C.c_uint),('same_screen',I)]
class Client(C.Structure):_fields_=[('type',I),('serial',U),('send',I),('display',P),('window',U),('message_type',U),('format',I),('data',C.c_long*5)]
class Event(C.Union):_fields_=[('key',Key),('client',Client),('pad',C.c_long*24)]
class XImage(C.Structure):_fields_=[('width',I),('height',I),('xoffset',I),('format',I),('data',P),('byte_order',I),('bitmap_unit',I),('bitmap_bit_order',I),('bitmap_pad',I),('depth',I),('bytes_per_line',I),('bits_per_pixel',I)]
display=x.XOpenDisplay(None);assert display,'X11 display unavailable';root=x.XDefaultRootWindow(display)
def windows():
 r=U();parent=U();children=C.POINTER(U)();count=C.c_uint();x.XQueryTree(display,root,C.byref(r),C.byref(parent),C.byref(children),C.byref(count));ids=[children[i] for i in range(count.value)];x.XFree(children);return ids

def title(w):
 name=C.c_char_p();x.XFetchName(display,w,C.byref(name));s=name.value or b''
 if name:x.XFree(name)
 return s
class FocusComplete(Exception):pass
window=None;steps=[];shots=[];save_proofs=[];worker_pids=[];process_ids=[];native_edits=[];process=None;error=None;start=time.monotonic()
producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert not subprocess.check_output(['git','status','--porcelain','--','desktop/faithful'],cwd=ROOT,text=True),'commit producer first'
with tempfile.TemporaryDirectory(prefix='T65-window-') as directory:
 directory=Path(directory);binary=directory/'onenil-desktop';shutil.copy2(args.binary,binary);binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest();statepath=directory/'state.json';save=directory/'games'/'squad.json';stderr=directory/'stderr.txt'
 def state():
  if statepath.exists():return json.loads(statepath.read_text())
  return {}
 def wait(check,timeout=10):
  end=time.monotonic()+timeout
  while time.monotonic()<end:
   s=state()
   if check(s):return s
   if process.poll() is not None:raise AssertionError('app exited: '+stderr.read_text())
   time.sleep(.02)
  raise AssertionError('condition timeout: '+str(state()))
 def key(name):
  code=x.XKeysymToKeycode(display,x.XStringToKeysym(name.encode()));assert code
  for typ,mask in [(2,1),(3,2)]:
   event=Event();event.key=Key(typ,0,1,display,window,root,0,0,1,1,1,1,0,code,1);assert x.XSendEvent(display,window,0,mask,C.byref(event));x.XFlush(display);time.sleep(.06)
 def step(name,check,timeout=10):
  s=wait(check,timeout)
  s['worker_absent']=not s.get('worker_pid')
  if name=='edited-admission-refuses':s['refused_worker_absent']=not s.get('worker_pid')
  steps.append(dict(name=name,state={k:v for k,v in s.items() if k not in ('worker_pid','save_path')}));return s
 def shot(name):
  time.sleep(.12);r=U();xx=I();yy=I();w=C.c_uint();h=C.c_uint();border=C.c_uint();depth=C.c_uint()
  assert x.XGetGeometry(display,window,C.byref(r),C.byref(xx),C.byref(yy),C.byref(w),C.byref(h),C.byref(border),C.byref(depth))
  image=x.XGetImage(display,window,0,0,w,h,U(-1),2);assert image;meta=C.cast(image,C.POINTER(XImage)).contents;assert meta.bits_per_pixel==32
  data=C.string_at(meta.data,meta.bytes_per_line*h.value);out=args.record.with_name(args.record.stem+'-'+name+'.png');assert not out.exists()
  Image.frombytes('RGB',(w.value,h.value),data,'raw','BGRX',meta.bytes_per_line,1).save(out);x.XDestroyImage(image)
  shots.append(dict(name=name,path=out.name,size=[w.value,h.value],sha256=hashlib.sha256(out.read_bytes()).hexdigest()))
 def keep_save(name):
  out=args.record.with_name(args.record.stem+'-'+name+'.json');data=save.read_bytes()
  with out.open('xb') as f:f.write(data)
  save_proofs.append(dict(name=name,path=out.name,sha256=hashlib.sha256(data).hexdigest()))
 def close_window():
  event=Event();event.client=Client(33,0,1,display,window,x.XInternAtom(display,b'WM_PROTOCOLS',0),32,(C.c_long*5)(x.XInternAtom(display,b'WM_DELETE_WINDOW',0),0,0,0,0));x.XSendEvent(display,window,0,0,C.byref(event));x.XFlush(display)
 def launch(load=False):
  global process,window
  window=None
  with stderr.open('a') as err:process=subprocess.Popen([str(binary),'--root',str(ROOT),'--font',str(args.font.resolve()),'--save',str(save),'--observe',str(statepath)]+(['--load'] if load else []),stderr=err,stdout=subprocess.DEVNULL)
  end=time.monotonic()+15
  while window is None and time.monotonic()<end:
   window=next((w for w in windows() if b'One-Nil | saved club prototype' in title(w)),None);time.sleep(.05)
  assert window,'native window missing'
  end=time.monotonic()+15
  while time.monotonic()<end:
   attributes=Attributes()
   if x.XGetWindowAttributes(display,window,C.byref(attributes)) and attributes.map_state==2:break
   if process.poll() is not None:raise AssertionError('app exited before visible: '+stderr.read_text())
   time.sleep(.05)
  else:raise AssertionError('native window did not become viewable')
  x.XSetInputFocus(display,window,1,0);x.XFlush(display)
  process_ids.append(process.pid)

 try:
  launch()
  initial=step('initial-campaign',lambda s:s.get('screen')=='Main')['campaign'];shot('main')
  key('F7');step('initial-save',lambda s:'saved' in s.get('message',''));saved=save.read_bytes();keep_save('initial-save')
  key('Return');step('campaign-preparation',lambda s:s.get('screen')=='Preparation' and not s.get('exhibition'));shot('preparation')
  key('Return');s=step('campaign-worker',lambda s:s.get('screen')=='Preparing' and s.get('worker_pid'));pid=s['worker_pid'];worker_pids.append(pid)
  key('Escape');step('cancel-preserves-campaign',lambda s:s.get('screen')=='Preparation' and not s.get('worker_pid') and s.get('campaign')==initial)
  assert not Path(f'/proc/{pid}').exists() and save.read_bytes()==saved
  key('Return');step('fresh-match-pitch',lambda s:s.get('screen')=='Pitch' and s.get('players')==22 and s.get('match_clubs')==['Blackburn R','P St Germain'] and not s.get('exhibition'),180)
  key('Right');step('native-player-actions',lambda s:s.get('screen')=='Pitch' and s.get('frames',0)>=500 and s.get('events',0)>20,30);shot('pitch-actions')
  key('space');s=step('detailed-commentary',lambda s:s.get('screen')=='History' and s.get('recent_actions'));shot('commentary')
  assert all('Play continues' not in item['text'] for item in s['recent_actions'])
  key('Escape');key('Escape');step('bench',lambda s:s.get('screen')=='Bench');shot('bench');key('Return')
  step('half-time',lambda s:s.get('screen')=='HalfTime' and s.get('campaign')==initial,90);shot('half-time')
  key('Return');step('half-time-bench',lambda s:s.get('screen')=='Bench');key('Return')
  step('full-time',lambda s:s.get('screen')=='FullTime' and s.get('score')==[1,4] and s.get('frames')==3825 and s.get('events')==749 and s.get('campaign')==initial,90);shot('full-time')
  key('Return');recorded=step('result-recorded',lambda s:s.get('screen')=='Main' and s.get('campaign',{}).get('played_fixture',{}).get('score')==[1,4] and s['campaign']['revision']==1)['campaign'];shot('recorded-result')
  assert recorded['played_fixture']['request_sha256']==hashlib.sha256(saved).hexdigest()
  key('F7');step('completed-save',lambda s:'saved' in s.get('message','') and s.get('campaign')==recorded);keep_save('completed-save');completed=save.read_bytes()
  key('Return');key('Return');step('completed-fixture-refuses',lambda s:s.get('screen')=='Preparation' and 'already has a result' in s.get('message','') and not s.get('worker_pid') and s.get('campaign')==recorded);shot('completed-refusal')
  assert save.read_bytes()==completed
  close_window();assert process.wait(timeout=3)==0
  statepath.unlink()
  launch(load=True)
  step('fresh-process-reload',lambda s:s.get('screen')=='Main' and s.get('campaign')==recorded and not s.get('worker_pid') and s.get('frames') is None);shot('reloaded-result')
  key('Return');key('Return');step('restored-fixture-refuses',lambda s:s.get('screen')=='Preparation' and 'already has a result' in s.get('message','') and not s.get('worker_pid') and s.get('campaign')==recorded)
  assert save.read_bytes()==completed
  close_window();assert process.wait(timeout=3)==0
  assert all(not Path(f'/proc/{pid}').exists() for pid in worker_pids)
 except BaseException as exc:
  error=f'{type(exc).__name__}: {exc}'
 finally:
  if process is not None and process.poll() is None:
   if window:close_window()
   try:process.wait(timeout=3)
   except subprocess.TimeoutExpired:process.kill();process.wait()
  record=dict(schema='native-campaign-result-window/1',producer=producer,runtime=dict(window_backend='software/minifb',python=sys.version.split()[0],cargo_lock_sha256=hashlib.sha256((ROOT/'desktop/faithful/Cargo.lock').read_bytes()).hexdigest()),binary_sha256=binary_sha256,font_sha256=hashlib.sha256(args.font.read_bytes()).hexdigest(),names_sha256=hashlib.sha256((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes()).hexdigest(),elapsed_seconds=time.monotonic()-start,steps=steps,screenshots=shots,save_proofs=save_proofs,fresh_process_count=len(process_ids),cancelled_and_closed_workers_reaped=all(not Path(f'/proc/{pid}').exists() for pid in worker_pids),status='failed' if error or xerrors else 'passed',x11_errors=xerrors,error=error)
  with args.record.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
  print(record['status'],len(steps),'native keyboard steps',len(shots),'screenshots',flush=True)
 if error or xerrors:raise AssertionError(error or xerrors)
