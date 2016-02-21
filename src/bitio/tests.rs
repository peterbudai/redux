#![cfg(test)]

use std::io::Cursor;

use super::*;
use super::super::Error::Eof;

#[test]
fn test_read_eof()
{
    let mut data = Cursor::new(vec![]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(Err(Eof), reader.read_bit());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(Err(Eof), reader.read_bit());
    assert_eq!(Err(Eof), reader.read_byte());
}

#[test]
fn test_read_bytes() {
    let mut data = Cursor::new(vec![1u8, 2u8, 3u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(Ok(1u8), reader.read_byte());
    assert_eq!(Ok(2u8), reader.read_byte());
    assert_eq!(Ok(3u8), reader.read_byte());
    assert_eq!(Err(Eof), reader.read_byte());
}

#[test]
fn test_read_bits() {
    let mut data = Cursor::new(vec![0b10101010u8, 0b1111u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Err(Eof), reader.read_byte());
}

#[test]
fn test_read_mixed() {
    let mut data = Cursor::new(vec![0xAAu8, 0x00u8, 0x0Fu8, 0xF0u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(0x00u8), reader.read_byte());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(0xF0u8), reader.read_byte());
    assert_eq!(Err(Eof), reader.read_byte());
}
