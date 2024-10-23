use local_ip_address::local_ip;
use qrcode::{QrCode, render::unicode};

fn main() {
    // Step 1: Find the local network IP address
    match local_ip() {
        Ok(ip) => {
            let address = format!("{}:1234", ip);
            println!("Local network address: {}", address);

            // Step 2: Generate a QR code for the address
            let code = QrCode::new(&address).unwrap();

            // Step 3: Render QR code to terminal
            let qr_display = code.render::<unicode::Dense1x2>().build();
            println!("QR Code:\n{}", qr_display);
        },
        Err(e) => println!("Failed to get local IP address: {}", e),
    }
}

