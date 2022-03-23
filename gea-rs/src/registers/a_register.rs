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
// File Name : registers/a_register.rs
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

/// The A register is initialized from the S or W register, and will
/// contribute to generate the output of the script.
pub struct ARegister(pub u64);

impl ARegister {
    pub fn initialize(vector_data: u128, vector_size: usize) -> Self {
        let mut a_register = Self(0);
        a_register.initial_clock(vector_data, vector_size);
        if a_register.0 == 0 {
            a_register.0 = 1;
        }
        a_register
    }
}

impl LinearFeedbackShiftRegister for ARegister {
    fn clock(&mut self, input_bit: Option<bool>) {
        let bit = input_bit.unwrap_or(false) as u64;
        self.0 = (self.0 >> 1) |
            (((self.0 & 1) ^ bit) << 30);
        
        if self.0 >> 30 == 1 {
            self.0 ^= 0b11101110110001001101110001101;
        }
    }
    
    fn f_function(&mut self) -> bool {
        F_LOOKUP_TABLE[(
            ((self.0 >> (22 - 0)) & 0b000_0001) |
            ((self.0 << (1 - 0)) & 0b000_0010) |
            ((self.0 >> (13 - 2)) & 0b000_0100) |
            ((self.0 >> (21 - 3)) & 0b000_1000) |
            ((self.0 >> (25 - 4)) & 0b001_0000) |
            ((self.0 << (5 - 2)) & 0b010_0000) |
            ((self.0 >> (7 - 6)) & 0b100_0000)
        ) as usize]
    }
}
