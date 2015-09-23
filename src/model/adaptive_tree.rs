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
            tree: vec![0; p.symbol_count],
            params: p,
        };

        m.tree[0] = 1u64;
        for i in 1..m.tree.len() {
            m.tree[i] = i.last_one() as u64;
        }

        return Box::new(m);
    }

    /// Returns the cumulated frequency of the symbol
    fn get_frequency_single(&self, symbol: usize) -> u64 {
        let mut i = symbol;
        let mut sum = self.tree[0];
        while i > 0 {
            sum += self.tree[i];
            i -= i.last_one();
        }
        return sum;
    }

    /// Updates the cumulative frequencies for the given symbol.
    fn update(&mut self, symbol: usize) {
        if symbol == 0 {
            self.tree[0] += 1;
        } else {
            let mut i = symbol;
            while i <= self.params.symbol_eof {
                self.tree[i] += 1;
                i += i.last_one();
            }
        }
    }
}

impl Model for AdaptiveTreeModel {
    fn parameters<'a>(&'a self) -> &'a Parameters {
        &self.params
    }

    fn total_frequency(&self) -> u64 {
        self.get_frequency_single(self.params.symbol_eof)
    }

    fn get_frequency(&mut self, symbol: usize) -> Result<(u64, u64)> {
        if symbol > self.params.symbol_eof {
            return Err(Error::InvalidInput);
        }

        let result = if symbol > 0 {
            let mut sumh = 0u64;
            let mut suml = 0u64;
            let mut h = symbol;
            let mut l = symbol - 1;
            while h != l {
                if h > l {
                    sumh += self.tree[h];
                    h -= h.last_one();
                } else {
                    suml += self.tree[l];
                    l -= l.last_one();
                }
            }

            let sumr = self.get_frequency_single(h);
            (suml + sumr, sumh + sumr)
        } else {
            (0u64, self.tree[0])
        };

        self.update(symbol);
        return Ok(result);
    }

    fn get_symbol(&mut self, value: u64) -> Result<(usize, u64, u64)> {
        let mut m = self.params.symbol_eof;
        let mut i = 0usize;
        let mut v = value;
        while (m > 0) && (i < self.params.symbol_eof) {
            let ti = i + m;
            let tv = self.tree[ti];

            if v >= tv {
                i = ti;
                v -= tv;
            }
            m >>= 1;
        }

        let (l, h) = try!(self.get_frequency(i));
        self.update(i);
        return Ok((i, l, h));
    }
}

