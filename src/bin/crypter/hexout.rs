
use std::io::{Write, Stdout, stdout, Result};

pub fn stdhex() -> Hexout {
    Hexout::new(stdout())
}

pub struct Hexout(Stdout);

impl Hexout {
    pub fn new(os: Stdout) -> Self {
        Hexout(os)
    }
}

impl Write for Hexout {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(::hex::encode(buf).as_bytes())
    }
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
    fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.0.write_all(::hex::encode(buf).as_bytes())
    }
}
