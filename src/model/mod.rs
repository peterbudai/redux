//! Symbol frequency distribution models.

mod adaptive_linear;

use super::Error;
use super::Result;

pub use self::adaptive_linear::AdaptiveLinearModel;

/// Trait for the probability models behing arithmetic coding.
/// Possible implementations may include static models with fixed probabilities
/// or and adaptive model that continuously updates cumulative frequencies.
pub trait Model {
    /// Returns the arithmetic parameters.
    fn parameters<'a>(&'a self) -> &'a Parameters;
    /// Returns the maximum cumulative frequency.
    fn total_frequency(&self) -> u64;
    /// Returns the cumulative frequency range for the given input symbol.
    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)>;
    /// Returns the symbol that falls into the given cumulative frequency.
    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)>;
}

/// Model parameters that specifies the common property of the models.
pub struct Parameters {
    /// Bit width of the symbols being encoded.
    /// Usually 8 for byte oriented inputs.
    pub symbol_bits: usize,
    /// Code for the EOF symbol.
    /// This is the next symbol code after the valid symbols to encode.
    pub symbol_eof: usize,
    /// Number of possible symbols including the EOF symbol.
    pub symbol_count: usize,
    /// Number of bits representing symbol frequencies.
    pub freq_bits: usize,
    /// Maximum cumulated frequency value for symbols.
    pub freq_max: u64,
    /// Number of bits representing the current code ranges.
    pub code_bits: usize,
    /// Minimum value for code range.
    /// This is always zero.
    pub code_min: u64,
    /// Delimiter for the one fourth of the valid code range.
    pub code_one_fourth: u64,
    /// Delimiter for the half of the valid code range.
    pub code_half: u64,
    /// Delimiter for the three fourths of the valid code range.
    pub code_three_fourths: u64,
    /// Upper limit of the valid code range.
    pub code_max: u64,
}

impl Parameters {
    /// Calculates all parameter values based on the symbol, frequency and code width.
    pub fn new(sym: usize, freq: usize, code: usize) -> Result<Parameters> {
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

