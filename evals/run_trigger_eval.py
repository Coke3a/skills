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


def run_once(query: str, cwd: str, timeout: int, max_tools: int, max_skills: int = 1) -> list[str]:
    """Return the skill names that fired, in order, up to max_skills.

    max_skills=1 answers "who wins the prompt". Raising it answers a different
    and equally important question: when a generic process skill wins, does it
    then hand off to the specialist, or does it swallow the task? A loss at
    position 1 that becomes a hit at position 2 is a sequencing detail, not a
    triggering failure, and the two call for opposite fixes.
    """
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
    skills: list[str] = []
    timer = threading.Timer(timeout, proc.kill)
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
                    skills.append((block.get("input") or {}).get("skill", "") or "(unnamed)")
                    if len(skills) >= max_skills:
                        proc.kill()
                        return skills
                    continue
                tools += 1
                if tools >= max_tools:
                    proc.kill()
                    return skills
    finally:
        timer.cancel()
        proc.kill()
    return skills


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--eval-set", required=True)
    ap.add_argument("--expect", required=True, help="Qualified skill name, e.g. coke-eng:rust-code-review")
    ap.add_argument("--cwd", default=".", help="Directory to run the prompts in — use a real project")
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--workers", type=int, default=6)
    ap.add_argument("--timeout", type=int, default=180)
    ap.add_argument("--max-tools", type=int, default=DEFAULT_MAX_TOOLS)
    ap.add_argument("--max-skills", type=int, default=1,
                    help="Record this many skill calls per run. >1 reveals handoffs")
    ap.add_argument("--json", dest="json_out", default=None)
    args = ap.parse_args()

    cwd = str(Path(args.cwd).expanduser().resolve())
    items = json.loads(Path(args.eval_set).read_text())
    jobs = [(i, item) for i, item in enumerate(items) for _ in range(args.runs)]

    print(f"{args.expect}  —  {len(items)} queries x {args.runs} runs  in {cwd}", file=sys.stderr)

    fired: dict[int, list[list[str]]] = {i: [] for i in range(len(items))}
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = {
            pool.submit(run_once, item["query"], cwd, args.timeout, args.max_tools, args.max_skills): i
            for i, item in jobs
        }
        for fut, i in futures.items():
            try:
                fired[i].append(fut.result())
            except Exception as exc:  # a dead subprocess should not sink the run
                fired[i].append([f"(error: {type(exc).__name__})"])

    results, passed = [], 0
    for i, item in enumerate(items):
        got = fired[i]
        hits = sum(1 for seq in got if args.expect in seq)
        rate = hits / len(got)
        ok = rate >= 0.5 if item["should_trigger"] else rate < 0.5
        passed += ok
        # Where in the sequence the expected skill landed — 1 means it won outright.
        positions = [seq.index(args.expect) + 1 for seq in got if args.expect in seq]
        others = Counter(seq[0] for seq in got if seq and seq[0] != args.expect)
        results.append({
            "query": item["query"],
            "should_trigger": item["should_trigger"],
            "trigger_rate": rate,
            "pass": ok,
            "positions": positions,
            "won_first": others.most_common(),
            "sequences": got,
        })
        mark = "PASS" if ok else "FAIL"
        note = ""
        if others:
            note = "  <- " + ", ".join(f"{n} x{c}" for n, c in others.most_common(2))
        if positions and any(p > 1 for p in positions):
            note += f"  [handoff at position {sorted(set(positions))}]"
        print(f"  [{mark}] {hits}/{len(got)} expect={str(item['should_trigger']):5} "
              f"{item['query'][:52]}{note}", file=sys.stderr)

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
