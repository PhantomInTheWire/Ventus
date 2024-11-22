use crate::codec::FtpClient;
mod codec;

pub fn apple_sync(host: String, port: u32, local_dir: String, remote_dir: String) -> bool {
    let client = FtpClient::new(host.to_string(), port as u16);
    if let Err(e) = client.sync(&local_dir, &remote_dir) {
        eprintln!("Error syncing directories: {}", e);
        false;
    }
    true
}
uniffi::include_scaffolding!("ftp_client");
