use std::io::Read;
use std::io::Write;

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

    fn fill(&mut self, istream: &mut Read) -> bool {
        match istream.read(&mut self.data[0..]) {
            Ok(0) => false,
            Ok(n) => { self.len = n; self.cur = 0; true },
            Err(_) => false
        }
    }

    fn pull(&mut self, value: &mut u8, istream: &mut Read) -> bool {
        *value = (self.data[self.cur] & (1 << self.bit)) >> self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.cur += 1;
            if self.cur == self.len && !self.fill(istream) {
                return false;
            }
        } else {
            self.bit -= 1;
        }
        return true;
    }

    fn flush(&mut self, ostream: &mut Write) -> bool {
        // Flush the remaining incomplete byte as well
        if self.bit < 7 {
            self.len += 1;
        }

        match ostream.write(&self.data[..self.len]) {
            Ok(n) => { self.len -= n; self.len == 0 },
            _ => false
        }
    }

    fn push(&mut self, value: u8, ostream: &mut Write) -> bool {
        self.data[self.len] |= value << self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.len += 1;
            if self.len == self.data.len() && !self.flush(ostream) {
                return false;
            }
            self.data[self.len] = 0;
        } else {
            self.bit -= 1;
        }
        return true;
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

    st.input.fill(istream);
    while st.input.pull(&mut v, istream) && st.output.push(v, ostream) {
        count += 1;
    }
    st.output.flush(ostream);

    println!("{} bits", count);
}

