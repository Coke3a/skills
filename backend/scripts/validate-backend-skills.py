#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPECTED = [
    "rust-clean-coke-architecture-patterns",
    "tdd-feature-workflow",
    "rust-code-review",
    "rust-ci-cd",
    "rust-performance-optimization",
]
MARKDOWN_EXTENSIONS = {".md"}
YAML_EXTENSIONS = {".yml", ".yaml"}

errors: list[str] = []
warnings: list[str] = []


def err(message: str) -> None:
    errors.append(message)


def warn(message: str) -> None:
    warnings.append(message)


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def parse_skill_frontmatter(path: Path) -> None:
    text = read_text(path)
    lines = text.splitlines()

    if len(lines) < 6:
        err(f"{path}: SKILL.md is too short to contain valid frontmatter and a title")
        return

    if lines[0] != "---":
        err(f"{path}: missing opening frontmatter delimiter '---'")
        return

    try:
        closing_index = lines.index("---", 1)
    except ValueError:
        err(f"{path}: missing closing frontmatter delimiter '---'")
        return

    frontmatter = lines[1:closing_index]
    if len(frontmatter) != 2:
        err(f"{path}: frontmatter must contain exactly 'name' and 'description'")
        return

    name_line, description_line = frontmatter
    if not name_line.startswith("name: "):
        err(f"{path}: frontmatter must start with 'name: ...'")
    if not description_line.startswith("description: "):
        err(f"{path}: frontmatter must include 'description: ...'")

    if closing_index + 1 >= len(lines) or lines[closing_index + 1] != "":
        err(f"{path}: frontmatter must be followed by a blank line")
        return

    if closing_index + 2 >= len(lines) or not lines[closing_index + 2].startswith("# "):
        err(f"{path}: markdown body must start with a level-1 title heading")


def validate_markdown(path: Path) -> None:
    text = read_text(path)
    lines = text.splitlines()

    fence_count = 0

    for index, line in enumerate(lines, start=1):
        stripped = line.strip()

        if stripped.startswith("```"):
            fence_count += 1
            continue

        if fence_count % 2 == 1:
            continue

        if "\t" in line:
            warn(f"{path}:{index}: contains a tab character")

        if re.search(r"\s+$", line):
            warn(f"{path}:{index}: trailing whitespace")

        if line.count("# ") > 1:
            err(f"{path}:{index}: multiple headings appear on one line")

        if line.count("- [ ]") > 1 or line.count("- [x]") > 1 or line.count("- [X]") > 1:
            err(f"{path}:{index}: multiple checklist markers appear on one line")

        if line.count("```") > 1:
            err(f"{path}:{index}: multiple code fences appear on one line")

        if "|" in line and not stripped.startswith("|"):
            if stripped.count("|") >= 4 and "http" not in stripped:
                err(f"{path}:{index}: possible collapsed markdown table row")

        if stripped.startswith(("- ", "* ", "+ ")):
            marker, content = stripped[:2], stripped[2:].strip()
            if any(token in content for token in (" - ", " * ", " + ")) and "`" not in content:
                warn(f"{path}:{index}: long bullet may need manual review")

    if fence_count % 2 != 0:
        err(f"{path}: unbalanced fenced code blocks")


def ruby_yaml_available() -> bool:
    result = subprocess.run(
        ["ruby", "-e", "require 'yaml'"],
        capture_output=True,
        text=True,
        check=False,
    )
    return result.returncode == 0


def validate_yaml_with_ruby(path: Path) -> None:
    script = (
        "require 'yaml'; "
        "YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: true)"
    )
    result = subprocess.run(
        ["ruby", "-e", script, str(path)],
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        message = result.stderr.strip() or result.stdout.strip() or "unknown YAML error"
        err(f"{path}: YAML parse error: {message}")


def validate_shell_script(path: Path) -> None:
    if not path.exists():
        return

    text = read_text(path)
    if "\n" not in text:
        err(f"{path}: file must be multi-line")

    if not text.startswith("#!/usr/bin/env bash\n"):
        err(f"{path}: expected bash shebang")


def validate_dockerfile(path: Path) -> None:
    text = read_text(path)
    required_tokens = [
        "FROM ",
        "WORKDIR ",
        "COPY ",
        "CMD ",
    ]

    if "\n" not in text:
        err(f"{path}: Dockerfile template must be multi-line")

    for token in required_tokens:
        if token not in text:
            err(f"{path}: missing Dockerfile instruction '{token.strip()}'")


def validate_repository_structure() -> None:
    for skill_name in EXPECTED:
        skill_dir = ROOT / skill_name
        if not skill_dir.is_dir():
            err(f"missing skill folder: {skill_dir}")
            continue

        for subdir in ("workflows", "references", "templates"):
            if not (skill_dir / subdir).is_dir():
                err(f"{skill_dir}: missing {subdir}/")

        skill_path = skill_dir / "SKILL.md"
        if not skill_path.exists():
            err(f"{skill_dir}: missing SKILL.md")
        else:
            parse_skill_frontmatter(skill_path)

        if not (skill_dir / "EVALS.md").exists():
            err(f"{skill_dir}: missing EVALS.md")

    if not (ROOT / "README.md").exists():
        err("missing backend/README.md")

    if not (ROOT / "templates" / "AGENTS.md").exists():
        err("missing backend/templates/AGENTS.md")


def validate_files() -> None:
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file():
            continue

        if path.suffix in MARKDOWN_EXTENSIONS:
            validate_markdown(path)

        if path.name == "SKILL.md":
            parse_skill_frontmatter(path)

        if path.suffix in YAML_EXTENSIONS:
            validate_yaml_with_ruby(path)

    validate_shell_script(ROOT / "scripts" / "validate-backend-skills.sh")
    validate_shell_script(ROOT / "rust-ci-cd" / "templates" / "smoke-test.sh")
    validate_dockerfile(ROOT / "rust-ci-cd" / "templates" / "dockerfile-rust-service")


def main() -> None:
    parser = argparse.ArgumentParser(description="Validate backend skill assets.")
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Fail on warnings in addition to errors.",
    )
    args = parser.parse_args()

    validate_repository_structure()

    if not ruby_yaml_available():
        err("Ruby with YAML support is required to validate backend YAML templates")
    else:
        validate_files()

    print("skills checked:", ", ".join(EXPECTED))
    print("errors:", len(errors))
    for message in errors:
        print(" -", message)
    print("warnings:", len(warnings))
    for message in warnings:
        print(" -", message)

    failed = bool(errors) or (args.strict and bool(warnings))
    print("result:", "FAIL" if failed else "PASS")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
