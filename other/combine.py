import sys
import string

#./combine.py wordlist.txt domain

domain = sys.argv[2]

with open('out', 'w') as out:
    with open(sys.argv[1]) as f:
            for line in f:
                    out.write("{}.{}\n".format(line.rstrip(),domain))
