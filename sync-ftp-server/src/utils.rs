use std::net::TcpStream;
use std::io::{Read, Write};
use crate::command::ResultCode;

pub fn send_cmd(stream: &mut TcpStream, code: ResultCode, message: &str) {
    let msg = if message.is_empty() {
        format!("{} \r\n", code as u32)
    } else {
        format!("{} {}\r\n", code as u32, message)
    };

    println!("<--- {}", msg);
    write!(stream, "{}", msg).unwrap();
}

pub fn read_all_message(stream: &mut TcpStream) -> Vec<u8> {
    let mut out = Vec::with_capacity(100);
    let mut buf = [0u8; 1];

    loop {
        match stream.read(&mut buf) {
            Ok(received) if received > 0 => {
                if out.is_empty() && buf[0] == b' ' {
                    continue
                }

                out.push(buf[0]);
            }

            _ => return Vec::new(),
        }

        let len = out.len();
        if len > 1 && out[len - 2] == b'\r' && out[len - 1] == b'\n' {
            out.pop();
            out.pop();
            return out;
        }
    }
}
