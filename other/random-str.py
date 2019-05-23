import uuid
import sys
import string
import random

stringLength = 8

# python3 other/random-str.py 1000000 > tmp.txt

for _ in range(int(sys.argv[1])):
    print(''.join(random.choices(string.ascii_uppercase + string.digits, k=stringLength)))
