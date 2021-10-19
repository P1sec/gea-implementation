use crate::lfsr::LinearFeedbackShiftRegister;
use crate::link_direction::LinkDirection;
use crate::f_lookup_table::F_LOOKUP_TABLE;

/// The S register is a LFSR whose state is generated from the key, IV
/// and Direction bit associated with the current GPRS session, and
/// is used to derive the initial state of the A, B, C LFSRs.
///
/// It is the GEA-1 equivalent of the GEA-2 W initialization registrer.
pub struct SRegister(pub u128);

impl SRegister {
    /// Initialize the S Register from a GEA-1 key, IV
    /// and direction bit.
    pub fn initialize(key: u64, iv: u32, direction: LinkDirection) -> Self {
        let mut s_register = Self(0);
        s_register.initial_clock(iv as u128, 32);
        s_register.initial_clock(match direction {
            LinkDirection::Uplink => 0,
            LinkDirection::Downlink => 1
        }, 1);
        s_register.initial_clock(key as u128, 64);
        s_register.initial_clock(0, 128);
        s_register
    }
}

impl LinearFeedbackShiftRegister for SRegister {
    fn clock(&mut self, input_bit: Option<bool>) {
        let bit = input_bit.unwrap() as u128;
        self.0 = (self.0 >> 1) |
            (((self.0 & 1) ^ (self.f_function() as u128) ^ bit) << 63);
    }
    
    fn f_function(&mut self) -> bool {
        F_LOOKUP_TABLE[(
            ((self.0 >> (3 - 0)) & 0b000_0001) |
            ((self.0 >> (12 - 1)) & 0b000_0010) |
            ((self.0 >> (22 - 2)) & 0b000_0100) |
            ((self.0 >> (38 - 3)) & 0b000_1000) |
            ((self.0 >> (42 - 4)) & 0b001_0000) |
            ((self.0 >> (55 - 5)) & 0b010_0000) |
            ((self.0 >> (63 - 6)) & 0b100_0000)
        ) as usize]
    }
}
