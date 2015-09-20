//! Model-independent compression and decompression module.

use std::boxed::Box;
use super::Error;
use super::Result;
use super::bitio::ByteRead;
use super::bitio::BitRead;
use super::bitio::ByteWrite;
use super::bitio::BitWrite;
use super::model::Model;

/// The current state of the encoder and decoder.
pub struct Codec {
    /// The start of the active range.
    low: u64,
    /// The end of the active range.
    high: u64,
    /// Number of pending code bits to putput when encoding.
    /// The current symbol value being decoded.
    pending: u64,
    /// Number of trailing bits to output for unambigous encoding.
    /// Number of leading bits to read when decoding.
    extra: usize,
    /// The probability model being used.
    model: Box<Model>,
}

impl Codec {
    /// Creates and initializes the codec for encoding or decoding.
    pub fn new(m: Box<Model>) -> Codec {
        Codec {
            low: { m.parameters().code_min },
            high: { m.parameters().code_max },
            pending: 0,
            extra: { m.parameters().code_bits },
            model: m,
        }
    }

    /// Outputs a bit and the preceeding pending bits, if any.
    fn put_bit(&mut self, bit: bool, output: &mut BitWrite) -> Result<()> {
        try!(output.write_bit(bit));
        while self.pending > 0 {
            try!(output.write_bit(!bit));
            self.pending -= 1;
        }
        return Ok(());
    }

    /// Inputs a bit.
    fn get_bit(&mut self, input: &mut BitRead) -> Result<()> {
        self.pending = (self.pending << 1) | if try!(input.read_bit()) { 1 } else { 0 };
        Ok(())
    }

    /// Compresses a symbol and outputs some bits depending on the state of the codec.
    pub fn compress_symbol(&mut self, symbol: usize, output: &mut BitWrite) -> Result<()> {
        let count = self.model.total_frequency();
        let (low, high) = try!(self.model.get_frequency(symbol));
        let range = self.high - self.low + 1;
        self.high = self.low + (range * high / count) - 1;
        self.low = self.low + (range * low / count);

        loop {
           if self.high < { self.model.parameters().code_half } {
               try!(self.put_bit(false, output));

               if symbol == { self.model.parameters().symbol_eof } {
                   self.extra -= 1;
               }
           } else if self.low >= { self.model.parameters().code_half } {
               try!(self.put_bit(true, output));

               if symbol == { self.model.parameters().symbol_eof } {
                   self.extra -= 1;
               }
           } else if self.low >= { self.model.parameters().code_one_fourth } && self.high < { self.model.parameters().code_three_fourths } {
               self.pending += 1;
               self.low -= { self.model.parameters().code_one_fourth };
               self.high -= { self.model.parameters().code_one_fourth };

               if symbol == { self.model.parameters().symbol_eof } {
                   self.extra -= 1;
               }
           } else {
               break;
           }

           self.high = ((self.high << 1) + 1) & { self.model.parameters().code_max };
           self.low = (self.low << 1) & { self.model.parameters().code_max };
        }

        if symbol == { self.model.parameters().symbol_eof } {
            while self.extra > 0 {
                let mask = self.low & { self.model.parameters().code_half };
                try!(self.put_bit(mask != 0, output));
                self.low = (self.low << 1) & { self.model.parameters().code_max };
                self.extra -= 1;
            }
            try!(output.flush_bits());
        }
        return Ok(());
    }

    /// Compresses an entire byte stream outputting the EOF symbol and all bits for unambigous encoding.
    pub fn compress_bytes(&mut self, input: &mut ByteRead, output: &mut BitWrite) -> Result<()> {
        if { self.model.parameters().symbol_bits } != 8 {
            return Err(Error::InvalidInput)
        }

        loop {
            let symbol = match input.read_byte() {
                Ok(b) => b as usize,
                Err(Error::Eof) => { self.model.parameters().symbol_eof },
                Err(e) => { return Err(e); }
            };

            try!(self.compress_symbol(symbol, output));

            if symbol == { self.model.parameters().symbol_eof } {
                break;
            }
        }

        return Ok(());
    }

    /// Decompresses a symbol reading some bits until the symbol can be decoded.
    pub fn decompress_symbol(&mut self, input: &mut BitRead) -> Result<usize> {
        while self.extra > 0 {
            try!(self.get_bit(input));
            self.extra -= 1;
        }

        let range = self.high - self.low + 1;
        let count = self.model.total_frequency();
        let value = ((self.pending - self.low + 1) * count - 1) / range;
        let (symbol, low, high) = try!(self.model.get_symbol(value));
        self.high = self.low + (range * high / count) - 1;
        self.low = self.low + (range * low / count);

        if symbol == { self.model.parameters().symbol_eof } {
            return Ok(symbol);
        }

        loop {
            if self.high < { self.model.parameters().code_half } {
                // do nothing
            } else if self.low >= { self.model.parameters().code_half } {
                self.pending -= { self.model.parameters().code_half };
                self.low -= { self.model.parameters().code_half };
                self.high -= { self.model.parameters().code_half };
            } else if self.low >= { self.model.parameters().code_one_fourth } && self.high < { self.model.parameters().code_three_fourths } {
                self.pending -= { self.model.parameters().code_one_fourth };
                self.low -= { self.model.parameters().code_one_fourth };
                self.high -= { self.model.parameters().code_one_fourth };
            } else {
                break;
            }

            self.low = self.low << 1;
            self.high = (self.high << 1) + 1;
            try!(self.get_bit(input));
        }

        return Ok(symbol);
    }

    /// Decompresses a whole bit stream until the EOF symbol is found.
    pub fn decompress_bytes(&mut self, input: &mut BitRead, output: &mut ByteWrite) -> Result<()> {
        if { self.model.parameters().symbol_bits } != 8 {
            return Err(Error::InvalidInput)
        }

        loop {
            let symbol = try!(self.decompress_symbol(input));

            if symbol == { self.model.parameters().symbol_eof } {
                break;
            } else {
                try!(output.write_byte(symbol as u8));
            }
        }

        return Ok(());
    }
}

