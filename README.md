# Ventus

Cross Platform FTP based file synchronization system

## How it Works

Ventus is a cross-platform FTP-based file synchronization system. It consists of a central FTP server, a desktop application, an iOS application, and a command-line interface.

### Components

*   **`sync-ftp-server` (Rust):** The core of the system. It's an FTP server that listens for connections from clients and handles file synchronization. It uses the `qrcode-helper` to display a QR code in the terminal for easy connection from mobile devices.
*   **`cli` (Rust):** A command-line interface for interacting with the FTP server. It can be used to upload, download, and synchronize files. It's also used by the `daemon` and `desktop-app`.
*   **`ftp_client` (Rust):** A library that provides FTP client functionality. It's used by the iOS app to communicate with the server. It uses `uniffi` to generate a Swift FFI.
*   **`qrcode-helper` (Rust):** A small utility that generates a QR code containing the local IP address of the server. This QR code is displayed by the `sync-ftp-server` to allow for easy connection from the iOS app.
*   **`web-setup` (Rust):** An `actix-web` server that provides a web interface for setting up the synchronization. It has an endpoint `/run-setup` that uses the `cli` to initiate a sync.
*   **`desktop-app` (Python & PyQt5):** A graphical user interface for managing the file synchronization. It allows users to connect to the server, manage settings, and view the synchronized files. It uses the `cli` to perform the actual synchronization.
*   **`daemon` (Python):** A background process that monitors a local directory for changes and automatically synchronizes them with the server. It uses the `watchdog` library to monitor the file system and the `cli` to perform the synchronization.
*   **`Ventus-iOS` (Swift):** An iOS application that allows users to connect to the server and synchronize files from their mobile devices. It uses the `ftp_client` library to communicate with the server.

### How it all comes together

1.  The `sync-ftp-server` is started. It displays a QR code in the terminal.
2.  The user can connect to the server using one of the clients:
    *   **`desktop-app`:** The user can scan the QR code or manually enter the IP address to connect. The app then uses the `cli` to synchronize files between a local directory and the server.
    *   **`Ventus-iOS`:** The user can scan the QR code to connect. The app then uses the `ftp_client` library to synchronize files.
    *   **`cli`:** The user can use the command-line interface to manually upload, download, or synchronize files.
3.  The `daemon` can be used to automatically synchronize a directory in the background.
4.  The `web-setup` provides an alternative way to configure and initiate the synchronization through a web interface.

### FFI for iOS and macOS

The `ftp_client` library is written in Rust and used in the `Ventus-iOS` app (written in Swift). This is made possible by using a Foreign Function Interface (FFI). In this project, `uniffi` is used to generate the Swift bindings for the Rust code.

Here's how it works:

1.  **`ftp_client.udl`:** An interface definition file (`.udl`) is created that defines the functions and data structures that should be exposed to other languages. In this case, it defines the `apple_sync` function.
2.  **`uniffi-bindgen`:** The `uniffi-bindgen` tool is used to generate the Swift FFI code from the `.udl` file. This tool is run as part of the build process.
3.  **Swift Integration:** The generated Swift code provides a simple and type-safe way to call the Rust functions from Swift. The `Ventus-iOS` app can then call the `apple_sync` function as if it were a native Swift function.

This approach allows for the core FTP client logic to be written in Rust, which is fast and secure, while the UI can be written in Swift, which is the native language for iOS and macOS development.

## 🤝 Contributing

All contributions welcome
