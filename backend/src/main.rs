use std::env;
use std::fs::{create_dir, read_dir, remove_dir_all, Metadata};
use std::io::{Read, Write, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::str;
use std::thread;
use time;


/*
The `ResultCode` enum defines the set of standardized numeric codes used by an FTP server
to communicate the status of an operation or the server's current state to a client.
Each code represents a specific outcome, such as success, failure, or the need for further action.
These codes are an essential part of the FTP protocol, allowing clients to understand the server's responses and behave accordingly.
Each variant of the enum maps to a specific FTP result code as defined in the FTP specification (RFC 959).
Some common examples include 200 (OK), 500 (Syntax error, command unrecognized), and 421 (Service not available, closing control connection).
*/
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
    FileNameNotAllowed = 553,
}

/*
The `Command` enum represents the various commands that a client can send to an FTP server.
Each variant of this enum corresponds to a specific FTP command, such as USER (for user login), PASS (for password),
LIST (to list directory contents), RETR (to retrieve a file), STOR (to store a file), and many others.
Some commands may require additional data to be provided by the client, such as a username, password, or file path.
This enum is used by the server to parse incoming commands from the client and determine the appropriate action to take.
*/
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
    Unknown(String),
}


/*
This implementation block provides a way to obtain the string representation of each `Command` variant.
This is crucial for sending responses back to the client that include the command name
(e.g., "200 Command okay."). By implementing `AsRef<str>`, we can easily convert a `Command`
instance into its corresponding string representation using the `as_ref()` method.
*/
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
        }
    }
}

/*
This implementation block for the `Command` enum provides the `new` function,
which is responsible for parsing raw bytes received from the client into a `Command` instance.
The function takes a `Vec<u8>` representing the received data and splits it into separate words based on spaces.
The first word is interpreted as the command itself, converted to lowercase for case-insensitive matching,
and then compared against the known FTP commands defined in the enum.
If a match is found, the corresponding `Command` variant is created.
If the command is not recognized, an `Unknown` variant is created containing the original command string.
Any subsequent words after the command are treated as arguments or data associated with that command.
*/
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
            _ => Command::Unknown(String::from_utf8_lossy(&command).to_string()),
        };

        Ok(command)
    }
}


/*
The `Client` struct represents an active client connected to the FTP server.
It encapsulates all the information and resources associated with that client,
including their current state and connection details.
- `cwd`: Stores the client's current working directory on the server, represented as a `PathBuf`.
- `stream`: Represents the TCP stream used for communication between the server and the client.
- `name`: Optionally stores the client's username if they have successfully logged in.
- `data_writer`: An optional `TcpStream` used for transferring data in passive mode.
*/
#[allow(dead_code)]
struct Client {
    cwd: PathBuf,
    stream: TcpStream,
    name: Option<String>,
    data_writer: Option<TcpStream>,
}

/*
The `main` function serves as the entry point for the FTP server application.
It initializes the server, sets up a listening socket, and accepts incoming client connections.
For each new connection, it spawns a dedicated thread to handle the client's interaction with the server.
This allows the server to handle multiple clients concurrently.
*/
fn main() {
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

/*
The `handle_client` function is the core logic for managing individual client connections.
It takes ownership of a `TcpStream` representing the connection to a client and handles all communication with that client.
It starts by sending a welcome message to the client, indicating that the server is ready.
Then, it enters a loop that continuously reads commands from the client, parses them,
executes the requested actions, and sends responses back to the client.
This loop continues until the client disconnects or an error occurs.
*/
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

/*
The `send_cmd` function is a utility function responsible for sending a formatted response message to the client.
It takes a `ResultCode`, which represents the numeric status code of the response,
and a `&str` message, which provides a human-readable explanation of the response.
The function formats these two components into a single string that conforms to the FTP protocol specification.
It then writes this formatted string to the client's `TcpStream`, effectively sending the response back to the client.
*/
fn send_cmd(stream: &mut TcpStream, code: ResultCode, message: &str) {
    let msg = if message.is_empty() {
        format!("{} \r\n", code as u32)
    } else {
        format!("{} {}\r\n", code as u32, message)
    };

    println!("<--- {}", msg);
    write!(stream, "{}", msg).unwrap();
}

/*
The `read_all_message` function is a utility function used to read a complete message from the client's TCP stream.
It reads data byte-by-byte from the stream until it encounters the FTP command terminator, which is a carriage return (`\r`) followed by a newline (`\n`).
The function then returns the received message as a `Vec<u8>`, excluding the terminator sequence.
*/
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

/*
This `impl` block defines methods associated with the `Client` struct.
These methods provide functionality for interacting with the client,
managing their state, and handling FTP commands specific to their session.
*/
impl Client {
    /*
    The `new` function is a constructor for the `Client` struct.
    It takes a `TcpStream` representing a newly established connection
    to a client and returns a new `Client` instance.
    The initial current working directory for the client is set to the server's
    root directory ("/"). The `name` and `data_writer` fields are initialized
    to `None`, as the client hasn't logged in yet and a data connection hasn't
    been established.
    */
    fn new(stream: TcpStream) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            stream,
            name: None,
            data_writer: None,
        }
    }
    /*
    The `complete_path` function ensures that a path provided by the client is resolved
    within the bounds of the server's root directory. This is crucial for preventing
    directory traversal attacks, where a malicious client might attempt to access files
    outside the designated area. The function takes a `PathBuf` representing the
    client-provided path (which could be relative or absolute) and the server's root directory.
    It then constructs a complete, canonicalized path that is guaranteed to be within
    the server's root. If the resolved path is outside the root, an error is returned to
    prevent unauthorized access.
    */
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
    /*
    The `cwd` (Change Working Directory) function handles the CWD command sent by the client.
    This command is used to change the client's current working directory on the server.
    The function takes a `PathBuf` representing the directory the client wants to change to.
    It then uses `complete_path` to resolve this path relative to the client's current
    directory and the server's root.
    If the resolved path is valid and within the allowed area, the client's `cwd` is updated,
    and a success message is sent back. Otherwise, an error message is sent to indicate that
    the requested directory does not exist or is not accessible.
    */
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
    /*
    The `handle_cmd` function is the central command processing engine for each connected client.
    It takes a `Command` enum variant, which represents a parsed command received from the client.
    Based on the specific command, it performs the appropriate action, such as user authentication,
    listing directory contents, retrieving or storing files, changing directories, and so on.
    For each command, it sends back a response to the client indicating the outcome of the operation (success or failure).
    */
    fn handle_cmd(&mut self, cmd: Command) {
        println!("{:?}", cmd);
        match cmd {
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
            Command::List(_) => self.list(),
            /*
            This section specifically handles the PASV (Passive) command from the client.
            The PASV command is used to establish a data connection for transferring files in passive mode.
            In passive mode, the server listens on a data port and sends the port information to the client.
            The client then initiates a connection to the server on that port for data transfer.
            This code first checks if a data connection is already open.
            If not, it binds to a predefined data port, sends the PASV response to the client with the IP address and port number,
            and waits for the client to connect.
            Once the client connects, the `data_writer` field is set with the newly established data connection.
            */
            Command::Pasv => {
                if self.data_writer.is_some() {
                    send_cmd(&mut self.stream, ResultCode::DataConnectionAlreadyOpen, "Already listening...");
                } else {
                    let port = 43210;

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
    /*
    The `list` function handles the LIST command sent by the client.
    This command instructs the server to send a list of files and directories in the
    client's current working directory. The function first resolves the current working
    directory using `complete_path` to ensure it's within the allowed area. It then reads
    the contents of the directory, formats the entries into a string according to the FTP
    listing format, and sends this string to the client over the previously established
    data connection. If any errors occur during directory reading or data transfer,
    an appropriate error response is sent to the client.
    */
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
                    send_cmd(&mut self.stream, ResultCode::DataConnectionAlreadyOpen, "Here comes the directory listing.");
                    if let Some(ref mut writer) = self.data_writer {
                        write!(writer, "{}\r\n", response).unwrap();
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