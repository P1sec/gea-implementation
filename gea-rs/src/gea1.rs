//-----------------------------------------------------------------------------/
// Software Name : gea12
// Version : 0.1
//
// Copyright 2021. Marin Moulinier. P1Sec.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
//--------------------------------------------------------
// File Name : gea1.rs
// Created : 2021-10-19
// Authors : Marin Moulinier
//--------------------------------------------------------
//  
//  Implementation of the GPRS encryption algorithms GEA1 and GEA2
//  From the research paper:
//  https://eprint.iacr.org/2021/819.pdf
//-----------------------------------------------------------------------------/


use crate::registers::s_register::SRegister;
use crate::registers::a_register::ARegister;
use crate::registers::b_register::BRegister;
use crate::registers::c_register::CRegister;
use crate::lfsr::LinearFeedbackShiftRegister;

pub struct GEA1State {
    pub a_register: ARegister,
    pub b_register: BRegister,
    pub c_register: CRegister,
    pub s_register: SRegister
}

impl GEA1State {
    /// Create a new GEA1 encryption/decryption object
    /// from a S Register object, which has been
    /// prealably initialized from a key, IV and
    /// direction bit.
    pub fn initialize(s_register: SRegister) -> Self {
        Self {
            a_register: ARegister::initialize(s_register.0, 64),
            b_register: BRegister::initialize((s_register.0 >> 16) |
                ((s_register.0 & ((1 << 16) - 1)) << (64 - 16)), 64),
            c_register: CRegister::initialize((s_register.0 >> 32) |
                ((s_register.0 & ((1 << 32) - 1)) << 32), 64),
            s_register: s_register
        }
    }
    
    /// Encrypt or decrypt a stream of data in a cipher
    /// stream fashion, xor'ing a byte of ciphertext or
    /// plaintext at once with keystream bytes.
    pub fn crypt_stream(&mut self, stream: &[u8]) -> Vec<u8> {
        let keystream = self.generate_stream(stream.len());
        let mut outstream: Vec<u8> = vec![];
        
        for byte_pos in 0..stream.len() {
            outstream.push(stream[byte_pos] ^ keystream[byte_pos]);
        }
        outstream
    }
    
    /// Generate an arbitrary quantity of keystream from the
    /// internal state of the current GEA1 object.
    pub fn generate_stream(&mut self, num_bytes: usize) -> Vec<u8> {
        let mut keystream: Vec<u8> = vec![];
        
        for _num_byte in 0..num_bytes {
            let mut new_byte = 0;
            for num_bit in 0..8 {
                new_byte |= (self.a_register.f_function() as u8 ^
                    self.b_register.f_function() as u8 ^
                    self.c_register.f_function() as u8) << num_bit;
                    
                self.a_register.clock(None);
                self.b_register.clock(None);
                self.c_register.clock(None);
            }
            keystream.push(new_byte);
        }
        
        keystream
    }
}
