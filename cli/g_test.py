import socket
import os
import time

# FTP server details
FTP_HOST = "127.0.0.1"
FTP_PORT = 1234

def test_ftp_connection():
    """Tests basic FTP connection."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        response = s.recv(1024).decode("utf-8")
        print(f"Response: {response}")
        assert "220 Welcome to this FTP server!" in response

def test_ftp_user_login():
    """Tests user login."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)  # Receive the initial welcome message
        s.sendall(b"USER testuser\r\n")
        response = s.recv(1024).decode("utf-8")
        print(f"Response after: {response}")
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
        print(f"Response after: {response}")
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
        print(f"Response after: {response}")
        assert "227" in response

        # Extract IP and port from PASV response
        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(b"LIST\r\n")
            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "125" in response or "125 File Status Ok" in response
            
            listing = data_socket.recv(4096).decode("utf-8")
            # You might want to add more specific assertions about the directory listing here
            assert listing  # Assert that the listing is not empty

            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "226 Closing data connection" in response or "226" in response

def test_ftp_file_upload_and_download():
    """Tests file upload and download."""
    test_filename = "testfile.txt"
    file_content = b"This is a test file.\n"

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((FTP_HOST, FTP_PORT))
        s.recv(1024)  
        s.sendall(b"USER testuser\r\n")
        s.recv(1024)  
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        print(f"Response after: {response}")
        assert "227" in response

        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        # Upload the file
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"STOR {test_filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "150" in response
            
            data_socket.sendall(file_content)
            data_socket.close()
            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "226" in response

        # Download the file 
        s.sendall(b"PASV\r\n")
        response = s.recv(1024).decode("utf-8")
        print(f"Response after: {response}")
        assert "227" in response

        pasv_info = response.split("(")[1].split(")")[0].split(",")
        data_host = ".".join(pasv_info[:4])
        data_port = int(pasv_info[4]) * 256 + int(pasv_info[5])

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as data_socket:
            data_socket.connect((data_host, data_port))
            s.sendall(f"RETR {test_filename}\r\n".encode("utf-8"))
            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "150" in response

            downloaded_content = data_socket.recv(4096)
            data_socket.close()
            response = s.recv(1024).decode("utf-8")
            print(f"Response after: {response}")
            assert "226" in response
            assert downloaded_content == file_content

    # Cleanup - delete the test file if it exists
    if os.path.exists(test_filename):
        os.remove(test_filename)

# You can add more test functions here for other FTP commands (CWD, CDUP, MKD, RMD, etc.)

if __name__ == "__main__":
    test_ftp_connection()
    test_ftp_user_login()
    test_ftp_pwd()
    test_ftp_pasv_and_list()
    test_ftp_file_upload_and_download()
    print("All tests passed!")
