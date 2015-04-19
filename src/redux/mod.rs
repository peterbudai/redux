use std::io::Read;
use std::io::Write;
use std::io::Result;

mod buffer;
mod codec;

use self::codec::Codec;

pub fn compress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut codec = Codec::init();
    try!(codec.compress(istream, ostream));
    return Ok(codec.get_metrics());
}

pub fn decompress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut codec = Codec::init();
    try!(codec.decompress(istream, ostream));
    return Ok(codec.get_metrics());
}

