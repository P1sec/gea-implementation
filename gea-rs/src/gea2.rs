use crate::registers::w_register::WRegister;
use crate::registers::a_register::ARegister;
use crate::registers::b_register::BRegister;
use crate::registers::c_register::CRegister;
use crate::registers::d_register::DRegister;
use crate::lfsr::LinearFeedbackShiftRegister;

pub struct GEA2State {
    pub a_register: ARegister,
    pub b_register: BRegister,
    pub c_register: CRegister,
    pub d_register: DRegister,
    pub w_register: WRegister
}

impl GEA2State {
    /// Create a new GEA2 encryption/decryption object
    /// from a S Register object, which has been
    /// prealably initialized from a key, IV and
    /// direction bit.
    pub fn initialize(w_register: WRegister) -> Self {
        Self {
            a_register: ARegister::initialize((w_register.0 >> 16) |
                ((w_register.0 & ((1 << 16) - 1)) << (97 - 16)), 97),
            b_register: BRegister::initialize((w_register.0 >> 33) |
                ((w_register.0 & ((1 << 33) - 1)) << (97 - 33)), 97),
            c_register: CRegister::initialize((w_register.0 >> 51) |
                ((w_register.0 & ((1 << 51) - 1)) << (97 - 51)), 97),
            d_register: DRegister::initialize(w_register.0, 97),
            w_register: w_register
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
    /// internal state of the current GEA2 object.
    pub fn generate_stream(&mut self, num_bytes: usize) -> Vec<u8> {
        let mut keystream: Vec<u8> = vec![];
        
        for _num_byte in 0..num_bytes {
            let mut new_byte = 0;
            for num_bit in 0..8 {
                new_byte |= (self.a_register.f_function() as u8 ^
                    self.b_register.f_function() as u8 ^
                    self.c_register.f_function() as u8 ^
                    self.d_register.f_function() as u8) << num_bit;
                    
                self.a_register.clock(None);
                self.b_register.clock(None);
                self.c_register.clock(None);
                self.d_register.clock(None);
            }
            keystream.push(new_byte);
        }
        
        keystream
    }
}
