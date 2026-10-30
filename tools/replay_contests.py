#!/usr/bin/env python3
"""Check decoded original cases through Rust; no engine formula is implemented in Python.

    python3 tools/replay_contests.py --check
    python3 tools/replay_contests.py --benchmark --out records/T01
"""
import argparse
import hashlib
import json
import platform
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "engine/tests/fixtures"
HEX64 = re.compile(r"^[0-9a-f]{64}$")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def load_cases():
    raw = (FIXTURES / "index.json").read_bytes()
    index = json.loads(raw)
    provenance = json.loads((FIXTURES / "provenance.json").read_text())
    if (provenance["repository"] != "https://github.com/diegoami/onenil-archaeology"
            or not re.fullmatch(r"[0-9a-f]{40}", provenance["commit"])
            or provenance["index_sha256"] != sha(raw)):
        raise ValueError("fixture index provenance or digest differs")
    if (index["schema"] != "quick-contests/1" or index["weeks"] != 6
            or index["traces"] != 15 or len(index["records"]) != 6):
        raise ValueError("expected six distinct original weeks and fifteen T08 traces")
    files, cases, weeks = {}, [], set()
    for record in index["records"]:
        name = record["cases"]
        if Path(name).name != name or name in files:
            raise ValueError(f"invalid or duplicate case file {name}")
        if record["week"] in weeks:
            raise ValueError("duplicate original week")
        weeks.add(record["week"])
        data = (FIXTURES / name).read_bytes()
        if sha(data) != record["cases_sha256"]:
            raise ValueError(f"{name}: fixture digest differs")
        rows = [json.loads(line) for line in data.splitlines()]
        if len(rows) != record["calls"]:
            raise ValueError(f"{name}: call count differs")
        for i, case in enumerate(rows):
            source = case["source"]
            if (case["schema"] != "routine-case/1" or case["week"] != record["week"]
                    or case["call"] != i or source["trace"] != record["source_trace"]
                    or source["sha256"] != record["source_trace_sha256"]
                    or source["draw_end"] - source["draw_start"] != len(case["draws"])):
                raise ValueError(f"{name}: call {i}: provenance or span differs")
        files[name] = record
        cases.extend(rows)
    coverage = index["coverage"]
    if (len(cases) != 558 or index["calls"] != 558 or len(coverage) != 15
            or len({c["t04_record"] for c in coverage}) != 15
            or len({c["trace"] for c in coverage}) != 15
            or {c["cases"] for c in coverage} != set(files)):
        raise ValueError("routine/corpus coverage is incomplete")
    for entry in coverage:
        if entry["cases"] not in files or not HEX64.fullmatch(entry["trace_sha256"]):
            raise ValueError("invalid T08 coverage reference")
    return cases, index, provenance


def rust(case):
    import onenil_engine
    inputs = case["input"]
    return onenil_engine.quick_contests(inputs["k"], inputs["seed"], inputs["ratings"], inputs["goals"])


def check(cases):
    draws = 0
    for case in cases:
        got = json.loads(rust(case))
        label = f"{case['week']} call {case['call']}"
        for i, (actual, expected) in enumerate(zip(got["draws"], case["draws"])):
            if actual != expected:
                raise ValueError(f"{label}: draw {i}: expected {expected}, got {actual}")
        if len(got["draws"]) != len(case["draws"]):
            raise ValueError(f"{label}: draw count differs")
        if got["seed"] != case["output"]["seed"] or got["goals"] != case["output"]["goals"]:
            raise ValueError(f"{label}: final seed/goals differ")
        draws += len(got["draws"])
    print(f"Rust bridge replay: {len(cases)} original routine calls, {draws} draws, 15 T08 traces covered")
    return draws


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], text=True).strip()


def benchmark(cases, draws, provenance, repetitions, out):
    from onenil_engine import onenil_engine as backend
    from importlib.metadata import version
    if git("status", "--porcelain", "--", "engine", "tools/replay_contests.py", "pyproject.toml"):
        raise ValueError("commit engine, fixtures and replay tooling before measuring")
    started = datetime.now(timezone.utc).isoformat()
    begin, output_bytes = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for case in cases:
            output_bytes += len(rust(case))
    elapsed = time.perf_counter_ns() - begin
    out = ROOT / out
    out.mkdir(parents=True, exist_ok=True)
    n = 1 + max((int(p.stem.split("-r")[-1]) for p in out.glob("T01-r*.json")), default=0)
    path = out / f"T01-r{n:04d}.json"
    rec = {"schema": "routine-benchmark/1", "run_id": path.stem,
           "code_commit": git("rev-parse", "HEAD"), "started": started,
           "ended": datetime.now(timezone.utc).isoformat(), "platform": platform.platform(),
           "python": platform.python_version(),
           "build": {"crate_version": backend.__version__,
                     "debug_assertions": backend.__debug_assertions__,
                     "maturin": version("maturin"),
                     "extension_sha256": sha(Path(backend.__file__).read_bytes())},
           "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
           "cargo_lock_sha256": sha((ROOT / "engine/Cargo.lock").read_bytes()),
           "archaeology": provenance, "routine": "0AB9:D983", "distinct_cases": len(cases),
           "source_traces": 15, "repetitions": repetitions, "calls": len(cases) * repetitions,
           "draws": draws * repetitions, "elapsed_ns": elapsed, "serialized_output_bytes": output_bytes,
           "method": "Python-to-Rust calls including DS image setup, trace collection and JSON serialization; after an untimed replay check; excludes JSON decoding during timed loop",
           "limits": "Routine slice only; not match/week speed or parity; no comparison with original emulator speed"}
    with path.open("x") as f:
        f.write(json.dumps(rec, indent=2) + "\n")
    print(f"{path.relative_to(ROOT)}: {rec['calls']} routine calls in {elapsed / 1e9:.6f} s")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true")
    ap.add_argument("--benchmark", action="store_true")
    ap.add_argument("--repetitions", type=int, default=20)
    ap.add_argument("--out", default="records/T01")
    args = ap.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        ap.error("choose --check or --benchmark; repetitions must be positive")
    cases, _, provenance = load_cases()
    draws = check(cases)
    if args.benchmark:
        benchmark(cases, draws, provenance, args.repetitions, args.out)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, OSError, ImportError) as error:
        print(f"replay failed: {error}", file=sys.stderr)
        sys.exit(1)
