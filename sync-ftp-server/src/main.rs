use colored::Colorize;
use std::net::TcpListener;
use std::thread;
use qrcode::QrCode;
use qrcode::render::unicode;
use local_ip_address::local_ip;
mod client;
mod command;
mod utils;

fn get_local_ip() -> String {
    match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(_) => String::from("127.0.0.1")
    }
}

fn generate_qr_code(ip: &str, port: u16) -> String {
    let address = format!("{}:{}", ip, port);
    let code = QrCode::new(address.as_bytes()).unwrap();
    code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .build()
}

fn main() {
    let ascii = r#"
           .%@@@@@@@@@@@@@@@@@@@@@@@%:.                       .=@@@@@@@@@@@@@@@@@@@@@@@@+.
            :#@@@@@@@@@@@@@@@@@@@@@@@%-.                    ..*@@@@@@@@@@@@@@@@@@@@@@@%=.
                           .+@@@@@@@@@%:.                   .*@@@@@@@@@@=.
               ............  =@@@@@@@@@@-.                 .*@@@@@@@@@%-. ...........
               .#@@@@@@@@@%:..=@@@@@@@@@@=.              ..*@@@@@@@@@@- .-@@@@@@@@@@=.
                .#@@@@@@@@@%. .-@@@@@@@@@@-              .*@@@@@@@@@@:. -%@@@@@@@@@=.
                 .*@@@@@@@@@@:. -@@@@@@@@@@+.           .#@@@@@@@@@%. .-@@@@@@@@@@-
                  .*@@@@@@@@@%- .-@@@@@@@@@@+.        .:%@@@@@@@@@%:..=@@@@@@@@@@-.
                    =@@@@@@@@@@-. :@@@@@@@@@@+.       .%@@@@@@@@@%. .=@@@@@@@@@@:.
                    .+@@@@@@@@@@=..-@@@@@@@@@@*.    .:@@@@@@@@@@#...+@@@@@@@@@%:
                     .=@@@@@@@@@@=. :@@@@@@@@@@*.   -%@@@@@@@@@#. .+@@@@@@@@@@-.
                      .=@@@@@@@@@@+. :%@@@@@@@@@#. :@@@@@@@@@@*. .*@@@@@@@@@%:.
                       .=@@@@@@@@@@+..:%@@@@@@@@@%*@@@@@@@@@@*...#@@@@@@@@@#:
                        .-@@@@@@@@@@+...%@@@@@@@@@@@@@@@@@@@*. .#@@@@@@@@@%..
                          :@@@@@@@@@@#...#@@@@@@@@@@@@@@@@@+  .#@@@@@@@@@#.
                          .:@@@@@@@@@@#...#@@@@@@@@@@@@@@@+. :%@@@@@@@@@#:
                           ..%@@@@@@@@@#...*@@@@@@@@@@@@@=. :#@@@@@@@@@#.
                             :#@@@@@@@@@%:..*@@@@@@@@@@@-..:%@@@@@@@@@+.
                              .#@@@@@@@@@%:..*@@@@@@@@@-..-%@@@@@@@@@*.
                               .*@@@@@@@@@@:..+@@@@@@@:..:%@@@@@@@@@*.
                                .*@@@@@@@@@@-..+@@@@%:..-@@@@@@@@@@+.
                                ..*@@@@@@@@@@-..=@@%:..-@@@@@@@@@@=.
                                  .+@@@@@@@@@@=..-+. .=@@@@@@@@@@=.
                                  ..+@@@@@@@@@@=.   .+@@@@@@@@@@-.
                                    .=@@@@@@@@@@+. .+@@@@@@@@@@-.
                                     .=@@@@@@@@@@*:+@@@@@@@@@%:.
                                      .-@@@@@@@@@@@@@@@@@@@@@-.
                                       .:%@@@@@@@@@@@@@@@@@%:.
                                        .-@@@@@@@@@@@@@@@@%:
                                         .:@@@@@@@@@@@@@@%:.
                                           :%@@@@@@@@@@@#.
                                           ..%@@@@@@@@@#.
                                             .#@@@@@@@#.
                                              .*@@@@@+.
                                               .*@@@*.
                                                .+@*.
"#;
    println!("{}", ascii.purple());
    
    let port = 1234;
    let ip = get_local_ip();
    let qr_code = generate_qr_code(&ip, port);
    
    println!("\n[*] Server Information:");
    println!("[*] IP Address: {}", ip.green());
    println!("[*] Port: {}", port.to_string().green());
    println!("\n[*] Scan this QR code to connect:");
    println!("{}", qr_code.blue());
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .expect("Couldn't bind this address...");
    println!("\n[*] Waiting for clients to connect...");
    
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || {
                client::Client::handle_client(stream);
            });
        } else {
            println!("[*] A client tried to connect...");
        }
    }
}
