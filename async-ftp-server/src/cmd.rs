use std::path::{Path, PathBuf};
use std::str::{self, FromStr};
use crate::error::{Error};
use std::io;

#[derive(Clone, Debug)]
pub enum Command {
    Auth,
    Cwd(PathBuf),
    List(Option<PathBuf>),
    Mkd(PathBuf),
    NoOp,
    Port(u16),
    Pasv,
    Pwd,
    Quit,
    Retr(PathBuf),
    Rmd(PathBuf),
    Stor(PathBuf),
    Syst,
    Type(TransferType),
    CdUp,
    Unknown(String),
    User(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Command::Auth => "AUTH",
            Command::Cwd(_) => "CWD",
            Command::List(_) => "LIST",
            Command::Pasv => "PASV",
            Command::Port(_) => "PORT",
            Command::Pwd => "PWD",
            Command::Quit => "QUIT",
            Command::Retr(_) => "RETR",
            Command::Stor(_) => "STOR",
            Command::Syst => "SYST",
            Command::Type(_) => "TYPE",
            Command::User(_) => "USER",
            Command::CdUp => "CDUP",
            Command::Mkd(_) => "MKD",
            Command::Rmd(_) => "RMD",
            Command::NoOp => "NOOP",
            Command::Unknown(_) => "UNKN", // doesn't exist
        }
    }
}

impl Command {
    pub fn new(input: Vec<u8>) -> io::Result<Self> {
        let mut iter = input.split(|&byte| byte == b' ');
        let mut command = iter.next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "empty command"))?
            .to_vec();
        to_uppercase(&mut command);
        let data = iter.next();
        
        let command = match command.as_slice() {
            b"AUTH" => Command::Auth,
            b"CWD" => {
                let path = data.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no path provided"))?;
                let path_str = str::from_utf8(path).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                Command::Cwd(Path::new(path_str).to_path_buf())
            },
            b"LIST" => {
                Command::List(
                    data.and_then(|bytes| {
                        str::from_utf8(bytes)
                            .ok()
                            .map(|s| Path::new(s).to_path_buf())
                    })
                )
            },
            b"PASV" => Command::Pasv,
            b"PORT" => {
                let addr = data.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no address provided"))?
                    .split(|&byte| byte == b',')
                    .filter_map(|bytes| str::from_utf8(bytes).ok()
                        .and_then(|string| u8::from_str(string).ok()))
                    .collect::<Vec<u8>>();
                
                if addr.len() != 6 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid address/port"));
                }

                let port = (addr[4] as u16) << 8 | (addr[5] as u16);
                if port <= 1024 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Port can't be less than 10025"));
                }
                Command::Port(port)
            },
            b"PWD" => Command::Pwd,
            b"QUIT" => Command::Quit,
            b"RETR" | b"STOR" | b"MKD" | b"RMD" => {
                let path = data.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no path provided"))?;
                let path_str = str::from_utf8(path).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                match command.as_slice() {
                    b"RETR" => Command::Retr(Path::new(path_str).to_path_buf()),
                    b"STOR" => Command::Stor(Path::new(path_str).to_path_buf()),
                    b"MKD" => Command::Mkd(Path::new(path_str).to_path_buf()),
                    b"RMD" => Command::Rmd(Path::new(path_str).to_path_buf()),
                    _ => unreachable!(),
                }
            },
            b"SYST" => Command::Syst,
            b"TYPE" => {
                let type_byte = data.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no type provided"))?;
                match TransferType::from(type_byte[0]) {
                    TransferType::Unknown => {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                            "command not implemented for that parameter"))
                    },
                    typ => Command::Type(typ),
                }
            },
            b"CDUP" => Command::CdUp,
            b"USER" => {
                let username = data.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "no username provided"))?;
                Command::User(String::from_utf8(username.to_vec())
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
            },
            b"NOOP" => Command::NoOp,
            s => Command::Unknown(str::from_utf8(s).unwrap_or("").to_owned()),
        };
        Ok(command)
    }
}

fn to_uppercase(data: &mut [u8]) {
    for byte in data {
        if *byte >= 'a' as u8 && *byte <= 'z' as u8 {
            *byte -= 32;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransferType {
    Ascii,
    Image,
    Unknown,
}

impl From<u8> for TransferType {
    fn from(c: u8) -> TransferType {
        match c {
            b'A' => TransferType::Ascii,
            b'I' => TransferType::Image,
            _ => TransferType::Unknown,
        }
    }
}