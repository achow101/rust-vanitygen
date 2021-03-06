#! /usr/bin/env python3

import argparse
import subprocess
import time

parser = argparse.ArgumentParser()
parser.add_argument("prefix", help="Prefix to search for")
parser.add_argument("--processes", "-j", type=int, help="Number of processes", default=1)
parser.add_argument("--merkle", help="Merkle root")
args = parser.parse_args()

cmd = ["target/release/rust-vanitygen", args.prefix]
if args.merkle:
    cmd.append(args.merkle)

procs = []
for _ in range(0, args.processes):
    procs.append(subprocess.Popen(cmd))

done = False
while not done:
    time.sleep(1)
    for p in procs:
        if p.poll() is not None:
            done = True
            continue
for p in procs:
    p.terminate()
    p.wait()
