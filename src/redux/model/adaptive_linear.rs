use super::Model;
use super::Parameters;
use super::{VALUE_EOF, VALUE_COUNT};
use super::super::Result;
use super::super::Error::InvalidInput;

/// Adaptive model that uses a simple array for cumulative freq
/// and simple, but slow linear algorithms for operations.
pub struct AdaptiveLinearModel<T> {
    /// Model parameters
    params: Parameters<T>,
    /// Array of comulative frequencies
    freq: [T; VALUE_COUNT + 1],
}

impl<T> AdaptiveLinearModel<T> {
    fn init(code: usize, freq: usize) -> AdaptiveLinearModel<T> {
        let m = AdaptiveLinearModel {
            params: Parameters::<T>::init(code, freq),
            freq: [0; VALUE_COUNT + 1]
        };
        for i in 1..m.freq.len() + 1 {
            m.freq[i] = i;
        }
        return m;
    }

    fn update(&mut self, chr: usize) {
        if self.get_count() < self.params.freq_max {
            for i in chr + 1..self.freq.len() {
                self.freq[i] += 1;
            }
        }
    }
}

impl<T> Model<T> for AdaptiveLinearModel<T> {
    fn params(&self) -> &Parameters<T> {
        self.params
    }

    fn get_count(&self) -> T {
        self.freq[VALUE_COUNT]
    }

    fn get_probability(&mut self, chr: usize) -> Result<(T, T)> {
        if chr > VALUE_EOF {
            Err(InvalidInput())
        } else {
            let prob = (self.freq[chr], self.freq[chr + 1]);
            self.update(chr);
            Ok(prob)
        }
    }

    fn get_char(&mut self, value: T) -> Result<(usize, T, T)> {
        for i in 0..self.freq.len() {
            if value < self.freq[i + 1] {
                let prob = (i, self.freq[i], self.freq[i + 1]);
                self.update(i);
                return Ok(prob);
            }
        }
        Err(InvalidInput())
    }
}

