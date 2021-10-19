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
