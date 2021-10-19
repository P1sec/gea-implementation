/*----------------------------------------------------------------------------*/
/* Software Name : gea12
 * Version : 0.1
 *
 * Copyright 2021. Benoit Michau. P1Sec.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 *--------------------------------------------------------
 * File Name : gea12.h
 * Created : 2021-10-19
 * Authors : Benoit Michau 
 *--------------------------------------------------------
  
  Implementation of the GPRS encryption algorithms GEA1 and GEA2
  From the research paper:
  https://eprint.iacr.org/2021/819.pdf
  
  This implementation has been developped for a 64 bits little-endian system
  complying with the C99 standard                                             */
/*----------------------------------------------------------------------------*/


// Basic system / libc libraries 
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
//#include <stdio.h>


// Algorithms input, has to be initialized by the caller
typedef struct {
    uint8_t iv[4];  // 32 bits
    uint8_t dir;    // 1 bit: uplink / downlink
    uint8_t key[8]; // 64 bits
} GEAInput ;


// Algorithms output, has to be initialized by the caller
// .len must also be filled in by the caller
typedef struct {
    int      len; // keystream length in bytes
    uint8_t* ks;  // keystream
} GEAOutput ;


// Algorithms internal states
typedef struct {
    // LFSR internal states
    uint64_t Sreg; // S register for initialization, 64 bits
    uint64_t Areg; // A register for keystream generation, 31 bits
    uint64_t Breg; // B register for keystream generation, 32 bits
    uint64_t Creg; // C register for keystream generation, 33 bits
} GEA1Ctx ;


typedef struct {
    // LFSR internal states
    uint64_t Wreg[2]; // W register for initialization, 97 bits (33 MSB || 64 LSB)
    uint64_t Areg;    // A register for keystream generation, 31 bits
    uint64_t Breg;    // B register for keystream generation, 32 bits
    uint64_t Creg;    // C register for keystream generation, 33 bits
    uint64_t Dreg;    // D register for keystream generation, 29 bits
} GEA2Ctx ;


// exported function prototypes
// GEA1
void gea1(GEAInput *in, GEAOutput *out);
void gea1_init(GEA1Ctx *ctx, GEAInput *in);
void gea1_gen(GEA1Ctx *ctx, GEAOutput *out);
// GEA2
void gea2(GEAInput *in, GEAOutput *out);
void gea2_init(GEA2Ctx *ctx, GEAInput *in);
void gea2_gen(GEA2Ctx *ctx, GEAOutput *out);

