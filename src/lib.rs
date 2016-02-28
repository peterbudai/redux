//! Adaptive arithmetic compression library.
//!
//! This crate provides standard [arithmetic coding](https://en.wikipedia.org/wiki/Arithmetic_coding) 
//! implementation that can use customized symbol probability models.
//! This crate offers two adaptive models: `AdaptiveLinearModel` and `AdaptiveTreeModel`. Adaptive 
//! models continuously update the symbol probability distribution with each encoded symbol.
//!
//! * `AdaptiveLinearModel` is a straightforward, but slow implementation, present mainly for
//! tasting and benchmarking purposes.
//! 
//! * `AdaptiveTreeModel` is a [Fenwick tree](https://en.wikipedia.org/wiki/Fenwick_tree)-based
//! implementation, it is advised to use this model for any uses.
//!
//! It is possible to use a custom model (it may or may not be adaptive) by implementing the 
//! `model::Model` trait.
//!
//! # Examples
//!
//! Any byte stream can be encoded and decoded that implements the `std::io::Read` trait, and the
//! output can be anything that implements `std::io::Write` trait. Thus, it is possible to process
//! files or memory objects as well.
//!
//! ```rust
//! use redux::model::*;
//!
//! let data = vec![114u8, 101u8, 100u8, 117u8, 120u8];
//!
//! // Encode
//! let mut cursor1 = std::io::Cursor::new(&data);
//! let mut compressed = Vec::<u8>::new();
//! redux::compress(&mut cursor1, &mut compressed, AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap()));
//!
//! // Decode
//! let mut cursor2 = std::io::Cursor::new(&compressed);
//! let mut decompressed = Vec::<u8>::new();
//! redux::decompress(&mut cursor2, &mut decompressed, AdaptiveTreeModel::new(Parameters::new(8, 14, 16).unwrap()));
//!
//! assert_eq!(decompressed, data);
//! ```

#![warn(missing_docs)]

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
    /// An invalid combination of data has occured on the input that the library was unable to process.
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

/// Compresses `istream` into `ostream` using the given `model`.
/// Returns the number of bytes both in the decompressed and compressed stream.
pub fn compress(istream: &mut io::Read, ostream: &mut io::Write, model: Box<model::Model>) -> Result<(u64, u64)> {
    let mut codec = Codec::new(model);
    let mut input = BitReader::new(istream);
    let mut output = BitWriter::new(ostream);

    try!(codec.compress_bytes(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

/// Decompresses `istream` into `ostream` using the given `model`.
/// Returns the number of bytes both in the compressed and decompressed stream.
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
    fn error_eq() {
        assert_eq!(Eof, Eof);
        assert_eq!(InvalidInput, InvalidInput);
        assert_eq!(IoError(io::Error::new(io::ErrorKind::Other, "Other")), IoError(io::Error::new(io::ErrorKind::NotFound, "NotFound")));
        assert_ne!(Eof, InvalidInput);
        assert_ne!(InvalidInput, IoError(io::Error::new(io::ErrorKind::Other, "Other")));
        assert_ne!(IoError(io::Error::new(io::ErrorKind::NotFound, "NotFound")), Eof);
    }
}
