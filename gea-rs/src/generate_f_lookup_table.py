#!/usr/bin/python3
#-*- encoding: Utf-8 -*-

#/**
# * Software Name : gea12
# * Version : 0.1
# *
# * Copyright 2021. Marin Moulinier. P1Sec.
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
# * File Name : generate_f_lookup_table.py
# * Created : 2021-10-19
# * Authors : Marin Moulinier
# *--------------------------------------------------------
#*/


"""
    This script was used to generate the original contents
    of the "f_lookup_table.rs" file
"""

def f(x0, x1, x2, x3, x4, x5, x6):
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

print('\n'.join(
    '    %s,' % ('true' if f(
        i & 1,
        (i >> 1) & 1,
        (i >> 2) & 1,
        (i >> 3) & 1,
        (i >> 4) & 1,
        (i >> 5) & 1,
        (i >> 6) & 1,
    ) else 'false')
    for i in range(128)
))
