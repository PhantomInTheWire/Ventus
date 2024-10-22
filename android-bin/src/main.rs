use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::str::FromStr;
use clap::{App, Arg, SubCommand};
use colored::*;
use std::io::BufReader;

struct FtpClient {
    ftp_host: String,
    ftp_port: u16,
    timeout: std::time::Duration,
}

impl FtpClient {
    fn new(ftp_host: String, ftp_port: u16) -> Self {
        FtpClient {
            ftp_host,
            ftp_port,
            timeout: std::time::Duration::from_secs(10),
        }
    }

    fn print_colored(&self, message: &str, color: &str) {
        match color {
            "purple" => println!("{}", message.purple()),
            "cyan" => println!("{}", message.cyan()),
            "yellow" => println!("{}", message.yellow()),
            "red" => println!("{}", message.red()),
            "green" => println!("{}", message.green()),
            "blue" => println!("{}", message.blue()),
            _ => println!("{}", message),
        }
    }

    fn connect(&self) -> std::io::Result<TcpStream> {
        self.print_colored(
            &format!("Connecting to {}:{}", self.ftp_host, self.ftp_port),
            "purple",
        );

        let mut stream = TcpStream::connect((&*self.ftp_host, self.ftp_port))?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        let mut response = [0u8; 1024];
        let n = stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(&format!("Response: {}", response_str), "cyan");

        if !response_str.contains("220") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid FTP welcome message",
            ));
        }

        Ok(stream)
    }

    fn login(&self, stream: &mut TcpStream, user: &str) -> std::io::Result<()> {
        let command = format!("USER {}\r\n", user);
        stream.write_all(command.as_bytes())?;

        let mut response = [0u8; 1024];
        let n = stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(&format!("Response after login: {}", response_str), "cyan");

        if !response_str.contains("230") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Login failed",
            ));
        }

        Ok(())
    }

    fn pasv_mode(&self, stream: &mut TcpStream) -> std::io::Result<(String, u16)> {
        stream.write_all(b"PASV\r\n")?;

        let mut response = [0u8; 1024];
        let n = stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(&format!("Response after PASV: {}", response_str), "yellow");

        if !response_str.contains("227") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "PASV mode failed",
            ));
        }

        // Parse PASV response
        let start = response_str.find('(').unwrap() + 1;
        let end = response_str.find(')').unwrap();
        let pasv_info: Vec<u8> = response_str[start..end]
            .split(',')
            .map(|s| u8::from_str(s.trim()).unwrap())
            .collect();

        let data_host = format!(
            "{}.{}.{}.{}",
            pasv_info[0], pasv_info[1], pasv_info[2], pasv_info[3]
        );
        let data_port = (pasv_info[4] as u16 * 256) + pasv_info[5] as u16;

        Ok((data_host, data_port))
    }

    fn upload_file(&self, filename: &str) -> std::io::Result<()> {
        if !Path::new(filename).exists() {
            self.print_colored(&format!("File {} does not exist for upload.", filename), "red");
            return Ok(());
        }

        let mut control_stream = self.connect()?;
        self.login(&mut control_stream, "testuser")?;
        let (data_host, data_port) = self.pasv_mode(&mut control_stream)?;

        let mut data_stream = TcpStream::connect((data_host, data_port))?;

        let command = format!("STOR {}\r\n", filename);
        control_stream.write_all(command.as_bytes())?;

        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);

        if !response_str.contains("150") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initiate file transfer",
            ));
        }

        let mut file = File::open(filename)?;
        let mut buffer = [0; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 { break; }
            data_stream.write_all(&buffer[..n])?;
        }

        drop(data_stream);

        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(
            &format!("Upload of {} completed: {}", filename, response_str),
            "blue",
        );
        drop(control_stream);
        Ok(())
    }

    fn download_file(&self, filename: &str) -> std::io::Result<()> {
        let mut control_stream = self.connect()?;
        self.login(&mut control_stream, "testuser")?;
        let (data_host, data_port) = self.pasv_mode(&mut control_stream)?;

        let mut data_stream = TcpStream::connect((data_host, data_port)).expect("Failed to connect");

        let command = format!("RETR {}\r\n", filename);
        control_stream.write_all(command.as_bytes())?;

        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);

        if !response_str.contains("150") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to initiate file download",
            ));
        }

        let mut file = File::create(filename)?;
        let mut buffer = [0; 4096];

        loop {
            let n = data_stream.read(&mut buffer)?;
            if n == 0 { break; }
            file.write_all(&buffer[..n])?;
        }


        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(
            &format!("Download of {} completed: {}", filename, response_str),
            "blue",
        );
        drop(control_stream);
        drop(data_stream);
        Ok(())
    }

    fn sync(&self, local_dir: &str, remote_dir: &str) -> std::io::Result<()> {
        self.print_colored(
            &format!("Starting sync between {} and {}", local_dir, remote_dir),
            "purple",
        );

        self.sync_local_to_remote(local_dir, remote_dir)?;
        self.sync_remote_to_local(local_dir, remote_dir)?;

        self.print_colored("Sync completed.", "green");
        Ok(())
    }

    fn sync_local_to_remote(&self, local_dir: &str, remote_dir: &str) -> std::io::Result<()> {
        self.make_remote_dir(remote_dir)?;

        for entry in fs::read_dir(local_dir)? {
            let entry = entry?;
            let local_path = entry.path();
            let remote_path = Path::new(remote_dir).join(entry.file_name());

            if local_path.is_dir() {
                self.print_colored(
                    &format!(
                        "Syncing directory {:?} to {:?}",
                        local_path,
                        remote_path
                    ),
                    "purple",
                );
                self.sync_local_to_remote(
                    local_path.to_str().unwrap(),
                    remote_path.to_str().unwrap(),
                )?;
            } else {
                self.print_colored(
                    &format!(
                        "Uploading file {:?} to {:?}",
                        local_path,
                        remote_path
                    ),
                    "purple",
                );
                self.upload_file(local_path.to_str().unwrap())?;
            }
        }

        Ok(())
    }

    fn sync_remote_to_local(&self, local_dir: &str, remote_dir: &str) -> std::io::Result<()> {
        fs::create_dir_all(local_dir)?;

        let (files, dirs) = self.list_files(remote_dir)?;

        for file in files {
            let local_path = Path::new(local_dir).join(&file);
            let remote_path = Path::new(remote_dir).join(&file);

            self.print_colored(
                &format!(
                    "Downloading file {:?} to {:?}",
                    remote_path,
                    local_path
                ),
                "purple",
            );
            self.download_file(remote_path.to_str().unwrap())?;
        }

        for dir in dirs {
            let local_subdir = Path::new(local_dir).join(&dir);
            let remote_subdir = Path::new(remote_dir).join(&dir);

            fs::create_dir_all(&local_subdir)?;

            self.print_colored(
                &format!(
                    "Syncing remote directory {:?} to local {:?}",
                    remote_subdir,
                    local_subdir
                ),
                "purple",
            );

            self.sync_remote_to_local(
                local_subdir.to_str().unwrap(),
                remote_subdir.to_str().unwrap(),
            )?;
        }

        Ok(())
    }

    fn make_remote_dir(&self, remote_dir: &str) -> std::io::Result<()> {
        self.print_colored(
            &format!("Ensuring remote directory {} exists.", remote_dir),
            "purple",
        );

        let mut stream = self.connect()?;
        self.login(&mut stream, "testuser")?;

        let command = format!("MKD {}\r\n", remote_dir);
        stream.write_all(command.as_bytes())?;

        let mut response = [0u8; 1024];
        let n = stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);

        if response_str.contains("550") {
            self.print_colored(
                &format!(
                    "Remote directory {} already exists or failed to create.",
                    remote_dir
                ),
                "yellow",
            );
        } else if response_str.contains("257") {
            self.print_colored(
                &format!("Remote directory {} created successfully.", remote_dir),
                "green",
            );
        }

        Ok(())
    }

    fn list_files(&self, remote_dir: &str) -> std::io::Result<(Vec<String>, Vec<String>)> {
        self.print_colored(
            &format!("Listing files in remote directory: {}", remote_dir),
            "purple",
        );

        let mut control_stream = self.connect()?;
        self.login(&mut control_stream, "testuser")?;

        let command = format!("CWD {}\r\n", remote_dir);
        control_stream.write_all(command.as_bytes())?;

        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(&format!("Response after CWD: {}", response_str), "yellow");

        let (data_host, data_port) = self.pasv_mode(&mut control_stream)?;
        let mut data_stream = TcpStream::connect((data_host, data_port))?;

        control_stream.write_all(b"LIST\r\n")?;
        let mut response = [0u8; 1024];
        let n = control_stream.read(&mut response)?;
        drop(control_stream);
        let response_str = String::from_utf8_lossy(&response[..n]);
        self.print_colored(&format!("Response after LIST: {}", response_str), "yellow");



        println!("Directory listing received");

        let mut listing = String::new();
        BufReader::new(&mut data_stream).read_to_string(&mut listing)?;

        println!("{}", listing);

        let mut files = Vec::new();
        let mut dirs = Vec::new();

        for line in listing.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() == 3 {
                match parts[0] {
                    "FILE" => files.push(parts[2].to_string()),
                    "DIR" => dirs.push(parts[2].to_string()),
                    _ => {}
                }
            }
        }

        Ok((files, dirs))
    }
}

fn main() {
    let matches = App::new("FTP Client")
        .version("1.0")
        .author("Your Name")
        .about("FTP Client for testing and syncing")
        .subcommand(
            SubCommand::with_name("upload")
                .about("Upload a file")
                .arg(Arg::with_name("file").required(true))
                .arg(
                    Arg::with_name("host")
                        .long("host")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("Download a file")
                .arg(Arg::with_name("file").required(true))
                .arg(
                    Arg::with_name("host")
                        .long("host")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync directories")
                .arg(
                    Arg::with_name("local-dir")
                        .long("local-dir")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("remote-dir")
                        .long("remote-dir")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("host")
                        .long("host")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("port")
                        .long("port")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("upload", Some(upload_matches)) => {
            let host = upload_matches.value_of("host").unwrap();
            let port = upload_matches
                .value_of("port")
                .unwrap()
                .parse::<u16>()
                .expect("Invalid port number");
            let file = upload_matches.value_of("file").unwrap();

            let ftp_client = FtpClient::new(host.to_string(), port);
            if let Err(e) = ftp_client.upload_file(file) {
                eprintln!("Error uploading file: {}", e);
            }
        }
        ("download", Some(download_matches)) => {
            let host = download_matches.value_of("host").unwrap();
            let port = download_matches
                .value_of("port")
                .unwrap()
                .parse::<u16>()
                .expect("Invalid port number");
            let file = download_matches.value_of("file").unwrap();

            let ftp_client = FtpClient::new(host.to_string(), port);
            if let Err(e) = ftp_client.download_file(file) {
                eprintln!("Error downloading file: {}", e);
            }
        }
        ("sync", Some(sync_matches)) => {
            let host = sync_matches.value_of("host").unwrap();
            let port = sync_matches
                .value_of("port")
                .unwrap()
                .parse::<u16>()
                .expect("Invalid port number");
            let local_dir = sync_matches.value_of("local-dir").unwrap();
            let remote_dir = sync_matches.value_of("remote-dir").unwrap();

            let ftp_client = FtpClient::new(host.to_string(), port);
            if let Err(e) = ftp_client.sync(local_dir, remote_dir) {
                eprintln!("Error syncing directories: {}", e);
            }
        }
        _ => {
            println!("Invalid command. Use --help for usage information.");
        }
    }
}