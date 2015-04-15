use std::io::Read;
use std::io::Write;
use std::result::Result;

struct Buffer {
    data: [u8; 16384],
    len: usize,
    cur: usize,
    bit: u8
}

impl Buffer {
    fn create() -> Buffer {
        return Buffer { data: [0u8; 16384], len: 0us, cur: 0us, bit: 0u8 };
    }

    fn fill(&mut self, istream: &mut Read) -> bool {
        if self.len > 0 {
            true
        } else {
            match istream.read(&mut self.data) {
                Ok(0) => false,
                Ok(n) => { self.len = n; true },
                Err(_) => false
            }
        }
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
    while st.input.fill(istream) {
        println!("length: {}", st.input.len);
        st.input.len = 0;
    }
}

