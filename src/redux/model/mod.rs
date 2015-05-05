pub mod adaptive_linear;

use super::Result;
use super::Error::InvalidInput;

/// Number of bits of the input symbols
const SYMBOL_BITS: usize = 8;
/// End-Of-File symbol
pub const SYMBOL_EOF: usize = 1 << SYMBOL_BITS;
/// Number of different symbols
const SYMBOL_COUNT: usize = SYMBOL_EOF + 1;
/// Minimum number of frequency bits
const FREQ_BITS_MIN: usize = SYMBOL_BITS + 1;
/// Maximum number of frequency bits
const FREQ_BITS_MAX: usize = 64 / 2 - 1;

/// Trait for the probability models behing arithmetic coding.
/// Possible implementations may include static models with fixed probabilities
/// or and adaptive model that continuously updates cumulative frequencies.
pub trait Model {
    /// Returns the number of bits used for cumulative frequencies
    fn get_frequency_bits(&self) -> usize;
    /// Returns the maximum cumulative frequency
    fn get_total_frequency(&self) -> u64;
    /// Returns the cumulative frequency range for the given input symbol
    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)>;
    /// Returns the symbol that falls into the given cumulative frequency
    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)>;
}

