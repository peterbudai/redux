//! Symbol frequency distribution models.

mod adaptive_linear;
mod adaptive_tree;
#[cfg(test)]
mod tests;

use super::Error;
use super::Result;

pub use self::adaptive_linear::AdaptiveLinearModel;
pub use self::adaptive_tree::AdaptiveTreeModel;

/// Trait for the probability models behind arithmetic coding.
/// Possible implementations may include static models with fixed probabilities
/// or and adaptive model that continuously updates cumulative frequencies.
pub trait Model {
    /// Returns the arithmetic compression parameters.
    fn parameters<'a>(&'a self) -> &'a Parameters;
    /// Returns the maximum cumulative frequency.
    fn total_frequency(&self) -> u64;
    /// Returns the cumulative frequency range for the given input symbol.
    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)>;
    /// Returns the symbol that corresponds to the given cumulative frequency.
    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)>;
    /// Returns the cumulative frequency table for debugging purposes.
    #[cfg(debug_assertions)]
    fn get_freq_table(&self) -> Vec<(u64, u64)>;
}

/// Model parameters that specifies the common property of the models.
#[derive(Clone)]
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
    /// Calculates all parameter values based on the `symbol`, `frequency` and `code` width.
    pub fn new(symbol: usize, frequency: usize, code: usize) -> Result<Parameters> {
        if symbol < 1 || frequency < symbol + 2 || code < frequency + 2 || 64 < code + frequency {
            Err(Error::InvalidInput)
        } else {
            Ok(Parameters {
                symbol_bits: symbol,
                symbol_eof: 1 << symbol,
                symbol_count: (1 << symbol) + 1,
                freq_bits: frequency,
                freq_max: (1 << frequency) - 1,
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
