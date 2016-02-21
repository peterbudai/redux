//! Adaptive arithmetic de/compression library.

use std::boxed::Box;
use std::io;
use std::result;
use std::fmt;
use self::codec::Codec;
use self::bitio::ByteCount;
use self::bitio::BitReader;
use self::bitio::BitWriter;

pub mod bitio;
pub mod codec;
pub mod model;

/// Possible errors that occur throughout this library
pub enum Error {
    /// The input stream has ended (unexpectedly)
    Eof,
    /// An invalid combination of data has occured on the input that
    /// the library was unable to process.
    InvalidInput,
    /// An I/O error occured.
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::Eof => f.write_str("Unexpected end of file"),
            Error::InvalidInput => f.write_str("Invalid data found while processing input"),
            Error::IoError(ref e) => f.write_fmt(format_args!("I/O error: {}", e)),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::Eof => f.write_str("Eof"),
            Error::InvalidInput => f.write_str("InvalidInput"),
            Error::IoError(ref e) => f.write_fmt(format_args!("IoError({:?})", e)),
        }
    }
}

#[cfg(test)]
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
       match *self {
           Error::Eof => match *other { Error::Eof => true, _ => false },
           Error::InvalidInput => match *other { Error::InvalidInput => true, _ => false },
           Error::IoError(_)  => match *other { Error::IoError(_) => true, _ => false },
       }
    }
}

/// Specialized `Result` type for the `redux` library.
pub type Result<T> = result::Result<T, Error>;

/// Compresses an entire byte stream using the given model and parameters.
pub fn compress(istream: &mut io::Read, ostream: &mut io::Write, model: Box<model::Model>) -> Result<(u64, u64)> {
    let mut codec = Codec::new(model);
    let mut input = BitReader::new(istream);
    let mut output = BitWriter::new(ostream);

    try!(codec.compress_bytes(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

/// Deompresses an entire byte stream using the given model and parameters.
pub fn decompress(istream: &mut io::Read, ostream: &mut io::Write, model: Box<model::Model>) -> Result<(u64, u64)> {
    let mut codec = Codec::new(model);
    let mut input = BitReader::new(istream);
    let mut output = BitWriter::new(ostream);

    try!(codec.decompress_bytes(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

#[cfg(test)]
mod tests {
    use super::Error::*;
    use std::io;
    
    macro_rules! assert_ne {
        ($a:expr, $b:expr) => ($a != $b)
    }

    #[test]
    fn test_error_eq() {
        assert_eq!(Eof, Eof);
        assert_eq!(InvalidInput, InvalidInput);
        assert_eq!(IoError(io::Error::new(io::ErrorKind::Other, "Other")), IoError(io::Error::new(io::ErrorKind::NotFound, "NotFound")));
        assert_ne!(Eof, InvalidInput);
        assert_ne!(InvalidInput, IoError(io::Error::new(io::ErrorKind::Other, "Other")));
        assert_ne!(IoError(io::Error::new(io::ErrorKind::NotFound, "NotFound")), Eof);
    }
}
