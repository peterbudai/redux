use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

const BUFFER_SIZE: usize = 256;
const EOF_VALUE: usize = 256;
const CODE_BITS: usize = 33;
const CODE_MIN: u64 = 0;
const CODE_MAX: u64 = (1 << CODE_BITS) - 1;
const ONE_FOURTH: u64 = 1 << (CODE_BITS - 2);
const ONE_HALF: u64 = 2 * ONE_FOURTH;
const THREE_FOURTHS: u64 = 3 * ONE_FOURTH;

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

    fn fill_buffer(&mut self, istream: &mut Read) -> Result<bool> {
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

    fn get_byte(&mut self, istream: &mut Read) -> Result<(u8, bool)> {
        let value = self.data[self.cur];
        self.cur += 1;
        if self.cur == self.len {
            return match self.fill_buffer(istream) {
                Ok(b) => Ok((value, b)),
                Err(e) => Err(e)
            }
        } else {
            return Ok((value, false));
        }
    }

    fn get_bit(&mut self, istream: &mut Read) -> Result<(u8, bool)> {
        let value = (self.data[self.cur] & (1 << self.bit)) >> self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.cur += 1;
            if self.cur == self.len {
                return match self.fill_buffer(istream) {
                    Ok(b) => Ok((value, b)),
                    Err(e) => Err(e)
                }
            }
        } else {
            self.bit -= 1;
        }
        return Ok((value, false));
    }

    fn flush_buffer(&mut self, ostream: &mut Write) -> Result<()> {
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

    fn put_byte(&mut self, value: u8, ostream: &mut Write) -> Result<()> {
        return Ok(());
    }

    fn put_bit(&mut self, value: u8, ostream: &mut Write) -> Result<()> {
        self.data[self.len] |= value << self.bit;

        if self.bit == 0 {
            self.bit = 7;
            self.len += 1;
            if self.len == self.data.len() {
                if let Err(e) = self.flush_buffer(ostream) {
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
            low: CODE_MIN,
            high: CODE_MAX,
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

    fn get_probability(&self, chr: usize) -> (u64, u64, u64) {
        (self.freq[chr], self.freq[chr + 1], self.freq[self.freq.len() - 1])
    }

    fn put_pending_bits(&mut self, bit: u8, ostream: &mut Write) -> Result<()> {
        try!(self.output.put_bit(bit, ostream));
        while self.pending > 0 {
            try!(self.output.put_bit(bit, ostream));
            self.pending -= 1;
        }
        return Ok(());
    }

    fn metrics(&self) -> (u64, u64) {
        (self.input.sum , self.output.sum)
    }
}

pub fn compress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut status = Status::init();

    let mut eof = try!(status.input.fill_buffer(istream));
    loop {
        let chr = if eof {
            EOF_VALUE
        } else {
            match status.input.get_byte(istream) {
                Ok((c, e)) => { eof = e; c as usize },
                Err(e) => { return Err(e); }
            }
        };

        {
            let (low, high, count) = status.get_probability(chr);
            status.range = status.high - status.low + 1;
            status.high = status.low + (status.range * high / count) - 1;
            status.low = low + (status.range * low / count);
        }

        loop {
           if status.high < ONE_HALF {
               try!(status.put_pending_bits(0, ostream));
           } else if status.low >= ONE_HALF {
               try!(status.put_pending_bits(1, ostream));
           } else if status.low >= ONE_FOURTH && status.high < THREE_FOURTHS {
                status.pending += 1;
                status.low -= ONE_FOURTH;
                status.high -= ONE_FOURTH;
           } else {
               break;
           }

           status.high = ((status.high << 1) + 1) & CODE_MAX;
           status.low = (status.low << 1) & CODE_MAX;
        }

        if eof {
            break;
        }
    }

    status.pending += 1;
    if status.low < ONE_FOURTH {
        try!(status.put_pending_bits(0, ostream));
    } else {
        try!(status.put_pending_bits(1, ostream));
    }
    try!(status.output.flush_buffer(ostream));

    return Ok(status.metrics());
}

pub fn decompress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut status = Status::init();

    let mut eof = try!(status.input.fill_buffer(istream));
    loop {
        break;
    }
    try!(status.output.flush_buffer(ostream));

    return Ok(status.metrics());
}

