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
        Syncs files between a local directory and the server.
        """
        self.console.print_with_color(f"Starting sync between {local_dir} and {remote_dir}", 'purple')
        
        # This is a simple implementation. It can be extended with checksums or timestamp comparisons.
        local_files = set(os.listdir(local_dir))
        # Placeholder for fetching remote files - typically requires parsing `LIST` output
        remote_files = self.list_files(remote_dir)

        # Files to upload
        for filename in local_files - remote_files:
            try:
                self.upload_file(os.path.join(local_dir, filename))
            except exception as e:
                self.console.print_with_color(f"this is not supposed: {e} ")

        # Files to download
        for filename in remote_files - local_files:
            try:
                self.download_file(os.path.join(remote_dir, filename))
            except exception as e:
                self.console.print_with_color(f"this is not supposed: {e} ")

        self.console.print_with_color("Sync completed.", 'bgreen')

    def list_files(self, remote_dir):
        """
        Lists files in the specified remote directory.
        Returns a set of filenames (excluding directories).
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
            s.sendall(b"LIST\r\n")  # List the current working directory (which is now 'files')
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after LIST: {response}", 'yellow')
            assert "150" in response or "125" in response  # Check for a response indicating the server is sending data
    
            # Receive the directory listing from the data connection
            listing = data_socket.recv(4096).decode("utf-8")
            self.console.print_with_color(f"Directory listing:\n{listing}", 'bcyan')
    
            # Extract filenames from the listing (Robust for the provided format)
            filenames = set()
            for line in listing.splitlines():
                parts = line.split()
                if len(parts) >= 3 and parts[0] != "DIR":  # Check if it's a file and not a directory
                    filenames.add(parts[-1])  # Filename is the last part
    
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after data transfer: {response}", 'bcyan')
            assert "226" in response  # Successful transfer completion
    
        s.close()
        return filenames
    

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
