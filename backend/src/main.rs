use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::str;
use std::thread;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
enum ResultCode {
    RestartMarkerReply = 110,
    ServiceReadyInXXXXMinutes = 120,
    DataConnectionAlreadyOpen = 125,
    FileStatusOk = 150,
    Ok = 200,
    CommandNotImplementedSuperfluousAtThisSite = 202,
    SystemStatus = 211,
    DirectoryStatus = 212,
    FileStatus = 213,
    HelpMessage = 214,
    SystemType = 215,
    ServiceReadyForNewUser = 220,
    ServiceClosingControlConnection = 221,
    DataConnectionOpen = 225,
    ClosingDataConnection = 226,
    EnteringPassiveMode = 227,
    UserLoggedIn = 230,
    RequestedFileActionOkay = 250,
    PATHNAMECreated = 257,
    NeedAccountForLogin = 331,
    UserAccountForLogin = 332,
    RequestFurtherInformation = 350,
    ServiceNotAvailable = 421,
    CantOpenDataConnection = 425,
    ConnectionClosed = 426,
    FileBusy = 450,
    LocalErrorInProcessing = 451,
    InsufficientStorageSpace = 452,
    UnknownCommand = 500,
    InvalidParameterOrArgument = 501,
    BadSequenceOfCommands = 503,
    CommandNotImplemented = 502,
    CommandNotImplementedForThatParameter = 504,
    NotLoggedIn = 530,
    NeedAccountForStoringFiles = 532,
    FileTypeUnknown = 550,
    PageTypeUnknown = 551,
    ExceededStorageAllocation = 552,
    FileNameNotAllowed = 553,
    FileNotFound = 554, // Add FileNotFound to ResultCode
}

#[derive(Clone, Debug)]
enum Command {
    Auth,
    Syst,
    User(String),
    Pwd,
    Type,
    List,
    Pasv,
    Unknown(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Command::Auth => "AUTH",
            Command::Syst => "SYST",
            Command::User(_) => "USER",
            Command::Pwd => "PWD",
            Command::Type => "TYPE",
            Command::List => "LIST",
            Command::Pasv => "PASV",
            Command::Unknown(_) => "UNKN",
        }
    }
}

impl Command {
    pub fn new(input: Vec<u8>) -> std::io::Result<Self> {
        let mut iter = input.split(|&byte| byte == b' ');
        let mut command = iter.next().expect("command in input").to_vec();

        command.make_ascii_lowercase();
        let data = iter.next().map(|d| d.to_vec()); // Convert slice to Vec<u8>

        let command = match command.as_slice() {
            b"auth" => Command::Auth,
            b"syst" => Command::Syst,
            b"user" => Command::User(
                data.map(|bytes| String::from_utf8(bytes).unwrap())
                    .unwrap_or_else(|| "Cannot convert bytes to String".to_owned()),
            ),
            b"pwd" => Command::Pwd,
            b"type" => Command::Type,
            b"list" => Command::List,
            b"pasv" => Command::Pasv,
            _ => Command::Unknown(str::from_utf8(&command).unwrap_or("??").to_owned()),
        };

        Ok(command)
    }
}

#[allow(dead_code)]
struct Client {
    cwd: PathBuf,
    stream: TcpStream,
    name: Option<String>,
    data_writer: Option<TcpStream>,
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").expect("Couldn't bind this address...");

    println!("[*] Waiting for clients to connect...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            _ => {
                println!("[*] A client tried to connect...");
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("[+] New client connected!");
    send_cmd(
        &mut stream,
        ResultCode::ServiceReadyForNewUser,
        "Welcome to this FTP server!",
    );

    let mut client = Client::new(stream);
    loop {
        let data = read_all_message(&mut client.stream);
        if data.is_empty() {
            println!("[+] Client disconnected...");
            break;
        }

        client.handle_cmd(Command::new(data).unwrap());
    }
}

fn send_cmd(stream: &mut TcpStream, code: ResultCode, message: &str) {
    let msg = if message.is_empty() {
        format!("{} \r\n", code as u32)
    } else {
        format!("{} {}\r\n", code as u32, message)
    };

    println!("<--- {}", msg);
    write!(stream, "{}", msg).unwrap();
}

fn read_all_message(stream: &mut TcpStream) -> Vec<u8> {
    let mut out = Vec::with_capacity(100);
    let mut buf = [0u8; 1];

    loop {
        match stream.read(&mut buf) {
            Ok(received) if received > 0 => {
                if out.is_empty() && buf[0] == b' ' {
                    continue;
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

impl Client {
    fn new(stream: TcpStream) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            stream,
            name: None,
            data_writer: None,
        }
    }

    fn handle_cmd(&mut self, cmd: Command) {
        println!("{:?}", cmd);
        match cmd {
            Command::Auth => send_cmd(
                &mut self.stream,
                ResultCode::CommandNotImplemented,
                "Not implemented",
            ),
            Command::Syst => send_cmd(&mut self.stream, ResultCode::Ok, "I won't tell!"),
            Command::User(username) => {
                if username.is_empty() {
                    send_cmd(
                        &mut self.stream,
                        ResultCode::InvalidParameterOrArgument,
                        "Invalid username",
                    );
                } else {
                    self.name = Some(username);
                    send_cmd(
                        &mut self.stream,
                        ResultCode::UserLoggedIn,
                        &format!("Welcome {}", self.name.as_ref().unwrap()),
                    );
                }
            }
            Command::Pwd => {
                let msg = format!("{}", self.cwd.to_str().unwrap_or(""));
                if !msg.is_empty() {
                    send_cmd(&mut self.stream, ResultCode::PATHNAMECreated, &msg);
                } else {
                    send_cmd(
                        &mut self.stream,
                        ResultCode::FileNotFound,
                        "No such file or directory",
                    );
                }
            }
            Command::Type => send_cmd(
                &mut self.stream,
                ResultCode::Ok,
                "Transfer type changed successfully!",
            ),
            Command::List => {
                if let Some(mut writer) = self.data_writer.take() {
                    // Implement your logic to write the directory listing to the writer
                    // For simplicity, we'll just send a sample listing
                    writeln!(
                        writer,
                        "drwxr-xr-x  1 owner group  4096 Jan 1 00:00 dir1"
                    )
                    .unwrap();
                    writeln!(
                        writer,
                        "-rw-r--r--  1 owner group 1024 Jan 1 00:00 file1.txt"
                    )
                    .unwrap();
                    send_cmd(
                        &mut self.stream,
                        ResultCode::ClosingDataConnection,
                        "Closing data connection.",
                    );
                } else {
                    send_cmd(
                        &mut self.stream,
                        ResultCode::CantOpenDataConnection,
                        "No data connection established.",
                    );
                }
            }
            Command::Pasv => {
                if self.data_writer.is_some() {
                    send_cmd(
                        &mut self.stream,
                        ResultCode::DataConnectionAlreadyOpen,
                        "Already listening...",
                    );
                } else {
                    let port = 43210;

                    send_cmd(
                        &mut self.stream,
                        ResultCode::EnteringPassiveMode,
                        &format!("127,0,0,1,{},{}", port >> 8, port & 0xFF),
                    );

                    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
                    let listener = TcpListener::bind(&addr).unwrap();

                    match listener.incoming().next() {
                        Some(Ok(client)) => {
                            self.data_writer = Some(client);
                            send_cmd(
                                &mut self.stream,
                                ResultCode::DataConnectionOpen,
                                "Data connection opened.",
                            );
                        }
                        _ => {
                            send_cmd(
                                &mut self.stream,
                                ResultCode::ServiceNotAvailable,
                                "Issues happen...",
                            );
                        }
                    }
                }
            }
            Command::Unknown(s) => send_cmd(
                &mut self.stream,
                ResultCode::UnknownCommand,
                "Not implemented",
            ),
        }
    }
}
