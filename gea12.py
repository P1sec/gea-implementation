#!/usr/bin/env python3
# -*- coding: UTF-8 -*-

# GPRS encryption algorithm 1 and 2
# implemented from the excellent research paper on its cryptanalysis:
#   https://eprint.iacr.org/2021/819.pdf

from functools import reduce


#------------------------------------------------------------------------------#
# formatting routines
#------------------------------------------------------------------------------#

def uint_to_bitlist(uint, bitlen):
    """Convert a big-endian unsigned int `uint` of length `bitlen` to a list of bits
    """
    bl = bin(uint)[2:]
    if len(bl) < bitlen:
        # extend v
        bl = '0'*(bitlen-len(bl)) + bl
    return [0 if b == '0' else 1 for b in bl]


def bitlist_to_uint(bl):
    """Convert a list of bits `bl` to a big-endian unsinged integer
    """
    return reduce(lambda x, y: (x<<1)+y, bl)


def byte_rev(uint, l=None):
    """revert byte order in a big endian unsigned integer
    """
    if l is None:
        bl = uint.bit_length()
        l  = bl // 8
        if bl % 8:
            l += 1
    #
    b  = []
    for i in range(0, l):
        b.append( uint & 0xff )
        uint >>= 8
    return reduce(lambda x, y: (x<<8)+y, b)


class LFSR(object):
    """parent class for all LFSR
    """
    # global debugging level (0, 1 or 2)
    dbg = 1


#------------------------------------------------------------------------------#
# f boolean function
#------------------------------------------------------------------------------#

def f(x0, x1, x2, x3, x4, x5, x6):
    """Boolean function f on seven variables of degree 4
    
    section 2.1:
    x0x2x5x6 + x0x3x5x6 + x0x1x5x6 + x1x2x5x6 + x0x2x3x6 + x1x3x4x6
    + x1x3x5x6 + x0x2x4 + x0x2x3 + x0x1x3 + x0x2x6 + x0x1x4 + x0x1x6
    + x1x2x6 + x2x5x6 + x0x3x5 + x1x4x6 + x1x2x5 + x0x3 + x0x5 + x1x3
    + x1x5 + x1x6 + x0x2 + x1 + x2x3 + x2x5 + x2x6 + x4x5 + x5x6 + x2 + x3 + x5
    """
    return (
        x0*x2*x5*x6 ^ x0*x3*x5*x6 ^ x0*x1*x5*x6 ^ x1*x2*x5*x6 ^ \
        x0*x2*x3*x6 ^ x1*x3*x4*x6 ^ x1*x3*x5*x6 ^ \
        x0*x2*x4 ^ x0*x2*x3 ^ x0*x1*x3 ^ x0*x2*x6 ^ \
        x0*x1*x4 ^ x0*x1*x6 ^ x1*x2*x6 ^ x2*x5*x6 ^ \
        x0*x3*x5 ^ x1*x4*x6 ^ x1*x2*x5 ^ \
        x0*x3 ^ x0*x5 ^ x1*x3 ^ x1*x5 ^ \
        x1*x6 ^ x0*x2 ^ x2*x3 ^ x2*x5 ^ \
        x2*x6 ^ x4*x5 ^ x5*x6 ^ \
        x1 ^ x2 ^ x3 ^ x5
        )


#------------------------------------------------------------------------------#
# S LFSR for initialization
#------------------------------------------------------------------------------#

class S(LFSR):
    """64 bits S register used for initialization
    """
    
    def __init__(self, iv, dir, key):
        """Initialize the S LFSR [64 bits] 
        
        Args:
            iv  : 32 bits as uint32_t (big endian)
            dir : 1 bit (LSB), uint8_t
            key : 64 bits, uint64_t (big endian)
        """
        self.IN  = 128 * [0] + \
                   uint_to_bitlist(key, 64) + \
                   uint_to_bitlist(dir,  1) + \
                   uint_to_bitlist(iv,  32)
        self.R   = 64 * [0]
        self.clk = 0
    
    def load(self):   
        while self.IN:
            self.clock()
        if self.dbg > 1:
            print('S init: 0x%.16x' % bitlist_to_uint(self.R))
    
    def clock(self):
        # compute input bit
        inp = self.R[0] ^ self.f() ^ self.IN.pop()
        # shift LFSR
        self.R = self.R[1:] + [inp]
        self.clk += 1
    
    def f(self):
         return f(
            self.R[3],
            self.R[12],
            self.R[22],
            self.R[38],
            self.R[42],
            self.R[55],
            self.R[63],
            )


#------------------------------------------------------------------------------#
# LFSRs for keystream generation in GEA1
#------------------------------------------------------------------------------#

class A(LFSR):
    """31 bits A register used for keystream generation
    """
    
    def __init__(self, IN):
        self.IN  = IN
        self.R   = 31 * [0]
        self.clk = 0
    
    def load(self):
        while self.IN:
            self.clock(self.IN.pop())
        if all([b == 0 for b in self.R]):
            self.R[0] = 1
        if self.dbg > 1:
            print('A init: 0x%.16x' % bitlist_to_uint(self.R))
    
    def clock(self, inp=None):
        if inp is not None:
            R0 = self.R[0] ^ inp
        else:
            R0 = self.R[0]
        # feedback
        if R0:
            self.R[1]  ^= R0
            self.R[3]  ^= R0
            self.R[4]  ^= R0
            self.R[8]  ^= R0
            self.R[9]  ^= R0
            self.R[10] ^= R0
            self.R[12] ^= R0
            self.R[13] ^= R0
            self.R[16] ^= R0
            self.R[20] ^= R0
            self.R[21] ^= R0
            self.R[23] ^= R0
            self.R[24] ^= R0
            self.R[25] ^= R0
            self.R[27] ^= R0
            self.R[28] ^= R0
            self.R[29] ^= R0
        # shift
        self.R = self.R[1:] + [R0]
        self.clk += 1
    
    def f(self):
        return f(
            self.R[22],
            self.R[0],
            self.R[13],
            self.R[21],
            self.R[25],
            self.R[2],
            self.R[7],
            )


class B(LFSR):
    """32 bits B register used for keystream generation
    """
    
    def __init__(self, IN):
        self.IN  = IN
        self.R   = 32 * [0]
        self.clk = 0
    
    def load(self):
        while self.IN:
            self.clock(self.IN.pop())
        if all([b == 0 for b in self.R]):
            self.R[0] = 1
        if self.dbg > 1:
            print('B init: 0x%.16x' % bitlist_to_uint(self.R))
    
    def clock(self, inp=None):
        if inp is not None:
            R0 = self.R[0] ^ inp
        else:
            R0 = self.R[0]
        # feedback
        if R0:
            self.R[1]  ^= R0
            self.R[3]  ^= R0
            self.R[7]  ^= R0
            self.R[13] ^= R0
            self.R[14] ^= R0
            self.R[15] ^= R0
            self.R[16] ^= R0
            self.R[23] ^= R0
            self.R[24] ^= R0
            self.R[25] ^= R0
            self.R[29] ^= R0
            self.R[30] ^= R0
            self.R[31] ^= R0
        # shift
        self.R = self.R[1:] + [R0]
        self.clk += 1
    
    def f(self):
        return f(
            self.R[12],
            self.R[27],
            self.R[0],
            self.R[1],
            self.R[29],
            self.R[21],
            self.R[5],
            )


class C(LFSR):
    """33 bits C register used for keystream generation
    """
    
    def __init__(self, IN):
        self.IN  = IN
        self.R   = 33 * [0]
        self.clk = 0
    
    def load(self):
        while self.IN:
            self.clock(self.IN.pop())
        if all([b == 0 for b in self.R]):
            self.R[0] = 1
        if self.dbg > 1:
            print('C init: 0x%.16x' % bitlist_to_uint(self.R))
    
    def clock(self, inp=None):
        if inp is not None:
            R0 = self.R[0] ^ inp
        else:
            R0 = self.R[0]
        # feedback
        if R0:
            self.R[3]  ^= R0
            self.R[6]  ^= R0
            self.R[10] ^= R0
            self.R[12] ^= R0
            self.R[13] ^= R0
            self.R[14] ^= R0
            self.R[15] ^= R0
            self.R[16] ^= R0
            self.R[18] ^= R0
            self.R[19] ^= R0
            self.R[22] ^= R0
            self.R[23] ^= R0
            self.R[24] ^= R0
            self.R[29] ^= R0
            self.R[31] ^= R0
        # shift
        self.R = self.R[1:] + [R0]
        self.clk += 1
    
    def f(self):
        return f(
            self.R[10],
            self.R[30],
            self.R[32],
            self.R[3],
            self.R[19],
            self.R[0],
            self.R[4],
            )


#------------------------------------------------------------------------------#
# Additional LFSRs for initialization and keystream generation in GEA2
#------------------------------------------------------------------------------#

class W(LFSR):
    """97 bits W register used for initialization in GEA2
    """
    
    def __init__(self, iv, dir, key):
        """Initialize the W LFSR [97 bits] 
        
        Args:
            iv  : 32 bits as uint32_t (big endian)
            dir : 1 bit (LSB), uint8_t
            key : 64 bits, uint64_t (big endian)
        """
        self.IN  = 194 * [0] + \
                   uint_to_bitlist(key, 64) + \
                   uint_to_bitlist(dir,  1) + \
                   uint_to_bitlist(iv,  32)
        self.R   = 97 * [0]
        self.clk = 0
    
    def load(self):   
        while self.IN:
            self.clock()
        if self.dbg > 1:
            print('W init: 0x%.32x' % bitlist_to_uint(self.R))
    
    def clock(self):
        # compute input bit
        inp = self.R[0] ^ self.f() ^ self.IN.pop()
        # shift LFSR
        self.R = self.R[1:] + [inp]
        self.clk += 1
    
    def f(self):
         return f(
            self.R[4],
            self.R[18],
            self.R[33],
            self.R[57],
            self.R[63],
            self.R[83],
            self.R[96],
            )


class D(LFSR):
    """29 bits D register used for keystream generation in GEA2
    """
    
    def __init__(self, IN):
        self.IN  = IN
        self.R   = 29 * [0]
        self.clk = 0
    
    def load(self):
        while self.IN:
            self.clock(self.IN.pop())
        if all([b == 0 for b in self.R]):
            self.R[0] = 1
        if self.dbg > 1:
            print('D init: 0x%.16x' % bitlist_to_uint(self.R))
    
    def clock(self, inp=None):
        if inp is not None:
            R0 = self.R[0] ^ inp
        else:
            R0 = self.R[0]
        # feedback
        if R0:
            self.R[1]  ^= R0
            self.R[4]  ^= R0
            self.R[5]  ^= R0
            self.R[6]  ^= R0
            self.R[7]  ^= R0
            self.R[8]  ^= R0
            self.R[9]  ^= R0
            self.R[10] ^= R0
            self.R[12] ^= R0
            self.R[14] ^= R0
            self.R[16] ^= R0
            self.R[17] ^= R0
            self.R[20] ^= R0
            self.R[21] ^= R0
            self.R[23] ^= R0
            self.R[26] ^= R0
            self.R[28] ^= R0
        # shift
        self.R = self.R[1:] + [R0]
        self.clk += 1
    
    def f(self):
        return f(
            self.R[12],
            self.R[23],
            self.R[3],
            self.R[0],
            self.R[10],
            self.R[27],
            self.R[17],
            )


#------------------------------------------------------------------------------#
# GEA
#------------------------------------------------------------------------------#

class GEA1(object):
    """GPRS Encryption Algorithm 1
    
    Warning: this is a highly insecure encryption algorithm, providing only 40
    bits of security from a key of 64 bits.
    This algorithm should not be supported neither used in production systems
    """
    
    def __init__(self, iv, dir, key):
        self._iv, self._dir, self._key = iv, dir, key
        # initialization phase
        self.S = S(iv, dir, key)
        self.S.load()
        self.A = A(list(reversed(self.S.R)))
        self.A.load()
        self.B = B(list(reversed(self.S.R[16:] + self.S.R[:16])))
        self.B.load()
        self.C = C(list(reversed(self.S.R[32:] + self.S.R[:32])))
        self.C.load()
    
    def gen(self, bl):
        # keystream generation phase
        self.K = []
        for i in range(bl):
            self.K.append(self.A.f() ^ self.B.f() ^ self.C.f())
            self.A.clock()
            self.B.clock()
            self.C.clock()
        return list(reversed(self.K))


class GEA2(object):
    """GPRS Encryption Algorithm 2
    """
    
    def __init__(self, iv, dir, key):
        self._iv, self._dir, self._key = iv, dir, key
        # initialization phase
        self.W = W(iv, dir, key)
        self.W.load()
        self.D = D(list(reversed(self.W.R)))
        self.D.load()
        self.A = A(list(reversed(self.W.R[16:] + self.W.R[:16])))
        self.A.load()
        self.B = B(list(reversed(self.W.R[33:] + self.W.R[:33])))
        self.B.load()
        self.C = C(list(reversed(self.W.R[51:] + self.W.R[:51])))
        self.C.load()
    
    def gen(self, bl):
        # keystream generation phase
        self.K = []
        for i in range(bl):
            self.K.append(self.A.f() ^ self.B.f() ^ self.C.f() ^ self.D.f())
            self.A.clock()
            self.B.clock()
            self.C.clock()
            self.D.clock()
        return list(reversed(self.K))


#------------------------------------------------------------------------------#
# test vectors
#------------------------------------------------------------------------------#

TestVectorsGEA1 = []

for cipher, plain, (key, iv, dir) in zip(
    [
        0x1FA198AB2114C38A9EBCCB63AD4813A740C1,
        0x58dad06457b9fe1015da0776ed19907b7888,
        0xd72197f65d4d67b14d2cee812cb0b9bea0c9
    ],
    [
        0x000000000000000000000000000000000000,
        0x6e00cfe7b7fb974892b8cde5e43363397d85,
        0x96e7b1d92b1ea8fcdda41233c63294055383
    ],
    [
        (0x0000000000000000, 0x00000000, 0),
        (0x55e303eb7d55b685, 0xda637a83, 1),
        (0xa7265d1932a0d618, 0x0e9b8adf, 0)
    ]
):
    TestVectorsGEA1.append(
        (iv, dir, key, cipher, plain)
    )


def test_gea1():
    for i, (iv, dir, key, cipher, plain) in enumerate(TestVectorsGEA1[:3]):
        ks = bitlist_to_uint(GEA1(iv, dir, key).gen(144))
        # ks byte-order need to be reverted
        print('DEBUG MMR 18OCT2021 (GEA1): %r => %r' % (
            hex(plain),
            hex(byte_rev(byte_rev(plain, 18) ^ ks, 18))
        )) # DBEUG
        if LFSR.dbg:
            print('Keystream: 0x%x' % ks)
        if byte_rev(plain, 18) ^ byte_rev(cipher, 18) == ks:
            print('GEA1 test vector %i: OK :)' % i)
        else:
            print('GEA1 test vector %i: NOK' % i)

TestVectorsGEA2 = []

for cipher, plain, (key, iv, dir) in zip(
    [
        0x045115d5e5a2d62541da078b18baa53ffe14,
        0x5156569d2ab98257be1a37d60ddf07ae9075,
        0x509c19b78d1d4ceb49c3b1f43df014f74cda
    ],
    [
        0x000000000000000000000000000000000000,
        0xeabf6d3c6ba5dbf76ebb3c4c0ac0240cb0ab,
        0xf9373de52ea62c49069711e83389d037fc17
    ],
    [
        (0x0000000000000000, 0x00000000, 0),
        (0xb10f389b78a61648, 0x24c05b01, 1),
        (0x0c34b2940a9707fd, 0xf59cc96a, 0)
    ]
):
    TestVectorsGEA2.append(
        (iv, dir, key, cipher, plain)
    )


def test_gea2():
    for i, (iv, dir, key, cipher, plain) in enumerate(TestVectorsGEA2[:3]):
        ks = bitlist_to_uint(GEA2(iv, dir, key).gen(144))
        #ks = bitlist_to_uint(GEA1(byte_rev(iv, 4), dir, byte_rev(key, 8), _clkbefore=True).gen(144))
        # ks byte-order need to be reverted
        if LFSR.dbg:
            print('Keystream: 0x%x' % ks)
        print('DEBUG MMR 18OCT2021: %r => %r' % (
            hex(plain),
            hex(byte_rev(byte_rev(plain, 18) ^ ks, 18))
        )) # DBEUG
        if byte_rev(plain, 18) ^ byte_rev(cipher, 18) == ks:
            print('GEA2 test vector %i: OK :)' % i)
        else:
            print('GEA2 test vector %i: NOK' % i)


if __name__ == '__main__':
    test_gea1()
    test_gea2()

