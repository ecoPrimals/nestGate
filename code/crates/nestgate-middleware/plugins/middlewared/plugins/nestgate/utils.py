#!/usr/bin/env python3

import asyncio
from dataclasses import dataclass


@dataclass
class RunResult:
    returncode: int = 0
    stdout: bytes = b""
    stderr: bytes = b""


async def run(cmd, check=True, **kwargs):
    """Mock run function that simulates execution of a command."""
    # For test purposes, return a successful result
    return RunResult(returncode=0, stdout=b"", stderr=b"") 