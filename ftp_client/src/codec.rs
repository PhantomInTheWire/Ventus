use std::fs::{self, File};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::str::FromStr;
use colored::*;
use std::io::BufReader;
use std::time::Duration;

#[derive(unifii:Enum)]
pub struct FtpClient {
    ftp_host: String,
    ftp_port: u16,
    timeout: Duration,
    max_retries: u32,
    retry_delay: Duration,
}

impl FtpClient {
    pub fn new(ftp_host: String, ftp_port: u16) -> Self {
        FtpClient {
            ftp_host,
            ftp_port,
            timeout: Duration::from_millis(50),
            max_retries: 3,
            retry_delay: Duration::from_millis(50),
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
        for attempt in 0..self.max_retries {
            self.print_colored(
                &format!(
                    "Connecting to {}:{} (attempt {}/{})",
                    self.ftp_host, self.ftp_port, attempt + 1, self.max_retries
                ),
                "purple",
            );

            match TcpStream::connect((&*self.ftp_host, self.ftp_port)) {
                Ok(mut stream) => {
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

                    return Ok(stream);
                }
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Connection failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }

    fn login(&self, stream: &mut TcpStream, user: &str) -> std::io::Result<()> {
        for attempt in 0..self.max_retries {
            match self.attempt_login(stream, user) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Login failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        unreachable!()
    }

    fn attempt_login(&self, stream: &mut TcpStream, user: &str) -> std::io::Result<()> {
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
        for attempt in 0..self.max_retries {
            match self.attempt_pasv_mode(stream) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("PASV mode failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }

    fn attempt_pasv_mode(&self, stream: &mut TcpStream) -> std::io::Result<(String, u16)> {
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

        // Parse PASV response (same as before)
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

    pub fn upload_file(&self, filename: &str) -> std::io::Result<()> {
        if !Path::new(filename).exists() {
            self.print_colored(&format!("File {} does not exist for upload.", filename), "red");
            return Ok(());
        }

        for attempt in 0..self.max_retries {
            match self.attempt_upload_file(filename) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Upload failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        unreachable!()
    }

    fn attempt_upload_file(&self, filename: &str) -> std::io::Result<()> {
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
            if n == 0 {
                break;
            }
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

    pub fn download_file(&self, filename: &str) -> std::io::Result<()> {
        for attempt in 0..self.max_retries {
            match self.attempt_download_file(filename) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Download failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }

    fn attempt_download_file(&self, filename: &str) -> std::io::Result<()> {
        let mut control_stream = self.connect()?;
        self.login(&mut control_stream, "testuser")?;
        let (data_host, data_port) = self.pasv_mode(&mut control_stream)?;

        let mut data_stream = loop {
            match TcpStream::connect((data_host.clone(), data_port)) { // Clone data_host
                Ok(stream) => break stream,
                Err(e) => {
                    self.print_colored(
                        &format!("Failed to connect to data port: {}. Retrying...", e),
                        "yellow",
                    );
                    std::thread::sleep(self.retry_delay);
                }
            }
        };

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
            if n == 0 {
                break;
            }
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

    pub fn sync(&self, local_dir: &str, remote_dir: &str) -> std::io::Result<()> {
        self.print_colored(
            &format!("Starting sync between {} and {}", local_dir, remote_dir),
            "purple",
        );

        for attempt in 0..self.max_retries {
            match self.attempt_sync(local_dir, remote_dir) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Sync failed: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }

    fn attempt_sync(&self, local_dir: &str, remote_dir: &str) -> std::io::Result<()> {
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
                        local_path, remote_path
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
                        local_path, remote_path
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
                    remote_path, local_path
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
                    remote_subdir, local_subdir
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

        for attempt in 0..self.max_retries {
            match self.attempt_make_remote_dir(remote_dir) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Failed to make remote directory: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }

    fn attempt_make_remote_dir(&self, remote_dir: &str) -> std::io::Result<()> {
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

        for attempt in 0..self.max_retries {
            match self.attempt_list_files(remote_dir) {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt < self.max_retries - 1 {
                        self.print_colored(
                            &format!("Failed to list files: {}. Retrying...", e),
                            "yellow",
                        );
                        std::thread::sleep(self.retry_delay);
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        unreachable!()
    }


    fn attempt_list_files(&self, remote_dir: &str) -> std::io::Result<(Vec<String>, Vec<String>)> {
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