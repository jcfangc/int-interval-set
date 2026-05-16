#!/usr/bin/env python3
"""Check generated files are up to date without modifying the workspace."""

from __future__ import annotations

import subprocess


def run(args: list[str]) -> None:
    print("+", " ".join(args))
    subprocess.run(args, check=True)


def main() -> None:
    try:
        run(["cargo", "run", "-p", "codegen", "--", "--unsigned", "--check"])
        run(["cargo", "run", "-p", "codegen", "--", "--signed", "--check"])
    except subprocess.CalledProcessError as exc:
        raise SystemExit(
            "generated files are stale; run codegen locally and commit the result"
        ) from exc


if __name__ == "__main__":
    main()
