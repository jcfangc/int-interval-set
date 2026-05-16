#!/usr/bin/env python3
"""Detect package version change and write GitHub Actions outputs."""

from __future__ import annotations

import os
import subprocess
import tomllib
from pathlib import Path


def run(args: list[str]) -> subprocess.CompletedProcess[str]:
    return subprocess.run(args, check=True, text=True, capture_output=True)


def package_version_from_text(text: str) -> str:
    data = tomllib.loads(text)
    return data["package"]["version"]


def current_version() -> str:
    with Path("Cargo.toml").open("rb") as f:
        data = tomllib.load(f)
    return data["package"]["version"]


def has_parent_commit() -> bool:
    return (
        subprocess.run(
            ["git", "rev-parse", "-q", "--verify", "HEAD^1"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        ).returncode
        == 0
    )


def previous_version() -> str:
    if not has_parent_commit():
        return ""

    try:
        prev_cargo_toml = run(["git", "show", "HEAD^1:Cargo.toml"]).stdout
    except subprocess.CalledProcessError:
        return ""

    return package_version_from_text(prev_cargo_toml)


def write_output(name: str, value: str) -> None:
    output = os.environ.get("GITHUB_OUTPUT")

    if output:
        with Path(output).open("a", encoding="utf-8") as f:
            f.write(f"{name}={value}\n")
    else:
        print(f"{name}={value}")


def main() -> None:
    cur = current_version()
    prev = previous_version()
    changed = "true" if prev and cur != prev else "false"

    write_output("version", cur)
    write_output("prev_version", prev)
    write_output("changed", changed)

    print(f"current version: {cur}")
    print(f"previous version: {prev or '<none>'}")
    print(f"changed: {changed}")


if __name__ == "__main__":
    main()
