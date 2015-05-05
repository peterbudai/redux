use super::Result;
use super::Error::{Eof, InvalidInput};
use super::bitio::{ByteRead, BitRead};
use super::bitio::{ByteWrite, BitWrite};
use super::model::Model;
use super::model::SYMBOL_EOF;

const CODE_BITS_MAX: usize = 64 / 2 + 1;

pub struct Codec<'a> {
    code_bits: usize,
    code_min: u64,
    code_one_fourth: u64,
    code_half: u64,
    code_three_fourths: u64,
    code_max: u64,

    low: u64,
    high: u64,
    pending: u64,
    extra: usize,
    model: &'a mut Model,
}

impl<'a> Codec<'a> {
    pub fn init(bits: usize, m: &'a mut Model) -> Result<Codec<'a>> {
        if(bits < m.get_frequency_bits() + 2 || CODE_BITS_MAX < bits) {
            return Err(InvalidInput);
        }

        Ok(Codec {
            code_bits: bits,
            code_min: 0,
            code_one_fourth: 1 << (bits - 2),
            code_half: 2 << (bits - 2),
            code_three_fourths: 3 << (bits - 2),
            code_max: (1 << bits) - 1,

            low: 0,
            high: (1 << bits) - 1,
            pending: 0,
            extra: bits,
            model: m,
        })
    }

    fn put_bits(&mut self, bit: bool, output: &mut BitWrite) -> Result<()> {
        try!(output.write_bit(bit));
        while self.pending > 0 {
            try!(output.write_bit(!bit));
            self.pending -= 1;
        }
        return Ok(());
    }

    pub fn compress(&mut self, input: &mut ByteRead, output: &mut BitWrite) -> Result<()> {
        loop {
            let symbol = match input.read_byte() {
                Ok(b) => b as usize,
                Err(Eof) => SYMBOL_EOF,
                Err(e) => { return Err(e); }
            };

            {
                let count = self.model.get_total_frequency();
                let (low, high) = try!(self.model.get_frequency(symbol));
                let range = self.high - self.low + 1;
                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
               if self.high < self.code_half {
                   try!(self.put_bits(false, output));

                   if symbol == SYMBOL_EOF {
                       self.extra -= 1;
                   }
               } else if self.low >= self.code_half {
                   try!(self.put_bits(true, output));

                   if symbol == SYMBOL_EOF {
                       self.extra -= 1;
                   }
               } else if self.low >= self.code_one_fourth && self.high < self.code_three_fourths {
                   self.pending += 1;
                   self.low -= self.code_one_fourth;
                   self.high -= self.code_one_fourth;

                   if symbol == SYMBOL_EOF {
                       self.extra -= 1;
                   }
               } else {
                   break;
               }

               self.high = ((self.high << 1) + 1) & self.code_max;
               self.low = (self.low << 1) & self.code_max;
            }

            if symbol == SYMBOL_EOF {
                break;
            }
        }

        while self.extra > 0 {
            try!(self.put_bits(self.low & self.code_half != 0, output));
            self.low = (self.low << 1) & self.code_max;
            self.extra -= 1;
        }
        try!(output.flush_bits());

        return Ok(());
    }

    fn get_bit(&mut self, input: &mut BitRead) -> Result<()> {
        self.pending = (self.pending << 1) | if try!(input.read_bit()) { 1 } else { 0 };
        Ok(())
    }

    pub fn decompress(&mut self, input: &mut BitRead, output: &mut ByteWrite) -> Result<()> {
        while self.extra > 0 {
            try!(self.get_bit(input));
            self.extra -= 1;
        }

        loop {
            {
                let range = self.high - self.low + 1;
                let count = self.model.get_total_frequency();
                let value = ((self.pending - self.low + 1) * count - 1) / range;
                let (symbol, low, high) = try!(self.model.get_symbol(value));

                if symbol == SYMBOL_EOF {
                    break;
                } else {
                    try!(output.write_byte(symbol as u8));
                }

                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
                if self.high < self.code_half {
                    // do nothing
                } else if self.low >= self.code_half {
                    self.pending -= self.code_half;
                    self.low -= self.code_half;
                    self.high -= self.code_half;
                } else if self.low >= self.code_one_fourth && self.high < self.code_three_fourths {
                    self.pending -= self.code_one_fourth;
                    self.low -= self.code_one_fourth;
                    self.high -= self.code_one_fourth;
                } else {
                    break;
                }

                self.low = self.low << 1;
                self.high = (self.high << 1) + 1;
                try!(self.get_bit(input));
            }
        }

        return Ok(());
    }
}

