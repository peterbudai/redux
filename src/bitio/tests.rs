#![cfg(test)]

use std::io::Cursor;

use super::*;
use super::super::Error::Eof;

#[test]
fn write_empty() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(0, writer.get_count());
    }
    assert_eq!(0, data.len());
}

#[test]
fn write_bytes() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_byte(1u8));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_byte(2u8));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_byte(3u8));
        assert_eq!(3, writer.get_count());
    }
    assert_eq!(vec![1u8, 2u8, 3u8], data);
}

#[test]
fn write_bits() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(2, writer.get_count());
    }
    assert_eq!(vec![0b10101010u8, 0b1111u8], data);
}

#[test]
fn write_mixed() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_byte(0x00u8));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(3, writer.get_count());
        assert_eq!(Ok(()), writer.write_byte(0xF0u8));
        assert_eq!(4, writer.get_count());
    }
    assert_eq!(vec![0xAAu8, 0x00u8, 0x0Fu8, 0xF0u8], data);
}

#[test]
fn write_flush() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(Ok(()), writer.write_bit(true));
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bit(false));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(2, writer.get_count());
    }
    assert_eq!(vec![0xA0u8, 0x00u8], data);
}

#[test]
fn read_eof()
{
    let mut data = Cursor::new(vec![]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Err(Eof), reader.read_bit());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(Err(Eof), reader.read_bit());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(0, reader.get_count());
}

#[test]
fn read_bytes() {
    let mut data = Cursor::new(vec![1u8, 2u8, 3u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(1u8), reader.read_byte());
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(2u8), reader.read_byte());
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(3u8), reader.read_byte());
    assert_eq!(3, reader.get_count());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(3, reader.get_count());
}

#[test]
fn read_bits() {
    let mut data = Cursor::new(vec![0b10101010u8, 0b1111u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(2, reader.get_count());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(2, reader.get_count());
}

#[test]
fn read_mixed() {
    let mut data = Cursor::new(vec![0xAAu8, 0x00u8, 0x0Fu8, 0xF0u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0x00u8), reader.read_byte());
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(3, reader.get_count());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(false), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(Ok(true), reader.read_bit());
    assert_eq!(3, reader.get_count());
    assert_eq!(Ok(0xF0u8), reader.read_byte());
    assert_eq!(4, reader.get_count());
    assert_eq!(Err(Eof), reader.read_byte());
    assert_eq!(4, reader.get_count());
}
