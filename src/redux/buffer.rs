use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

const BUFFER_SIZE: usize = 16384;

pub struct Buffer {
    data: [u8; BUFFER_SIZE],
    len: usize,
    cur: usize,
    bit: u8,
    sum: u64
}

impl Buffer {
    pub fn create() -> Buffer {
        return Buffer {
            data: [0u8; BUFFER_SIZE],
            len: 0us,
            cur: 0us,
            bit: 7u8,
            sum: 0u64
        };
    }

    pub fn fill_buffer(&mut self, istream: &mut Read) -> Result<bool> {
        match istream.read(&mut self.data[0..]) {
            Ok(0) => Ok(true),
            Ok(n) => {
                self.sum += n as u64;
                self.len = n;
                self.cur = 0;
                Ok(false)
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_byte(&mut self, istream: &mut Read) -> Result<(u8, bool)> {
        let value = self.data[self.cur];
        self.cur += 1;
        if self.cur == self.len {
            return match self.fill_buffer(istream) {
                Ok(b) => Ok((value, b)),
                Err(e) => Err(e)
            }
        } else {
            return Ok((value, false));
        }
    }

    pub fn get_bit(&mut self, istream: &mut Read) -> Result<(u8, bool)> {
        let value = (self.data[self.cur] & (1 << self.bit)) >> self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.cur += 1;
            if self.cur == self.len {
                return match self.fill_buffer(istream) {
                    Ok(b) => Ok((value, b)),
                    Err(e) => Err(e)
                }
            }
        } else {
            self.bit -= 1;
        }
        return Ok((value, false));
    }

    pub fn flush_buffer(&mut self, ostream: &mut Write) -> Result<()> {
        // Flush the remaining incomplete byte as well
        if self.bit < 7 {
            self.len += 1;
        }

        while self.cur < self.len {
            match ostream.write(&self.data[self.cur..self.len]) {
                Ok(0) => { return Err(Error::new(ErrorKind::WriteZero, "Zero bytes written to output")); }
                Ok(n) => {
                    self.cur += n;
                    self.sum += n as u64;
                },
                Err(e) => { return Err(e); }
            }
        }

        self.len = 0;
        self.cur = 0;
        return Ok(());
    }

    pub fn put_byte(&mut self, value: u8, ostream: &mut Write) -> Result<()> {
        self.data[self.len] = value;
        self.len += 1;
        if self.len == self.data.len() {
            if let Err(e) = self.flush_buffer(ostream) {
                return Err(e);
            }
        }
        return Ok(());
    }

    pub fn put_bit(&mut self, value: u8, ostream: &mut Write) -> Result<()> {
        self.data[self.len] |= value << self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.len += 1;
            if self.len == self.data.len() {
                if let Err(e) = self.flush_buffer(ostream) {
                    return Err(e);
                }
            }
            self.data[self.len] = 0;
        } else {
            self.bit -= 1;
        }
        return Ok(());
    }

    pub fn get_count(&self) -> u64 {
        return self.sum;
    }
}

