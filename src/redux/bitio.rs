use std::io::Read;
use std::io::Write;
use super::Result;
use super::Error::{Eof, InvalidInput, IoError};

pub trait ByteWrite {
    fn write_byte(&mut self, byte: u8) -> Result<()>;
}

pub trait BitWrite {
    fn write_bit(&mut self, bit: bool) -> Result<()>;
    fn flush_bits(&mut self) -> Result<()>;
}

pub struct BitWriter<'a> {
    output: &'a mut Write,
    next: u8,
    mask: u8,
    count: u64,
}

impl<'a> BitWriter<'a> {
    pub fn create(w: &'a mut Write) -> BitWriter<'a> {
        BitWriter { output: w, next: 0x00, mask: 0x80, count: 0 }
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }
}

impl<'a> ByteWrite for BitWriter<'a> {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        let buf = [byte];
        match self.output.write_all(&buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(IoError(e)),
        }
    }
}

impl<'a> BitWrite for BitWriter<'a> {
    fn write_bit(&mut self, bit: bool) -> Result<()> {
        if bit {
            self.next |= self.mask;
        }

        self.mask >>= 1;
        if self.mask == 0 {
            self.flush_bits()
        } else {
            Ok(())
        }
    }

    fn flush_bits(&mut self) -> Result<()> {
        if self.mask == 0x80 {
            Ok(())
        } else if let Err(e) = self.write_byte(self.next) {
            Err(e)
        } else {
            self.mask = 0x80;
            self.next = 0x00;
            Ok(())
        }
    }
}

pub trait ByteRead {
    fn read_byte(&mut self) -> Result<u8>;
}

pub trait BitRead {
    fn read_bit(&mut self) -> Result<bool>;
}

pub struct BitReader<'a> {
    input: &'a mut Read,
    next: u8,
    mask: u8,
    count: u64,
}

impl<'a> BitReader<'a> {
    pub fn create(r: &'a mut Read) -> BitReader<'a> {
        BitReader { input: r, next: 0x00, mask: 0x00, count: 0 }
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }
}

impl<'a> ByteRead for BitReader<'a> {
    fn read_byte(&mut self) -> Result<u8> {
        let buf = [0u8];
        match self.input.read(&mut buf) {
            Ok(0) => Err(Eof),
            Ok(_) => Ok(buf[0]),
            Err(e) => Err(IoError(e)),
        }
    }
}

impl<'a> BitRead for BitReader<'a> {
    fn read_bit(&mut self) -> Result<bool> {
        if self.mask == 0 {
            self.next = match self.read_byte() {
                Ok(b) => b,
                Err(e) => { return Err(e); }
            };
            self.mask = 0x80;
        }
        let bit = (self.next & self.mask) != 0;
        self.mask >>= 1;
        return Ok(bit);
    }
}

