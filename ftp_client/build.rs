fn main() {
    uniffi::generate_scaffolding("src/ftp_client.udl").unwrap();
}