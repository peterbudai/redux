use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

use super::buffer::Buffer;

const VALUE_BITS: usize = 8;
const VALUE_EOF: usize = 1 << VALUE_BITS;
const VALUE_COUNT: usize = VALUE_EOF + 2;

const CODE_BITS: usize = 16; // 33;
const CODE_MIN: u64 = 0;
const CODE_MAX: u64 = (1 << CODE_BITS) - 1;
const ONE_FOURTH: u64 = 1 << (CODE_BITS - 2);
const ONE_HALF: u64 = 2 * ONE_FOURTH;
const THREE_FOURTHS: u64 = 3 * ONE_FOURTH;

const FREQ_BITS: usize = 14; // 64 - CODE_BITS;
const FREQ_MAX: u64 = (1 << FREQ_BITS) - 1;

pub struct Codec {
    low: u64,
    high: u64,
    pending: u64,
    freq: [u64; VALUE_COUNT],
    input: Buffer,
    output: Buffer
}

impl Codec {
    pub fn init() -> Codec {
        let mut res = Codec {
            low: CODE_MIN,
            high: CODE_MAX,
            pending: 0u64,
            freq: [0u64; VALUE_COUNT],
            input: Buffer::create(),
            output: Buffer::create()
        };
        for i in 1..res.freq.len() {
            res.freq[i] = i as u64;
        }
        return res;
    }

    fn update(&mut self, chr: usize) {
        if self.get_count() < FREQ_MAX {
            for index in chr+1..self.freq.len() {
                self.freq[index] += 1;
            }
        }
    }

    fn get_count(&self) -> u64 {
        self.freq[self.freq.len() - 1]
    }

    fn get_probability(&mut self, chr: usize) -> (u64, u64, u64) {
        let res = (self.freq[chr], self.freq[chr + 1], self.get_count());
        self.update(chr);
        return res;
    }

    fn get_char(&mut self, scaled: u64) -> (usize, u64, u64) {
        let chr = match self.freq.binary_search(&scaled) {
            Ok(i) => i,
            Err(i) => i - 1
        };
        let res = (chr, self.freq[chr], self.freq[chr + 1]);
        self.update(chr);
        return res;
    }

    pub fn get_metrics(&self) -> (u64, u64) {
        (self.input.get_count(), self.output.get_count())
    }

    fn put_bits(&mut self, bit: u8, ostream: &mut Write) -> Result<()> {
        try!(self.output.put_bit(bit, ostream));
        while self.pending > 0 {
            try!(self.output.put_bit(!bit & 1u8, ostream));
            self.pending -= 1;
        }
        return Ok(());
    }

    pub fn compress(&mut self, istream: &mut Read, ostream: &mut Write) -> Result<()> {
        let mut eof = try!(self.input.fill_buffer(istream));
        loop {
            let chr = if eof {
                VALUE_EOF
            } else {
                match self.input.get_byte(istream) {
                    Ok((c, e)) => { eof = e; c as usize },
                    Err(e) => { return Err(e); }
                }
            };

            {
                let (low, high, count) = self.get_probability(chr);
                let range = self.high - self.low + 1;
                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
               if self.high < ONE_HALF {
                   try!(self.put_bits(0, ostream));
               } else if self.low >= ONE_HALF {
                   try!(self.put_bits(1, ostream));
               } else if self.low >= ONE_FOURTH && self.high < THREE_FOURTHS {
                    self.pending += 1;
                    self.low -= ONE_FOURTH;
                    self.high -= ONE_FOURTH;
               } else {
                   break;
               }

               self.high = ((self.high << 1) + 1) & CODE_MAX;
               self.low = (self.low << 1) & CODE_MAX;
            }

            if chr == VALUE_EOF {
                break;
            }
        }

        self.pending += 1;
        if self.low < ONE_FOURTH {
            try!(self.put_bits(0, ostream));
        } else {
            try!(self.put_bits(1, ostream));
        }
        try!(self.output.flush_buffer(ostream));

        return Ok(());
    }

    pub fn decompress(&mut self, istream: &mut Read, ostream: &mut Write) -> Result<()> {
        if try!(self.input.fill_buffer(istream)) {
            return Err(Error::new(ErrorKind::InvalidInput, "Empty input stream"));
        }
        let mut eof = false;
        for _ in 0..CODE_BITS {
            self.pending = if eof {
                (self.pending << 1) | (self.pending & 1u64)
            } else {
                match self.input.get_bit(istream) {
                    Ok((b, e)) => { eof = e; (self.pending << 1) | (b as u64) },
                    Err(e) => { return Err(e); }
                }
            }
        }

        loop {
            {
                let range = self.high - self.low + 1;
                let count = self.get_count();
                let scaled = ((self.pending - self.low + 1) * count - 1) / range;
                let (chr, low, high) = self.get_char(scaled);

                if chr == VALUE_EOF {
                    break;
                } else {
                    try!(self.output.put_byte(chr as u8, ostream));
                }

                self.high = self.low + (range * high / count) - 1;
                self.low = self.low + (range * low / count);
            }

            loop {
                if self.high < ONE_HALF {
                    // do nothing
                } else if self.low >= ONE_HALF {
                    self.pending -= ONE_HALF;
                    self.low -= ONE_HALF;
                    self.high -= ONE_HALF;
                } else if self.low >= ONE_FOURTH && self.high < THREE_FOURTHS {
                    self.pending -= ONE_FOURTH;
                    self.low -= ONE_FOURTH;
                    self.high -= ONE_FOURTH;
                } else {
                    break;
                }

                self.low = self.low << 1;
                self.high = (self.high << 1) + 1;

                self.pending = if eof {
                    return Err(Error::new(ErrorKind::InvalidInput, "Unexpected end of input stream"));
                } else {
                    match self.input.get_bit(istream) {
                        Ok((b, e)) => { eof = e; (self.pending << 1) | (b as u64) },
                        Err(e) => { return Err(e); }
                    }
                }
            }
        }

        try!(self.output.flush_buffer(ostream));
        return Ok(());
    }
}

