#!/usr/bin/env python3
"""Fetch/check the independently licensed T51 replacement font into ignored cache."""
import argparse,importlib.util
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
p=argparse.ArgumentParser();p.add_argument('--path',type=Path,default=ROOT/'.cache/T51-font/PxPlus_IBM_VGA_8x16.ttf');args=p.parse_args()
spec=importlib.util.spec_from_file_location('study',ROOT/'docs/desktop-fidelity/mockups/build.py');module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
module.font_bytes(args.path,fetch=True)
print(args.path)
print('Independent VileR font: CC BY-SA4.0; attribution/license in docs/desktop-fidelity/mockups.')
