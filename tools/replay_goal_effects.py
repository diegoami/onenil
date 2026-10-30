#!/usr/bin/env python3
"""Replay original logical goal effects through Rust; no Python engine formulas.

    python3 tools/replay_goal_effects.py --check
    python3 tools/replay_goal_effects.py --benchmark --out records/T03
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
FIXTURES = ROOT / "engine/tests/fixtures/goal_effects"
HEX64 = re.compile(r"[0-9a-f]{64}")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def same(a, b):
    return json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)


def read_file(name, digest):
    if not isinstance(name, str) or Path(name).name != name:
        raise ValueError("invalid fixture filename")
    raw = (FIXTURES / name).read_bytes()
    if not HEX64.fullmatch(digest) or sha(raw) != digest:
        raise ValueError(f"{name}: fixture digest differs")
    return [json.loads(line) for line in raw.splitlines()]


def load_cases():
    raw = (FIXTURES / "index.json").read_bytes()
    index = json.loads(raw)
    pin = json.loads((FIXTURES / "provenance.json").read_text())
    if (pin["repository"] != "https://github.com/diegoami/onenil-archaeology"
            or not re.fullmatch(r"[0-9a-f]{40}", pin["commit"])
            or sha(raw) != pin["index_sha256"]):
        raise ValueError("goal-effect provenance or index digest differs")
    if (index["schema"] != "goal-effects-corpus/1"
            or (index["calls"], index["weeks"], index["traces"]) != (17, 6, 15)):
        raise ValueError("expected 17 effects / six weeks / 15 source traces")
    placement_raw = (ROOT / "engine/tests/fixtures/placement/index.json").read_bytes()
    if sha(placement_raw) != index["t13_index_sha256"]:
        raise ValueError("placement source index digest differs")
    placement_records = {r["week"]: r for r in json.loads(placement_raw)["records"]}
    cases, files, weeks = [], set(), set()
    for record in index["records"]:
        name = record["cases"]
        rows = read_file(name, record["cases_sha256"])
        if name in files or record["week"] in weeks or len(rows) != record["calls"]:
            raise ValueError("duplicate or incomplete goal-effect record")
        for case in rows:
            source = case["source"]
            placement = placement_records[case["week"]]
            if (case["schema"] != "goal-effects-case/1" or case["week"] != record["week"]
                    or source["t13_step"] != case["step"]
                    or Path(source["t13_cases"]).name != placement["cases"]
                    or source["t13_cases_sha256"] != placement["cases_sha256"]
                    or source["t13_record"] != placement["record"]
                    or source["t13_record_sha256"] != placement["record_sha256"]
                    or any(not HEX64.fullmatch(source[key]) for key in
                           ("t12_record_sha256", "t13_record_sha256", "t13_cases_sha256", "t08_sha256"))
                    or source["draw_end"] - source["draw_start"] != 1
                    or len(case["request"]["draws"]) != 1 or len(case["helper_calls"]) != 2):
                raise ValueError("goal-effect case provenance or boundary differs")
        files.add(name)
        weeks.add(record["week"])
        cases.extend(rows)
    coverage = index["coverage"]
    if (len(cases) != 17 or len(files) != 6 or len(coverage) != 15
            or len({c["trace"] for c in coverage}) != 15
            or len({c["t04_record"] for c in coverage}) != 15
            or {c["cases"] for c in coverage} != files):
        raise ValueError("incomplete goal-effect corpus coverage")
    if any(not HEX64.fullmatch(entry["trace_sha256"]) for entry in coverage):
        raise ValueError("invalid source trace digest")
    helper = index["helper_checks"]
    checks = read_file(helper["file"], helper["sha256"])
    if len(checks) != 34 or helper["cases"] != 34:
        raise ValueError("expected 34 original byte6 helpers")
    if any(h["schema"] != "byte6-case/1" for h in checks):
        raise ValueError("invalid byte6 helper schema")
    return cases, checks, index, pin


def rust(case):
    import onenil_engine
    # Only before-state and the already-consumed native request enter the API.
    return onenil_engine.apply_goal_effects(
        json.dumps(case["before"], separators=(",", ":")),
        json.dumps(case["request"], separators=(",", ":")))


def check(cases, helpers):
    import onenil_engine
    from replay_placement import load_cases as load_placements

    placements = {(c["week"], c["step"]): c for c in load_placements()[0]}
    for case in cases:
        got = json.loads(rust(case))
        label = f"{case['week']} step {case['step']}"
        if not same(got["state"], case["after"]):
            raise ValueError(f"{label}: logical state differs")
        if not same(got["commentary"], case["commentary"]):
            raise ValueError(f"{label}: numeric commentary differs")
        original = placements[(case["week"], case["step"])]
        composed = json.loads(onenil_engine.composed_goal_effects(
            json.dumps(original["contest"]["input"], separators=(",", ":")),
            json.dumps(original["input"], separators=(",", ":")),
            json.dumps(case["before"], separators=(",", ":"))))
        if (composed["seed_before"] != composed["seed_after"]
                or composed["seed_before"] != case["request"]["draws"][0]["seed_after"]
                or not same(composed["placement"]["callbacks"], [case["request"]])
                or not same(composed["placement"]["draws"], original["draws"])
                or not same(composed["placement"]["output"], original["output"])
                or not same(composed["effects"], got)):
            raise ValueError(f"{label}: composed handoff or post-coin seed differs")
    for helper in helpers:
        got = json.loads(onenil_engine.apply_byte6(
            json.dumps(helper["input"], separators=(",", ":"))))
        if not same(got, helper["output"]):
            raise ValueError(f"{helper['week']} step {helper['step']}: byte6 helper differs")
    # Exercise rejection across the serialized bridge as well as Rust unit tests.
    bad_request = dict(cases[0]["request"])
    bad_request["coin"] ^= 1
    try:
        onenil_engine.apply_goal_effects(json.dumps(cases[0]["before"]), json.dumps(bad_request))
    except ValueError:
        pass
    else:
        raise ValueError("Rust bridge accepted a coin inconsistent with the consumed draw")
    print(f"Rust goal effects: {len(cases)} callbacks / {len(helpers)} byte6 helpers / "
          "15 T08 traces; composed post-coin seed unchanged")


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], text=True).strip()


def benchmark(cases, pin, repetitions, out):
    from onenil_engine import onenil_engine as backend
    if git("status", "--porcelain", "--", "engine", "tools/replay_goal_effects.py", "pyproject.toml"):
        raise ValueError("commit goal-effect code and fixtures before timing")
    inputs = [(json.dumps(c["before"], separators=(",", ":")),
               json.dumps(c["request"], separators=(",", ":"))) for c in cases]
    started = datetime.now(timezone.utc).isoformat()
    begin, nbytes = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for state, request in inputs:
            nbytes += len(backend.apply_goal_effects(state, request))
    elapsed = time.perf_counter_ns() - begin
    out = ROOT / out
    out.mkdir(parents=True, exist_ok=True)
    n = 1 + max((int(p.stem.split("-r")[-1]) for p in out.glob("T03-r*.json")), default=0)
    path = out / f"T03-r{n:04d}.json"
    record = {"schema": "routine-benchmark/1", "run_id": path.stem,
              "code_commit": git("rev-parse", "HEAD"), "started": started,
              "ended": datetime.now(timezone.utc).isoformat(), "platform": platform.platform(),
              "python": platform.python_version(),
              "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "cargo_lock_sha256": sha((ROOT / "engine/Cargo.lock").read_bytes()),
              "build": {"crate_version": backend.__version__,
                        "debug_assertions": backend.__debug_assertions__,
                        "maturin": version("maturin"),
                        "extension_sha256": sha(Path(backend.__file__).read_bytes())},
              "archaeology": pin, "routine": "5083 logical goal effects with already-consumed coin",
              "distinct_cases": len(cases), "source_traces": 15, "repetitions": repetitions,
              "calls": len(cases) * repetitions, "rng_draws": 0, "elapsed_ns": elapsed,
              "serialized_output_bytes": nbytes,
              "method": "Pre-serialized state/request JSON; timed bridge parsing, memory-image setup, native logical updates, projection and output serialization; input preparation and output decoding excluded; after untimed full replay",
              "limits": "Logical goal effects only; no formatted text/pixel/timer/full-DD02/match/week parity or emulator speed comparison"}
    with path.open("x") as output:
        output.write(json.dumps(record, indent=2) + "\n")
    print(f"{path.relative_to(ROOT)}: {record['calls']} effects in {elapsed / 1e9:.6f} s")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--benchmark", action="store_true")
    parser.add_argument("--repetitions", type=int, default=100)
    parser.add_argument("--out", default="records/T03")
    args = parser.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        parser.error("choose --check or --benchmark; repetitions must be positive")
    cases, helpers, _, pin = load_cases()
    check(cases, helpers)
    if args.benchmark:
        benchmark(cases, pin, args.repetitions, args.out)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError) as error:
        print(f"goal-effect replay failed: {error}", file=sys.stderr)
        sys.exit(1)
