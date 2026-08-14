#!/usr/bin/env python3
"""Measure which skill actually fires for a set of prompts.

Unlike skill-creator's trigger eval, this does not create a proxy skill. It runs
`claude -p` against the real environment — every installed plugin and skill is
present and competing — and records the name of the first Skill tool call. That
is the only way to get a truthful answer once your skills are installed: a proxy
copy always loses to the real thing, so a proxy-based harness reports zero
triggers for every query.

Recording the winner (rather than a boolean) is the point. A skill that fails to
fire because a third-party skill outranked it is a very different problem from
one whose description simply does not match, and the fix differs too.

Safety: each run is killed the moment the first Skill call appears, so the agent
never gets far enough to act. Write/Edit/Bash are disallowed as well, because
these prompts are meant to be run inside real project directories.

Usage:
  python3 evals/run_trigger_eval.py \
      --eval-set plugins/eng/skills/rust-code-review/evals/trigger_eval.json \
      --expect coke-eng:rust-code-review \
      --cwd ~/Projects/checkilo/backend
"""

import argparse
import json
import os
import subprocess
import sys
import threading
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

BLOCKED = ["Write", "Edit", "MultiEdit", "NotebookEdit", "Bash"]

# How many non-Skill tool calls to allow before concluding no skill will fire.
# The agent often reads a file or two before deciding; past that it has clearly
# started doing the work itself.
DEFAULT_MAX_TOOLS = 4


def run_once(query: str, cwd: str, timeout: int, max_tools: int) -> str:
    """Return the skill name that fired first, '' if none, or '(timeout)'."""
    cmd = [
        "claude", "-p", query,
        "--output-format", "stream-json",
        "--verbose",
        "--disallowedTools", *BLOCKED,
    ]
    # CLAUDECODE guards against interactive nesting; a subprocess is safe.
    env = {k: v for k, v in os.environ.items() if k != "CLAUDECODE"}

    proc = subprocess.Popen(
        cmd, cwd=cwd, env=env,
        stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
        text=True, bufsize=1,
    )
    result = {"value": ""}
    timer = threading.Timer(timeout, lambda: (result.__setitem__("value", "(timeout)"), proc.kill()))
    timer.start()
    try:
        tools = 0
        for line in proc.stdout:
            try:
                data = json.loads(line)
            except (ValueError, TypeError):
                continue
            content = (data.get("message") or {}).get("content") or []
            for block in content:
                if not isinstance(block, dict) or block.get("type") != "tool_use":
                    continue
                if block.get("name") == "Skill":
                    result["value"] = (block.get("input") or {}).get("skill", "") or "(unnamed)"
                    proc.kill()
                    return result["value"]
                tools += 1
                if tools >= max_tools:
                    proc.kill()
                    return result["value"]
    finally:
        timer.cancel()
        proc.kill()
    return result["value"]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--eval-set", required=True)
    ap.add_argument("--expect", required=True, help="Qualified skill name, e.g. coke-eng:rust-code-review")
    ap.add_argument("--cwd", default=".", help="Directory to run the prompts in — use a real project")
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--workers", type=int, default=6)
    ap.add_argument("--timeout", type=int, default=180)
    ap.add_argument("--max-tools", type=int, default=DEFAULT_MAX_TOOLS)
    ap.add_argument("--json", dest="json_out", default=None)
    args = ap.parse_args()

    cwd = str(Path(args.cwd).expanduser().resolve())
    items = json.loads(Path(args.eval_set).read_text())
    jobs = [(i, item) for i, item in enumerate(items) for _ in range(args.runs)]

    print(f"{args.expect}  —  {len(items)} queries x {args.runs} runs  in {cwd}", file=sys.stderr)

    fired: dict[int, list[str]] = {i: [] for i in range(len(items))}
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = {
            pool.submit(run_once, item["query"], cwd, args.timeout, args.max_tools): i
            for i, item in jobs
        }
        for fut in futures:
            pass  # submitted; collected below
        for fut, i in futures.items():
            try:
                fired[i].append(fut.result())
            except Exception as exc:  # a dead subprocess should not sink the run
                fired[i].append(f"(error: {type(exc).__name__})")

    results, passed = [], 0
    for i, item in enumerate(items):
        got = fired[i]
        hits = sum(1 for g in got if g == args.expect)
        rate = hits / len(got)
        ok = rate >= 0.5 if item["should_trigger"] else rate < 0.5
        passed += ok
        others = Counter(g for g in got if g and g != args.expect)
        results.append({
            "query": item["query"],
            "should_trigger": item["should_trigger"],
            "trigger_rate": rate,
            "pass": ok,
            "won_instead": others.most_common(),
        })
        mark = "PASS" if ok else "FAIL"
        instead = ""
        if others:
            instead = "  <- " + ", ".join(f"{n} x{c}" for n, c in others.most_common(2))
        print(f"  [{mark}] {hits}/{len(got)} expect={str(item['should_trigger']):5} "
              f"{item['query'][:58]}{instead}", file=sys.stderr)

    summary = {"expect": args.expect, "cwd": cwd, "runs": args.runs,
               "total": len(items), "passed": passed, "failed": len(items) - passed,
               "results": results}
    print(f"  => {passed}/{len(items)} passed", file=sys.stderr)
    if args.json_out:
        Path(args.json_out).write_text(json.dumps(summary, indent=2))
    else:
        print(json.dumps(summary, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
