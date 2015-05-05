use super::Model;
use super::{SYMBOL_EOF, SYMBOL_COUNT, FREQ_BITS_MIN, FREQ_BITS_MAX};
use super::super::Result;
use super::super::Error::InvalidInput;

/// Adaptive model that uses a simple array for cumulative freq
/// and simple, but slow linear algorithms for operations.
pub struct AdaptiveLinearModel {
    /// Array of comulative frequencies
    freq: [u64; SYMBOL_COUNT + 1], 
    /// Number of bits used for cumulative frequencies
    freq_bits: usize,
    /// Maximum allowed value for cumulative frequencies
    freq_max: u64,
}

impl AdaptiveLinearModel {
    pub fn init(bits: usize) -> Result<AdaptiveLinearModel> {
        if(bits < FREQ_BITS_MIN || FREQ_BITS_MAX < bits) {
            return Err(InvalidInput);
        }

        let m = AdaptiveLinearModel {
            freq: [0; SYMBOL_COUNT + 1],
            freq_bits: bits,
            freq_max: (1 << bits) - 1,
        };
        for i in 1..m.freq.len() {
            m.freq[i] = i as u64;
        }
        return Ok(m);
    }

    fn update(&mut self, symbol: usize) {
        if self.get_total_frequency() < self.freq_max {
            for i in symbol + 1..self.freq.len() {
                self.freq[i] += 1;
            }
        }
    }
}

impl Model for AdaptiveLinearModel {
    fn get_frequency_bits(&self) -> usize {
        self.freq_bits
    }

    fn get_total_frequency(&self) -> u64 {
        self.freq[SYMBOL_COUNT]
    }

    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)> {
        if symbol > SYMBOL_EOF {
            Err(InvalidInput)
        } else {
            let res = (self.freq[symbol], self.freq[symbol + 1]);
            self.update(symbol);
            Ok(res)
        }
    }

    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)> {
        for i in 0..self.freq.len() {
            if value < self.freq[i + 1] {
                let res = (i, self.freq[i], self.freq[i + 1]);
                self.update(i);
                return Ok(res);
            }
        }
        Err(InvalidInput)
    }
}

