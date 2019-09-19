import sys
import string

#./combine.py wordlist.txt domain
domain = sys.argv[1]

with open(sys.argv[3], 'w') as out:
    with open(sys.argv[2]) as f:
            for line in f:
                    out.write("{}.{}\n".format(line.rstrip(),domain))
