#!/usr/bin/env python3
"""Publish crate or run cargo publish dry-run."""

from __future__ import annotations

import os
import subprocess


def run(args: list[str]) -> None:
    print("+", " ".join(args))
    subprocess.run(args, check=True)


def main() -> None:
    event_name = os.environ.get("GITHUB_EVENT_NAME", "")
    mode = os.environ.get("RELEASE_MODE", "")

    is_dispatch_dryrun = event_name == "workflow_dispatch" and mode != "real"

    if is_dispatch_dryrun:
        run(["cargo", "publish", "--dry-run", "--locked"])
    else:
        run(["cargo", "publish", "--locked"])


if __name__ == "__main__":
    main()
