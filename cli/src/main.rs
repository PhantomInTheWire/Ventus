use std::io::{Read, Write};
use std::str::FromStr;
use colored::*;
use std::time::Duration;
mod client;

fn main() {
    loop {
        let matches = clap::App::new("FTP Client")
            .version("1.0")
            .author("Your Name")
            .about("FTP Client for testing and syncing")
            .subcommand(
                clap::SubCommand::with_name("upload")
                    .about("Upload a file")
                    .arg(clap::Arg::with_name("file").required(true))
                    .arg(
                        clap::Arg::with_name("host")
                            .long("host")
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        clap::Arg::with_name("port")
                            .long("port")
                            .required(true)
                            .takes_value(true),
                    ),
            )
            .subcommand(
                clap::SubCommand::with_name("download")
                    .about("Download a file")
                    .arg(clap::Arg::with_name("file").required(true))
                    .arg(
                        clap::Arg::with_name("host")
                            .long("host")
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        clap::Arg::with_name("port")
                            .long("port")
                            .required(true)
                            .takes_value(true),
                    ),
            )
            .subcommand(
                clap::SubCommand::with_name("sync")
                    .about("Sync directories")
                    .arg(
                        clap::Arg::with_name("local-dir")
                            .long("local-dir")
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        clap::Arg::with_name("remote-dir")
                            .long("remote-dir")
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        clap::Arg::with_name("host")
                            .long("host")
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        clap::Arg::with_name("port")
                            .long("port")
                            .required(true)
                            .takes_value(true),
                    ),
            )
            .get_matches();

        let mut success = true;

        match matches.subcommand() {
            ("upload", Some(upload_matches)) => {
                let host = upload_matches.value_of("host").unwrap();
                let port = upload_matches
                    .value_of("port")
                    .unwrap()
                    .parse::<u16>()
                    .expect("Invalid port number");
                let file = upload_matches.value_of("file").unwrap();

                let ftp_client = client::FtpClient::new(host.to_string(), port);
                if let Err(e) = ftp_client.upload_file(file) {
                    eprintln!("Error uploading file: {}", e);
                    success = false;
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

                let ftp_client = client::FtpClient::new(host.to_string(), port);
                if let Err(e) = ftp_client.download_file(file) {
                    eprintln!("Error downloading file: {}", e);
                    success = false;
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

                let ftp_client = client::FtpClient::new(host.to_string(), port);
                if let Err(e) = ftp_client.sync(local_dir, remote_dir) {
                    eprintln!("Error syncing directories: {}", e);
                    success = false;
                }
            }
            _ => {
                println!("Invalid command. Use --help for usage information.");
                success = false;
            }
        }

        if success {
            break;
        } else {
            println!("Operation failed. Restarting in 1 seconds...");
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}
