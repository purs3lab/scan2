use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

pub struct Scan2<R: Read> {
    rdr: BufReader<R>,
    curr_byte: [u8; 1],
    consumed_curr_byte: bool,
    buf: Vec<u8>,
    consumed_curr_buf: bool,
}

impl<R> Scan2<R>
where
    R: Read + Seek,
{
    pub fn new(rdr: R) -> Self {
        Self {
            rdr: BufReader::new(rdr),

            curr_byte: [0; 1],
            consumed_curr_byte: true,

            // 40 is enought for an 128 bit int
            buf: Vec::with_capacity(40),
            consumed_curr_buf: true,
        }
    }

    pub fn seek(&mut self, seek_from: SeekFrom) -> io::Result<u64> {
        self.consumed_curr_byte = true;
        self.buf.clear();
        self.consumed_curr_buf = true;
        self.rdr.seek(seek_from)
    }

    fn parse_buf_i32(&mut self) -> Option<i32> {
        let buf_str = std::str::from_utf8(&self.buf).ok()?;
        buf_str.parse::<i32>().ok()
    }

    fn read_byte(&mut self) -> io::Result<()> {
        self.rdr.read_exact(&mut self.curr_byte)
    }

    pub fn is_eof(&mut self) -> io::Result<bool> {
        let buffer = self.rdr.fill_buf()?;
        return Ok(buffer.is_empty());
    }
        
    pub fn next_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.rdr.read_exact(&mut buf)?;
        return Ok(buf[0]);
    }

    pub fn next_i32(&mut self, i: &mut i32) -> io::Result<bool> {
        if self.consumed_curr_buf {
            // from now on, we read till the next non-digit character
            self.consumed_curr_buf = false;
            loop {
                if self.consumed_curr_byte {
                    match self.read_byte() {
                        Ok(_) => {}
                        Err(error) => match error.kind() {
                            io::ErrorKind::UnexpectedEof => break,
                            _ => return Err(error),
                        },
                    };
                    self.consumed_curr_byte = false;
                }
                let b = self.curr_byte[0];
                if self.buf.is_empty() && b.is_ascii_whitespace() {
                    // skip whitespace
                    self.consumed_curr_byte = true;
                    continue;
                } else if (self.buf.is_empty() && (b == b'-' || b == b'+')) || b.is_ascii_digit() {
                    // eprintln!(">> pushing byte ({b}) to buf...");
                    self.consumed_curr_byte = true;
                    self.buf.push(b);
                } else {
                    break;
                }
            }
        } else {
            // re-preocess existing buf
            match self.parse_buf_i32() {
                Some(n) => {
                    self.buf.clear();
                    self.consumed_curr_buf = true;
                    *i = n;
                    return Ok(true);
                }
                None => {
                    return Ok(false);
                }
            }
        }

        if self.consumed_curr_buf {
            return Ok(false);
        }

        match self.parse_buf_i32() {
            Some(n) => {
                self.buf.clear();
                self.consumed_curr_buf = true;
                *i = n;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
