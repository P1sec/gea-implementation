use crate::lfsr::LinearFeedbackShiftRegister;
use crate::f_lookup_table::F_LOOKUP_TABLE;

/// The B register is initialized from the S or W register, and will
/// contribute to generate the output of the script.
pub struct BRegister(pub u64);

impl BRegister {
    pub fn initialize(vector_data: u128, vector_size: usize) -> Self {
        let mut b_register = Self(0);
        b_register.initial_clock(vector_data, vector_size);
        if b_register.0 == 0 {
            b_register.0 = 1;
        }
        b_register
    }
}

impl LinearFeedbackShiftRegister for BRegister {
    fn clock(&mut self, input_bit: Option<bool>) {
        let bit = input_bit.unwrap_or(false) as u64;
        self.0 = (self.0 >> 1) |
            (((self.0 & 1) ^ bit) << 31);
        
        if self.0 >> 31 == 1 {
            self.0 ^= 0b1110001110000001111000001000101;
        }
    }
    
    fn f_function(&mut self) -> bool {
        F_LOOKUP_TABLE[(
            ((self.0 >> (12 - 0)) & 0b000_0001) |
            ((self.0 >> (27 - 1)) & 0b000_0010) |
            ((self.0 << (2 - 0)) & 0b000_0100) |
            ((self.0 << (3 - 1)) & 0b000_1000) |
            ((self.0 >> (29 - 4)) & 0b001_0000) |
            ((self.0 >> (21 - 5)) & 0b010_0000) |
            ((self.0 << (6 - 5)) & 0b100_0000)
        ) as usize]
    }
}
