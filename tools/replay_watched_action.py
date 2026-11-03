#!/usr/bin/env python3
"""[designed] Original passing/contact roots and native replay;no Python engine model."""
import argparse
from pathlib import Path
import watched_oracle

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--capture',action='store_true',required=True);p.add_argument('--research-root',type=Path,required=True);a=p.parse_args()
    watched_oracle.capture(a.research_root)
if __name__=='__main__':main()
