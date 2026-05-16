#!/usr/bin/env python3
"""Create and push release tag."""

from __future__ import annotations

import os
import subprocess


def run(args: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(args))
    return subprocess.run(args, check=check, text=True, capture_output=False)


def tag_exists(tag: str) -> bool:
    result = subprocess.run(
        ["git", "rev-parse", "-q", "--verify", f"refs/tags/{tag}"],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    return result.returncode == 0


def main() -> None:
    event_name = os.environ.get("GITHUB_EVENT_NAME", "")
    mode = os.environ.get("RELEASE_MODE", "")
    tag_dryrun = os.environ.get("TAG_DRYRUN", "false")
    version = os.environ["RELEASE_VERSION"]
    run_id = os.environ["GITHUB_RUN_ID"]

    if event_name == "workflow_dispatch" and mode != "real":
        if tag_dryrun != "true":
            print("skip tag (dryrun)")
            return
        tag = f"v{version}-dryrun.{run_id}"
    else:
        tag = f"v{version}"

    run(["git", "fetch", "--tags", "origin"])

    if tag_exists(tag):
        raise SystemExit(f"tag already exists: {tag}")

    run(["git", "config", "user.name", "github-actions[bot]"])
    run(["git", "config", "user.email", "github-actions[bot]@users.noreply.github.com"])

    run(["git", "tag", "-a", tag, "-m", f"Release {tag}"])
    run(["git", "push", "origin", f"refs/tags/{tag}"])


if __name__ == "__main__":
    main()
