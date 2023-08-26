#!/usr/bin/env python3
import sys
import os

try:
	sys.argv[1]
except IndexError:
	print("Usage: add-protocol.py <file> <protocol>")
	sys.exit(1)

with open(sys.argv[1]) as f:
	for line in f.readlines():
		print(sys.argv[2] + ":" + line, end="")
