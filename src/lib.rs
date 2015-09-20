use std::io;
use std::result;
use std::fmt;
use self::codec::Codec;
use self::bitio::BitReader;
use self::bitio::BitWriter;
use self::model::Parameters;
use self::model::adaptive_linear::AdaptiveLinearModel;

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

pub type Result<T> = result::Result<T, Error>;

pub fn compress(istream: &mut io::Read, ostream: &mut io::Write) -> Result<(u64, u64)> {
    return compress_custom(istream, ostream, AdaptiveLinearModel::init(try!(Parameters::init(8, 14, 16))));
}

pub fn compress_custom(istream: &mut io::Read, ostream: &mut io::Write, model: Box<model::Model>) -> Result<(u64, u64)> {
    let mut codec = Codec::init(model);
    let mut input = BitReader::create(istream);
    let mut output = BitWriter::create(ostream);

    try!(codec.compress_bytes(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

pub fn decompress(istream: &mut io::Read, ostream: &mut io::Write) -> Result<(u64, u64)> {
    return decompress_custom(istream, ostream, AdaptiveLinearModel::init(try!(Parameters::init(8, 14, 16))));
}

pub fn decompress_custom(istream: &mut io::Read, ostream: &mut io::Write, model: Box<model::Model>) -> Result<(u64, u64)> {
    let mut codec = Codec::init(model);
    let mut input = BitReader::create(istream);
    let mut output = BitWriter::create(ostream);

    try!(codec.decompress_bytes(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

