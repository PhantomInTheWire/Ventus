import argparse
import os
import socket
from logging import exception

from ftp_utils import ConsoleUtils

class FtpClient:
    def __init__(self, ftp_host, ftp_port):
        self.ftp_host = ftp_host
        self.ftp_port = ftp_port
        self.console = ConsoleUtils()
        self.timeout = 10

    def connect(self):
        """Establishes a connection to the FTP server."""
        self.console.print_with_color(f"Connecting to {self.ftp_host}:{self.ftp_port}", 'purple')
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect((self.ftp_host, self.ftp_port))
        response = s.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response: {response}", 'cyan')
        assert "220" in response  # FTP welcome message
        return s

    def login(self, s, user="testuser"):
        """Logs into the FTP server."""
        s.sendall(f"USER {user}\r\n".encode("utf-8"))
        response = s.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response after login: {response}", 'cyan')
        assert "230" in response  # Login successful

    def pasv_mode(self, s):
        """Enters passive mode and returns data connection details."""
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response after PASV: {response}", 'yellow')
        assert "227" in response

        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])
        return data_host, data_port

    def upload_file(self, filename):
        """Uploads a file to the FTP server."""
        if not os.path.exists(filename):
            self.console.print_with_color(f"File {filename} does not exist for upload.", 'red')
            return

        s = self.connect()
        self.login(s)
        data_host, data_port = self.pasv_mode(s)

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"STOR {filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            assert "150" in response

            with open(filename, "rb") as f:
                while chunk := f.read(4096):
                    data_socket.sendall(chunk)
            data_socket.close()
            try:
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Upload of {filename} completed: {response}", 'bcyan')
                assert "226" in response
            except socket.timeout:
                self.console.print_with_color(f"Timeout waiting for server response after uploading {filename}", 'red')

    def download_file(self, filename):
        """Downloads a file from the FTP server."""
        s = self.connect()
        self.login(s)
        data_host, data_port = self.pasv_mode(s)

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"RETR {filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            assert "150" in response

            with open(filename, "wb") as f:
                while data := data_socket.recv(4096):
                    f.write(data)
            data_socket.close()
            try:
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Download of {filename} completed: {response}", 'bcyan')
                assert "226" in response

            except socket.timeout:
                self.console.print_with_color(f"Timeout waiting for server response after downloading {filename}", 'red')

    def sync(self, local_dir, remote_dir):
        """
        Syncs files and directories between a local directory and the server.
        """
        self.console.print_with_color(f"Starting sync between {local_dir} and {remote_dir}", 'purple')
        
        # Sync local to remote
        self.sync_local_to_remote(local_dir, remote_dir)

        # Sync remote to local
        self.sync_remote_to_local(local_dir, remote_dir)

        self.console.print_with_color("Sync completed.", 'bgreen')

    def sync_local_to_remote(self, local_dir, remote_dir):
        """Syncs files and directories from local to remote."""
        local_items = os.listdir(local_dir)
        
        # Ensure remote directory exists or create it
        self.make_remote_dir(remote_dir)
        
        for item in local_items:
            local_path = os.path.join(local_dir, item)
            remote_path = os.path.join(remote_dir, item)

            if os.path.isdir(local_path):
                self.console.print_with_color(f"Syncing directory {local_path} to {remote_path}", 'purple')
                self.sync_local_to_remote(local_path, remote_path)  # Recursively sync the directory
            else:
                self.console.print_with_color(f"Uploading file {local_path} to {remote_path}", 'purple')
                self.upload_file(local_path)

    def sync_remote_to_local(self, local_dir, remote_dir):
        """Syncs files and directories from remote to local."""
        remote_files, remote_dirs = self.list_files(remote_dir)  # Get files and directories

        # Ensure local directory exists
        os.makedirs(local_dir, exist_ok=True)

        # Sync files from remote to local
        for remote_file in remote_files:
            local_path = os.path.join(local_dir, remote_file)
            remote_path = os.path.join(remote_dir, remote_file)

            self.console.print_with_color(f"Downloading file {remote_path} to {local_path}", 'purple')
            self.download_file(remote_path)

        # Recursively sync directories
        for remote_subdir in remote_dirs:
            local_subdir_path = os.path.join(local_dir, remote_subdir)
            remote_subdir_path = os.path.join(remote_dir, remote_subdir)

            # Ensure subdirectory exists locally
            os.makedirs(local_subdir_path, exist_ok=True)

            self.console.print_with_color(f"Syncing remote directory {remote_subdir_path} to local {local_subdir_path}", 'purple')

            # Recursively sync this subdirectory
            self.sync_remote_to_local(local_subdir_path, remote_subdir_path)

    def make_remote_dir(self, remote_dir):
        """Creates a directory on the remote server if it doesn't exist."""
        self.console.print_with_color(f"Ensuring remote directory {remote_dir} exists.", 'purple')
        s = self.connect()
        self.login(s)
        s.sendall(f"MKD {remote_dir}\r\n".encode("utf-8"))
        response = s.recv(1024).decode("utf-8")
        if "550" in response:
            self.console.print_with_color(f"Remote directory {remote_dir} already exists or failed to create.", 'yellow')
        elif "257" in response:
            self.console.print_with_color(f"Remote directory {remote_dir} created successfully.", 'green')
        s.close()

    def is_remote_directory(self, remote_path):
        """Checks if a path on the remote server is a directory."""
        s = self.connect()
        self.login(s)
        s.sendall(f"LIST {remote_path}\r\n".encode("utf-8"))
        response = s.recv(1024).decode("utf-8")
        s.close()

        # If the path is a directory, the LIST command will return details about its contents
        return "dir" in response.lower() or len(response.splitlines()) > 1

    def list_files(self, remote_dir):
        """
        Lists files and directories in the specified remote directory.
        Returns two sets: filenames and directory names.
        """
        self.console.print_with_color(f"Listing files in remote directory: {remote_dir}", 'purple')

        # Connect to the server and log in
        s = self.connect()
        self.login(s)

        # Change working directory to the remote directory
        s.sendall(f"CWD {remote_dir}\r\n".encode("utf-8"))
        response = s.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response after CWD: {response}", 'yellow')
        assert "250" in response or "200" in response  # Successful directory change

        # Enter passive mode and get data connection details
        data_host, data_port = self.pasv_mode(s)

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(b"LIST\r\n")  # List the current working directory
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after LIST: {response}", 'yellow')
            assert "150" in response or "125" in response  # Check for a response indicating the server is sending data

            # Receive the directory listing from the data connection
            listing = data_socket.recv(4096).decode("utf-8")
            self.console.print_with_color(f"Directory listing:\n{listing}", 'bcyan')

            # Extract filenames from the listing
            filenames = set()
            directory_names = set()  # To keep track of directories
            for line in listing.splitlines():
                parts = line.split("\t")
                if len(parts) == 3:
                    entry_type, size, name = parts
                    if entry_type == "FILE":
                        filenames.add(name)
                    elif entry_type == "DIR":
                        directory_names.add(name)  # Keep track of directories

            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after data transfer: {response}", 'bcyan')
            assert "226" in response  # Successful transfer completion

        s.close()

        # Return both file names and directory names separately
        return filenames, directory_names


def main():
    parser = argparse.ArgumentParser(description='FTP Client for testing and syncing.')
    parser.add_argument('command', choices=['upload', 'download', 'sync'], help='Command to execute.')
    parser.add_argument('--host', required=True, help='FTP server host.')
    parser.add_argument('--port', type=int, required=True, help='FTP server port.')
    parser.add_argument('--file', help='File to upload or download.')
    parser.add_argument('--local-dir', help='Local directory for sync.')
    parser.add_argument('--remote-dir', help='Remote directory for sync.')
    args = parser.parse_args()

    ftp_client = FtpClient(args.host, args.port)

    if args.command == 'upload' and args.file:
        ftp_client.upload_file(args.file)
    elif args.command == 'download' and args.file:
        ftp_client.download_file(args.file)
    elif args.command == 'sync' and args.local_dir and args.remote_dir:
        ftp_client.sync(args.local_dir, args.remote_dir)
    else:
        print("Invalid arguments for the chosen command. Use --help for more information.")

if __name__ == "__main__":
   main()

