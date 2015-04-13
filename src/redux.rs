use std::io::Read;
use std::io::Write;

struct Status {
    low: u64,
    high: u64,
    range: u64,
    pending: u8,
    freq: [u64; 258],
    buf: [u8; 65536],
    len: usize
}

impl Status {
    fn init() -> Status {
        let mut res = Status { 
            low: 0x00000000_00000000u64, 
            high: 0xFFFFFFFF_FFFFFFFFu64, 
            range: 0u64, 
            pending: 0u8, 
            freq: [0u64; 258],
            buf: [0u8; 65536],
            len: 0us
        };
        for i in 1..258 {
            res.freq[i] = i as u64;
        }
        return res;
    }
}

pub fn compress(input: &mut Read, output: &mut Write) {
    let st = Status::init();
}

