#!/usr/bin/env python3
# -*- coding: UTF-8 -*-
#/**
# * Software Name : test_gea12.py
# * Version : 0.1
# *
# * Copyright 2021. Benoit Michau. P1Sec.
# *
# * This program is free software: you can redistribute it and/or modify
# * it under the terms of the GNU Affero General Public License as published by
# * the Free Software Foundation, either version 3 of the License, or
# * (at your option) any later version.
# *
# * This program is distributed in the hope that it will be useful,
# * but WITHOUT ANY WARRANTY; without even the implied warranty of
# * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# * GNU Affero General Public License for more details.
# *
# * You should have received a copy of the GNU Affero General Public License
# * along with this program.  If not, see <http://www.gnu.org/licenses/>.
# *
# *--------------------------------------------------------
# * File Name : test_gea12.py
# * Created : 2022-03-25
# * Authors : Benoit Michau, Vadim Yanitskiy
# *--------------------------------------------------------
#*/

import unittest
import logging

from gea12 import bitlist_to_uint, byte_rev
from gea12 import LFSR, GEA1, GEA2

class TestGEA1(unittest.TestCase):
    TestVectors = []

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
        TestVectors.append(
            (iv, dir, key, cipher, plain)
        )

    def test_gea1(self):
        for i, (iv, dir, key, cipher, plain) in enumerate(self.TestVectors[:3]):
            with self.subTest(i):
                ks = bitlist_to_uint(GEA1(iv, dir, key).gen(144))
                # keystream byte-order needs to be reverted
                if LFSR.dbg:
                    logging.debug('Keystream: 0x%x', ks)
                self.assertEqual(byte_rev(plain, 18) ^ byte_rev(cipher, 18), ks)


class TestGEA2(unittest.TestCase):
    TestVectors = []

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
        TestVectors.append(
            (iv, dir, key, cipher, plain)
        )


    def test_gea2(self):
        for i, (iv, dir, key, cipher, plain) in enumerate(self.TestVectors[:3]):
            with self.subTest(i):
                ks = bitlist_to_uint(GEA2(iv, dir, key).gen(144))
                # keystream byte-order needs to be reverted
                if LFSR.dbg:
                    logging.debug('Keystream: 0x%x', ks)
                self.assertEqual(byte_rev(plain, 18) ^ byte_rev(cipher, 18), ks)


if __name__ == '__main__':
    unittest.main()
