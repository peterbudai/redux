//! Bit level I/O operations.

use std::io::Read;
use std::io::Write;
use super::Result;
use super::Error::{Eof, IoError};

/// A trait for object that allow reading one byte at a time.
///
/// This trait can be considered as an extension to (std::io::Write) with a simpler method signature.
pub trait ByteWrite {
    /// Writes a single byte to the output.
    ///
    /// # Failures
    /// If the byte could not be written IoError is returned.
    fn write_byte(&mut self, byte: u8) -> Result<()>;
}

pub trait BitWrite {
    fn write_bit(&mut self, bit: bool) -> Result<()>;
    fn flush_bits(&mut self) -> Result<()>;
}

pub struct BitWriter<'a> {
    output: &'a mut Write,
    next: [u8; 1],
    mask: u8,
    count: u64,
}

impl<'a> BitWriter<'a> {
    pub fn create(w: &'a mut Write) -> BitWriter<'a> {
        BitWriter { output: w, next: [0x00], mask: 0x80, count: 0 }
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn write_next(&mut self) -> Result<()> {
        match self.output.write_all(&self.next) {
            Ok(_) => { self.count += 1; Ok(()) },
            Err(e) => Err(IoError(e)),
        }
    }
}

impl<'a> ByteWrite for BitWriter<'a> {
    fn write_byte(&mut self, byte: u8) -> Result<()> {
        self.next[0] = byte;
        self.write_next()
    }
}

impl<'a> BitWrite for BitWriter<'a> {
    fn write_bit(&mut self, bit: bool) -> Result<()> {
        if bit {
            self.next[0] |= self.mask;
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
        } else if let Err(e) = self.write_next() {
            Err(e)
        } else {
            self.mask = 0x80;
            self.next[0] = 0x00;
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
    next: [u8; 1],
    mask: u8,
    count: u64,
}

impl<'a> BitReader<'a> {
    pub fn create(r: &'a mut Read) -> BitReader<'a> {
        BitReader { input: r, next: [0x00], mask: 0x00, count: 0 }
    }

    pub fn get_count(&self) -> u64 {
        self.count
    }

    fn read_next(&mut self) -> Result<()> {
        match self.input.read(&mut self.next) {
            Ok(0) => Err(Eof),
            Ok(_) => { self.count += 1; Ok(()) },
            Err(e) => Err(IoError(e)),
        }
    }
}

impl<'a> ByteRead for BitReader<'a> {
    fn read_byte(&mut self) -> Result<u8> {
        match self.read_next() {
            Ok(()) => Ok(self.next[0]),
            Err(e) => Err(e),
        }
    }
}

impl<'a> BitRead for BitReader<'a> {
    fn read_bit(&mut self) -> Result<bool> {
        if self.mask == 0 {
            if let Err(e) = self.read_next() {
                return Err(e);
            }
            self.mask = 0x80;
        }
        let bit = (self.next[0] & self.mask) != 0;
        self.mask >>= 1;
        return Ok(bit);
    }
}

