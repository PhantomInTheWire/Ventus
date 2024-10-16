use std::env;
use std::fs::{create_dir, read_dir, remove_dir_all, Metadata, File, OpenOptions};
use std::io::{Read, Write, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str;
use std::thread;
use colored::Colorize;
use time;


#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
enum ResultCode {
    RestartMarkerReply = 110,
    ServiceReadyInXXXXMinutes = 120,
    FileStatusOk = 125,
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
    LocalErrorInProcessing = 451,
    InsufficientStorageSpace = 452,
    UnknownCommand = 500,
    InvalidParameterOrArgument = 501,
    BadSequenceOfCommands = 503,
    CommandNotImplemented = 502,
    CommandNotImplementedForThatParameter = 504,
    NotLoggedIn = 530,
    NeedAccountForStoringFiles = 532,
    PageTypeUnknown = 551,
    FileNameNotAllowed = 553,
    OpeningDataConnection = 150,
    FileActionNotTaken = 450,
    FileUnavailable = 550,
}

#[derive(Clone, Debug)]
enum Command {
    Auth,
    Syst,
    User(String),
    Pwd,
    Type,
    List(Option<PathBuf>),
    Pasv,
    Cwd(PathBuf),
    Cdup,
    Mkdir(PathBuf),
    Rmd(PathBuf),
    Stor(PathBuf),
    Retr(PathBuf),
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
            Command::List(_) => "LIST",
            Command::Pasv => "PASV",
            Command::Cwd(_) => "CWD",
            Command::Cdup => "CDUP",
            Command::Mkdir(_) => "MKD",
            Command::Rmd(_) => "RMD",
            Command::Unknown(_) => "UNKN",
            Command::Stor(_) => "STOR",
            Command::Retr(_) => "RETR",
        }
    }
}

impl Command {
    pub fn new(input: Vec<u8>) -> std::io::Result<Self> {
        let mut iter = input.split(|&byte| byte == b' ');
        let command = iter.next().expect("command in input").to_vec();
        let command_lowercase: Vec<u8> = command.to_ascii_lowercase();
        let data = iter.next();

        let command = match command_lowercase.as_slice() {
            b"auth" => Command::Auth,
            b"syst" => Command::Syst,
            b"user" => Command::User(data.map(|bytes| String::from_utf8_lossy(bytes).to_string()).unwrap_or_default()),
            b"pwd" => Command::Pwd,
            b"type" => Command::Type,
            b"list" => Command::List(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string()))),
            b"pasv" => Command::Pasv,
            b"cwd" => Command::Cwd(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"cdup" => Command::Cdup,
            b"mkd" => Command::Mkdir(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"rmd" => Command::Rmd(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"stor" => Command::Stor(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"retr" => Command::Retr(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            _ => Command::Unknown(String::from_utf8_lossy(&command).to_string()),
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
    let ascii = r#"
        ..@@@@@@@@@@@@@@@@@@@@@@@#..                              -@@@@@@@@@@@@@@@@@@@@@@@@+.
         .#@@@@@@@@@@@@@@@@@@@@@@@%:.                           .=@@@@@@@@@@@@@@@@@@@@@@@@*.
         .:%@@@@@@@@@@@@@@@@@@@@@@@%:                          .+@@@@@@@@@@@@@@@@@@@@@@@@*.
           .%@@@@@@@@@@@@@@@@@@@@@@@%:.                       .=@@@@@@@@@@@@@@@@@@@@@@@@+.
            :#@@@@@@@@@@@@@@@@@@@@@@@%-.                    ..*@@@@@@@@@@@@@@@@@@@@@@@%=.
                           .+@@@@@@@@@%:.                   .*@@@@@@@@@@=.
               ............  =@@@@@@@@@@-.                 .*@@@@@@@@@%-. ...........
               .#@@@@@@@@@%:..=@@@@@@@@@@=.              ..*@@@@@@@@@@- .-@@@@@@@@@@=.
                .#@@@@@@@@@%. .-@@@@@@@@@@-              .*@@@@@@@@@@:. -%@@@@@@@@@=.
                 .*@@@@@@@@@@:. -@@@@@@@@@@+.           .#@@@@@@@@@%. .-@@@@@@@@@@-
                  .*@@@@@@@@@%- .-@@@@@@@@@@+.        .:%@@@@@@@@@%:..=@@@@@@@@@@-.
                    =@@@@@@@@@@-. :@@@@@@@@@@+.       .%@@@@@@@@@%. .=@@@@@@@@@@:.
                    .+@@@@@@@@@@=..-@@@@@@@@@@*.    .:@@@@@@@@@@#...+@@@@@@@@@%:
                     .=@@@@@@@@@@=. :@@@@@@@@@@*.   -%@@@@@@@@@#. .+@@@@@@@@@@-.
                      .=@@@@@@@@@@+. :%@@@@@@@@@#. :@@@@@@@@@@*. .*@@@@@@@@@%:.
                       .=@@@@@@@@@@+..:%@@@@@@@@@%*@@@@@@@@@@*...#@@@@@@@@@#:
                        .-@@@@@@@@@@+...%@@@@@@@@@@@@@@@@@@@*. .#@@@@@@@@@%..
                          :@@@@@@@@@@#...#@@@@@@@@@@@@@@@@@+  .#@@@@@@@@@#.
                          .:@@@@@@@@@@#...#@@@@@@@@@@@@@@@+. :%@@@@@@@@@#:
                           ..%@@@@@@@@@#...*@@@@@@@@@@@@@=. :#@@@@@@@@@#.
                             :#@@@@@@@@@%:..*@@@@@@@@@@@-..:%@@@@@@@@@+.
                              .#@@@@@@@@@%:..*@@@@@@@@@-..-%@@@@@@@@@*.
                               .*@@@@@@@@@@:..+@@@@@@@:..:%@@@@@@@@@*.
                                .*@@@@@@@@@@-..+@@@@%:..-@@@@@@@@@@+.
                                ..*@@@@@@@@@@-..=@@%:..-@@@@@@@@@@=.
                                  .+@@@@@@@@@@=..-+. .=@@@@@@@@@@=.
                                  ..+@@@@@@@@@@=.   .+@@@@@@@@@@-.
                                    .=@@@@@@@@@@+. .+@@@@@@@@@@-.
                                     .=@@@@@@@@@@*:+@@@@@@@@@%:.
                                      .-@@@@@@@@@@@@@@@@@@@@@-.
                                       .:%@@@@@@@@@@@@@@@@@%:.
                                        .-@@@@@@@@@@@@@@@@%:
                                         .:@@@@@@@@@@@@@@%:.
                                           :%@@@@@@@@@@@#.
                                           ..%@@@@@@@@@#.
                                             .#@@@@@@@#.
                                              .*@@@@@+.
                                               .*@@@*.
                                                .+@*.
"#;
    println!("{}", ascii.purple());
    let listener = TcpListener::bind("0.0.0.0:1234").expect("Couldn't bind this address...");

    println!("[*] Waiting for clients to connect...");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                handle_client(stream);
            });
        } else {
            println!("[*] A client tried to connect...");
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    println!("[+] New client connected!");
    send_cmd(&mut stream, ResultCode::ServiceReadyForNewUser, "Welcome to this FTP server!");

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


impl Client {

    fn new(stream: TcpStream) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            stream,
            name: None,
            data_writer: None,
        }
    }

    fn complete_path(&self, path: PathBuf, server_root: &PathBuf) -> Result<PathBuf, std::io::Error> {
        let directory = server_root.join(if path.has_root() {
            path.iter().skip(1).collect()
        } else {
            path
        });

        if let Ok(ref dir) = directory.canonicalize() {
            let dir = dir.to_str().unwrap();
            if !dir.starts_with(server_root.to_str().unwrap()) {
                return Err(std::io::Error::new(ErrorKind::PermissionDenied, "Permission denied"));
            }
        }
        Ok(directory)
    }

    fn cwd(&mut self, directory: PathBuf) {
        let server_root = env::current_dir().unwrap();
        let path = self.cwd.join(&directory);
        if let Ok(dir) = self.complete_path(path, &server_root) {
            if let Ok(prefix) = dir.strip_prefix(&server_root).map(|p| p.to_path_buf()) {
                self.cwd = prefix;
                send_cmd(&mut self.stream, ResultCode::Ok, &format!("Directory changed to \"{}\"", directory.display()));
                return;
            }
        }

        send_cmd(&mut self.stream, ResultCode::InvalidParameterOrArgument, "No such file or directory");
    }

    fn handle_cmd(&mut self, cmd: Command) {
        println!("{:?}", cmd);
        match cmd {
            Command::Stor(path) => self.stor(path),
            Command::Retr(path) => self.retr(path),
            Command::Auth => send_cmd(&mut self.stream, ResultCode::CommandNotImplemented, "Not implemented"),
            Command::Syst => send_cmd(&mut self.stream, ResultCode::Ok, "UNIX Type: L8"),
            Command::User(username) => {
                if username.is_empty() {
                    send_cmd(&mut self.stream, ResultCode::InvalidParameterOrArgument, "Invalid username");
                } else {
                    self.name = Some(username);
                    send_cmd(&mut self.stream, ResultCode::UserLoggedIn, &format!("Welcome {}", self.name.as_ref().unwrap()));
                }
            },
            Command::Pwd => {
                let msg = format!("\"{}\"", self.cwd.to_str().unwrap_or(""));
                if !msg.is_empty() {
                    send_cmd(&mut self.stream, ResultCode::PATHNAMECreated, &format!("{}", msg));
                } else {
                    send_cmd(&mut self.stream, ResultCode::InvalidParameterOrArgument, "No such file or directory");
                }
            },
            Command::Type => send_cmd(&mut self.stream, ResultCode::Ok, "Switching to Binary mode."),
            Command::List(_) => {
                self.list()
            },

            Command::Pasv => {
                if self.data_writer.is_some() {
                    send_cmd(&mut self.stream, ResultCode::FileStatusOk, "Already listening...");
                } else {
                    let port = 43210; // Or choose a random available port

                    // Calculate p1 and p2 for the PASV response (address is hardcoded as 127,0,0,1)
                    let p1 = port / 256;
                    let p2 = port % 256;
                    send_cmd(&mut self.stream, ResultCode::EnteringPassiveMode,
                             &format!("Entering Passive Mode (127,0,0,1,{},{})", p1, p2));

                    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
                    let listener = TcpListener::bind(&addr).unwrap();

                    match listener.incoming().next() {
                        Some(Ok(client)) => {
                            self.data_writer = Some(client);
                        },
                        _ => {
                            send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "Failed to open data connection.");
                        }
                    }
                }
            },
            Command::Cwd(directory) => self.cwd(directory),
            Command::Cdup => {
                // Using canonical parent path for better compatibility
                let parent = self.cwd.parent().map(|p| p.to_path_buf()).unwrap_or(self.cwd.clone());
                self.cwd(parent);
            }
            Command::Mkdir(directory) => {
                let server_root = env::current_dir().unwrap();
                let path = self.cwd.join(&directory);
                if let Ok(dir) = self.complete_path(path, &server_root) {
                    if let Err(_) = create_dir(&dir) {
                        send_cmd(&mut self.stream, ResultCode::FileNameNotAllowed, "Couldn't create directory");
                    } else {
                        send_cmd(&mut self.stream, ResultCode::PATHNAMECreated, "Directory created");
                    }
                } else {
                    send_cmd(&mut self.stream, ResultCode::FileNameNotAllowed, "Permission denied");
                }
            }

            Command::Rmd(directory) => {
                let server_root = env::current_dir().unwrap();
                let path = self.cwd.join(&directory);
                if let Ok(dir) = self.complete_path(path, &server_root) {
                    if let Err(_) = remove_dir_all(&dir) {
                        send_cmd(&mut self.stream, ResultCode::FileNameNotAllowed, "Couldn't remove directory");
                    } else {
                        send_cmd(&mut self.stream, ResultCode::RequestedFileActionOkay, "Directory removed");
                    }
                } else {
                    send_cmd(&mut self.stream, ResultCode::FileNameNotAllowed, "Permission denied");
                }
            }
            Command::Unknown(command) => {
                send_cmd(&mut self.stream, ResultCode::UnknownCommand, &format!("Unknown command: {}", command));
            }
        }
    }
    fn stor(&mut self, path: PathBuf) {
        let server_root = env::current_dir().unwrap();
        let path = self.cwd.join(path);

        if let Ok(file_path) = self.complete_path(path, &server_root) {
            send_cmd(&mut self.stream, ResultCode::OpeningDataConnection, "Opening binary mode data connection for file upload.");
            if let Some(ref mut writer) = self.data_writer {
                let mut file = OpenOptions::new().write(true).create(true).open(file_path).unwrap();
                let mut buffer = [0u8; 1024];
                loop {
                    match writer.read(&mut buffer) {
                        Ok(0) => break,
                        Ok(n) => {
                            file.write_all(&buffer[..n]).unwrap();
                        }
                        Err(_) => {
                            send_cmd(&mut self.stream, ResultCode::FileActionNotTaken, "Failed to receive file.");
                            return;
                        }
                    }
                }
                send_cmd(&mut self.stream, ResultCode::ClosingDataConnection, "File transfer complete.");
            } else {
                send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "No data connection.");
            }
        } else {
            send_cmd(&mut self.stream, ResultCode::FileUnavailable, "Invalid path.");
        }
        self.data_writer = None;
    }

    fn retr(&mut self, path: PathBuf) {
        let server_root = env::current_dir().unwrap();
        let path = self.cwd.join(path);

        if let Ok(file_path) = self.complete_path(path, &server_root) {
            if let Ok(mut file) = File::open(file_path) {
                send_cmd(&mut self.stream, ResultCode::OpeningDataConnection, "Opening binary mode data connection for file download.");
                if let Some(ref mut writer) = self.data_writer {
                    let mut buffer = [0u8; 1024];
                    loop {
                        match file.read(&mut buffer) {
                            Ok(0) => break,
                            Ok(n) => {
                                writer.write_all(&buffer[..n]).unwrap();
                            }
                            Err(_) => {
                                send_cmd(&mut self.stream, ResultCode::FileActionNotTaken, "Failed to send file.");
                                return;
                            }
                        }
                    }
                    send_cmd(&mut self.stream, ResultCode::ClosingDataConnection, "File transfer complete.");
                } else {
                    send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "No data connection.");
                }
            } else {
                send_cmd(&mut self.stream, ResultCode::FileUnavailable, "File not found.");
            }
        } else {
            send_cmd(&mut self.stream, ResultCode::FileUnavailable, "Invalid path.");
        }
        self.data_writer = None;
    }

    fn list(&mut self) {
        let server_root = env::current_dir().unwrap();
        let path = self.cwd.join(".");
        if let Ok(dir) = self.complete_path(path, &server_root) {
            match read_dir(&dir) {
                Ok(entries) => {
                    let mut response = String::new();
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let metadata = entry.metadata().unwrap();
                            let file_type = if metadata.is_dir() { "DIR" } else { "FILE" };
                            response.push_str(&format!(
                                "{}\t{}\t{}\r\n",
                                file_type,
                                metadata.len(),
                                entry.file_name().to_string_lossy()
                            ));
                        }
                    }
                    send_cmd(&mut self.stream, ResultCode::FileStatusOk, "Here comes the directory listing.");
                    if let Some(ref mut writer) = self.data_writer {
                        write!(writer, "{}\r\n", response).unwrap();
                        writer.flush().unwrap(); // Flush the data socket
                    }
                    send_cmd(&mut self.stream, ResultCode::ClosingDataConnection, "Directory send OK.");
                }
                Err(_) => {
                    send_cmd(&mut self.stream, ResultCode::InvalidParameterOrArgument, "Failed to list directory.");
                }
            }
        } else {
            send_cmd(&mut self.stream, ResultCode::InvalidParameterOrArgument, "Permission denied");
        }
    }
}
