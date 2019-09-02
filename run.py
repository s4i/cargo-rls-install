#!/usr/bin/env python3

import subprocess
import sys

cmd = "cargo run --bin cargo-rls-install rls-install"

sys.argv.pop(0)
for arg in sys.argv:
    cmd += ' ' + arg

subprocess.call(cmd.split())
