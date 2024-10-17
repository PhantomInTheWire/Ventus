import socket
import os

# FTP server details
FTP_HOST = "127.0.0.1"
FTP_PORT = 1234

# Color codes
COLORS = {
    'red': "\033[1;31m",
    'green': "\033[1;32m",
    'yellow': "\033[1;33m",
    'blue': "\033[1;34m",
    'cyan': "\033[1;36m",
    'purple': "\033[1;35m",
    'reset': "\033[0m",
}

def ascii_art():
    """Displays the FTP server banner ASCII art with color gradients."""
    art = r"""

██╗   ██╗███████╗███╗   ██╗████████╗██╗   ██╗███████╗
██║   ██║██╔════╝████╗  ██║╚══██╔══╝██║   ██║██╔════╝
██║   ██║█████╗  ██╔██╗ ██║   ██║   ██║   ██║███████╗
╚██╗ ██╔╝██╔══╝  ██║╚██╗██║   ██║   ██║   ██║╚════██║
 ╚████╔╝ ███████╗██║ ╚████║   ██║   ╚██████╔╝███████║
  ╚═══╝  ╚══════╝╚═╝  ╚═══╝   ╚═╝    ╚═════╝ ╚══════╝

    """
    print(COLORS['red'] + art + COLORS['reset'])

def print_with_color(message, color='reset'):
    """Prints a message with the specified color."""
    print(COLORS[color] + message + COLORS['reset'])

def run_test(test_name, test_func):
    """Runs a single test."""
    print_with_color(f"Running {test_name}...", 'purple')
    result = "Passed"
    try:
        test_func()
    except AssertionError as e:
        result = f"Failed: {str(e)}"
    except Exception as e:
        result = f"Error: {str(e)}"

    if result == "Passed":
        print_with_color(f"{test_name} - {result}", 'green')
        print()  # New line after passing test
    else:
        print_with_color(f"{test_name} - {result}", 'red')

def test_ftp_connection():
    """Tests basic FTP connection."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response: {response}", 'cyan')
        assert "220 Welcome to this FTP server!" in response

def test_ftp_user_login():
    """Tests user login."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)  # Receive the initial welcome message
        s.sendall(b"USER testuser\r\n")
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response after login: {response}", 'green')
        assert "230 Welcome testuser" in response

def test_ftp_pwd():
    """Tests getting the current working directory."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)
        s.sendall(b"USER testuser\r\n")
        s.recv(1024)
        s.sendall(b"PWD\r\n")
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response after PWD: {response}", 'blue')
        assert "257 \"/\"" in response or '257 ""' in response or "250" in response

def test_ftp_pasv_and_list():
    """Tests entering passive mode and listing directory contents."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)
        s.sendall(b"USER testuser\r\n")
        s.recv(1024)
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response after PASV: {response}", 'yellow')
        assert "227" in response

        # Extract IP and port from PASV response
        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(b"LIST\r\n")
            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after LIST: {response}", 'green')
            assert "125" in response or "125 File Status Ok" in response

            listing = data_socket.recv(4096).decode("utf-8")
            assert listing  # Assert that the listing is not empty

            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after data transfer: {response}", 'blue')
            assert "226 Closing data connection" in response or "226" in response


def test_ftp_file_upload_and_download():
    """Tests file upload and download without deleting files."""
    upload_filename = "sample.pdf"
    download_filename = "sample2.pdf"

    # Check if the upload file exists
    if not os.path.exists(upload_filename):
        print_with_color(f"File {upload_filename} does not exist for upload.", 'red')
        return

    # Upload the file
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)
        s.sendall(b"USER testuser\r\n")
        s.recv(1024)
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response after PASV: {response}", 'yellow')
        assert "227" in response

        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"STOR {upload_filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after upload: {response}", 'green')
            assert "150" in response

            with open(upload_filename, "rb") as f:
                while True:
                    chunk = f.read(4096)
                    if not chunk:
                        break
                    data_socket.sendall(chunk)
            data_socket.close()
            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after upload complete: {response}", 'green')
            assert "226" in response

    # Download the file
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)
        s.sendall(b"USER testuser\r\n")
        s.recv(1024)
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        print_with_color(f"Response after PASV for download: {response}", 'yellow')
        assert "227" in response

        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"RETR {download_filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after download request: {response}", 'blue')
            assert "150" in response

            with open(download_filename, "wb") as f:
                while True:
                    data = data_socket.recv(4096)
                    if not data:
                        break
                    f.write(data)
            data_socket.close()
            response = s.recv(1024).decode("utf-8")
            print_with_color(f"Response after download complete: {response}", 'blue')
            assert "226" in response

    print_with_color(f"File {upload_filename} uploaded.", 'green')
    print_with_color(f"File {download_filename} downloaded.", 'blue')

def run_tests():
    """Runs all the tests."""
    tests = [
        ("Connection", test_ftp_connection),
        ("User Login", test_ftp_user_login),
        ("PWD", test_ftp_pwd),
        ("PASV and LIST", test_ftp_pasv_and_list),
        ("File Upload and Download", test_ftp_file_upload_and_download),
    ]

    for test_name, test_func in tests:
        run_test(test_name, test_func)

if __name__ == "__main__":
    ascii_art()
    run_tests()
