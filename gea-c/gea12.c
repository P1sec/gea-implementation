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
 * File Name : gea12.c
 * Created : 2021-10-19
 * Authors : Benoit Michau 
 *--------------------------------------------------------
  
  Implementation of the GPRS encryption algorithms GEA1 and GEA2
  From the research paper:
  https://eprint.iacr.org/2021/819.pdf
  
  This implementation has been developped for a 64 bits little-endian system
  complying with the C99 standard                                             */
/*----------------------------------------------------------------------------*/


#include "gea12.h"


// Boolean function f bidimensional lookup table
// 3 MSB {x5, x6, x7} and 4 LSB {x0, x1, x2, x3}
const uint64_t F_LUT[8][16] = {
    {0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1},
    {0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1},
    {1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1},
    {0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0},
    {0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1},
    {0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1},
    {0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0},
    {1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1},
};

// Boolean function f
static inline uint64_t
_F(uint64_t x) {
    return F_LUT[(x>>4)&0x7][x&0xf];
};


// GEA1 initialization S LFSR
static inline void
_lfsr_clock_S(uint64_t* Sreg, uint64_t bit) {
    
    // feedback input value to S
    uint64_t inp;
    inp = _F(
         ((*Sreg>>60)&1)       + 
        (((*Sreg>>51)&1) << 1) +
        (((*Sreg>>41)&1) << 2) + 
        (((*Sreg>>25)&1) << 3) + 
        (((*Sreg>>21)&1) << 4) + 
        (((*Sreg>> 8)&1) << 5) + 
         ((*Sreg     &1) << 6)
        ) ^ (*Sreg>>63) ^ bit;
    
    // shift S and insert input value
    *Sreg = (*Sreg<<1) + inp;
};


#define GEA_W_MSB_MASK 0x00000001ffffffff

// GEA2 initialization W LFSR
static inline void
_lfsr_clock_W(uint64_t Wreg[2], uint64_t bit) {
    
    // feedback input value to W
    uint64_t inp;
    inp = _F(
         ((Wreg[0]>>28)&1)       +
        (((Wreg[0]>>14)&1) << 1) +
        (((Wreg[1]>>63)&1) << 2) +
        (((Wreg[1]>>39)&1) << 3) +
        (((Wreg[1]>>33)&1) << 4) +
        (((Wreg[1]>>13)&1) << 5) +
         ((Wreg[1]     &1) << 6)
        ) ^ (Wreg[0]>>32) ^ bit;
    
    // shift W and insert input value
    Wreg[0] = ((Wreg[0] << 1) & GEA_W_MSB_MASK) + (Wreg[1]>>63);
    Wreg[1] = (Wreg[1] << 1) + inp;

};


// GEA A, B, C and D LFSRs for keystream generation
// LFSR length
#define GEA_A_LEN  31
#define GEA_B_LEN  32
#define GEA_C_LEN  33
#define GEA_D_LEN  29

// LFSR taps
#define GEA_A_TAPS 0x000000002C7646EE
#define GEA_B_TAPS 0x00000000510781C7
#define GEA_C_TAPS 0x00000000245F670A
#define GEA_D_TAPS 0x0000000009FD59A5

// LFSR outputs to F
const uint8_t GEA_A_FIN[7] = { 8, 30, 17,  9,  5, 28, 23};
const uint8_t GEA_B_FIN[7] = {19,  4, 31, 30,  2, 10, 26};
const uint8_t GEA_C_FIN[7] = {22,  2,  0, 29, 13, 32, 28};
const uint8_t GEA_D_FIN[7] = {16,  5, 25, 28, 18,  1, 11};


// LFSR clocking
static inline void 
_lfsr_clock(uint64_t* R, int Rlen, uint64_t Rtaps, uint64_t bit) {
    // after init state is introduced into R
    // `bit` just need to be set to 0 to clock the LFSR for keystream generation
    
    // LFSR mask
    uint64_t mask = ((1ULL<<Rlen)-1ULL);
    
    // feedback input to R
    uint64_t inp;
    inp = (*R>>(Rlen-1)) ^ bit;
    
    if (inp == 1) {
        // flip the taps
        *R ^= Rtaps;
    };
    
    // shift the LFSR and insert input value
    *R = ((*R<<1) + inp) & mask;
};


// LFSR output through F
static inline uint8_t
_lfsr_output(uint64_t R, const uint8_t* fin) {
    return _F(
         ((R>>fin[0])&1)       +
        (((R>>fin[1])&1) << 1) +
        (((R>>fin[2])&1) << 2) +
        (((R>>fin[3])&1) << 3) +
        (((R>>fin[4])&1) << 4) +
        (((R>>fin[5])&1) << 5) +
        (((R>>fin[6])&1) << 6)
        );
};


// GEA1 complete initialization procedure
void gea1_init(GEA1Ctx *ctx, GEAInput *in) {
    
    int i, j;
    uint64_t SB, SC;
    
    // 1.1) load in->iv into S
    for (i=3; i>=0; i--) {
        for (j=0; j<8; j++) {
            _lfsr_clock_S(&ctx->Sreg, (uint64_t) (in->iv[i]>>j)&1);
        };
    };
    
    // 1.2) load in->dir into S
    _lfsr_clock_S(&ctx->Sreg, (uint64_t) in->dir);
    
    // 1.3) load in->key into S
    for (i=7; i>=0; i--) {
        for (j=0; j<8; j++) {
            _lfsr_clock_S(&ctx->Sreg, (uint64_t) (in->key[i]>>j)&1);
        };
    };
    
    // 1.4) load 128 0 into S
    for (i=0; i<128; i++) {
        _lfsr_clock_S(&ctx->Sreg, 0);
    };
    
    // 2.1) load S into A, B, C
    SB = (ctx->Sreg<<16) + (ctx->Sreg>>48);
    SC = (ctx->Sreg<<32) + (ctx->Sreg>>32);
    for (i=63; i>=0; i--) {
       _lfsr_clock(&ctx->Areg, GEA_A_LEN, GEA_A_TAPS, (ctx->Sreg>>i)&1);
       _lfsr_clock(&ctx->Breg, GEA_B_LEN, GEA_B_TAPS, (SB>>i)&1);
       _lfsr_clock(&ctx->Creg, GEA_C_LEN, GEA_C_TAPS, (SC>>i)&1);
       
    };
    
    // 2.2) in case an LFSR is null, set its MSB
    if (ctx->Areg == 0) {
        ctx->Areg += 1ULL<<(GEA_A_LEN-1);
    };
    if (ctx->Breg == 0) {
        ctx->Breg += 1ULL<<(GEA_B_LEN-1);
    };
    if (ctx->Creg == 0) {
        ctx->Creg += 1ULL<<(GEA_C_LEN-1);
    };
};


// GEA1 keystream generation procedure
void gea1_gen(GEA1Ctx *ctx, GEAOutput *out) {
    
    int i, j;
    
    // clock all LFSRs as required for producing the keystream
    for (i = 0; i < out->len; i++) {
        // byte by byte
        for (j = 0; j < 8; j++) {
            // bit by bit
            // get LFSRs outputs
            out->ks[i] |= (
                _lfsr_output(ctx->Areg, GEA_A_FIN) ^
                _lfsr_output(ctx->Breg, GEA_B_FIN) ^
                _lfsr_output(ctx->Creg, GEA_C_FIN)
                ) << j;
            
            // clock all LFSRs
            _lfsr_clock(&ctx->Areg, GEA_A_LEN, GEA_A_TAPS, 0);
            _lfsr_clock(&ctx->Breg, GEA_B_LEN, GEA_B_TAPS, 0);
            _lfsr_clock(&ctx->Creg, GEA_C_LEN, GEA_C_TAPS, 0);
        };
    };
};


// main call to GEA1
void gea1(GEAInput *in, GEAOutput *out) {
    
    // init the algorithm internal state locally on the stack
    GEA1Ctx ctx;
    memset(&ctx.Sreg, 0, sizeof(ctx.Sreg));
    memset(&ctx.Areg, 0, sizeof(ctx.Areg));
    memset(&ctx.Breg, 0, sizeof(ctx.Breg));
    memset(&ctx.Creg, 0, sizeof(ctx.Creg));
    
    // schedule the input into the LFSRs
    gea1_init(&ctx, in);
    
    /*
    printf("S init: 0x%.16jx\n", ctx.Sreg);
    printf("A init: 0x%.16jx\n", ctx.Areg);
    printf("B init: 0x%.16jx\n", ctx.Breg);
    printf("C init: 0x%.16jx\n", ctx.Creg);
    */
    
    // generate the keystream
    gea1_gen(&ctx, out);

};


// GEA2 complete initialization procedure
void gea2_init(GEA2Ctx *ctx, GEAInput *in) {
    
    int i, j;
    uint64_t WA[2], WB[2], WC[2];
    
    // 1.1) load in->iv into W
    for (i=3; i>=0; i--) {
        for (j=0; j<8; j++) {
            _lfsr_clock_W(ctx->Wreg, (uint64_t) (in->iv[i]>>j)&1);
        };
    };
    
    // 1.2) load in->dir into W
    _lfsr_clock_W(ctx->Wreg, (uint64_t) in->dir);
    
    // 1.3) load in->key into W
    for (i=7; i>=0; i--) {
        for (j=0; j<8; j++) {
            _lfsr_clock_W(ctx->Wreg, (uint64_t) (in->key[i]>>j)&1);
        };
    };
    
    // 1.4) load 194 0 into W
    for (i=0; i<194; i++) {
        _lfsr_clock_W(ctx->Wreg, 0);
    };
    
    // 2.1) load W into A, B, C, D
    // A: 16 bits left rotation
    WA[0] = ((ctx->Wreg[0]<<16) + (ctx->Wreg[1]>>48)) & GEA_W_MSB_MASK;
    WA[1] = (ctx->Wreg[1]<<16) + (ctx->Wreg[0]>>17);
    // B: 33 bits left rotation
    WB[0] = (ctx->Wreg[1]>>31) & GEA_W_MSB_MASK;
    WB[1] = (ctx->Wreg[1]<<33) + ctx->Wreg[0];
    // C: 51 bits left rotation
    WC[0] = (ctx->Wreg[1]>>13) & GEA_W_MSB_MASK;
    WC[1] = (ctx->Wreg[1]<<51) + (ctx->Wreg[0]<<18) + (ctx->Wreg[1]>>46);
    
    for (i=32; i>=0; i--) {
       _lfsr_clock(&ctx->Areg, GEA_A_LEN, GEA_A_TAPS, (WA[0]>>i)&1);
       _lfsr_clock(&ctx->Breg, GEA_B_LEN, GEA_B_TAPS, (WB[0]>>i)&1);
       _lfsr_clock(&ctx->Creg, GEA_C_LEN, GEA_C_TAPS, (WC[0]>>i)&1);
       _lfsr_clock(&ctx->Dreg, GEA_D_LEN, GEA_D_TAPS, (ctx->Wreg[0]>>i)&1);
    };
    for (i=63; i>=0; i--) {
       _lfsr_clock(&ctx->Areg, GEA_A_LEN, GEA_A_TAPS, (WA[1]>>i)&1);
       _lfsr_clock(&ctx->Breg, GEA_B_LEN, GEA_B_TAPS, (WB[1]>>i)&1);
       _lfsr_clock(&ctx->Creg, GEA_C_LEN, GEA_C_TAPS, (WC[1]>>i)&1);
       _lfsr_clock(&ctx->Dreg, GEA_D_LEN, GEA_D_TAPS, (ctx->Wreg[1]>>i)&1);
    };
    
    // 2.2) in case an LFSR is null, set its MSB
    if (ctx->Areg == 0) {
        ctx->Areg += 1ULL<<(GEA_A_LEN-1);
    };
    if (ctx->Breg == 0) {
        ctx->Breg += 1ULL<<(GEA_B_LEN-1);
    };
    if (ctx->Creg == 0) {
        ctx->Creg += 1ULL<<(GEA_C_LEN-1);
    };
    if (ctx->Dreg == 0) {
        ctx->Dreg += 1ULL<<(GEA_D_LEN-1);
    };
};


// GEA1 keystream generation procedure
void gea2_gen(GEA2Ctx *ctx, GEAOutput *out) {
    
    int i, j;
    
    // clock all LFSRs as required for producing the keystream
    for (i = 0; i < out->len; i++) {
        // byte by byte
        for (j = 0; j < 8; j++) {
            // bit by bit
            // get LFSRs outputs
            out->ks[i] |= (
                _lfsr_output(ctx->Areg, GEA_A_FIN) ^
                _lfsr_output(ctx->Breg, GEA_B_FIN) ^
                _lfsr_output(ctx->Creg, GEA_C_FIN) ^
                _lfsr_output(ctx->Dreg, GEA_D_FIN)
                ) << j;
            
            // clock all LFSRs
            _lfsr_clock(&ctx->Areg, GEA_A_LEN, GEA_A_TAPS, 0);
            _lfsr_clock(&ctx->Breg, GEA_B_LEN, GEA_B_TAPS, 0);
            _lfsr_clock(&ctx->Creg, GEA_C_LEN, GEA_C_TAPS, 0);
            _lfsr_clock(&ctx->Dreg, GEA_D_LEN, GEA_D_TAPS, 0);
        };
    };
};


// main call to GEA2
void gea2(GEAInput *in, GEAOutput *out) {
    
    // init the algorithm internal state locally on the stack
    GEA2Ctx ctx;
    memset(ctx.Wreg,  0, sizeof(ctx.Wreg));
    memset(&ctx.Areg, 0, sizeof(ctx.Areg));
    memset(&ctx.Breg, 0, sizeof(ctx.Breg));
    memset(&ctx.Creg, 0, sizeof(ctx.Creg));
    memset(&ctx.Dreg, 0, sizeof(ctx.Dreg));
    
    // schedule the input into the LFSRs
    gea2_init(&ctx, in);
    
    /*
    printf("W init: 0x%.16jx%.16jx\n", ctx.Wreg[0], ctx.Wreg[1]);
    printf("A init: 0x%.16jx\n", ctx.Areg);
    printf("B init: 0x%.16jx\n", ctx.Breg);
    printf("C init: 0x%.16jx\n", ctx.Creg);
    printf("D init: 0x%.16jx\n", ctx.Dreg);
    */
    
    // generate the keystream
    gea2_gen(&ctx, out);

};

