# GEA-1 and 2 implementations (in Python, C and Rust)

This repository contains software implementations of the **GPRS Encryption 
Algorithm 1 and 2**. The ["*Cryptanalysis of the GPRS Encryption Algorithms GEA-1
and GEA-2*"](https://eprint.iacr.org/2021/819.pdf) research paper provides the
complete description of both algorithms, and an efficient cryptanalysis against 
GEA-1 (allowing to weaken the key strength to 40 bits instead of 64).
A [2nd paper]() proposes a cryptanalysis against GEA2

Both GEA-1 and GEA-2 are stream-cipher based on 3, and respectively 4, LFSRs and a boolean
function.


## Licensing

The code contained in this repository provided as-is without warranty, under the
[GNU AGPL v3](https://www.gnu.org/licenses/agpl-3.0.txt) software license.
If you are interested by another kind of licensing, please contact 
[P1 Security](https://www.p1sec.com/corp/contact/).


## Usage

### Installation

As the code provided is to be used into any kind of application, there is no installation required.


### Python

The code is made to work with Python3.

In order to use the Python version, you can load the python file as a module and call
the `GEA1` or `GEA` classes. Beware that arguments are in an uncommon format: inputs IV, 
direction and Key must be passed as integral values, and the `gen()` method takes a length
in bits for the requested keystream, and returns a list of bits (0 or 1). Docstrings are 
provided within the modules.
Moreover, the `LFSR.dbg` attribute can be set in order to print the initialized values
within registers.

```
>>> from gea12 import *
>>> hex(bitlist_to_uint(GEA1(0, 0, 0).gen(192)))
'0x8faa15c45874c140a71348ad63cbbc9e8ac31421ab98a11f'
>>> LFSR.dbg = 1
>>> hex(bitlist_to_uint(GEA1(0, 0, 0).gen(192)))
S init: 0x0000000000000000
A init: 0x0000000040000000
B init: 0x0000000080000000
C init: 0x0000000100000000
'0x8faa15c45874c140a71348ad63cbbc9e8ac31421ab98a11f'
```


Calling the `gea12.py` file as is runs the 3 test vectors for the 2 algorithms.
```
$ python ./gea12.py 
GEA1 test vector 0: OK
GEA1 test vector 1: OK
GEA1 test vector 2: OK
GEA2 test vector 0: OK
GEA2 test vector 1: OK
GEA2 test vector 2: OK
```


### C

The C implementation is provided under the `gea-c` sub-directory.
The implementation for GEA1 and GEA2 is available in the `gea12.c` and `gea12.h` files,
while the `gea12_test.c` implements the testing with the 3 test vectors. A very simple 
Makefile enables to build an application running the test vectors or a shared library:

```
$ make
gcc -std=c99 -O2 -Wall -Wno-unused-function -fPIC -o test gea12.c gea12_test.c 
$ ./test
all tests passed
$ make lib
gcc -std=c99 -O2 -Wall -Wno-unused-function -fPIC -shared -o gea12.so gea12.c 
$ ll gea12.so
-rwxrwxr-x 1 user user 20656 oct.  19 16:32 gea12.so*
$ make clean
rm -f test gea12.so
```

Warning: the code makes use of the `uint64_t` type for each register, hence requires
a 64 bit machine. Moreover, it has been tested on a little-endian system only.


### rust

TODO


## How do those LFSR-based algorithms work

It should first be noted that GEA-1 and GEA-2, which are very similar (GEA-1 is just 
an extension of GEA-2 with an higher amount of processing) are bit-oriented stream ciphers.

A stream cipher, like the well-known RC4 or GEA-1, works by the Xor operation. 
Also, the Xor operation being symmetrical, this means that encrypting is the same 
operation as decrypting: GEA-1 and GEA-2 are basically pseudo-random data generators, 
taking a seed (the key, IV and direction bit of the GPRS data, which are concatenated), 
and the generated random data (the keystream) is Xor'd with the clear-text data (the plaintext) 
for encrypting. Then, later, the keystream is Xor'd with the encrypted data (the ciphertext) 
for decrypting. That is why the functions called in the target library for decrypting 
and encrypting are same.

GEA-1 and GEA-2 are bit-oriented, unlike RC4 which is*byte-oriented, because their 
algorithms generate only one bit of pseudo-random data at once (derived from their internal state), 
while algorithms like RC4 generate no less than one byte at once (in RC4's case, derived 
from permutation done in its internal state). Even though the keystream bits are put 
together by the current encryption / decryption C and rust libraries into bytes in order to 
generate usable keystream, obviously.

Now, you can understand that GEA-1 and GEA-2 are LFSR: 
[Linear Feedback Shift Register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)-oriented ciphers, 
because their internal state is stored into fixed-size registers. This includes the S and W 
registers which serve from initialization / key scheduling purposes and are respectively 
64 and 97-bit wide registers, and the A, B, C and for GEA-2 only D registers which serve 
from the purpose of keystream generation, which are respectively 31, 32, 33 and 29-bit wide 
registers.

At each iteration of the keystream generation, each register is 
[bit-wise rotated](https://en.wikipedia.org/wiki/Circular_shift)  of one position, and the bit being rotated from 
the left towards the right side (or conversely depending on in which bit order you internally 
represent your registers) is being fed to the algorithm and mutated depending on given conditions, 
hence the shifted out bit is derived from something and reinserted while being possibly 
flipped depending on conditions at the other side of the given register. That is why 
the name of linear feedback shift register (shift because of the shift operation required 
for the rotation, and linear feedback because of the constant-time transform operation involved).

The rest of the register may also be mutated at each iteration steps, as in the case of the GEA-1 and 2, 
whole fixed Xor sequences (which differ for each register) may be applied depending on whether 
the rotated bit is a 0 or a 1.

Note that a step where the register iterates is called *clocking* (the register is *clocked*), 
and that the fixed points where the register may be Xor'ed when the rotated bit becomes a 1 are called *taps*.
The linear function which may transmute the rotated bit at the clocking step (taking several bits 
of the original register as an input) is called the *F function*.

