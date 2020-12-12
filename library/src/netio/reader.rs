use byteorder::{ByteOrder, ReadBytesExt};
use bytes::BytesMut;
use std::io;
use std::io::Cursor;

pub enum IOReadErrorValue {
    NotEnoughBytes,
    IO(io::Error),
}

pub struct IOReadError {
    pub value: IOReadErrorValue,
}

impl From<IOReadErrorValue> for IOReadError {
    fn from(val: IOReadErrorValue) -> Self {
        IOReadError { value: val }
    }
}

impl From<io::Error> for IOReadError {
    fn from(error: io::Error) -> Self {
        IOReadError {
            value: IOReadErrorValue::IO(error),
        }
    }
}

pub struct Reader {
    buffer: BytesMut,
}

impl Reader {
    pub fn new(input: BytesMut) -> Reader {
        Reader { buffer: input }
    }
    pub fn extend_from_slice(&mut self, extend: &[u8]) {
        self.buffer.extend_from_slice(extend)
    }
    pub fn read_bytes(&mut self, bytes_num: usize) -> Result<BytesMut, IOReadError> {
        if self.buffer.len() < bytes_num {
            return Err(IOReadError {
                value: IOReadErrorValue::NotEnoughBytes,
            });
        }
        Ok(self.buffer.split_to(bytes_num))
    }

    pub fn read_bytes_cursor(&mut self, bytes_num: usize) -> Result<Cursor<BytesMut>, IOReadError> {
        let tmp_bytes = self.read_bytes(bytes_num)?;
        let tmp_cursor = Cursor::new(tmp_bytes);
        Ok(tmp_cursor)
    }

    pub fn read_u8(&mut self) -> Result<u8, IOReadError> {
        let mut cursor = self.read_bytes_cursor(1)?;

        Ok(cursor.read_u8()?)
    }

    pub fn read_u24<T: ByteOrder>(&mut self) -> Result<u32, IOReadError> {
        let mut cursor = self.read_bytes_cursor(3)?;
        let val = cursor.read_u24::<T>()?;
        Ok(val)
    }

    pub fn read_u32<T: ByteOrder>(&mut self) -> Result<u32, IOReadError> {
        let mut cursor = self.read_bytes_cursor(4)?;
        let val = cursor.read_u32::<T>()?;

        Ok(val)
    }
}