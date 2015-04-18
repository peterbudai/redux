use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

const BUFFER_SIZE : usize = 256us;

struct Buffer {
    data: [u8; BUFFER_SIZE],
    len: usize,
    cur: usize,
    bit: u8,
    sum: u64
}

impl Buffer {
    fn create() -> Buffer {
        return Buffer { 
            data: [0u8; BUFFER_SIZE], 
            len: 0us, 
            cur: 0us, 
            bit: 7u8,
            sum: 0u64
        };
    }

    fn fill(&mut self, istream: &mut Read) -> Result<bool> {
        match istream.read(&mut self.data[0..]) {
            Ok(0) => Ok(true),
            Ok(n) => { 
                self.sum += n as u64;
                self.len = n; 
                self.cur = 0; 
                Ok(false) 
            },
            Err(e) => Err(e)
        }
    }

    fn pull(&mut self, istream: &mut Read) -> Result<(u8, bool)> {
        let value = (self.data[self.cur] & (1 << self.bit)) >> self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.cur += 1;
            if self.cur == self.len {
                return match self.fill(istream) {
                    Ok(b) => Ok((value, b)),
                    Err(e) => Err(e)
                }
            }
        } else {
            self.bit -= 1;
        }
        return Ok((value, false));
    }

    fn flush(&mut self, ostream: &mut Write) -> Result<()> {
        // Flush the remaining incomplete byte as well
        if self.bit < 7 {
            self.len += 1;
        }

        while self.cur < self.len {
            match ostream.write(&self.data[self.cur..self.len]) {
                Ok(0) => { return Err(Error::new(ErrorKind::WriteZero, "Zero bytes written to output")); }
                Ok(n) => { 
                    self.cur += n; 
                    self.sum += n as u64;
                },
                Err(e) => { return Err(e); }
            }
        }

        self.len = 0;
        self.cur = 0;
        return Ok(());
    }

    fn push(&mut self, value: u8, ostream: &mut Write) -> Result<()> {
        self.data[self.len] |= value << self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.len += 1;
            if self.len == self.data.len() {
                if let Err(e) = self.flush(ostream) {
                    return Err(e);
                }
            }
            self.data[self.len] = 0;
        } else {
            self.bit -= 1;
        }
        return Ok(());
    }
}

struct Status {
    low: u64,
    high: u64,
    range: u64,
    pending: u8,
    freq: [u64; 258],
    input: Buffer,
    output: Buffer
}

impl Status {
    fn init() -> Status {
        let mut res = Status { 
            low: 0x00000000_00000000u64, 
            high: 0xFFFFFFFF_FFFFFFFFu64, 
            range: 0u64, 
            pending: 0u8, 
            freq: [0u64; 258],
            input: Buffer::create(),
            output: Buffer::create()
        };
        for i in 1..258 {
            res.freq[i] = i as u64;
        }
        return res;
    }

    fn metrics(&self) -> (u64, u64) {
        return (self.input.sum , self.output.sum);
    }
}

pub fn compress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut st = Status::init();
    let mut count = 0u64;

    if try!(st.input.fill(istream)) {
        return Ok(st.metrics());
    }

    loop {
        let (v, eof) = try!(st.input.pull(istream));
        try!(st.output.push(v, ostream));

        count += 1;
        if count >= 236 || eof {
            break;
        }
    }

    try!(st.output.flush(ostream));

    println!("{} bits", count);
    return Ok(st.metrics());
}

