# Genx

Genx is a fast alternative domain name generator to help in generating potential subdomains of a target. Useful when starting recon on a domain.

## Objectives of genx

1. Create a tool which has all the capabilities of altdns and more, and which can generate large amounts of data very efficiently
2. Be faster than it's python equivalent
3. Allows for an array of cli commands
4. Finally: simply practice Rust

## Compilation

To compile run:

A Makefile has been created so you can simply run:
```
make
```

Otherwise, compile manually:
```
rustc -O genx.rs
```

To cleanup:
```
make clean
```

## Usage

```
./genx <domains> <wordlist.txt> <out_file> [-g]

```

## WIP: Speed comparaison

### Simple keyword to domain concatenation

Comparaison with the current version of `genx` which reads input from a file, and outputs the combined keyword+domain to a file or stdout.

Lets compare the speed with a python program aimed at the same task:
```
import sys
import string

#./combine.py domain wordlist.txt output.txt

domain = sys.argv[3]

with open(sys.argv[2], 'w') as out:
    with open(sys.argv[1]) as f:
            for line in f:
                    out.write("{}.{}".format(line.rstrip(),domain))
```

We start by generating a file (`tmp.txt`) with 10,000,000 lines of 8 chars each which we will pass to both programs. (See generator at `other/speed-test/random-str.py` for source-code)

We run the program as such: 

`python3 other/speed-test/random-str.py 10000000 > tmp.txt`

For this test we will use the domain name "example.com".

We use `time` to record the time it takes for the program to execute and we run the program 10 times to get an average execution time.

#### Python Script

We run the script as such:

`time for i in {1..10}; do python other/speed-test/combine.py "example.com" tmp.txt /dev/null; done`

and get the following times:

```
bash-3.2$ time for i in {1..10}; do python other/speed-test/combine.py "example.com" tmp.txt /dev/null; done

real    1m18.426s
user    1m17.457s
sys     0m0.637s
```

#### Genx

We run the script as such:

`time for i in {1..10}; do ./genx "example.com" tmp.txt /dev/null; done`

and get the following times:

```
bash-3.2$ time for i in {1..10}; do ./genx "example.com" tmp.txt /dev/null; done

real    0m20.861s
user    0m20.271s
sys     0m0.468s

```

#### Result

78.426/20.861 = 3.759

This results in 276% faster subdomain generation when using genx vs its python equivalent.


### Domain text list to keyword concatenation

For this second test we will use the same tools as in the previous test except we will run this python script as comparaison:

```python
import sys
import string

with open(sys.argv[1]) as domains:
    with open(sys.argv[3], 'w') as out:
        for domain in domains:
            tmp = domain.rstrip()
            with open(sys.argv[2]) as keywords:
                for keyword in keywords:
                    out.write("{}.{}\n".format(keyword.rstrip(),tmp))

```

We reduce the number of keywords to 1,000,000 and setup a file with 10 domains:
```
example1.com
example2.com
example3.com
example4.com
example5.com
example6.com
example7.com
example8.com
example9.com
example0.com
```

#### Python Script

We used this command to run the script:

`time for i in {1..10}; do python other/speed-test/combine-list.py other/speed-test/domains.txt tmp.txt /dev/null; done`

and got the following total time:
```
bash-3.2$ time for i in {1..10}; do python other/speed-test/combine-list.py other/speed-test/domains.txt tmp.txt /dev/null; done

real    1m21.109s
user    1m18.348s
sys     0m1.154s
```

#### C program

We used this command to run the program:

`time for i in {1..10}; do ./other/speed-test/genc other/speed-test/domains.txt tmp.txt /dev/null; done`

and got the following times:
```
bash-3.2$ time for i in {1..10}; do ./other/speed-test/genc other/speed-test/domains.txt tmp2.txt /dev/null; done

real    0m24.739s
user    0m23.589s
sys     0m0.750s
```

##### Genx

We used this command to run the program:

`time for i in {1..10}; do ./genx other/speed-test/domains.txt tmp.txt /dev/null; done`

and got the following times:
```
bash-3.2$ time for i in {1..10}; do ./genx other/speed-test/domains.txt tmp.txt /dev/null; done
real    0m21.099s
user    0m20.373s
sys     0m0.552s
```

#### Results

Genx outperformed both C and Python at the same task. 

With an increase of 284% over its Python equivalent and 17.3% faster than the C version.


