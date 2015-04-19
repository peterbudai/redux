use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::io::Error;
use std::io::ErrorKind;

const BUFFER_SIZE: usize = 2048;

const VALUE_BITS: usize = 8;
const VALUE_EOF: usize = 1 << VALUE_BITS;
const VALUE_COUNT: usize = VALUE_EOF + 2;

const CODE_BITS: usize = 16;
const CODE_MIN: u64 = 0;
const CODE_MAX: u64 = (1 << CODE_BITS) - 1;
const ONE_FOURTH: u64 = 1 << (CODE_BITS - 2);
const ONE_HALF: u64 = 2 * ONE_FOURTH;
const THREE_FOURTHS: u64 = 3 * ONE_FOURTH;

const FREQ_BITS: usize = 14;
const FREQ_MAX: u64 = (1 << FREQ_BITS) - 1;

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
        self.data[self.len] = value;
        self.len += 1;
        if self.len == self.data.len() {
            if let Err(e) = self.flush_buffer(ostream) {
                return Err(e);
            }
        }
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
    freq: [u64; VALUE_COUNT],
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
            freq: [0u64; VALUE_COUNT],
            input: Buffer::create(),
            output: Buffer::create()
        };
        for i in 1..258 {
            res.freq[i] = i as u64;
        }
        return res;
    }

    fn get_count(&self) -> u64 {
        self.freq[self.freq.len() - 1]
    }

    fn get_probability(&mut self, chr: usize) -> (u64, u64, u64) {
        let res = (self.freq[chr], self.freq[chr + 1], self.get_count());
        self.update(chr);
        return res;
    }

    fn get_char(&mut self, scaled: u64) -> (usize, u64, u64) {
        let chr = match self.freq.binary_search(&scaled) {
            Ok(i) => i,
            Err(i) => i - 1
        };
        let res = (chr, self.freq[chr], self.freq[chr + 1]);
        self.update(chr);
        return res;
    }

    fn update(&mut self, chr: usize) {
        if self.get_count() < FREQ_MAX {
            for index in chr+1..self.freq.len() {
                self.freq[index] += 1;
            }
        }
    }

    fn put_pending_bits(&mut self, bit: u8, ostream: &mut Write) -> Result<()> {
        try!(self.output.put_bit(bit, ostream));
        while self.pending > 0 {
            try!(self.output.put_bit(!bit & 1u8, ostream));
            self.pending -= 1;
        }
        return Ok(());
    }

    fn metrics(&self) -> (u64, u64) {
        (self.input.sum, self.output.sum)
    }
}

pub fn compress(istream: &mut Read, ostream: &mut Write) -> Result<(u64, u64)> {
    let mut status = Status::init();

    let mut eof = try!(status.input.fill_buffer(istream));
    loop {
        let chr = if eof {
            VALUE_EOF
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
            status.low = status.low + (status.range * low / count);
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

        if chr == VALUE_EOF {
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
    let mut value = 0u64;

    if try!(status.input.fill_buffer(istream)) {
        return Err(Error::new(ErrorKind::InvalidInput, "Empty input stream"));
    }
    let mut eof = false;
    for _ in 0..CODE_BITS {
        value = if eof {
            (value << 1) | (value & 1u64)
        } else {
            match status.input.get_bit(istream) {
                Ok((b, e)) => { eof = e; (value << 1) | (b as u64) },
                Err(e) => { return Err(e); }
            }
        }
    }

    loop {
        {
            status.range = status.high - status.low + 1;
            let count = status.get_count();
            let scaled = ((value - status.low + 1) * count - 1) / status.range;
            let (chr, low, high) = status.get_char(scaled);

            if chr == VALUE_EOF {
                break;
            } else {
                try!(status.output.put_byte(chr as u8, ostream));
            }

            status.high = status.low + (status.range * high / count) - 1;
            status.low = status.low + (status.range * low / count);
        }

        loop {
            if status.high < ONE_HALF {
                // do nothing
            } else if status.low >= ONE_HALF {
                value -= ONE_HALF;
                status.low -= ONE_HALF;
                status.high -= ONE_HALF;
            } else if status.low >= ONE_FOURTH && status.high < THREE_FOURTHS {
                value -= ONE_FOURTH;
                status.low -= ONE_FOURTH;
                status.high -= ONE_FOURTH;
            } else {
                break;
            }

            status.low = status.low << 1;
            status.high = (status.high << 1) + 1;

            value = if eof {
                return Err(Error::new(ErrorKind::InvalidInput, "Unexpected end of input stream"));
            } else {
                match status.input.get_bit(istream) {
                    Ok((b, e)) => { eof = e; (value << 1) | (b as u64) },
                    Err(e) => { return Err(e); }
                }
            }
        }
    }

    try!(status.output.flush_buffer(ostream));
    return Ok(status.metrics());
}

