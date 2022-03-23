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
// File Name : lfsr.rs
// Created : 2021-10-19
// Authors : Marin Moulinier
//--------------------------------------------------------
//  
//  Implementation of the GPRS encryption algorithms GEA1 and GEA2
//  From the research paper:
//  https://eprint.iacr.org/2021/819.pdf
//-----------------------------------------------------------------------------/


pub trait LinearFeedbackShiftRegister {
    /// Initially clock the LFSR using successively all the bits passed
    /// into the concerned input register, the lowest bit of the
    /// "register_data" integer first. Can be called with up to 128
    /// bits of data at once.
    fn initial_clock(&mut self, mut register_data: u128, register_size: usize) {
        for _bit_pos in 0..register_size {
            self.clock(Some((register_data & 1) != 0));
            register_data >>= 1;
        }
    }
    
    /// Make a clock tick over the current register.
    ///
    /// In other words, make a one-bit right rotate over the internal-state
    /// integer representing the LFSR, possibly making other side transforms
    /// other the register such as xor'ing bits or flipping the rotated bit.
    ///
    /// Possibly pass an extra bit that should be injected into the LSFR.
    fn clock(&mut self, input_bit: Option<bool>);
    
    /// Run the "f" function from the GEA1/2 specification (outputting a
    /// single bit from six bits of the register) over the current LFSR.
    fn f_function(&mut self) -> bool;
}
