#!/usr/bin/env python3
from pathlib import Path
import subprocess, sys

files = [str(p) for p in Path().glob("*.hurl")] or sys.exit("No .hurl files to run.")
subprocess.run(["hurl", "--test", "--jobs", "1", "--variable", "host=http://localhost:8000", *files], check=True)