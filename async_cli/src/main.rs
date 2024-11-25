use clap::Parser;
use std::path::PathBuf;
use tokio;
use suppaftp::FtpStream;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;
use std::path::Path;
use notify::DebouncedEvent;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Local directory to sync
    #[arg(short, long)]
    local_dir: PathBuf,

    /// FTP server address
    #[arg(short, long, default_value = "127.0.0.1:1234")]
    server: String,

    /// FTP username
    #[arg(short, long, default_value = "anonymous")]
    username: String,

    /// FTP password
    #[arg(short, long, default_value = "anonymous")]
    password: String,
}

async fn sync_file(
    ftp_stream: &mut FtpStream,
    local_path: &Path,
    base_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let relative_path = local_path.strip_prefix(base_dir)?;
    let remote_path = relative_path.to_string_lossy();

    // Get local file size
    let local_size = fs::metadata(local_path)?.len();
    
    // Try to get remote file size, if it exists
    let remote_size = ftp_stream.size(&remote_path).unwrap_or(0) as u64;

    if local_size != remote_size {
        println!("Syncing file: {}", relative_path.display());
        
        // Ensure parent directories exist
        if let Some(parent) = relative_path.parent() {
            let _ = ftp_stream.mkdir(parent.to_string_lossy().as_ref());
        }

        // Open the file as a reader instead of reading all contents into memory
        let mut file = fs::File::open(local_path)?;
        ftp_stream.put_file(&remote_path, &mut file)?;
        println!("Successfully synced: {}", relative_path.display());
    }

    Ok(())
}

async fn sync_directory(
    ftp_stream: &mut FtpStream,
    dir_path: &Path,
    base_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Err(e) = sync_file(ftp_stream, &path, base_dir).await {
                eprintln!("Error syncing file {}: {}", path.display(), e);
            }
        } else if path.is_dir() {
            if let Err(e) = Box::pin(sync_directory(ftp_stream, &path, base_dir)).await {
                eprintln!("Error syncing directory {}: {}", path.display(), e);
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create FTP connection
    let mut ftp_stream = FtpStream::connect(&args.server)?;
    ftp_stream.login(&args.username, &args.password)?;
    println!("Connected to FTP server");

    // Perform initial sync
    println!("Performing initial directory sync...");
    sync_directory(&mut ftp_stream, &args.local_dir, &args.local_dir).await?;
    println!("Initial sync completed");

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(2))?;
    watcher.watch(&args.local_dir, RecursiveMode::Recursive)?;

    println!("Watching directory: {:?}", args.local_dir);
    println!("Press Ctrl+C to stop.");

    // Watch for changes
    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                        if path.is_file() {
                            if let Err(e) = sync_file(&mut ftp_stream, &path, &args.local_dir).await {
                                eprintln!("Error syncing file {}: {}", path.display(), e);
                            }
                        }
                    },
                    DebouncedEvent::Error(e, path) => {
                        eprintln!("Watch error for {:?}: {:?}", path, e);
                    },
                    _ => {} // Ignore other events for simplicity
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
}
