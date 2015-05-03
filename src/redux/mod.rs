mod bitio;
mod codec;
mod model;

use std::io;
use std::result;
use self::codec::Codec;
use self::bitio::BitReader;
use self::bitio::BitWriter;
use self::model::adaptive_linear::AdaptiveLinearModel;

/// Possible errors that occur throughout this library
pub enum Error {
    /// The input stream has ended (unexpectedly)
    Eof,
    /// An invalid combination of data has occured on the input that
    /// the library was unable to process.
    InvalidInput,
    /// An I/O error occured.
    IoError(io::Error)
}

pub type Result<T> = result::Result<T, Error>;

pub fn compress(istream: &mut io::Read, ostream: &mut io::Write) -> Result<(u64, u64)> {
    let mut model = AdaptiveLinearModel::<u64>::init(16, 14);
    let mut codec = Codec::init(&mut model);
    let mut input = BitReader::create(istream);
    let mut output = BitWriter::create(ostream);

    try!(codec.compress(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

pub fn decompress(istream: &mut io::Read, ostream: &mut io::Write) -> Result<(u64, u64)> {
    let mut model = AdaptiveLinearModel::<u64>::init(16, 14);
    let mut codec = Codec::init(&mut model);
    let mut input = BitReader::create(istream);
    let mut output = BitWriter::create(ostream);

    try!(codec.decompress(&mut input, &mut output));
    return Ok((input.get_count(), output.get_count()));
}

