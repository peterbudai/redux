mod adaptive_linear;

use std::u8;
use std::mem;
use super::Result;
use super::Error::InvalidInput;

/// End-Of-File symbol
const VALUE_EOF: usize = 256;
/// Number of different symbols
const VALUE_COUNT: usize = VALUE_EOF + 1;

/// Numeric parameters that control the operation of the arithmetic coder and the model
pub struct Parameters<T> {
    /// Number of bits used for range value
    code_bits: usize,
    /// Minimum value for the range
    code_min: T,
    /// Maximum value for the range
    code_max: T,
    /// Divisor value for the lowest quarter of the range
    code_one_fourth: T,
    /// Divisor value for the half of the range
    code_half: T,
    /// Divisor value for the highest quarter of the range
    code_three_fourths: T,

    /// Number of bits used for cumulative frequency values
    freq_bits: usize,
    /// Maximum cumulative frequency value
    freq_max: T,
}

impl<T> Parameters<T> {
    /// Initialize parameter values based on the two arguments
    pub fn init(code: usize, freq: usize) -> Result<Parameters<T>> {
        if code < freq + 2 || code + freq > mem::size_of::<T>() * 8 {
            Err(InvalidInput())
        } else {
            Ok(Parameters {
                code_bits: code,
                code_min: 0,
                code_max: (1 << code) - 1,
                code_one_fourth: 1 << (code - 2),
                code_half: 2 << (code - 2),
                code_three_fourths: 3 << (code - 2),
                freq_bits: freq,
                freq_max: (1 << freq) - 1,
            })
        }
    }
}

/// Trait for the probability models behing arithmetic coding.
/// Possible implementations may include static models with fixed probabilities
/// or and adaptive model that continuously updates cumulative frequencies.
pub trait Model<T> {
    /// Returns the codec parameters
    fn params(&self) -> &Parameters<T>;
    /// Returns the maximum cumulative frequency
    fn get_count(&self) -> T;
    /// Returns the cumulative frequency range for the given input symbol
    fn get_probability(&mut self, chr: usize) -> Result<(T, T)>;
    /// Returns the symbol that falls into the given cumulative frequency
    fn get_char(&mut self, value: T) -> Result<(usize, T, T)>;
}

