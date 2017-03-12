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
        assert_eq!(Ok(()), writer.write_bits(0x01, 8));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0x02, 8));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0x03, 8));
        assert_eq!(3, writer.get_count());
    }
    assert_eq!(vec![0x01u8, 0x02u8, 0x03u8], data);
}

#[test]
fn write_bits() {
    let mut data = Vec::new();
    {
        let mut writer = BitWriter::new(&mut data);
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(1, 1));
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
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0x00, 8));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(2, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(3, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0xF0, 8));
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
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(Ok(()), writer.write_bits(1, 1));
        assert_eq!(Ok(()), writer.write_bits(0, 1));
        assert_eq!(0, writer.get_count());
        assert_eq!(Ok(()), writer.flush_bits());
        assert_eq!(1, writer.get_count());
        assert_eq!(Ok(()), writer.write_bits(0, 1));
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
    assert_eq!(Err(Eof), reader.read_bits(1));
    assert_eq!(Err(Eof), reader.read_bits(8));
    assert_eq!(Err(Eof), reader.read_bits(1));
    assert_eq!(Err(Eof), reader.read_bits(8));
    assert_eq!(0, reader.get_count());
}

#[test]
fn read_bytes() {
    let mut data = Cursor::new(vec![0x01u8, 0x02u8, 0x03u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(0x01), reader.read_bits(8));
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0x02), reader.read_bits(8));
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(0x03), reader.read_bits(8));
    assert_eq!(3, reader.get_count());
    assert_eq!(Err(Eof), reader.read_bits(8));
    assert_eq!(3, reader.get_count());
}

#[test]
fn read_bits() {
    let mut data = Cursor::new(vec![0b10101010u8, 0b1111u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(2, reader.get_count());
    assert_eq!(Err(Eof), reader.read_bits(8));
    assert_eq!(2, reader.get_count());
}

#[test]
fn read_mixed() {
    let mut data = Cursor::new(vec![0xAAu8, 0x00u8, 0x0Fu8, 0xF0u8]);
    let mut reader = BitReader::new(&mut data);
    assert_eq!(0, reader.get_count());
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(1, reader.get_count());
    assert_eq!(Ok(0x00), reader.read_bits(8));
    assert_eq!(2, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(3, reader.get_count());
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(0usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(Ok(1usize), reader.read_bits(1));
    assert_eq!(3, reader.get_count());
    assert_eq!(Ok(0xF0), reader.read_bits(8));
    assert_eq!(4, reader.get_count());
    assert_eq!(Err(Eof), reader.read_bits(8));
    assert_eq!(4, reader.get_count());
}
