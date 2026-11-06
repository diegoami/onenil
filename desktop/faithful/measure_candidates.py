#!/usr/bin/env python3
"""[designed] Scoped warm debug-build/dependency and idle Linux-process measurements."""
import argparse,hashlib,json,os,platform,subprocess,tempfile,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--font',type=Path,required=True);p.add_argument('--record',type=Path,required=True);args=p.parse_args();assert not args.record.exists()
producer=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
assert not subprocess.check_output(['git','status','--porcelain','--','desktop/faithful'],cwd=ROOT,text=True)
target=Path(os.environ.get('CARGO_TARGET_DIR',ROOT/'desktop/faithful/target'));out=[]
for feature in ['software','egui']:
 command=['cargo','build','--manifest-path','desktop/faithful/Cargo.toml','--locked','--features',feature]
 started=time.monotonic();r=subprocess.run(command,cwd=ROOT,capture_output=True,text=True);assert r.returncode==0,r.stderr
 elapsed=time.monotonic()-started
 tree=subprocess.check_output(['cargo','tree','--manifest-path','desktop/faithful/Cargo.toml','--features',feature,'--edges','normal','--prefix','none','--format','{p}'],cwd=ROOT,text=True)
 packages=sorted(set(line.removesuffix(' (*)') for line in tree.splitlines()))
 binary=target/'debug/onenil-desktop';raw=binary.read_bytes();binary_sha=hashlib.sha256(raw).hexdigest()
 with tempfile.TemporaryDirectory(prefix='onenil-measure-') as d:
  d=Path(d);local=d/'onenil-desktop';local.write_bytes(raw);local.chmod(0o700)
  state=d/'state.json';err=d/'stderr.txt'
  with err.open('w') as f:process=subprocess.Popen([str(local),'--root',str(ROOT),'--font',str(args.font.resolve()),'--save',str(d/'save.json'),'--observe',str(state)],stdout=subprocess.DEVNULL,stderr=f)
  try:
   end=time.monotonic()+15
   while not state.exists():
    assert process.poll() is None,err.read_text();assert time.monotonic()<end,'startup timeout';time.sleep(.02)
   assert json.loads(state.read_text())['screen']=='Main';time.sleep(.5)
   def sample():
    text=Path(f'/proc/{process.pid}/stat').read_text();fields=text[text.rindex(')')+2:].split()
    cpu=int(fields[11])+int(fields[12]);status=Path(f'/proc/{process.pid}/status').read_text();rss=int(next(l.split()[1] for l in status.splitlines() if l.startswith('VmRSS:')));return cpu,rss
   before,rss=sample();started=time.monotonic();time.sleep(2);after,rss_after=sample();sample_seconds=time.monotonic()-started
   cpu_seconds=(after-before)/os.sysconf('SC_CLK_TCK')
  finally:
   process.terminate();process.wait(timeout=3)
  result=dict(candidate=feature,warm_debug_build_seconds=elapsed,build_command=command,build_stderr_tail=r.stderr[-700:],normal_dependency_packages=len(packages),packages=packages,debug_binary_bytes=len(raw),binary_sha256=binary_sha,idle_main_rss_kib=rss_after,idle_sample_seconds=sample_seconds,idle_cpu_seconds=cpu_seconds,idle_percent_one_core=100*cpu_seconds/sample_seconds,stderr=err.read_text())
  out.append(result)
record=dict(schema='desktop-candidate-cost/1',producer=producer,platform=platform.platform(),rustc=subprocess.check_output(['rustc','--version'],text=True).strip(),display=os.environ.get('DISPLAY'),scope='Warm debug builds with shared already-compiled engine/cache, 1440x900 Xvfb Linux main-menu idle for2seconds. Not cold/release/platform/distribution/frame-latency or match-engine performance.',candidates=out)
with args.record.open('x') as f:json.dump(record,f,indent=2);f.write('\n')
print([(c['candidate'],c['normal_dependency_packages'],c['debug_binary_bytes'],c['idle_main_rss_kib']) for c in out])
