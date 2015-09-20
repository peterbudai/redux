//! Bit level I/O operations.

use std::io::Read;
use std::io::Write;
use super::Result;
use super::Error::{Eof, IoError};

/// A trait for object that allow writing one byte at a time.
pub trait ByteWrite {
    /// Writes a single byte to the output.
    fn write_byte(&mut self, byte: u8) -> Result<()>;
}

/// A trait for object that allow writing one bit at a time.
pub trait BitWrite {
    /// Writes a single bit to the output.
    fn write_bit(&mut self, bit: bool) -> Result<()>;
    /// Flushes all remaining bits to the output after the last whole octet.
    fn flush_bits(&mut self) -> Result<()>;
}

/// A class for wrapping a byte output stream in a bit based interface.
pub struct BitWriter<'a> {
    /// The actual byte output stream.
    output: &'a mut Write,
    /// The buffer for the bits of the next byte to output.
    next: [u8; 1],
    /// The mask that selects the next bit in the buffer.
    mask: u8,
    /// Number of bytes output.
    count: u64,
}

impl<'a> BitWriter<'a> {
    /// Creates a new instance by wrapping a byte output stream.
    pub fn new(w: &'a mut Write) -> BitWriter<'a> {
        BitWriter { output: w, next: [0x00], mask: 0x80, count: 0 }
    }

    /// Returns the number of bytes written to the output.
    pub fn get_count(&self) -> u64 {
        self.count
    }

    /// Writes the next byte from the bit buffer to the output stream.
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

/// A trait for object that allow reading one byte at a time.
pub trait ByteRead {
    /// Reads a single byte from the input.
    fn read_byte(&mut self) -> Result<u8>;
}

/// A trait for object that allow reading one bit at a time.
pub trait BitRead {
    /// Reads a single bit from the input.
    fn read_bit(&mut self) -> Result<bool>;
}

/// A class for wrapping a byte input stream in a bit based interface.
pub struct BitReader<'a> {
    /// The actual byte input stream.
    input: &'a mut Read,
    /// The buffer for the bits of the current byte being input.
    next: [u8; 1],
    /// The mask that selects the next bit in the buffer.
    mask: u8,
    /// Number of bytes input.
    count: u64,
}

impl<'a> BitReader<'a> {
    /// Creates a new instance by wrapping a byte input stream.
    pub fn new(r: &'a mut Read) -> BitReader<'a> {
        BitReader { input: r, next: [0x00], mask: 0x00, count: 0 }
    }

    /// Returns the number of bytes read from the input.
    pub fn get_count(&self) -> u64 {
        self.count
    }

    /// Reads the next byte from the input stream to the bit buffer.
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

