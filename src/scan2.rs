use std::io::{self, BufReader, Read, Seek, SeekFrom};

pub struct Scan2<R: Read> {
    rdr: BufReader<R>,
    curr_byte: [u8; 1],
    consumed_curr_byte: bool,
    buf: Vec<u8>,
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
        }
    }

    pub fn seek(&mut self, seek_from: SeekFrom) -> io::Result<u64> {
        self.rdr.seek(seek_from)
    }

    pub fn next_i32(&mut self, i: &mut i32) -> bool {
        // eprintln!(">> in next_i32()");
        self.buf.clear();

        // skip the whitespace
        loop {
            // setup for the next byte (reading a new one, if we have already preocessed the
            // curr_byte)
            if self.consumed_curr_byte {
                match self.rdr.read_exact(&mut self.curr_byte) {
                    Ok(_) => {
                        self.consumed_curr_byte = false;
                    }
                    Err(_) => {
                        return false;
                    }
                }
            }

            if self.curr_byte[0].is_ascii_whitespace() {
                self.consumed_curr_byte = true;
                // eprintln!(">> skipping whitespace..");
            } else {
                break;
            }
        }

        // the first byte might be a sign
        if self.consumed_curr_byte {
            match self.rdr.read_exact(&mut self.curr_byte) {
                Ok(_) => {
                    self.consumed_curr_byte = false;
                }
                Err(_) => {
                    return false;
                }
            }
        }
        let b = self.curr_byte[0];
        if b == b'-' || b == b'+' || b.is_ascii_digit() {
            // eprintln!(">> pushing first byte ({b}) to buf...");
            self.buf.push(b);
            self.consumed_curr_byte = true;
        } else {
            // wrong first character, so we return a false
            return false;
        }

        // from now on, we read till the next whitespace
        loop {
            if self.consumed_curr_byte {
                match self.rdr.read_exact(&mut self.curr_byte) {
                    Ok(_) => {
                        self.consumed_curr_byte = false;
                    }
                    Err(_) => {
                        return false;
                    }
                }
            }
            let b = self.curr_byte[0];
            self.consumed_curr_byte = true;
            if b.is_ascii_whitespace() {
                // eprintln!(">> encountered a whitespace, beaking out...");
                break;
            }
            // eprintln!(">> pushing byte ({b}) to buf...");
            self.buf.push(b);
        }

        // convert the buf to a str
        let n_str = match std::str::from_utf8(&self.buf) {
            Ok(n_str) => n_str,
            Err(_) => {
                // eprintln!(
                //     ">> current buf ({:?}) could not be parsed as a str, skipping it",
                //     self.buf
                // );
                return self.next_i32(i);
            }
        };

        // finally parse the str
        match n_str.parse::<i32>() {
            Ok(n) => {
                // eprintln!("[*] setting *i to {n}");
                *i = n;
            }
            Err(_) => {
                // eprintln!(
                //     ">> current buf ({:?}) could not be parsed as a i32, skipping it",
                //     self.buf
                // );
                return self.next_i32(i);
            }
        }

        true
    }
}
