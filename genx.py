import sys
import string

#./combine.py wordlist.txt domains.txt

domain = sys.argv[2]

with open(sys.argv[1]) as f:
        for line in f:
                line = line.strip()
                print "{}.{}".format(line,domain)
