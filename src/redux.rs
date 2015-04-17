use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

enum ReadResult<T> {
    Ok(T),
    Eof(T),
    Err(Error)
}

struct Buffer {
    data: [u8; 256],
    len: usize,
    cur: usize,
    bit: u8
}

impl Buffer {
    fn create() -> Buffer {
        return Buffer { 
            data: [0u8; 256], 
            len: 0us, 
            cur: 0us, 
            bit: 7u8
        };
    }

    fn fill(&mut self, istream: &mut Read) -> ReadResult<()> {
        match istream.read(&mut self.data[0..]) {
            Ok(0) => ReadResult::Eof(()),
            Ok(n) => { self.len = n; self.cur = 0; ReadResult::Ok(()) },
            Err(e) => ReadResult::Err(e)
        }
    }

    fn pull(&mut self, istream: &mut Read) -> ReadResult<u8> {
        let value = (self.data[self.cur] & (1 << self.bit)) >> self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.cur += 1;
            if self.cur == self.len {
                return match self.fill(istream) {
                    ReadResult::Ok(_) => ReadResult::Ok(value),
                    ReadResult::Eof(_) => ReadResult::Eof(value),
                    ReadResult::Err(e) => ReadResult::Err(e)
                }
            }
        } else {
            self.bit -= 1;
        }
        return ReadResult::Ok(value);
    }

    fn flush(&mut self, ostream: &mut Write) -> Result<()> {
        // Flush the remaining incomplete byte as well
        if self.bit < 7 {
            self.len += 1;
        }

        while self.cur < self.len {
            match ostream.write(&self.data[self.cur..self.len]) {
                Ok(0) => { return Err(Error::new(ErrorKind::WriteZero, "Zero bytes written to output")); }
                Ok(n) => { self.cur += n; },
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
}

pub fn compress(istream: &mut Read, ostream: &mut Write) {
    let mut st = Status::init();
    let mut v = 0u8;
    let mut count = 0u64;
/*
    if st.input.fill(istream) != IOResult::Success {
        return;
    }

    loop {
        r = st.input.pull(&mut v, istream);

        if st.output.push(v, ostream) == IOResult::Error { 
            r = IOResult::Error;
            break;
        }

        if r == IOResult::EOF {
            break;
        }

        
            count += 1;
        }
        st.output.flush(ostream);
    }
*/
    println!("{} bits", count);
}

