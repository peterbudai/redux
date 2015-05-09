pub mod adaptive_linear;

use super::Error;
use super::Result;

/// Trait for the probability models behing arithmetic coding.
/// Possible implementations may include static models with fixed probabilities
/// or and adaptive model that continuously updates cumulative frequencies.
pub trait Model {
    /// Returns the arithmetic parameters
    fn parameters<'a>(&'a self) -> &'a Parameters;
    /// Returns the maximum cumulative frequency
    fn total_frequency(&self) -> u64;
    /// Returns the cumulative frequency range for the given input symbol
    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)>;
    /// Returns the symbol that falls into the given cumulative frequency
    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)>;
}

pub struct Parameters {
    pub symbol_bits: usize,
    pub symbol_eof: usize,
    pub symbol_count: usize,
    pub freq_bits: usize,
    pub freq_max: u64,
    pub code_bits: usize,
    pub code_min: u64,
    pub code_one_fourth: u64,
    pub code_half: u64,
    pub code_three_fourths: u64,
    pub code_max: u64,
}

impl Parameters {
    pub fn init(sym: usize, freq: usize, code: usize) -> Result<Parameters> {
        if sym < 1 || freq < sym + 2 || code < freq + 2 || 64 < code + freq {
            Err(Error::InvalidInput)
        } else {
            Ok(Parameters {
                symbol_bits: sym,
                symbol_eof: 1 << sym,
                symbol_count: (1 << sym) + 1,
                freq_bits: freq,
                freq_max: (1 << freq) - 1,
                code_bits: code,
                code_min: 0,
                code_one_fourth: 1 << (code - 2),
                code_half: 2 << (code - 2),
                code_three_fourths: 3 << (code - 2),
                code_max: (1 << code) - 1,
            })
        }
    }
}

