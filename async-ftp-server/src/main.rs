use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;
use std::path::{PathBuf, StripPrefixError};
use std::result;
use std::ffi::OsString;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use tokio_util::codec::{Framed, BytesCodec};
use tokio::io::AsyncWriteExt;
mod cmd;
mod codec;
mod error;
mod ftp;

use crate::cmd::Command;
use crate::cmd::TransferType;
use crate::codec::FtpCodec;
use crate::error::Result;
use crate::ftp::{Answer, ResultCode};

type DataReader = tokio_util::codec::FramedRead<tokio::net::TcpStream, BytesCodec>;
type DataWriter = tokio_util::codec::FramedWrite<tokio::net::TcpStream, BytesCodec>;

type Writer = tokio_util::codec::FramedWrite<tokio::net::TcpStream, FtpCodec>;

struct Client {
    cwd: PathBuf,
    data_port: Option<u16>,
    data_reader: Option<DataReader>,
    data_writer: Option<DataWriter>,
    server_root: PathBuf,
    transfer_type: TransferType,
    writer: Writer,
    reader: futures::stream::SplitStream<Framed<TcpStream, FtpCodec>>,
    data_connection: Option<Framed<TcpStream, BytesCodec>>,
}

impl Client {
    fn new(framed: Framed<TcpStream, FtpCodec>, server_root: PathBuf) -> Client {
        let (writer, reader) = framed.split();
        Client {
            cwd: PathBuf::from("/"),
            data_port: None,
            data_reader: None,
            data_writer: None,
            server_root,
            transfer_type: TransferType::Ascii,
            writer,
            reader,
            data_connection: None,
        }
    }

    async fn handle_cmd(mut self, cmd: Command) -> Result<Self> {
        println!("Received command: {:?}", cmd);
        match cmd {
            Command::Cwd(directory) => self = self.cwd(directory).await?,
            Command::Pwd => {
                let msg = format!("{}", self.cwd.to_str().unwrap_or(""));
                if !msg.is_empty() {
                    let message = format!("\"/{}\" ", msg);
                    self = self.send(Answer::new(ResultCode::PATHNAMECreated, &message)).await?;
                } else {
                    self = self.send(Answer::new(ResultCode::FileNotFound, "No such file or directory")).await?;
                }
            }
            Command::Pasv => self = self.pasv().await?,
            Command::Port(port) => {
                self.data_port = Some(port);
                self = self.send(Answer::new(ResultCode::Ok, &format!("Data port is now {}", port))).await?;
            }
            Command::Quit => self = self.quit().await?,
            Command::Syst => {
                self = self.send(Answer::new(ResultCode::Ok, "I won't tell!")).await?;
            }
            Command::CdUp => {
                if let Some(path) = self.cwd.parent().map(Path::to_path_buf) {
                    self.cwd = path;
                }
                self = self.send(Answer::new(ResultCode::Ok, "Done")).await?;
            }
            Command::Mkd(path) => self = self.mkd(path).await?,
            Command::Rmd(path) => self = self.rmd(path).await?,
            Command::NoOp => self = self.send(Answer::new(ResultCode::Ok, "Doing nothing")).await?,
            Command::Type(typ) => {
                self.transfer_type = typ;
                self = self.send(Answer::new(ResultCode::Ok, "Transfer type changed successfully")).await?;
            }
            Command::User(content) => {
                if content.is_empty() {
                    self = self.send(Answer::new(ResultCode::InvalidParameterOrArgument, "Invalid username")).await?;
                } else {
                    self = self.send(Answer::new(ResultCode::UserloggedIn, &format!("Welcome {}!", content))).await?;
                }
            }
            Command::Unknown(s) => {
                self = self.send(Answer::new(ResultCode::UnknownCommand, &format!("\"{}\": Not implemented", s))).await?;
            }
            _ => {
                self = self.send(Answer::new(ResultCode::CommandNotImplemented, "Not implemented")).await?;
            }
        }
        Ok(self)
    }

    async fn cwd(mut self, directory: PathBuf) -> Result<Self> {
        let path = self.cwd.join(&directory);
        let (new_self, res) = self.complete_path(path);
        self = new_self;
        if let Ok(dir) = res {
            let (new_self, res) = self.strip_prefix(dir);
            self = new_self;
            if let Ok(prefix) = res {
                self.cwd = prefix.to_path_buf();
                self = self.send(Answer::new(ResultCode::Ok, &format!("Directory changed to \"{}\"", directory.display()))).await?;
                return Ok(self);
            }
        }
        self = self.send(Answer::new(ResultCode::FileNotFound, "No such file or directory")).await?;
        Ok(self)
    }

    async fn mkd(mut self, path: PathBuf) -> Result<Self> {
        let path = self.cwd.join(&path);
        let parent = get_parent(path.clone());
        if let Some(parent) = parent {
            let parent = parent.to_path_buf();
            let (new_self, res) = self.complete_path(parent);
            self = new_self;
            if let Ok(mut dir) = res {
                if dir.is_dir() {
                    let filename = get_filename(path);
                    if let Some(filename) = filename {
                        dir.push(filename);
                        if create_dir(dir).is_ok() {
                            self = self.send(Answer::new(ResultCode::PATHNAMECreated, "Folder successfully created!")).await?;
                            return Ok(self);
                        }
                    }
                }
            }
        }
        self = self.send(Answer::new(ResultCode::FileNotFound, "Couldn't create folder")).await?;
        Ok(self)
    }

    async fn rmd(mut self, directory: PathBuf) -> Result<Self> {
        let path = self.cwd.join(&directory);
        let (new_self, res) = self.complete_path(path);
        self = new_self;
        if let Ok(dir) = res {
            if remove_dir_all(dir).is_ok() {
                self = self.send(Answer::new(ResultCode::RequestedFileActionOkay, "Folder successfully removed")).await?;
                return Ok(self);
            }
        }
        self = self.send(Answer::new(ResultCode::FileNotFound, "Couldn't remove folder")).await?;
        Ok(self)
    }

    async fn quit(mut self) -> Result<Self> {
        if self.data_writer.is_some() {
            unimplemented!();
        } else {
            self = self.send(Answer::new(ResultCode::ServiceClosingControlConnection, "Closing connection...")).await?;
            // Close the underlying TCP stream
            self.writer.get_mut().shutdown().await?;
        }
        Ok(self)
    }

    async fn pasv(mut self) -> Result<Self> {
        let port = self.data_port.unwrap_or(0);
        if self.data_writer.is_some() {
            self = self.send(Answer::new(ResultCode::DataConnectionAlreadyOpen, "Already listening...")).await?;
            return Ok(self);
        }

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(&addr).await?;
        let port = listener.local_addr()?.port();

        self = self.send(Answer::new(ResultCode::EnteringPassiveMode, &format!("127,0,0,1,{},{}", port >> 8, port & 0xFF))).await?;

        println!("Waiting clients on port {}...", port);
        if let Ok((stream, _)) = listener.accept().await {
            self.data_connection = Some(Framed::new(stream, BytesCodec::new()));
        }
        Ok(self)
    }

    fn complete_path(self, path: PathBuf) -> (Self, result::Result<PathBuf, io::Error>) {
        let directory = self.server_root.join(if path.has_root() {
            path.iter().skip(1).collect()
        } else {
            path
        });
        let dir = directory.canonicalize();
        if let Ok(ref dir) = dir {
            if !dir.starts_with(&self.server_root) {
                return (self, Err(io::ErrorKind::PermissionDenied.into()));
            }
        }
        (self, dir)
    }

    fn strip_prefix(self, dir: PathBuf) -> (Self, result::Result<PathBuf, StripPrefixError>) {
        let res = dir.strip_prefix(&self.server_root).map(|p| p.to_path_buf());
        (self, res)
    }

    async fn send(mut self, answer: Answer) -> Result<Self> {
        self.writer.send(answer).await?;
        Ok(self)
    }
}

async fn handle_client(stream: TcpStream, server_root: PathBuf) -> Result<()> {
    let framed = Framed::new(stream, FtpCodec);
    let mut client = Client::new(framed, server_root);

    client.writer.send(Answer::new(ResultCode::ServiceReadyForNewUser, "Welcome to this FTP server!")).await?;

    while let Some(cmd) = client.reader.next().await {
        match cmd {
            Ok(cmd) => {
                client = client.handle_cmd(cmd).await?;
            }
            Err(e) => {
                println!("Error receiving command: {:?}", e);
                break;
            }
        }
    }
    println!("Client closed");
    Ok(())
}

#[tokio::main]
async fn main() {
    let server_root = env::current_dir().expect("Cannot determine current directory");
    let port = 1234;
    let addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), port);
    let listener = TcpListener::bind(addr).await.expect("Unable to bind TCP listener");

    println!("Waiting clients on port {}...", port);
    while let Ok((stream, addr)) = listener.accept().await {
        println!("New client connected: {}", addr);
        tokio::spawn(handle_client(stream, server_root.clone()));
    }
}

fn get_parent(path: PathBuf) -> Option<PathBuf> {
    path.parent().map(|p| p.to_path_buf())
}

fn get_filename(path: PathBuf) -> Option<OsString> {
    path.file_name().map(|p| p.to_os_string())
}