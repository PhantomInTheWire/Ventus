use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::{Read, Write};
use tokio::fs;
use std::path::Path;
use std::sync::Arc;
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
struct FtpConfig {
    host: String,
    port: u16,
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SyncRequest {
    local_dir: String,
    remote_dir: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileListResponse {
    files: Vec<String>,
    directories: Vec<String>,
}

struct FtpClient {
    config: FtpConfig,
}

impl FtpClient {
    fn new(config: FtpConfig) -> Self {
        Self { config }
    }

    fn connect(&self) -> std::io::Result<TcpStream> {
        println!("{}", format!("Connecting to {}:{}", self.config.host, self.config.port).purple());
        let mut stream = TcpStream::connect((&*self.config.host, self.config.port))?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        println!("{}", format!("Response: {}", response).cyan());
        assert!(response.contains("220"));
        Ok(stream)
    }

    fn login(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let login_cmd = format!("USER {}\r\n", self.config.username);
        stream.write_all(login_cmd.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("{}", format!("Response after login: {}", response).cyan());
        assert!(response.contains("230"));
        Ok(())
    }

    fn enter_pasv_mode(&self, stream: &mut TcpStream) -> std::io::Result<(String, u16)> {
        stream.write_all(b"PASV\r\n")?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        println!("{}", format!("Response after PASV: {}", response).yellow());
        assert!(response.contains("227"));

        let pasv_info: Vec<&str> = response.split('(')
            .nth(1)
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split(',')
            .collect();

        let data_host = pasv_info[..4].join(".");
        let data_port = pasv_info[4].parse::<u16>().unwrap() * 256 +
                       pasv_info[5].parse::<u16>().unwrap();

        Ok((data_host, data_port))
    }
}

async fn upload_file(
    mut payload: Multipart,
    config: web::Data<Arc<FtpConfig>>,
) -> Result<HttpResponse> {
    let client = FtpClient::new(FtpConfig {
        host: config.host.clone(),
        port: config.port,
        username: config.username.clone(),
    });

    while let Some(mut field) = payload.try_next().await? {
        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("unknown")
            .to_string();

        let mut stream = client.connect()?;
        client.login(&mut stream)?;
        let (data_host, data_port) = client.enter_pasv_mode(&mut stream)?;

        let mut data_stream = TcpStream::connect((data_host, data_port))?;
        stream.write_all(format!("STOR {}\r\n", filename).as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        assert!(response.contains("150"));

        while let Some(chunk) = field.try_next().await? {
            data_stream.write_all(&chunk)?;
        }

        response.clear();
        stream.read_to_string(&mut response)?;
        assert!(response.contains("226"));
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

async fn download_file(
    filename: web::Path<String>,
    config: web::Data<Arc<FtpConfig>>,
) -> Result<HttpResponse> {
    let client = FtpClient::new(FtpConfig {
        host: config.host.clone(),
        port: config.port,
        username: config.username.clone(),
    });

    let mut stream = client.connect()?;
    client.login(&mut stream)?;
    let (data_host, data_port) = client.enter_pasv_mode(&mut stream)?;

    let mut data_stream = TcpStream::connect((data_host, data_port))?;
    stream.write_all(format!("RETR {}\r\n", filename).as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    assert!(response.contains("150"));

    let mut file_data = Vec::new();
    data_stream.read_to_end(&mut file_data)?;

    response.clear();
    stream.read_to_string(&mut response)?;
    assert!(response.contains("226"));

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .body(file_data))
}

async fn list_files(
    path: web::Path<String>,
    config: web::Data<Arc<FtpConfig>>,
) -> Result<HttpResponse> {
    let client = FtpClient::new(FtpConfig {
        host: config.host.clone(),
        port: config.port,
        username: config.username.clone(),
    });

    let mut stream = client.connect()?;
    client.login(&mut stream)?;

    stream.write_all(format!("CWD {}\r\n", path).as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    assert!(response.contains("250") || response.contains("200"));

    let (data_host, data_port) = client.enter_pasv_mode(&mut stream)?;
    let mut data_stream = TcpStream::connect((data_host, data_port))?;

    stream.write_all(b"LIST\r\n")?;
    response.clear();
    stream.read_to_string(&mut response)?;
    assert!(response.contains("150") || response.contains("125"));

    let mut listing = String::new();
    data_stream.read_to_string(&mut listing)?;

    let mut files = Vec::new();
    let mut directories = Vec::new();

    for line in listing.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 3 {
            match parts[0] {
                "FILE" => files.push(parts[2].to_string()),
                "DIR" => directories.push(parts[2].to_string()),
                _ => {}
            }
        }
    }

    Ok(HttpResponse::Ok().json(FileListResponse {
        files,
        directories,
    }))
}

async fn sync_directories(
    req: web::Json<SyncRequest>,
    config: web::Data<Arc<FtpConfig>>,
) -> Result<HttpResponse> {
    let client = FtpClient::new(FtpConfig {
        host: config.host.clone(),
        port: config.port,
        username: config.username.clone(),
    });

    // Create remote directory if it doesn't exist
    let mut stream = client.connect()?;
    client.login(&mut stream)?;
    stream.write_all(format!("MKD {}\r\n", req.remote_dir).as_bytes())?;

    // Implement syncing logic here...
    // For brevity, this is a simplified version that just acknowledges the request
    // You would need to implement the full syncing logic similar to the Python version

    Ok(HttpResponse::Ok().body("Sync operation initiated"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Arc::new(FtpConfig {
        host: std::env::var("FTP_HOST").unwrap_or_else(|_| "localhost".to_string()),
        port: std::env::var("FTP_PORT")
            .unwrap_or_else(|_| "21".to_string())
            .parse()
            .unwrap_or(21),
        username: std::env::var("FTP_USER").unwrap_or_else(|_| "anonymous".to_string()),
    });

    println!("Starting FTP backend server on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/upload", web::post().to(upload_file))
            .route("/download/{filename}", web::get().to(download_file))
            .route("/list/{path:.*}", web::get().to(list_files))
            .route("/sync", web::post().to(sync_directories))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}