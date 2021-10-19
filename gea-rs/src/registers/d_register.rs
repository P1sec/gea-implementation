use crate::lfsr::LinearFeedbackShiftRegister;
use crate::f_lookup_table::F_LOOKUP_TABLE;

/// The D register is initialized from the W register, and will
/// contribute to generate the output of the script.
///
/// It is used in GEA-2 only, not GEA-1.
pub struct DRegister(pub u64);

impl DRegister {
    pub fn initialize(vector_data: u128, vector_size: usize) -> Self {
        let mut d_register = Self(0);
        d_register.initial_clock(vector_data, vector_size);
        if d_register.0 == 0 {
            d_register.0 = 1;
        }
        d_register
    }
}

impl LinearFeedbackShiftRegister for DRegister {
    fn clock(&mut self, input_bit: Option<bool>) {
        let bit = input_bit.unwrap_or(false) as u64;
        self.0 = (self.0 >> 1) |
            (((self.0 & 1) ^ bit) << 28);
        
        if self.0 >> 28 == 1 {
            self.0 ^= 0b1010010110011010101111111001;
        }
    }
    
    fn f_function(&mut self) -> bool {
        F_LOOKUP_TABLE[(
            ((self.0 >> (12 - 0)) & 0b000_0001) |
            ((self.0 >> (23 - 1)) & 0b000_0010) |
            ((self.0 >> (3 - 2)) & 0b000_0100) |
            ((self.0 << (3 - 0)) & 0b000_1000) |
            ((self.0 >> (10 - 4)) & 0b001_0000) |
            ((self.0 >> (27 - 5)) & 0b010_0000) |
            ((self.0 >> (17 - 6)) & 0b100_0000)
        ) as usize]
    }
}
