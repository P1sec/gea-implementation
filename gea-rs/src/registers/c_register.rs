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
// File Name : registers/c_register.rs
// Created : 2021-10-19
// Authors : Marin Moulinier
//--------------------------------------------------------
//  
//  Implementation of the GPRS encryption algorithms GEA1 and GEA2
//  From the research paper:
//  https://eprint.iacr.org/2021/819.pdf
//-----------------------------------------------------------------------------/


use crate::lfsr::LinearFeedbackShiftRegister;
use crate::f_lookup_table::F_LOOKUP_TABLE;

/// The C register is initialized from the S or W register, and will
/// contribute to generate the output of the script.
pub struct CRegister(pub u64);

impl CRegister {
    pub fn initialize(vector_data: u128, vector_size: usize) -> Self {
        let mut c_register = Self(0);
        c_register.initial_clock(vector_data, vector_size);
        if c_register.0 == 0 {
            c_register.0 = 1;
        }
        c_register
    }
}

impl LinearFeedbackShiftRegister for CRegister {
    fn clock(&mut self, input_bit: Option<bool>) {
        let bit = input_bit.unwrap_or(false) as u64;
        self.0 = (self.0 >> 1) |
            (((self.0 & 1) ^ bit) << 32);
        
        if self.0 >> 32 == 1 {
            self.0 ^= 0b1010000111001101111101000100100;
        }
    }
    
    fn f_function(&mut self) -> bool {
        F_LOOKUP_TABLE[(
            ((self.0 >> (10 - 0)) & 0b000_0001) |
            ((self.0 >> (30 - 1)) & 0b000_0010) |
            ((self.0 >> (32 - 2)) & 0b000_0100) |
            ((self.0 >> (3 - 3)) & 0b000_1000) |
            ((self.0 >> (19 - 4)) & 0b001_0000) |
            ((self.0 << (5 - 0)) & 0b010_0000) |
            ((self.0 << (6 - 4)) & 0b100_0000)
        ) as usize]
    }
}
