import socket
import os
from ftp_utils import ConsoleUtils

class FtpTester:
    def __init__(self, ftp_host, ftp_port):
        self.ftp_host = ftp_host
        self.ftp_port = ftp_port
        self.console = ConsoleUtils()

    def test_ftp_connection(self):
        """Tests basic FTP connection."""
        self.console.print_with_color(f"Testing FTP connection to {self.ftp_host}:{self.ftp_port}", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response: {response}", 'cyan')
            assert "220 Welcome toftp_tester this FTP server!" in response

    def test_ftp_user_login(self):
        """Tests user login."""
        self.console.print_with_color("Testing user login...", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            s.recv(1024)  # Receive the initial welcome message
            s.sendall(b"USER testuser\r\n")
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after login: {response}", 'cyan')
            assert "230 Welcome testuser" in response

    def test_ftp_pwd(self):
        """Tests getting the current working directory."""
        self.console.print_with_color("Testing PWD command...", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            s.recv(1024)  # Receive the initial welcome message
            s.sendall(b"USER testuser\r\n")
            s.recv(1024)  # Receive the login confirmation
            s.sendall(b"PWD\r\n")  # Send PWD command
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after PWD: {response}", 'cyan')
            assert "257" in response  # PWD should return the current directory

    def test_ftp_pasv_and_list(self):
        """Tests entering passive mode and listing directory contents."""
        self.console.print_with_color("Testing PASV and LIST commands...", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            s.recv(1024)  # Receive the initial welcome message
            s.sendall(b"USER testuser\r\n")
            s.recv(1024)  # Receive the login confirmation
            s.sendall(b"PASV\r\n")  # Send PASV command to enter passive mode
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after PASV: {response}", 'yellow')
            assert "227" in response  # Check if we successfully entered passive mode

            # Parse the IP and port from the response
            pasv_info = response.split("(")[1].split(")")[0].split(",")
            data_host = ".".join(pasv_info[:4])
            data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

            # Open data connection for file list
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
                data_socket.connect((data_host, data_port))
                s.sendall(b"LIST\r\n")
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after LIST: {response}", 'cyan')
                assert "125" in response  # Ready to transfer listing data

                listing = data_socket.recv(4096).decode("utf-8")
                self.console.print_with_color(f"Directory listing:\n{listing}", 'bcyan')
                assert listing  # Assert that the listing is not empty

                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after data transfer: {response}", 'bcyan')
                assert "226" in response  # Check if the connection closed successfully

    def test_ftp_file_upload_and_download(self):
        """Tests file upload and download without deleting files."""
        upload_filename = "sample.pdf"
        download_filename = "sample2.pdf"

        # Check if the upload file exists
        if not os.path.exists(upload_filename):
            self.console.print_with_color(f"File {upload_filename} does not exist for upload.", 'red')
            return

        # Upload the file
        self.console.print_with_color("Testing file upload...", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            s.recv(1024)  # Receive the initial welcome message
            s.sendall(b"USER testuser\r\n")
            s.recv(1024)  # Receive the login confirmation
            s.sendall(b"PASV\r\n")  # Enter passive mode
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after PASV: {response}", 'yellow')
            assert "227" in response  # Check for passive mode entry

            # Parse the IP and port for the data connection
            pasv_info = response.split("(")[1].split(")")[0].split(",")
            data_host = ".".join(pasv_info[:4])
            data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

            # Upload file via data connection
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
                data_socket.connect((data_host, data_port))
                s.sendall(f"STOR {upload_filename}\r\n".encode("utf-8"))
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after STOR: {response}", 'cyan')
                assert "150" in response  # Ready to receive data

                with open(upload_filename, "rb") as f:
                    while chunk := f.read(4096):
                        data_socket.sendall(chunk)
                data_socket.close()
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after upload: {response}", 'bcyan')
                assert "226" in response  # Successful file transfer

        # Download the file
        self.console.print_with_color("Testing file download...", 'purple')
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.ftp_host, self.ftp_port))
            s.recv(1024)  # Receive the initial welcome message
            s.sendall(b"USER testuser\r\n")
            s.recv(1024)  # Receive the login confirmation
            s.sendall(b"PASV\r\n")  # Enter passive mode for downloading
            response = s.recv(1024).decode("utf-8")
            self.console.print_with_color(f"Response after PASV: {response}", 'yellow')
            assert "227" in response  # Check for passive mode entry

            # Parse the IP and port for the data connection
            pasv_info = response.split("(")[1].split(")")[0].split(",")
            data_host = ".".join(pasv_info[:4])
            data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

            # Download file via data connection
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
                data_socket.connect((data_host, data_port))
                s.sendall(f"RETR {download_filename}\r\n".encode("utf-8"))
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after RETR: {response}", 'cyan')
                assert "150" in response  # Ready to transfer data

                with open(download_filename, "wb") as f:
                    while data := data_socket.recv(4096):
                        f.write(data)
                data_socket.close()
                response = s.recv(1024).decode("utf-8")
                self.console.print_with_color(f"Response after download: {response}", 'bcyan')
                assert "226" in response  # Successful file transfer

        self.console.print_with_color(f"File {upload_filename} uploaded.", 'bgreen')
        self.console.print_with_color(f"File {download_filename} downloaded.\n", 'bgreen')

