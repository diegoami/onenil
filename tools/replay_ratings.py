#!/usr/bin/env python3
"""Replay original raw-input ratings and computed-rating D983 handoffs through Rust.

    python3 tools/replay_ratings.py --check
    python3 tools/replay_ratings.py --benchmark --out records/T04

No rating or predicate formula is implemented in Python. Expected outputs are
assertions after native calls, never arguments that steer native calculations.
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
FIXTURES = ROOT / "engine/tests/fixtures/ratings"
HEX64 = re.compile(r"[0-9a-f]{64}")


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def same(left, right):
    return json.dumps(left, sort_keys=True) == json.dumps(right, sort_keys=True)


def encoded(value):
    return json.dumps(value, separators=(",", ":"))


def rows(name, digest):
    if not isinstance(name, str) or Path(name).name != name:
        raise ValueError("invalid ratings fixture filename")
    raw = (FIXTURES / name).read_bytes()
    if not HEX64.fullmatch(digest) or sha(raw) != digest:
        raise ValueError(f"{name}: fixture digest differs")
    return [json.loads(line) for line in raw.splitlines()]


def load_cases():
    from replay_contests import load_cases as contests

    raw = (FIXTURES / "index.json").read_bytes()
    index = json.loads(raw)
    pin = json.loads((FIXTURES / "provenance.json").read_text())
    if (pin["repository"] != "https://github.com/diegoami/onenil-archaeology"
            or not re.fullmatch(r"[0-9a-f]{40}", pin["commit"])
            or pin["index_sha256"] != sha(raw)):
        raise ValueError("ratings source pin/index digest differs")
    if (index["schema"] != "ratings-corpus/1"
            or (index["calls"], index["weeks"], index["traces"]) != (1128, 6, 15)):
        raise ValueError("expected 1,128 original side calls/six weeks/15 traces")
    if sha((FIXTURES.parent / "index.json").read_bytes()) != index["t11_index_sha256"]:
        raise ValueError("T11 index differs from ratings provenance")
    contest_cases, contest_index, _ = contests()
    contest_records = {record["week"]: record for record in contest_index["records"]}
    sources = {}

    def source_matches(case):
        source = case["source"]
        original = contest_records[case["week"]]
        return (source["t11_cases"] == original["cases"]
                and source["t11_cases_sha256"] == original["cases_sha256"]
                and source["t11_record"] == original["record"]
                and source["t11_record_sha256"] == original["record_sha256"]
                and source["t08_trace"] == original["source_trace"]
                and source["t08_trace_sha256"] == original["source_trace_sha256"]
                and HEX64.fullmatch(source["t09_record_sha256"]))

    cases, files, weeks = [], set(), set()
    for record in index["records"]:
        name = record["cases"]
        measured = rows(name, record["cases_sha256"])
        if name in files or record["week"] in weeks or len(measured) != record["calls"]:
            raise ValueError("duplicate or incomplete ratings week")
        for n, case in enumerate(measured):
            if (case["schema"] != "ratings-case/1" or case["week"] != record["week"]
                    or case["call"] != n or case["draw_start"] != case["draw_end"]
                    or not source_matches(case)):
                raise ValueError("ratings call identity/span differs")
            sources.setdefault(case["week"], case["source"])
            if not same(sources[case["week"]], case["source"]):
                raise ValueError("ratings week source provenance changed")
        cases.extend(measured)
        files.add(name)
        weeks.add(record["week"])
    coverage = index["coverage"]
    if (len(cases) != 1128 or len(files) != 6 or len(coverage) != 15
            or len({entry["trace"] for entry in coverage}) != 15
            or len({entry["t04_record"] for entry in coverage}) != 15
            or {entry["cases"] for entry in coverage} != files
            or any(not HEX64.fullmatch(entry["trace_sha256"]) for entry in coverage)):
        raise ValueError("incomplete ratings T08 coverage")
    helpers = rows(index["helpers"]["file"], index["helpers"]["sha256"])
    handoffs = rows(index["handoffs"]["file"], index["handoffs"]["sha256"])
    if (len(helpers) != 15109 or index["helpers"]["cases"] != len(helpers)
            or len(handoffs) != 558 or index["handoffs"]["cases"] != len(handoffs)
            or sum(case["kind"] == "player" for case in helpers) != 12614
            or sum(case["kind"] == "bonus" for case in helpers) != 2495):
        raise ValueError("ratings helper/handoff coverage differs")
    by_helper = {}
    for helper in helpers:
        key = helper["week"], helper["kind"], helper["call"]
        if (helper["schema"] != "ratings-helper-case/1" or key in by_helper
                or not same(helper["source"], sources[helper["week"]])):
            raise ValueError("ratings helper identity/provenance differs")
        by_helper[key] = helper
    for case in cases:
        span = case["helper_indices"]
        for kind, plural in (("player", "players"), ("bonus", "bonus")):
            measured = [by_helper[(case["week"], kind, n)]
                        for n in range(span[kind + "_start"], span[kind + "_end"])]
            observed = [{key: helper[key] for key in ("input", "output")} for helper in measured]
            expected = case["helpers"][plural]
            if not same(observed, [expected] if kind == "bonus" else expected):
                raise ValueError("side helper occurrences differ from standalone helpers")
    original = {(case["week"], case["call"]): case for case in contest_cases}
    seen = set()
    side_map = {(case["week"], case["call"]): case for case in cases}
    for handoff in handoffs:
        key = handoff["week"], handoff["call"]
        if (handoff["schema"] != "ratings-contest-case/1" or key in seen or key not in original
                or not same(handoff["contest"], original[key])
                or not same(handoff["source"], sources[handoff["week"]])
                or len(handoff["sides"]) != 2
                or any((key[0], n) not in side_map for n in handoff["sides"])):
            raise ValueError("computed-rating handoff differs from literal T11 case")
        seen.add(key)
    if seen != set(original):
        raise ValueError("ratings handoffs omit a T11 call")
    return cases, helpers, handoffs, index, pin


def check(cases, helpers, handoffs):
    import onenil_engine

    case_map = {(case["week"], case["call"]): case for case in cases}
    for case in cases:
        got = json.loads(onenil_engine.rate_side(encoded(case["input"])))
        if not same(got, {"output": case["output"], "helpers": case["helpers"]}):
            raise ValueError(f"{case['week']} side {case['call']}: rating/helpers differ")
    for helper in helpers:
        function = {"player": onenil_engine.rate_player, "bonus": onenil_engine.rating_bonus}[helper["kind"]]
        if not same(json.loads(function(encoded(helper["input"]))), helper["output"]):
            raise ValueError(f"{helper['week']}: {helper['kind']} helper differs")
    for handoff in handoffs:
        sides = [case_map[(handoff["week"], n)] for n in handoff["sides"]]
        contest = handoff["contest"]
        # The request has no expected ratings. Native D983 reads the actual
        # image after both native side computations, keeping incoming seed/goals.
        request = {key: contest["input"][key] for key in ("k", "seed", "goals")}
        got = json.loads(onenil_engine.computed_rating_contests(
            encoded([case["input"] for case in sides]), encoded(request)))
        if (not same(got["sides"], [{"output": case["output"], "helpers": case["helpers"]} for case in sides])
                or not same(got["sides"][1]["output"]["ratings"], contest["input"]["ratings"])
                or got["sides"][1]["output"]["seed"] != contest["input"]["seed"]
                or not same(got["contest"]["draws"], contest["draws"])
                or got["contest"]["seed"] != contest["output"]["seed"]
                or not same(got["contest"]["goals"], contest["output"]["goals"])):
            raise ValueError(f"{handoff['week']} call {handoff['call']}: actual rating handoff differs")
    bad = json.loads(encoded(cases[0]["input"]))
    bad["players"].clear()
    try:
        onenil_engine.rate_side(encoded(bad))
    except ValueError:
        pass
    else:
        raise ValueError("native bridge accepted missing raw player inputs")
    print(f"Rust ratings: {len(cases)} sides / {len(helpers)} helpers / "
          f"{len(handoffs)} computed-rating contests / 15 T08 traces")


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], text=True).strip()


def benchmark(cases, pin, repetitions, output):
    from onenil_engine import onenil_engine as backend

    if git("status", "--porcelain", "--", "engine", "tools/replay_ratings.py", "pyproject.toml"):
        raise ValueError("commit ratings code and fixtures before timing")
    if backend.__debug_assertions__:
        raise ValueError("timing requires the release extension")
    inputs = [encoded(case["input"]) for case in cases]
    started = datetime.now(timezone.utc).isoformat()
    begin, serialized_bytes = time.perf_counter_ns(), 0
    for _ in range(repetitions):
        for raw in inputs:
            serialized_bytes += len(backend.rate_side(raw))
    elapsed = time.perf_counter_ns() - begin
    out = ROOT / output
    out.mkdir(parents=True, exist_ok=True)
    number = 1 + max((int(path.stem.split("-r")[-1]) for path in out.glob("T04-r*.json")), default=0)
    path = out / f"T04-r{number:04d}.json"
    record = {"schema": "routine-benchmark/1", "run_id": path.stem,
              "code_commit": git("rev-parse", "HEAD"), "started": started,
              "ended": datetime.now(timezone.utc).isoformat(), "platform": platform.platform(),
              "python": platform.python_version(),
              "rustc": subprocess.check_output(["rustc", "--version"], text=True).strip(),
              "cargo_lock_sha256": sha((ROOT / "engine/Cargo.lock").read_bytes()),
              "build": {"crate_version": backend.__version__,
                        "debug_assertions": backend.__debug_assertions__, "maturin": version("maturin"),
                        "extension_sha256": sha(Path(backend.__file__).read_bytes())},
              "archaeology": pin, "routine": "1B55:50EB raw-input side ratings and helpers",
              "distinct_cases": len(cases), "source_traces": 15, "repetitions": repetitions,
              "calls": len(cases) * repetitions, "rng_draws": 0, "elapsed_ns": elapsed,
              "serialized_output_bytes": serialized_bytes,
              "method": "Pre-serialized raw input JSON; bridge parsing, image setup, native ratings/helpers, output projection and serialization included; input preparation/output decoding excluded; after untimed full corpus replay",
              "limits": "Routine ratings timing only; no full-DD02/match/week, renderer/timer parity or emulator speed comparison"}
    with path.open("x") as stream:
        stream.write(json.dumps(record, indent=2) + "\n")
    print(f"{path.relative_to(ROOT)}: {record['calls']} ratings in {elapsed / 1e9:.6f} s")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--benchmark", action="store_true")
    parser.add_argument("--repetitions", type=int, default=20)
    parser.add_argument("--out", default="records/T04")
    args = parser.parse_args(argv)
    if not (args.check or args.benchmark) or args.repetitions < 1:
        parser.error("choose --check or --benchmark; repetitions must be positive")
    cases, helpers, handoffs, _, pin = load_cases()
    check(cases, helpers, handoffs)
    if args.benchmark:
        benchmark(cases, pin, args.repetitions, args.out)
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, ImportError) as error:
        print(f"ratings replay failed: {error}", file=sys.stderr)
        sys.exit(1)
