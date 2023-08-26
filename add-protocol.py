#!/usr/bin/env python3
import sys
import os

if not sys.argv[1] or not sys.argv[2]:
	print(usage)
	os.exit(-1)

with open(sys.argv[1]) as f:
	for line in f.readlines():
		print(sys.argv[2] + ":" + line, end="")
