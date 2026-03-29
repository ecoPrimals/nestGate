#!/usr/bin/env python3
"""Prepend SPDX + copyright to Rust sources under code/ when missing."""

from __future__ import annotations

import os
import sys

PREFIX = (
    "// SPDX-License-Identifier: AGPL-3.0-only\n"
    "// Copyright (c) 2025 ecoPrimals Collective\n\n"
)
SPDX_LINE = "// SPDX-License-Identifier: AGPL-3.0-only"


def main() -> int:
    root = os.path.join(os.path.dirname(__file__), "..", "code")
    root = os.path.normpath(root)
    if not os.path.isdir(root):
        print(f"error: not a directory: {root}", file=sys.stderr)
        return 1

    updated = 0
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = [d for d in dirnames if d != "target"]

        for name in filenames:
            if not name.endswith(".rs"):
                continue
            path = os.path.join(dirpath, name)
            try:
                with open(path, "r", encoding="utf-8") as f:
                    content = f.read()
            except OSError as e:
                print(f"error reading {path}: {e}", file=sys.stderr)
                return 1

            if content.startswith(SPDX_LINE):
                continue

            new_content = PREFIX + content
            try:
                with open(path, "w", encoding="utf-8", newline="\n") as f:
                    f.write(new_content)
            except OSError as e:
                print(f"error writing {path}: {e}", file=sys.stderr)
                return 1
            updated += 1

    print(f"Updated {updated} file(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
