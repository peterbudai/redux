use std::io::Read;
use std::io::Write;
use super::Result;
use super::Error::Eof;
use super::bitio::{ByteRead, BitRead};
use super::bitio::{ByteWrite, BitWrite};
use super::model::Model;
use super::model::VALUE_EOF;

pub struct Codec<'a, T> {
    low: T,
    high: T,
    pending: T,
    extra: usize,
    model: &'a mut Model<T>,
}

impl<'a, T> Codec<'a, T> {
    pub fn init(m: &'a mut Model<T>) -> Codec<'a, T> {
        Codec {
            low: m.params().code_min,
            high: m.params().code_max,
            pending: 0,
            extra: m.params().code_bits,
            model: m
        }
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
        let params = self.model.params();

        loop {
            let chr = match input.read_byte() {
                Ok(b) => b as usize,
                Err(Eof) => VALUE_EOF,
                Err(e) => { return Err(e); }
            };

            {
                let count = self.model.get_count();
                let (low, high) = self.model.get_probability(chr);
                let range = self.high - self.low + 1;
                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
               if self.high < params.code_one_half {
                   try!(self.put_bits(0, output));

                   if chr == VALUE_EOF {
                       self.extra -= 1;
                   }
               } else if self.low >= params.code_one_half {
                   try!(self.put_bits(1, output));

                   if chr == VALUE_EOF {
                       self.extra -= 1;
                   }
               } else if self.low >= params.code_one_fourth && self.high < params.code_three_fourths {
                   self.pending += 1;
                   self.low -= params.code_one_fourth;
                   self.high -= params.code_one_fourth;

                   if chr == VALUE_EOF {
                       self.extra -= 1;
                   }
               } else {
                   break;
               }

               self.high = ((self.high << 1) + 1) & params.code_max;
               self.low = (self.low << 1) & params.code_max;
            }

            if chr == VALUE_EOF {
                break;
            }
        }

        while self.extra > 0 {
            try!(self.put_bits(self.low & params.code_half != 0, output));
            self.low = (self.low << 1) & params.code_max;
            self.extra -= 1;
        }
        try!(output.flush_bits());

        return Ok(());
    }

    fn get_bit(&mut self, input: &mut BitRead) -> Result<()> {
        self.pending = (self.pending << 1) | if try!(input.read_bit()) { 1 } else { 0 };
    }

    pub fn decompress(&mut self, input: &mut BitRead, output: &mut ByteWrite) -> Result<()> {
        let params = self.model.params();

        while self.extra > 0 {
            try!(self.get_bit(input));
            self.extra -= 1;
        }

        loop {
            {
                let range = self.high - self.low + 1;
                let count = self.get_count();
                let value = ((self.pending - self.low + 1) * count - 1) / range;
                let (chr, low, high) = self.get_char(value);

                if chr == VALUE_EOF {
                    break;
                } else {
                    try!(output.write_byte(chr as u8));
                }

                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
                if self.high < params.code_one_half {
                    // do nothing
                } else if self.low >= params.code_one_half {
                    self.pending -= params.code_one_half;
                    self.low -= params.code_one_half;
                    self.high -= params.code_one_half;
                } else if self.low >= params.code_one_fourth && self.high < params.code_three_fourths {
                    self.pending -= params.code_one_fourth;
                    self.low -= params.code_one_fourth;
                    self.high -= params.code_one_fourth;
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

