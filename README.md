# Genx

Genx is an alternative domain name generator to help in generating potential subdomains of a target. Usefull when starting recon on a domain.

## Objectives of genx

1. Create a tool which has all the capabilities of altdns and more, and which can generate large amounts of data very efficiently
2. Be faster than it's python equivalent
3. Allows for an array of cli commands
4. Finally: simply practice Rust

## Usage

```
./genx <domains> <wordlist.txt> <out_file> [-g]


```

## WIP: Speed comparaison

Comparaison with the current version of `genx` which reads input from a file, and outputs the combined keyword+domain to a file or stdout.

Lets compare the speed with a python program aimed at the same task:
```
import sys
import string

#./combine.py wordlist.txt domain

domain = sys.argv[2]

with open('out', 'w') as out:
    with open(sys.argv[1]) as f:
            for line in f:
                    out.write("{}.{}".format(line.rstrip(),domain))
```

We start by generating a file (`tmp.txt`) with 1,000,000 lines of 8 chars each which we will pass to both programs. (See generator at `other/rando-str.py` for source-code)

We will use the domain name "example.com".



