#!/usr/bin/python3
#-*- encoding: Utf-8 -*-

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
