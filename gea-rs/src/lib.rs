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
// File Name : lib.rs
// Created : 2021-10-19
// Authors : Marin Moulinier
//--------------------------------------------------------
//  
//  Implementation of the GPRS encryption algorithms GEA1 and GEA2
//  From the research paper:
//  https://eprint.iacr.org/2021/819.pdf
//-----------------------------------------------------------------------------/


pub mod registers {
    // Registrer used in GEA-1:
    pub mod s_register; // Initialization register

    // Registers used in GEA-1 and GEA-2:
    pub mod a_register;
    pub mod b_register;
    pub mod c_register;

    // Registers specific to GEA-1:
    pub mod w_register; // Initialization register
    pub mod d_register;
}

mod lfsr;
pub mod link_direction;
mod f_lookup_table;
pub mod gea1;
pub mod gea2;

#[cfg(test)]
mod tests {
    use crate::gea1::GEA1State;
    use crate::gea2::GEA2State;
    use crate::registers::s_register::SRegister;
    use crate::link_direction::LinkDirection;
    use crate::registers::w_register::WRegister;
    
    // Execute test vectors with both GEA-1 and GEA-2

    #[test]
    fn gea1_test_vectors() {
        
        assert_eq!(
            GEA1State::initialize(SRegister::initialize(0, 0, LinkDirection::Uplink))
                .generate_stream(144 / 8),
            [0x1f, 0xa1, 0x98, 0xab, 0x21, 0x14, 0xc3, 0x8a, 0x9e, 0xbc, 0xcb, 0x63, 0xad, 0x48, 0x13, 0xa7, 0x40, 0xc1]
        );
        
        assert_eq!(
            GEA1State::initialize(SRegister::initialize(0x55e303eb7d55b685, 0xda637a83, LinkDirection::Downlink))
                .crypt_stream(&[0x6e, 0x00, 0xcf, 0xe7, 0xb7, 0xfb, 0x97, 0x48, 0x92, 0xb8, 0xcd, 0xe5, 0xe4, 0x33, 0x63, 0x39, 0x7d, 0x85]),
            [0x58, 0xda, 0xd0, 0x64, 0x57, 0xb9, 0xfe, 0x10, 0x15, 0xda, 0x07, 0x76, 0xed, 0x19, 0x90, 0x7b, 0x78, 0x88]
        );
        
        assert_eq!(
            GEA1State::initialize(SRegister::initialize(0xa7265d1932a0d618, 0x0e9b8adf, LinkDirection::Uplink))
                .crypt_stream(&[0x96, 0xe7, 0xb1, 0xd9, 0x2b, 0x1e, 0xa8, 0xfc, 0xdd, 0xa4, 0x12, 0x33, 0xc6, 0x32, 0x94, 0x05, 0x53, 0x83]),
            [0xd7, 0x21, 0x97, 0xf6, 0x5d, 0x4d, 0x67, 0xb1, 0x4d, 0x2c, 0xee, 0x81, 0x2c, 0xb0, 0xb9, 0xbe, 0xa0, 0xc9]
        );
    } 
    
    #[test]
    fn gea2_test_vectors() {
        
        assert_eq!(
            GEA2State::initialize(WRegister::initialize(0, 0, LinkDirection::Uplink))
                .generate_stream(144 / 8),
            [0x04, 0x51, 0x15, 0xD5, 0xE5, 0xA2, 0xD6, 0x25, 0x41, 0xDA, 0x07, 0x8B, 0x18, 0xBA, 0xA5, 0x3F, 0xFE, 0x14]
        );
        
        assert_eq!(
            GEA2State::initialize(WRegister::initialize(0xb10f389b78a61648, 0x24c05b01, LinkDirection::Downlink))
                .crypt_stream(&[0xea, 0xbf, 0x6d, 0x3c, 0x6b, 0xa5, 0xdb, 0xf7, 0x6e, 0xbb, 0x3c, 0x4c, 0x0a, 0xc0, 0x24, 0x0c, 0xb0, 0xab]),
            [0x51, 0x56, 0x56, 0x9d, 0x2a, 0xb9, 0x82, 0x57, 0xbe, 0x1a, 0x37, 0xd6, 0x0d, 0xdf, 0x07, 0xae, 0x90, 0x75]
        );
        
        assert_eq!(
            GEA2State::initialize(WRegister::initialize(0x0c34b2940a9707fd, 0xf59cc96a, LinkDirection::Uplink))
                .crypt_stream(&[0xf9, 0x37, 0x3d, 0xe5, 0x2e, 0xa6, 0x2c, 0x49, 0x06, 0x97, 0x11, 0xe8, 0x33, 0x89, 0xd0, 0x37, 0xfc, 0x17]),
            [0x50, 0x9c, 0x19, 0xb7, 0x8d, 0x1d, 0x4c, 0xeb, 0x49, 0xc3, 0xb1, 0xf4, 0x3d, 0xf0, 0x14, 0xf7, 0x4c, 0xda]
        );
        
    }
}
