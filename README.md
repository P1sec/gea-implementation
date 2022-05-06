# GEA-1 and 2 implementations (in Python, C and Rust)

This repository contains software implementations of the **GPRS Encryption 
Algorithm, version 1 and 2**.

The [*Cryptanalysis of the GPRS Encryption Algorithms GEA-1 and GEA-2*](https://eprint.iacr.org/2021/819.pdf) 
research paper provides the complete description of both algorithms, and an efficient cryptanalysis against 
GEA-1 (allowing to weaken the key strength to 40 bits instead of 64). It also 
provides hints for the cryptanalysis of GEA-2.

A second paper: [*o Shift or Not to Shift: Understanding GEA-1*](https://eprint.iacr.org/2021/829.pdf)
extends this cryptanalysis and provides a broader look at this kind of cryptographic construct.

A third one: [*Refined Cryptanalysis of the GPRS Ciphers GEA-1 and GEA-2*](https://eprint.iacr.org/2022/424.pdf)
enhances the cryptanalysis of both GEA1 and GEA2, e.g. reducing the memory requirements.


## Disclaimer

*DO NOT USE THESE ALGORITHMS FOR ANYTHING SERIOUS!*

The source code provided in this project is for educational purposes only, in order to help 
understanding the recently published cryptanalysis.


## History

Both GEA-1 and GEA-2 are stream ciphers based on 3, respectively 4, LFSRs and a 
boolean function. They are relying on a 64 bits symmetric key, established after
a successful authentication of a mobile subscriber. They can be used to protect
GPRS and EDGE connections, depending of the configuration done by the mobile 
operator in its [SGSN](https://en.wikipedia.org/wiki/GPRS_core_network#Serving_GPRS_support_node_(SGSN)).

They were initially designed in the 90's, together with the GPRS system.
From the cryptanalysis paper, it seems that GEA-1 was weakened on purpose; this looks 
similar as A5/2, which was a weakened encryption algorithm for GSM. With the development 
of 3G and UMTS, a new algorithm GEA3 was designed in the 2000's, based on Kasumi. 
This last one is hopefully used by most of the operators in their SGSN appliances today (in 2021).

A risk remains as most of the handsets continue supporting GEA-1 and GEA-2 (in 2021), 
even though there is increasing effort purporting to at least remove GEA-1 from these.
Keeping support for weak encryption algorithm in current handsets enables for potential 
semi-passive or plain man-in-the-middle attacks against GPRS and EDGE connections.

Unfortunately, other attacks exist against 2G connections, mainly due to weaknesses 
within the protocols and especially the weak authentication procedure for 2G subscribers.
Recently, the EFF indicated that the Android OS will provide a feature allow to disable 2G within smartphones.
Certain large mobile operators have also started decommissioning their 2G network, 
or are planning to do so in the years to come.


## Licensing

The code contained in this repository is provided as is, without warranty, under the
[GNU AGPL v3](https://www.gnu.org/licenses/agpl-3.0.txt) software license.


## Usage

### Installation

As the code provided is to be used as a library, there is no system-wide installation process required.


### Python

The code was designed to work with Python 3.

In order to use the Python version of the algorithm, you can load the python file as a module and call
the `GEA1` or `GEA` class. Beware that argument passing follows uncommon conventions: inputs IV, 
direction and Key must be passed as integer values, and the `gen()` method takes a length
in bits for the requested keystream, before returning a list of bit values (0 or 1). Docstrings are 
provided within the modules.

Moreover, the `LFSR.dbg` attribute can be set in order to have the initialized values
present within registers printed.

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


Calling the `test_gea12.py` file as is will run the 3 test vectors for the 2 algorithms,
as a unittest module.

```
$ python3 ./test_gea12.py 
..
----------------------------------------------------------------------
Ran 2 tests in 0.020s

OK
```

Warning: this Python implementation is slow as hell! In the context of any work that requires producing
large keystreams, please use the Rust or C ones.


### C

The C implementation is provided under the `gea-c` sub-directory.
The implementation for GEA1 and GEA2 is available in the `gea12.c` and `gea12.h` files,
while the `gea12_test.c` implements the testing procedure with the 3 test vectors. A very simple 
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

Warning: the C code makes use of the `uint64_t` type for each LFSR register, hence it requires
a 64-bit machine. Moreover, it has been tested on a little-endian system only.


### Rust

The Rust implementation is provided under the `gea-rs` subdirectory.
It is an independant crate which contains standard documentation.

Examples of usage are present in the [`lib.rs`](gea-rs/src/lib.rs) file.
These examples are also tests, which can be run using the following command:

```console-session
cargo test --release
```


## How do those LFSR-based algorithms work?

It should first be noted that GEA-1 and GEA-2, which are very similar (GEA-2 is just 
an extension of GEA-1 with an higher amount of processing, and apparently not weakened) 
are bit-oriented stream ciphers.

A stream cipher, such as the well-known RC4 or GEA-1, usually works through using the Xor operation against a plaintext. 
The Xor operation being symmetrical, this means that encrypting should be considered the same 
operation as decrypting: GEA-1 and GEA-2 are basically pseudo-random data generators, 
taking a seed (the key, IV and direction bit of the GPRS data, which are concatenated), 
and the generated random data (the keystream) is xored with the clear-text data (the plaintext) 
for encrypting. Then, later, the keystream is xored with the encrypted data (the ciphertext) 
for decrypting. That is why the functions called in the target library for decrypting 
and encrypting are the same.

GEA-1 and GEA-2 are bit-oriented, unlike RC4 which is byte-oriented, because their 
algorithms generate only one bit of pseudo-random data at once (derived from their internal state), 
while algorithms like RC4 generate no less than one byte at once (in RC4's case, derived 
from permutation done in its internal state). Even though the keystream bits are put 
together by the current encryption / decryption C and Rust libraries into bytes in order to 
generate usable keystream, obviously.

Based on this, you can understand that GEA-1 and GEA-2 are LFSR: 
[Linear Feedback Shift Register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register)-oriented ciphers, 
because their internal state is stored into fixed-size registers. This includes the S and W 
registers which serve for initialization / key scheduling purposes and are respectively 
64 and 97-bit wide registers, and the A, B, C (and for GEA-2 only D) registers which serve 
for the purpose of keystream generation, which are respectively 31, 32, 33 and 29-bit wide 
registers.

On each iteration of the keystream generation, each register is 
[bit-wise rotated](https://en.wikipedia.org/wiki/Circular_shift) by one position, while the bit being rotated from 
the left towards the right side (or conversely depending on in which bit order you internally 
represent your registers) is fed back to the algorithm and mutated depending on given conditions.
Hence, the shifted-out bit is derived from other processing, and reinserted, while being for this reason possibly 
flipped depending on conditions depending on bits present at the other side of the given register. This is the explanation for 
the name of linear feedback shift register (shift because of the shift operation required 
for the rotation, and linear feedback because of the constant-time transform operation involved).

The rest of the register may also be mutated at each iteration steps, as in the case of the GEA-1 and 2, 
whole fixed Xor sequences (which differ for each register) may be applied depending on whether 
the rotated bit is a 0 or a 1.

Note that a step where the register iterates is called *clocking* (the register is *clocked*), 
and that the fixed points where the register may be Xor'ed when the rotated bit becomes a 1 are called *taps*.
The linear function which may transmute the rotated bit at the clocking step (taking several bits 
of the original register as an input) is called the *F function*.

Those kind of bit-oriented LFSR algorithms, such as GEA-1 and 2 (for GPRS) and A5/1 and 2 (for GSM),
were designed this way for optimal hardware implementations in the late 80's and early 90's.


## Other related projects

Airbus SecLab released an implementation for the cryptanalysis and key recovery attack against GEA-1,
[GEA1_break](https://github.com/airbus-seclab/GEA1_break), based on the initial cryptanalysis paper.

Older projects related to the cryptanalysis of the GSM A5/1 and A5/2 encryption algorithms:
- SRLabs [A5/1 Decryption](https://opensource.srlabs.de/projects/a51-decrypt/)
- brmlab hackerspace prague [Deka](https://brmlab.cz/project/gsm/deka/start)
- rblaze [a52crack](https://github.com/rblaze/a52crack)

