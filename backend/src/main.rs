use std::io::{self, prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{PathBuf};
use std::str;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
enum ResultCode {
    RestartMarkerReply = 110,
    ServiceReadInXXXMinutes = 120,
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
    UserNameOkayNeedPassword = 331,
    NeedAccountForLogin = 332,
    RequestedFileActionPendingFurtherInformation = 350,
    ServiceNotAvailable = 421,
    CantOpenDataConnection = 425,
    ConnectionClosed = 426,
    FileBusy = 450,
    LocalErrorInProcessing = 451,
    InsufficientStorageSpace = 452,
    UnknownCommand = 500,
    InvalidParameterOrArgument = 501,
    CommandNotImplemented = 502,
    BadSequenceOfCommands = 503,
    CommandNotImplementedForThatParameter = 504,
    NotLoggedIn = 530,
    NeedAccountForStoringFiles = 532,
    FileNotFound = 550,
    PageTypeUnknown = 551,
    ExceededStorageAllocation = 552,
    FileNameNotAllowed = 553,
}


fn send_cmd(stream: &mut TcpStream, code: ResultCode, message: &str) {
    let msg = if message.is_empty() {
        format!("{}\r\n", code as u32)
    } else {
        format!("{} {}\r\n", code as u32, message)
    };
    println!("<==== {}", msg);
    write!(stream, "{}", msg).unwrap();
}


#[derive(Clone, Debug)]
enum Command {
    Auth,
    Cwd(PathBuf),
    Unknown(String),
}



impl Command {
    pub fn new(input: Vec<u8>) -> io::Result<Self> {
        let input_str = String::from_utf8_lossy(&input);
        let mut iter = input_str.split_whitespace();
        let command = iter.next().unwrap_or("").to_uppercase();

        let cmd = match command.as_str() {
            "AUTH" => Command::Auth,
            "CWD" => {
                let path_str = iter.collect::<Vec<&str>>().join(" "); // Collect remaining parts for path
                let path = PathBuf::from(path_str);
                Command::Cwd(path)
            },
            _ => Command::Unknown(command),
        };

        Ok(cmd)

    }
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match self {
            Command::Auth => "AUTH",
            Command::Cwd(_) => "CWD",
            Command::Unknown(s) => s,
        }
    }
}

fn read_all_message(stream: &mut TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let mut data = Vec::new();
    loop {
        let mut buf = [0; 1];
        match reader.read(&mut buf) {
            Ok(0) => break,  // Connection closed
            Ok(_) => {
                data.push(buf[0]);
                if data.ends_with(b"\r\n") {
                    data.truncate(data.len() - 2); // Remove \r\n
                    break;
                }
            },
            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
    println!("====> {}", String::from_utf8_lossy(&data));
    data

}



struct Client {
    cwd: PathBuf,
    stream: TcpStream,
    name: Option<String>,
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            stream: stream,
            name: None,
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("new client connected!");
    send_cmd(&mut stream, ResultCode::ServiceReadyForNewUser, "Welcome to this FTP server!");

    let mut client = Client::new(stream);

    loop {
        let data = read_all_message(&mut client.stream);
        if data.is_empty() {
            println!("client disconnected...");
            break;
        }

        match Command::new(data) {
            Ok(cmd) => {
                println!("Received command: {:?}", cmd);
                // Handle the command here...
            },
            Err(err) => {
                println!("Error parsing command: {}", err);
            }
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234")?;  // Bind to port 21
    println!("FTP server listening on port 1234");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {  // Handle clients in separate threads
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
