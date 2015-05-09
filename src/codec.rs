use std::boxed::Box;
use super::Error;
use super::Result;
use super::bitio::ByteRead;
use super::bitio::BitRead;
use super::bitio::ByteWrite;
use super::bitio::BitWrite;
use super::model::Model;

pub struct Codec {
    low: u64,
    high: u64,
    pending: u64,
    extra: usize,
    model: Box<Model>,
}

impl Codec {
    pub fn init(m: Box<Model>) -> Codec {
        Codec {
            low: { m.parameters().code_min },
            high: { m.parameters().code_max },
            pending: 0,
            extra: { m.parameters().code_bits },
            model: m,
        }
    }

    fn put_bit(&mut self, bit: bool, output: &mut BitWrite) -> Result<()> {
        try!(output.write_bit(bit));
        while self.pending > 0 {
            try!(output.write_bit(!bit));
            self.pending -= 1;
        }
        return Ok(());
    }

    fn get_bit(&mut self, input: &mut BitRead) -> Result<()> {
        self.pending = (self.pending << 1) | if try!(input.read_bit()) { 1 } else { 0 };
        Ok(())
    }

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

