//! Bit level I/O operations.

#[cfg(test)]
mod tests;

use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use super::Result;
use super::Error::{Eof, IoError, InvalidInput};

/// A trait for counting the number of bytes flowing trough a `Read` or `Write` implementation.
pub trait ByteCount {
    /// Returns the number of bytes in this stream.
    fn get_count(&self) -> u64;
}

/// A trait for object that allow reading symbols of variable bit lengths.
pub trait BitRead {
    /// Reads a single symbol from the input.
    fn read_bits(&mut self, bits: usize) -> Result<usize>;
}

/// A trait for object that allows writing symbols of variable bit lengths.
pub trait BitWrite {
    /// Writes a single symbol to the output.
    fn write_bits(&mut self, symbol: usize, bits: usize) -> Result<()>;
    /// Flushes all remaining bits to the output after the last whole octet, padded with zero bits.
    fn flush_bits(&mut self) -> Result<()>;
}

/// Common data fields used by the reader and writer implementations.
struct BitBuffer {
    /// Byte buffer that holds currently unread or unwritten bits.
    bytes: [u8; 1usize],
    /// Number of uread or unwritten bits in the buffer.
    bits: usize,
    /// Number of bytes read or written to the underlying buffer.
    count: u64,
}

impl BitBuffer {
    /// Creates a new instance with an empty buffer.
    fn new() -> BitBuffer {
        BitBuffer {
            bytes: [0u8; 1usize],
            bits: 0usize,
            count: 0u64,
        }
    }
}

/// Actual implementation of bit-oriented input reader.
pub struct BitReader<'a> {
    /// Temporary buffer to store unused bits.
    buffer: BitBuffer,
    /// Underlying byte-oriented I/O stream.
    input: &'a mut Read,
}

impl<'a> BitReader<'a> {
    /// Creates a new instance by wrapping a byte input stream.
    pub fn new(reader: &'a mut Read) -> BitReader<'a> {
        BitReader {
            buffer: BitBuffer::new(),
            input: reader,
        }
    }
}

impl<'a> ByteCount for BitReader<'a> {
    fn get_count(&self) -> u64 {
        self.buffer.count
    }
}

impl<'a> BitRead for BitReader<'a> {
    fn read_bits(&mut self, mut bits: usize) -> Result<usize> {
        if bits > size_of::<usize>() * 8 {
            return Err(InvalidInput);
        }

        let mut result = 0usize;
        while bits > 0 {
            if self.buffer.bits >= bits {
                // Get the upper bits from buffer (bytes: 000xxxyy -> 00000xxx)
                result <<= bits;
                result |= self.buffer.bytes[0] as usize >> (self.buffer.bits - bits);
                // Update buffer (bytes: 000xxxyy -> 000000yy)
                self.buffer.bits -= bits;
                self.buffer.bytes[0] &= (1 << self.buffer.bits) - 1;
                // Update reamining bits to read
                bits = 0
            } else if self.buffer.bits > 0 {
                // Get remaining bits from the buffer (bytes: 000000yy)
                result <<= self.buffer.bits;
                result |= self.buffer.bytes[0] as usize;
                // Update reamining bits to read
                bits -= self.buffer.bits;
                // Update buffer
                self.buffer.bytes[0] = 0;
                self.buffer.bits = 0;
            } else {
                // Read next byte from the underlying input stream
                match self.input.read(&mut self.buffer.bytes) {
                    Ok(0) => {
                        return Err(Eof);
                    },
                    Ok(_) => {
                        self.buffer.count += 1;
                        self.buffer.bits = 8;
                    },
                    Err(e) => {
                        return Err(IoError(e));
                    }
                }
            }
        }
        return Ok(result);
    }
}

/// Actual implementation of bit-oriented outour writer.
pub struct BitWriter<'a> {
    /// Temporary buffer to store unused bits.
    buffer: BitBuffer,
    /// Underlying byte-oriented I/O stream.
    output: &'a mut Write,
}

impl<'a> BitWriter<'a> {
    /// Creates a new instance by wrapping a byte output stream.
    pub fn new(writer: &'a mut Write) -> BitWriter<'a> {
        BitWriter {
            buffer: BitBuffer::new(),
            output: writer
        }
    }
}

impl<'a> ByteCount for BitWriter<'a> {
    fn get_count(&self) -> u64 {
        self.buffer.count
    }
}

impl<'a> BitWrite for BitWriter<'a> {
    fn write_bits(&mut self, mut symbol: usize, mut bits: usize) -> Result<()> {
        if (bits > size_of::<usize>() * 8) || (symbol >> bits > 0){
            return Err(InvalidInput);
        }

        while bits > 0 {
            if self.buffer.bits + bits <= 8 {
                // Put the upper bits into buffer (symbol: 00000yyy, bytes: 000000xx -> 000xxyyy)
                if self.buffer.bits > 0 {
                    self.buffer.bytes[0] <<= bits;
                }
                self.buffer.bytes[0] |= symbol as u8;
                self.buffer.bits += bits;
                // Update remaining bits to write
                bits = 0;
                symbol = 0;
            } else if self.buffer.bits < 8 {
                let num = 8 - self.buffer.bits;
                // Put the upper bits into buffer (symbol: 000yyyzz -> 00000yyy, bytes: 000xxxxx -> xxxxxyyy)
                if self.buffer.bits > 0 {
                    self.buffer.bytes[0] <<= num;
                }
                self.buffer.bytes[0] |= (symbol >> (bits - num)) as u8;
                self.buffer.bits += num;
                // Update remaining bits to write (symbol: 000yyyzz -> 000000zz)
                bits -= num;
                symbol &= (1 << bits) - 1;
            }
            if self.buffer.bits == 8 {
                try!(self.flush_bits())
            }
        }
        return Ok(())
    }

    fn flush_bits(&mut self) -> Result<()> {
        if self.buffer.bits > 0 {
            self.buffer.bytes[0] <<= 8 - self.buffer.bits;
            match self.output.write_all(&self.buffer.bytes) {
                Ok(_) => {
                    self.buffer.count += 1;
                    self.buffer.bytes[0] = 0;
                    self.buffer.bits = 0;
                },
                Err(e) => {
                    return Err(IoError(e));
                }
            }
        }
        return Ok(())
    }
}

