# GEA-1 and 2 implementations (in Python, C and Rust)

This repository contains software implementations of the **GPRS Encryption 
Algorithm 1 and 2**. The ["*Cryptanalysis of the GPRS Encryption Algorithms GEA-1
and GEA-2*"](https://eprint.iacr.org/2021/819.pdf) research paper provides the
complete description of both algorithms, and an efficient cryptanalysis against 
GEA-1 (allowing to weaken the key strength to 40 bits instead of 64).

Both GEA-1 and GEA-2 are stream-cipher based on 3, and respectively 4, LFSRs and a boolean
function.

## Licensing

The code contained in this repository is dual-licensed. Non-commercial use can be performed through respecting the terms of the [GNU AGPL v3](https://www.gnu.org/licenses/agpl-3.0.txt) software license. Please [contact P1 Security](https://www.p1sec.com/corp/contact/) for commercial use.

## How does the algorithms work

### How does a LFSR (Linear Feedback Shift Register)-based cipher works?

It should first be noted that GEA-1 and GEA-2, which are very similar (GEA-1 is just an extension of GEA-2 with an higher amount of processing) are **bit-oriented stream ciphers**.

A **stream cipher**, like the well-known RC4 or GEA-1, works by **the Xor operation**. Also, the Xor operation being symmetrical, this means that **encrypting is the same operation as decrypting**: GEA-1 and GEA-2 are basically pseudo-random data generators, taking a seed (the key, IV and direction bit of the GPRS data, which are concatenated), and **the generated random data (the keystream) is Xor'd with the clear-text data (the plaintext) for encrypting**. Then, later, **the keystream is Xor'd with the encrypted data (the ciphertext) for decrypting)**. That is why the functions called in the target library or hardware processor (GEA-1 and GEA-2 were made to be implemented in hardware, even though these are pretty weak for today's hardware) for decrypting and encrypting are same.

GEA-2 and GEA-1 are **bit-oriented**, unlike RC4 which is **byte-oriented**, because their algorithms generate only one bit of pseudo-random data at once (derived from their internal state), while algorithms like RC4 generate no less than one byte at once (in RC4's case, derived from permutation done in its internal state). Even though the keystream bits are put togheter by the current encryption/decryption library into bytes in order to generate usable keystream, obviously.

Now, you can understand that GEA-1 and GEA-2 are **LFSR ([Linear Feedback Shift Register](https://en.wikipedia.org/wiki/Linear-feedback_shift_register))**-oriented ciphers, because their internal state is stored into **fixed-size registers** (including the S and W registers which serve from initialization/key scheduling purposes and are respectively 64 and 97-bit wide registers, and the A, B, C and for GEA-2 only D registers which serve from the purpose of keystream generation, which are respectively 31, 32, 33 and 29-bit wide registers).

At each iteration of the keystream generation, each register is **[bit-wise rotated](https://en.wikipedia.org/wiki/Circular_shift)** of one position, and the **bit being rotated from the left towards the right side** (or conversely depending on in which bit order you internally represent your registers) is being **fed to the algorithm and mutated depending on given conditions**, hence the shifted out bit is derived from something and reinserted while being possibly flipped depending on conditions at the other side of the given register. That is why the name of **linear feedback shift** register (shift because of the shift operation required for the rotation, and linear feedback because of the constant-time transform operation involved).

The rest of the register may also be mutated at each iteration steps, as in the case of the GEA-1 and 2, whole fixed Xor sequences (which differ for each register) may be applied depending on whether the rotated bit is a 0 or a 1.

Note that a step where the register iterates is called **clocking** (the register is *clocked*), and that the fixed points where the register may be Xor'ed when the rotated bit becomes a 1 are called **taps**.

The linear function which may transmute the rotated bit at the clocking step (taking several bits of the original register as an input) is called the **F function**.
