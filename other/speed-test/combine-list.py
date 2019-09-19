import sys
import string

with open(sys.argv[1]) as domains:
    with open(sys.argv[3], 'w') as out:
        for domain in domains:
            tmp = domain.rstrip()
            with open(sys.argv[2]) as keywords:
                for keyword in keywords:
                    out.write("{}.{}\n".format(keyword.rstrip(),tmp))

