use crate::frame::Frame;
use bytes::Bytes;
use std::{fmt, str, vec};

#[derive(Debug)]
pub struct Parse {
    array: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            other => return Err(format!("protocol error, expected array, got {:?}", other).into()),
        };

        Ok(Parse {
            array: array.into_iter(),
        })
    }

    pub fn next(&mut self) -> Result<Frame, ParseError> {
        self.array.next().ok_or(ParseError::EndOfStream)
    }

    pub fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Bulk(bytes) => str::from_utf8(&bytes[..])
                .map(|s| s.to_string())
                .map_err(|_| "protocol error; invalid string".into()),
            ref other => {
                Err(format!("protocol error; expected bulk frame, got {:?}", other).into())
            }
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Bulk(bytes) => Ok(bytes),
            other => Err(format!("protocol error; expected bulk frame, got {:?}", other).into()),
        }
    }

    pub fn next_int(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;

        const ERR: &str = "protocol error; invalid number";

        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(data) => atoi::<u64>(data.as_bytes()).ok_or_else(|| ERR.into()),
            Frame::Bulk(bytes) => atoi::<u64>(&bytes).ok_or_else(|| ERR.into()),
            other => Err(format!("protocol error; expected int frame but got {:?}", other).into()),
        }
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        if self.array.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError {
        ParseError::Other(src.into())
    }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError {
        src.to_string().into()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
