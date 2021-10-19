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
