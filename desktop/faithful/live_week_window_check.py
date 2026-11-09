#!/usr/bin/env python3
"""[designed:T80] Isolated actual-window proof: the W030 week route after the
exactly-once result - the cash flow and transfer news screens, the advanced
management menu (funds and the following fixture), the explicit save and the
fresh-process reload - with replay refused (archaeology T81/T82 evidence)."""
import argparse,ctypes as C,hashlib,json,os,subprocess,tempfile,time,sys,shutil
from pathlib import Path
from PIL import Image
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--binary',type=Path,required=True);p.add_argument('--font',type=Path,required=True);p.add_argument('--record',type=Path,required=True);args=p.parse_args();assert not args.record.exists()
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
window=None;steps=[];shots=[];save_proofs=[];worker_pids=[];process_ids=[];keyboard=[];pending_keys=[];process=None;error=None;held_worker=None;start=time.monotonic()
producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
pass  # debug
with tempfile.TemporaryDirectory(prefix='T78-window-') as directory:
 directory=Path(directory);binary=directory/'onenil-desktop';shutil.copy2(args.binary,binary);binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest();statepath=directory/'state.json';save=directory/'games'/'week.json';stderr=directory/'stderr.txt'
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
  raise AssertionError('condition timeout: '+str({k:s.get(k) for k in ('screen','message','worker_pid','phase','match_stage')}))
 def key(name,delay=.06):
  keyboard.append(name);pending_keys.append(name)
  code=x.XKeysymToKeycode(display,x.XStringToKeysym(name.encode()));assert code
  for typ,mask in [(2,1),(3,2)]:
   event=Event();event.key=Key(typ,0,1,display,window,root,0,0,1,1,1,1,0,code,1);assert x.XSendEvent(display,window,0,mask,C.byref(event));x.XFlush(display);time.sleep(delay)
 def step(name,check,timeout=10):
  s=wait(check,timeout)
  s['worker_present']=bool(s.get('worker_pid'));s['worker_same']=bool(held_worker and s.get('worker_pid')==held_worker)
  steps.append(dict(name=name,keys=list(pending_keys),state={k:v for k,v in s.items() if k not in ('worker_pid','save_path')}));pending_keys.clear();return s
 def shot(name):
  from check_numbered_pitch_records import native_rgb
  import io
  end=time.monotonic()+15
  while time.monotonic()<end:
   before=state();r=U();xx=I();yy=I();w=C.c_uint();h=C.c_uint();border=C.c_uint();depth=C.c_uint()
   assert x.XGetGeometry(display,window,C.byref(r),C.byref(xx),C.byref(yy),C.byref(w),C.byref(h),C.byref(border),C.byref(depth))
   ptr=x.XGetImage(display,window,0,0,w,h,U(-1),2);assert ptr
   meta=C.cast(ptr,C.POINTER(XImage)).contents;assert meta.bits_per_pixel==32
   data=C.string_at(meta.data,meta.bytes_per_line*h.value)
   picture=Image.frombytes('RGB',(w.value,h.value),data,'raw','BGRX',meta.bytes_per_line,1);x.XDestroyImage(ptr)
   after=state()
   if before.get('raster_sha256')!=after.get('raster_sha256'):continue
   buffer=io.BytesIO();picture.save(buffer,format='PNG');data=buffer.getvalue()
   if hashlib.sha256(native_rgb(data)).hexdigest()!=after.get('raster_sha256'):continue
   out=args.record.with_name(args.record.stem+'-'+name+'.png')
   with out.open('xb') as f:f.write(data)
   after['worker_present']=bool(after.get('worker_pid'));after['worker_same']=bool(held_worker and after.get('worker_pid')==held_worker)
   shots.append(dict(name=name,path=out.name,size=[w.value,h.value],sha256=hashlib.sha256(data).hexdigest(),state={k:v for k,v in after.items() if k not in ('worker_pid','save_path')}))
   return
  raise AssertionError('actual window/state raster binding timeout: '+name)
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
 def shot_resize(name):
  shot(name)
  x.XResizeWindow(display,window,1280,800);x.XFlush(display);time.sleep(.3);shot(name+'-resized')
  x.XResizeWindow(display,window,1440,900);x.XFlush(display);time.sleep(.3)
 try:
  launch()
  step('main',lambda s:s.get('screen')=='Main')
  key('F7');step('initial-save',lambda s:'saved' in s.get('message',''));keep_save('initial');saved=save.read_bytes()
  key('Return');step('preparation',lambda s:s.get('screen')=='Preparation')
  key('Return');wait(lambda s:s.get('screen')=='Preparing' and s.get('worker_pid'),30)
  step('preparing-worker',lambda s:s.get('screen')=='Preparing' and s.get('live_worker'))
  key('Return');wait(lambda s:s.get('screen')=='Pitch' and s.get('live_worker') and s.get('match_stage')=='incomplete',180)
  held_worker=state()['worker_pid'];worker_pids.append(held_worker)
  step('half-time',lambda s:s.get('screen')=='HalfTime' and s.get('worker_pid')==held_worker,180)
  # The halftime keypress can land late under load; re-send while the screen
  # still shows the interval, with a bounded total wait.
  bench_end=time.monotonic()+30
  while time.monotonic()<bench_end and state().get('screen')=='HalfTime':
      key('Return');time.sleep(.5)
  step('interval-bench',lambda s:s.get('screen')=='Bench' and s.get('worker_same'))
  resume_end=time.monotonic()+12
  focus=U();rev=I()
  while time.monotonic()<resume_end and state().get('screen')=='Bench':
      x.XGetInputFocus(display,C.byref(focus),C.byref(rev))
      print('DEBUG bench loop: focus',focus.value,'window',window,'msg',repr(state().get('message','')),flush=True)
      key('Return');time.sleep(.5)
  resuming=step('resuming',lambda s:s.get('screen')=='Preparing' and s.get('resuming') and s.get('worker_same'))
  step('second-half',lambda s:s.get('screen')=='Pitch' and s.get('phase')=='SecondHalf' and s.get('match_stage')=='complete' and not s.get('worker_pid'),180)
  key('space');step('second-half-history',lambda s:s.get('screen')=='History');shot_resize('second-half')
  key('Escape');final=step('full-time',lambda s:s.get('screen')=='FullTime',90)
  shot('full-time')
  assert final.get('campaign',{}).get('cash')==452000 and final.get('campaign',{}).get('week_index')==0
  key('Return');recorded=step('recorded',lambda s:s.get('screen')=='Main' and s.get('campaign',{}).get('played_fixture'))['campaign']
  assert recorded['revision']==final['campaign']['revision']+1
  assert recorded['played_fixture']['request_sha256']==hashlib.sha256(saved).hexdigest()
  # [designed:T80] The W030 route: the authenticated cash flow and transfer news,
  # then the management menu with the advanced week.
  key('space');cash=step('cash-flow',lambda s:s.get('screen')=='CashFlow' and s.get('campaign',{}).get('cash')==452000)
  shot_resize('cash-flow')
  key('space');news=step('transfer-news',lambda s:s.get('screen')=='TransferNews')
  shot_resize('transfer-news')
  key('space');advanced=step('menu-advanced',lambda s:s.get('screen')=='Main' and s.get('campaign',{}).get('week_index')==1 and s.get('campaign',{}).get('cash')==421491)
  assert advanced['campaign']['next_fixture']['opponent_name']=='Sevilla' and advanced['campaign']['next_fixture']['opponent']==176
  assert advanced['campaign']['revision']==recorded['revision']+1
  shot_resize('menu-advanced')
  key('F7');step('saved-week',lambda s:'saved' in s.get('message',''));keep_save('completed');completed=save.read_bytes()
  close_window();assert process.wait(timeout=3)==0;statepath.unlink();launch(load=True)
  step('fresh-reload',lambda s:s.get('screen')=='Main' and s.get('campaign')==advanced and not s.get('worker_pid') and s.get('frames') is None)
  key('Return');key('Return');step('duplicate-refused',lambda s:s.get('screen')=='Preparation' and 'already' in s.get('message',''))
  assert save.read_bytes()==completed
  close_window();assert process.wait(timeout=3)==0;statepath.unlink();launch(load=True)
  step('fresh-reload',lambda s:s.get('screen')=='Main' and s.get('campaign')==recorded and not s.get('worker_pid') and s.get('frames') is None)
  key('Return');key('Return');step('duplicate-refused',lambda s:s.get('screen')=='Preparation' and 'already' in s.get('message','') and not s.get('worker_pid'))
  assert save.read_bytes()==completed
  close_window();assert process.wait(timeout=3)==0
 except BaseException as exc:
  error=f'{type(exc).__name__}: {exc}'
  import glob
  keep=Path('/tmp/t80-forensics');keep.mkdir(exist_ok=True)
  for f in glob.glob(str(directory/'**'),recursive=True):
   src=Path(f)
   if src.is_file():
    try:(keep/src.relative_to(directory)).parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,keep/src.relative_to(directory))
    except Exception:pass
  print('FORENSICS kept at',keep,flush=True)
 finally:
  if process is not None and process.poll() is None:
   if window:close_window()
   try:process.wait(timeout=3)
   except subprocess.TimeoutExpired:process.kill();process.wait()
  record=dict(producer_files={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in ['desktop/faithful/live_week_window_check.py','desktop/faithful/src/application.rs','desktop/faithful/src/controller.rs','desktop/faithful/src/timeline.rs','desktop/faithful/src/render.rs','desktop/faithful/src/live_worker.rs','campaign/src/result.rs','engine/src/watched_compiled/prepared.rs']},source_research='a7733617248233ecf50d3cba4aa3c55673933073',schema='native-week-route-window/1',keyboard=keyboard,owned_windows_closed=all(not Path(f'/proc/{pid}').exists() for pid in process_ids),case='week',producer=producer,runtime=dict(window_backend='software/minifb',python=sys.version.split()[0],cargo_lock_sha256=hashlib.sha256((ROOT/'desktop/faithful/Cargo.lock').read_bytes()).hexdigest()),binary_sha256=binary_sha256,font_sha256=hashlib.sha256(args.font.read_bytes()).hexdigest(),names_sha256=hashlib.sha256((ROOT/'desktop/faithful/assets/campaign-match-names.json').read_bytes()).hexdigest(),elapsed_seconds=time.monotonic()-start,steps=steps,screenshots=shots,save_proofs=save_proofs,fresh_process_count=len(process_ids),cancelled_and_closed_workers_reaped=all(not Path(f'/proc/{pid}').exists() for pid in worker_pids),status='failed' if error or xerrors else 'passed',x11_errors=xerrors,error=error)
  with args.record.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
  print(record['status'],len(steps),'keyboard steps',len(shots),'screenshots',flush=True)
 if error or xerrors:raise AssertionError(error or xerrors)
