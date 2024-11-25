use std::io;
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

use crate::cmd::Command;
use crate::error::Error;
use crate::ftp::Answer;

pub struct BytesCodec;
pub struct FtpCodec;

impl Decoder for FtpCodec {
    type Item = Command;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Command>> {
        if let Some(index) = find_crlf(buf) {
            let line = buf.split_to(index);
            buf.split_to(2); // Remove \r\n.
            Command::new(line.to_vec()).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl Encoder<Answer> for FtpCodec {
    type Error = io::Error;

    fn encode(&mut self, answer: Answer, buf: &mut BytesMut) -> io::Result<()> {
        let answer = if answer.message.is_empty() {
            format!("{}\r\n", answer.code as u32)
        } else {
            format!("{} {}\r\n", answer.code as u32, answer.message)
        };
        buf.extend(answer.as_bytes());
        Ok(())
    }
}

fn find_crlf(buf: &mut BytesMut) -> Option<usize> {
    buf.windows(2).position(|bytes| bytes == b"\r\n")
}

impl Decoder for BytesCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        if buf.is_empty() {
            return Ok(None);
        }
        let data = buf.split_to(buf.len()).to_vec();
        Ok(Some(data))
    }
}

impl Encoder<Vec<u8>> for BytesCodec {
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(data);
        Ok(())
    }
}
