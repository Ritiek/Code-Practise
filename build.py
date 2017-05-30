#!/usr/bin/env python

import os

print(os.listdir('.'))
for x in os.listdir('.'):
	os.system('git add ' + x)
	os.system('git commit -m "Add ' + x + '"')
