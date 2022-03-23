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
// File Name : f_lookup_table.rs
// Created : 2021-10-19
// Authors : Marin Moulinier
//--------------------------------------------------------
//  
//  Implementation of the GPRS encryption algorithms GEA1 and GEA2
//  From the research paper:
//  https://eprint.iacr.org/2021/819.pdf
//-----------------------------------------------------------------------------/


/// Please see the "generate_f_lookup_table.py" script present in the
/// current directory for context about how the following lookup table
/// has been originally generated.

pub const F_LOOKUP_TABLE: [bool; 128] = [
    false,
    false,
    true,
    true,
    true,
    false,
    false,
    true,
    true,
    false,
    true,
    true,
    true,
    false,
    true,
    true,
    false,
    false,
    true,
    false,
    true,
    true,
    false,
    true,
    true,
    false,
    true,
    false,
    true,
    true,
    true,
    true,
    true,
    false,
    true,
    false,
    true,
    true,
    false,
    false,
    false,
    true,
    true,
    true,
    true,
    false,
    true,
    true,
    false,
    true,
    false,
    false,
    false,
    true,
    true,
    true,
    true,
    false,
    false,
    true,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    true,
    false,
    false,
    true,
    false,
    true,
    false,
    false,
    true,
    false,
    true,
    false,
    true,
    false,
    false,
    true,
    true,
    false,
    true,
    false,
    true,
    true,
    false,
    false,
    false,
    false,
    false,
    false,
    true,
    false,
    true,
    true,
    false,
    false,
    false,
    false,
    false,
    true,
    true,
    false,
    true,
    false,
    true,
    false,
    false,
    true,
    false,
    true,
    true,
    true,
    false,
    false,
    false,
    false,
    false,
    true,
    true,
    true,
    true,
    true,
    true,
];
