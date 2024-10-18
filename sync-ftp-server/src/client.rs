use std::net::{TcpStream, TcpListener, SocketAddr, IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::fs::{create_dir, remove_dir_all, File, OpenOptions, read_dir};
use std::io::{Read, Write, ErrorKind};
use std::env;

use crate::command::{Command, ResultCode};
use crate::utils::{send_cmd, read_all_message};

pub struct Client {
    cwd: PathBuf,
    stream: TcpStream,
    name: Option<String>,
    data_writer: Option<TcpStream>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            cwd: PathBuf::from("/"),
            stream,
            name: None,
            data_writer: None,
        }
    }

    pub fn handle_client(mut stream: TcpStream) {
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

                    // Get local IP
                    let local_ip = self.stream.local_addr().unwrap().ip();

                    // Calculate p1 and p2 for the PASV response (address is hardcoded as 127,0,0,1)
                    let p1 = port / 256;
                    let p2 = port % 256;

                    let ip_parts: Vec<u8> = match local_ip {
                        IpAddr::V4(ip) => ip.octets().to_vec(),
                        _ => vec![127, 0, 0, 1], // Fallback to localhost if not IPv4
                    };

                    send_cmd(
                        &mut self.stream,
                        ResultCode::EnteringPassiveMode,
                        &format!("Entering Passive Mode ({},{},{},{},{},{})",
                                 ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3], p1, p2),
                    );
                    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port); // Bind to all interfaces
                    let listener = match TcpListener::bind(&addr) {
                        Ok(listener) => listener,
                        Err(e) => {
                            println!("Error binding to data port: {}", e);
                            send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "Failed to open data connection.");
                            return;
                        }
                    };

                    match listener.incoming().next() {
                        Some(Ok(client)) => {
                            self.data_writer = Some(client);
                        },
                        Some(Err(e)) => {
                            println!("Error accepting data connection: {}", e);
                            send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "Failed to open data connection.");
                            return;
                        }
                        None => {
                            println!("No incoming data connection.");
                            send_cmd(&mut self.stream, ResultCode::CantOpenDataConnection, "Failed to open data connection.");
                            return;
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
