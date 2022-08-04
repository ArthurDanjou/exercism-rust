use std::io::{Read, Result, Write};
use std::marker::PhantomData;

pub struct ReadStats<R> {
    wrapped: R,
    _phantom: PhantomData<R>,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        Self {
            wrapped: _wrapped,
            _phantom: PhantomData,
            reads: 0,
            bytes_through: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let reads_bytes = self.wrapped.read(buf)?;
        self.bytes_through += reads_bytes;
        self.reads += 1;
        Ok(reads_bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    _phantom: PhantomData<W>,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        Self {
            wrapped: _wrapped,
            _phantom: PhantomData,
            bytes_through: 0,
            writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let write_bytes = self.wrapped.write(buf)?;
        self.bytes_through += write_bytes;
        self.writes += 1;
        Ok(write_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
