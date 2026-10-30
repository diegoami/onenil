#!/usr/bin/env python3
"""Replay complete original placement inputs through Rust; Python contains no engine formula.

    python3 tools/replay_placement.py --check
    python3 tools/replay_placement.py --benchmark --out records/T02
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
from importlib.metadata import version
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "engine/tests/fixtures/placement"
HEX64 = re.compile(r"[0-9a-f]{64}")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_file(name, expected):
    if not isinstance(name, str) or Path(name).name != name:
        raise ValueError("invalid fixture filename")
    raw = (FIXTURES / name).read_bytes()
    if not HEX64.fullmatch(expected) or sha(raw) != expected:
        raise ValueError(f"{name}: fixture digest differs")
    return [json.loads(line) for line in raw.splitlines()]


def load_cases():
    raw = (FIXTURES / "index.json").read_bytes()
    index = json.loads(raw)
    provenance = json.loads((FIXTURES / "provenance.json").read_text())
    if (provenance["repository"] != "https://github.com/diegoami/onenil-archaeology"
            or not re.fullmatch(r"[0-9a-f]{40}", provenance["commit"])
            or sha(raw) != provenance["index_sha256"]):
        raise ValueError("placement index provenance or digest differs")
    if (index["schema"] != "placement-corpus/1" or index["calls"] != 558
            or index["draws"] != 1804 or index["callbacks"] != 17
            or index["weeks"] != 6 or index["traces"] != 15):
        raise ValueError("expected 558 placements / 1804 draws / 17 callbacks / six weeks / 15 traces")
    cases, files, weeks = [], set(), set()
    records = index["records"]
    if len(records) != 6:
        raise ValueError("expected six canonical placement files")
    for record in records:
        name = record["cases"]
        if name in files or record["week"] in weeks:
            raise ValueError("duplicate placement file/week")
        rows = read_file(name, record["cases_sha256"])
        if len(rows) != record["calls"]:
            raise ValueError(f"{name}: case count differs")
        for step, case in enumerate(rows):
            source = case["source"]
            if (case["schema"] != "placement-case/1" or case["week"] != record["week"]
                    or case["step"] != step or source["t12_step"] != step
                    or not HEX64.fullmatch(source["t12_record_sha256"])
                    or not HEX64.fullmatch(source["t12_cases_sha256"])
                    or not HEX64.fullmatch(source["t08_sha256"])
                    or source["draw_end"] - source["draw_start"] != len(case["draws"])):
                raise ValueError(f"{name}: step {step}: case provenance/span differs")
        files.add(name)
        weeks.add(record["week"])
        cases.extend(rows)
    coverage = index["coverage"]
    if (len(cases) != 558 or len(coverage) != 15
            or len({entry["trace"] for entry in coverage}) != 15
            or len({entry["t04_record"] for entry in coverage}) != 15
            or {entry["cases"] for entry in coverage} != files):
        raise ValueError("incomplete source trace coverage")
    for entry in coverage:
        if not HEX64.fullmatch(entry["trace_sha256"]):
            raise ValueError("invalid source trace digest")
    if sum(len(case["draws"]) for case in cases) != 1804 or sum(len(case["callbacks"]) for case in cases) != 17:
        raise ValueError("placement draw/callback totals differ")
    helper = index["helper_checks"]
    checks = read_file(helper["file"], helper["sha256"])
    if len(checks) != 12276 or helper["cases"] != len(checks):
        raise ValueError("expected all 12276 original helper checks")
    for check in checks:
        if check["schema"] != "availability-case/1" or check["unavailable"] not in (0, 1):
            raise ValueError("invalid original helper observation")
    return cases, checks, index, provenance


def rust(case):
    import onenil_engine
    # Only recorded inputs cross this API. Expected decisions/draws are not supplied.
    return onenil_engine.goal_placement(json.dumps(case["input"], separators=(",", ":")))


def check(cases, helpers):
    import onenil_engine
    draws = callbacks = 0
    for case in cases:
        label = f"{case['week']} step {case['step']}"
        got = json.loads(rust(case))
        for i, (actual, expected) in enumerate(zip(got["draws"], case["draws"])):
            if actual != expected:
                raise ValueError(f"{label}: draw {i}: expected {expected}, got {actual}")
        if len(got["draws"]) != len(case["draws"]):
            raise ValueError(f"{label}: draw count differs")
        for field in ("callbacks", "minute_fills", "attempts", "mappings", "output"):
            if got[field] != case[field]:
                raise ValueError(f"{label}: {field} differs")
        draws += len(got["draws"])
        callbacks += len(got["callbacks"])
    for helper in helpers:
        p = helper["input"]
        got = onenil_engine.player_unavailable(p["id"], p["p15_sbyte8"], p["p15_byte11"])
        if got != helper["unavailable"]:
            raise ValueError(f"{helper['week']} step {helper['step']} scene {helper['scene_slot']}: helper differs")
    print(f"Rust placement replay: {len(cases)} cases / {draws} draws / {callbacks} goal requests / "
          f"{len(helpers)} helper checks / 15 T08 traces")
    return draws, callbacks


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], text=True).strip()


def benchmark(cases, draws, callbacks, provenance, repetitions, out):
    from onenil_engine import onenil_engine as backend
    if git("status", "--porcelain", "--", "engine", "tools/replay_placement.py", "pyproject.toml"):
        raise ValueError("commit engine, fixtures and replay tooling before measuring")
    started = datetime.now(timezone.utc).isoformat()
    # Input JSON preparation is excluded; bridge parsing, DS setup, native computation,
    # trace/effect collection and output serialization are included.
    inputs = [json.dumps(case["input"], separators=(",", ":")) for case in cases]
    begin, output_bytes = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for raw in inputs:
            output_bytes += len(backend.goal_placement(raw))
    elapsed = time.perf_counter_ns() - begin
    out = ROOT / out
    out.mkdir(parents=True, exist_ok=True)
    n = 1 + max((int(p.stem.split("-r")[-1]) for p in out.glob("T02-r*.json")), default=0)
    path = out / f"T02-r{n:04d}.json"
    record = {"schema": "routine-benchmark/1", "run_id": path.stem,
              "code_commit": git("rev-parse", "HEAD"), "started": started,
              "ended": datetime.now(timezone.utc).isoformat(), "platform": platform.platform(),
              "python": platform.python_version(),
              "build": {"crate_version": backend.__version__, "debug_assertions": backend.__debug_assertions__,
                        "maturin": version("maturin"), "extension_sha256": sha(Path(backend.__file__).read_bytes())},
              "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "cargo_lock_sha256": sha((ROOT / "engine/Cargo.lock").read_bytes()),
              "archaeology": provenance, "routine": "0AB9:DD7C..DF35 placement and 5083 RNG projection",
              "distinct_cases": len(cases), "source_traces": 15, "repetitions": repetitions,
              "calls": len(cases) * repetitions, "draws": draws * repetitions,
              "goal_effect_requests": callbacks * repetitions, "elapsed_ns": elapsed,
              "serialized_output_bytes": output_bytes,
              "method": "Pre-serialized input JSON; timed Python/Rust bridge parsing, DS-image setup, native placement/helpers/RNG, trace/effect collection and output serialization; after untimed full replay; output decoding excluded",
              "limits": "Placement projection only; remaining 5083 statistics/log/display effects external; not full DD02, match/week or original-emulator speed"}
    with path.open("x") as f:
        f.write(json.dumps(record, indent=2) + "\n")
    print(f"{path.relative_to(ROOT)}: {record['calls']} placements in {elapsed / 1e9:.6f} s")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--benchmark", action="store_true")
    parser.add_argument("--repetitions", type=int, default=20)
    parser.add_argument("--out", default="records/T02")
    args = parser.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        parser.error("choose --check or --benchmark; repetitions must be positive")
    cases, helpers, _, provenance = load_cases()
    draws, callbacks = check(cases, helpers)
    if args.benchmark:
        benchmark(cases, draws, callbacks, provenance, args.repetitions, args.out)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError) as error:
        print(f"placement replay failed: {error}", file=sys.stderr)
        sys.exit(1)
