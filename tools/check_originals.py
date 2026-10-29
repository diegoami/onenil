#!/usr/bin/env python3
"""[designed: owner, 2026-10-29] No byte of the original game is tracked: refuse any tracked file
with one of its file types (CLAUDE.md, docs/artifacts.md). Exit 0 when clean, 1 otherwise.

    python3 tools/check_originals.py
"""
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
# One-Nil 95/96's files: executables, data, palettes, fonts, PIFs, batch files and its saves.
FORBIDDEN = {".exe", ".dat", ".pif", ".bat", ".pal", ".tfn", ".1-0", ".sav", ".com", ".ovl"}


def main() -> int:
    files = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout.split(b"\0")
    bad = [f.decode() for f in files if f and Path(f.decode()).suffix.lower() in FORBIDDEN]
    for name in bad:
        print(f"original-game file type tracked: {name}")
    print("original-file check: " + ("FAIL" if bad else "ok"))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
