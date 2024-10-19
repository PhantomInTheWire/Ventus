import socket
import os
import argparse

from ftp_utils import ConsoleUtils


class FtpCli:
    def __init__(self, ftp_host, ftp_port):
        self.ftp_host = ftp_host
        self.ftp_port = ftp_port
        self.console = ConsoleUtils()
        self.control_socket = None  # Socket for control commands
        self.data_socket = None    # Socket for data transfer

    def connect(self):
        """Connects to the FTP server."""
        self.control_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.control_socket.connect((self.ftp_host, self.ftp_port))
        response = self.control_socket.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response: {response}", 'cyan')
        if not response.startswith("220"):
            raise Exception("Could not connect to FTP server")

    def login(self, username, password):
        """Logs in to the FTP server."""
        self.send_command(f"USER {username}")
        response = self.receive_response()
        if not response.startswith("230"):
            self.send_command(f"PASS {password}")
            response = self.receive_response()
            if not response.startswith("230"):
                raise Exception("Could not log in")

    def send_command(self, command):
        """Sends a command to the FTP server (control socket)."""
        self.console.print_with_color(f"Sending command: {command}", 'purple')
        self.control_socket.sendall(f"{command}\r\n".encode("utf-8"))

    def receive_response(self):
        """Receives a response from the FTP server (control socket)."""
        response = self.control_socket.recv(1024).decode("utf-8")
        self.console.print_with_color(f"Response: {response}", 'cyan')
        return response

    def enter_passive_mode(self):
        """Enters passive mode and gets data connection details."""
        self.send_command("PASV")
        response = self.receive_response()
        if not response.startswith("227"):
            raise Exception("Could not enter passive mode")

        # Parse the IP and port from the PASV response
        start = response.find("(") + 1
        end = response.find(")")
        numbers = response[start:end].split(",")
        data_host = ".".join(numbers[:4])
        data_port = int(numbers[4]) * 256 + int(numbers[5])
        return data_host, data_port

    def open_data_connection(self, data_host, data_port):
        """Opens a new data connection."""
        self.data_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.data_socket.connect((data_host, data_port))

    def close_data_connection(self):
        """Closes the data connection."""
        if self.data_socket:
            self.data_socket.close()
            self.data_socket = None

    def list(self, directory=""):
        """Lists files in the current directory."""
        try:
            data_host, data_port = self.enter_passive_mode()
            self.open_data_connection(data_host, data_port) 
            self.send_command(f"LIST {directory}")
            response = self.receive_response() 
            if not response.startswith("150") and not response.startswith("125"):
                raise Exception("Could not list files")

            while True:
                data = self.data_socket.recv(1024).decode("utf-8", errors="ignore")
                if not data:
                    break
                print(data, end="")

            response = self.receive_response()  
            if not response.startswith("226"):
                raise Exception("Could not list files")

        except Exception as e:
            print(f"Error during LIST: {e}")
        finally:
            self.close_data_connection()  

    def cwd(self, directory):
        """Changes the current working directory."""
        self.send_command(f"CWD {directory}")
        response = self.receive_response()
        if not response.startswith("250"):
            raise Exception("Could not change directory")

    def cdup(self):
        """Changes to the parent directory."""
        self.send_command("CDUP")
        response = self.receive_response()
        if not response.startswith("250") and not response.startswith("200"):
            raise Exception("Could not change to parent directory")

    def pwd(self):
        """Prints the current working directory."""
        self.send_command("PWD")
        response = self.receive_response()
        if not response.startswith("257"):
            raise Exception("Could not get current working directory")
        start = response.find("\"") + 1
        end = response.find("\"", start)
        print(response[start:end])

    def mkdir(self, directory):
        """Creates a new directory."""
        self.send_command(f"MKD {directory}")
        response = self.receive_response()
        if not response.startswith("257"):
            raise Exception("Could not create directory")

    def rmd(self, directory):
        """Removes a directory."""
        self.send_command(f"RMD {directory}")
        response = self.receive_response()
        if not response.startswith("250"):
            raise Exception("Could not remove directory")

    def stor(self, local_file, remote_file):
        """Uploads a file to the server."""
        try:
            if not os.path.exists(local_file):
                raise Exception(f"Local file {local_file} not found")

            data_host, data_port = self.enter_passive_mode()
            self.open_data_connection(data_host, data_port)  
            self.send_command(f"STOR {remote_file}")
            response = self.receive_response()
            if not response.startswith("150"):
                raise Exception("Could not start upload")

            with open(local_file, "rb") as f:
                while chunk := f.read(4096):
                    self.data_socket.sendall(chunk)

            response = self.receive_response()  
            if not response.startswith("226"):
                raise Exception("Upload failed")
        except Exception as e:
            print(f"Error during STOR: {e}")
        finally:
            self.close_data_connection()

    def retr(self, remote_file, local_file):
        """Downloads a file from the server."""
        try:
            data_host, data_port = self.enter_passive_mode()
            self.open_data_connection(data_host, data_port) 
            self.send_command(f"RETR {remote_file}")
            response = self.receive_response()
            if not response.startswith("150"):
                raise Exception("Could not start download")

            with open(local_file, "wb") as f:
                while data := self.data_socket.recv(4096):
                    f.write(data)

            response = self.receive_response() 
            if not response.startswith("226"):
                raise Exception("Download failed")
        except Exception as e:
            print(f"Error during RETR: {e}")
        finally:
            self.close_data_connection()

    def quit(self):
        """Quits the FTP session."""
        self.send_command("QUIT")
        self.control_socket.close()


def main():
    console = ConsoleUtils()
    console.display_ascii_art()

    parser = argparse.ArgumentParser(description="FTP Client")
    parser.add_argument("host", help="FTP server host")
    parser.add_argument("port", type=int, help="FTP server port")
    parser.add_argument("-u", "--username", default="anonymous", help="FTP username")
    parser.add_argument("-p", "--password", default="", help="FTP password")
    args = parser.parse_args()

    ftp = FtpCli(args.host, args.port)
    try:
        ftp.connect()
        ftp.login(args.username, args.password)

        while True:
            command = input("ftp> ")
            parts = command.split()
            if not parts:
                continue

            cmd = parts[0].lower()

            if cmd == "ls" or cmd == "list":
                if len(parts) > 1:
                    ftp.list(parts[1])
                else:
                    ftp.list()
            elif cmd == "cd":
                if len(parts) > 1:
                    ftp.cwd(parts[1])
                else:
                    print("Usage: cd <directory>")
            elif cmd == "cdup":
                ftp.cdup()
            elif cmd == "pwd":
                ftp.pwd()
            elif cmd == "mkdir":
                if len(parts) > 1:
                    ftp.mkdir(parts[1])
                else:
                    print("Usage: mkdir <directory>")
            elif cmd == "rmd" or cmd == "rmdir":
                if len(parts) > 1:
                    ftp.rmd(parts[1])
                else:
                    print("Usage: rmd <directory>")
            elif cmd == "put" or cmd == "stor":
                if len(parts) == 3:
                    ftp.stor(parts[1], parts[2])
                elif len(parts) == 2:
                    ftp.stor(parts[1], parts[1]) 
                else:
                    print("Usage: put <local_file> [<remote_file>]")
            elif cmd == "get" or cmd == "retr":
                if len(parts) == 3:
                    ftp.retr(parts[1], parts[2])
                elif len(parts) == 2:
                    ftp.retr(parts[1], parts[1]) 
                else:
                    print("Usage: get <remote_file> [<local_file>]")
            elif cmd == "quit" or cmd == "exit":
                ftp.quit()
                break
            else:
                print("Invalid command")

    except Exception as e:
        print(f"Error: {e}")
        if ftp.control_socket:
            ftp.control_socket.close()


if __name__ == "__main__":
    main()
