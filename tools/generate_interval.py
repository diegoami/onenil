#!/usr/bin/env python3
"""[derived:T74] Compile original retained 7DD3/E008 continuation roots."""
import argparse
import subprocess
import sys
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
def main():
 p=argparse.ArgumentParser();p.add_argument('--research-root',type=Path,required=True);p.add_argument('--check',action='store_true');a=p.parse_args()
 base=ROOT/'engine/src/watched_compiled'
 command=[sys.executable,str(ROOT/'tools/compile_watched.py'),'--research-root',str(a.research_root),'--out',str(base/'interval_graph.rs'),'--root','0AB9:7E4A','--root','0AB9:E064','--source-addresses']
 for name in ['action_graph','match_graph']:
  command+=['--reuse-existing',str(base/(name+'.rs'))]
 if a.check:command+=['--check']
 subprocess.run(command,check=True)
if __name__=='__main__':main()
