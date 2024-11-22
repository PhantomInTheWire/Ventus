use crate::codec::FtpClient;
use uniffi::generate_scaffolding;

mod codec;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn apple_sync(host: String, port: u16, local_dir: String, remote_dir: String) -> bool {

    let ftp_client = FtpClient::new(host.to_string(), port);
    if let Err(e) = ftp_client.sync(&local_dir, &remote_dir) {
        eprintln!("Error syncing directories: {}", e);
        false;
    }
    true
}
fn main() {
    generate_scaffolding("src/ffi.udl").unwrap();
}