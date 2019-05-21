#!/bin/env python

import sys
import string

words = set()

with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        line = line.strip('.') # remove leading and ending .

        words.update(line.split('.')) # cut on . and append all words to set


tmp_set = set()
for word in words:
    tmp_set.update(word.replace("-"," ").replace("_"," ").split())

words |= tmp_set

tmp_set = set()
for word in words:
    tmp_set.add(''.join(map(lambda c: '' if c in '0123456789' else c, word))) # strip numbers from start and end
    tmp = ''.join(map(lambda c: '' if c.isalpha()  else c, word)) # strip letters from start and end
    if tmp not in '-_. ':
        tmp_set.add(tmp)

words |= tmp_set

for word in words:
    print word