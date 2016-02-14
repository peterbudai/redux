//! Adaptive linear symbol frequency model.

use std::boxed::Box;
use std::vec::Vec;
use super::Model;
use super::Parameters;
use super::super::Result;
use super::super::Error;

/// Adaptive model that uses a simple array for cumulative freq
/// and simple, but slow linear algorithms for operations.
pub struct AdaptiveLinearModel {
    /// Array of comulative frequencies.
    freq: Vec<u64>,
    /// Arithmetic parameters.
    params: Parameters,
}

impl AdaptiveLinearModel {
    /// Initializes the model with the given parameters.
    pub fn new(p: Parameters) -> Box<Model> {
        let mut m = AdaptiveLinearModel {
            freq: vec![0; p.symbol_count + 1],
            params: p,
        };
        for i in 1..m.freq.len() {
            m.freq[i] = i as u64;
        }
        return Box::new(m);
    }

    /// Updates the cumulative frequencies for the given symbol.
    fn update(&mut self, symbol: usize) {
        if self.total_frequency() < self.params.freq_max {
            for i in symbol + 1..self.freq.len() {
                self.freq[i] += 1;
            }
        }
    }
}

impl Model for AdaptiveLinearModel {
    fn parameters<'a>(&'a self) -> &'a Parameters {
        &self.params
    }

    fn total_frequency(&self) -> u64 {
        self.freq[self.params.symbol_count]
    }

    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)> {
        if symbol > self.params.symbol_eof {
            Err(Error::InvalidInput)
        } else {
            let res = (self.freq[symbol], self.freq[symbol + 1]);
            self.update(symbol);
            Ok(res)
        }
    }

    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)> {
        for i in 0..self.freq.len()-1 {
            if value < self.freq[i + 1] {
                let res = (i, self.freq[i], self.freq[i + 1]);
                self.update(i);
                return Ok(res);
            }
        }
        Err(Error::InvalidInput)
    }

    #[cfg(debug_assertions)]
    fn get_freq_table(&self) -> Vec<(u64, u64)> {
        let mut res = vec![(0u64, 0u64); self.params.symbol_count];
        for i in 0..self.params.symbol_count {
            res[i] = (self.freq[i], self.freq[i+1]);
        }
        res
    }
}

