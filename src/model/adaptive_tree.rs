//! Symbol frequency model implemented by Binary Indexed Tree.

use std::boxed::Box;
use std::vec::Vec;
use super::Model;
use super::Parameters;
use super::super::Result;
use super::super::Error;

/// Adaptive model that uses a Binary Indexed Tree for storing cumulative frequencies.
pub struct AdaptiveTreeModel {
    /// Tree of cumulative frequencies.
    tree: Vec<u64>,
    /// Arithmetic parameters.
    params: Parameters,
}

trait LastOne<T> {
    fn last_one(self) -> T;
}

impl LastOne<usize> for usize {
    fn last_one(self) -> usize {
        self & self.wrapping_neg()
    }
}

impl AdaptiveTreeModel {
    /// Initializes the model with the given parameters.
    pub fn new(p: Parameters) -> Box<AdaptiveTreeModel> {
        let mut m = AdaptiveTreeModel {
            tree: vec![0; p.symbol_count + 1],
            params: p,
        };
        for i in 1..m.params.symbol_count {
            m.tree[i] = i.last_one() as u64;
        }

        return Box::new(m);
    }

    /// Updates the cumulative frequencies for the given symbol.
    fn update(&mut self, symbol: usize) {
    }
}

impl Model for AdaptiveTreeModel {
    fn parameters<'a>(&'a self) -> &'a Parameters {
        &self.params
    }

    fn total_frequency(&self) -> u64 {
        0
    }

    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)> {
        if symbol > self.params.symbol_eof {
            return Err(Error::InvalidInput)
        }

        let mut i = symbol;
        let mut sum = 0u64;
        while i > 0 {
            sum += self.tree[i];
            i -= i.last_one();
        }
        return Ok((sum, 0));
    }

    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)> {
        Err(Error::InvalidInput)
    }
}

