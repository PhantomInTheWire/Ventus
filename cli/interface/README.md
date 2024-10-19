# Custom FTP Client

This command-line tool provides a basic interface for interacting with my custom-built FTP server. It supports uploading, downloading, and syncing files.

## Features

* **Upload:** Upload a file to the FTP server.
* **Download:** Download a file from the FTP server.
* **Sync:** Synchronize files between a local directory and a remote directory on the server.  This is a basic sync that currently only compares filenames.
* **Colored Console Output:**  Provides visually distinct output for different events.
* **PASV Mode:** Uses passive mode for data transfer.

## Requirements

* Python 3.6 or higher
* A running instance of my custom FTP server

## Installation

1. Clone the repository:

    ```bash
    git clone <repository_url> 
    ```

2. Navigate to the project directory:

    ```bash
    cd <project_directory>
    ```


## Usage

The client is used through the command line.  Here's the basic syntax:

    ```bash
    python ftp_client.py <command> --host <host> --port <port> [--file <file>] [--local-dir <local_dir>] [--remote-dir <remote_dir>]
    ```

### Commands

* `upload`: Uploads a file to the server. Requires `--file`.
* `download`: Downloads a file from the server. Requires `--file`.
* `sync`: Synchronizes files between local and remote directories. Requires `--local-dir` and `--remote-dir`.

### Options

* `--host`: The hostname or IP address of the FTP server (required).
* `--port`: The port number of the FTP server (required).
* `--file`: The name of the file to upload or download (required for `upload` and `download`).
* `--local-dir`: The path to the local directory for syncing (required for `sync`).
* `--remote-dir`: The path to the remote directory on the server for syncing (required for `sync`).

### Examples

**Upload a file:**

    ```bash
    python ftp_client.py upload --host <HOST> --port <PORT> --file my_file.txt
    ```

**Download a file:**

    ```bash
    python ftp_client.py download --host <HOST> --port <PORT> --file remote_file.txt 
    ```

**Synchronize directories:**

    ```bash
    python ftp_client.py sync --host <HOST> --port <PORT> --local-dir ./my_local_folder --remote-dir /remote_folder
    ```

**Get help:**

    ```bash
    python ftp_client.py --help
    ```


## Error Handling

The client includes basic error handling and will print messages to the console if there are issues connecting to the server, logging in, or transferring files.  Timeouts are also handled during file transfers.  More robust error handling (e.g., retry logic) could be added in future versions.


## Future Improvements

* **More Robust Syncing:** Implement checksum or timestamp comparison for more accurate synchronization.
* **Recursive Directory Syncing:** Add support for syncing entire directory trees.
* **Improved Error Handling:**  Add more detailed error messages and potentially retry mechanisms.
* **Configuration File:** Allow users to store server credentials and other settings in a configuration file.
