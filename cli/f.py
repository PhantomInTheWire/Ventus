import socket
import time

def send_and_receive(sock, message):
    """Sends a command and receives the response from the server."""
    sock.sendall(message.encode())
    return sock.recv(1024).decode()

def test_ftp_server():
    # Connect to the FTP server
    server_address = ('localhost', 1234)
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect(server_address)
    
    try:
        # Receive the welcome message from the server
        welcome_msg = sock.recv(1024).decode()
        print(f"Server: {welcome_msg}")
        
        # Test USER command with a valid username
        response = send_and_receive(sock, "USER testuser\r\n")
        print(f"USER testuser: {response}")
        assert "230" in response, "USER command failed"
        
        # Test PWD command
        response = send_and_receive(sock, "PWD\r\n")
        print(f"PWD: {response}")
        assert "257" in response, "PWD command failed"
        
        # Test LIST command (with no parameters)
        response = send_and_receive(sock, "PASV\r\n")
        print(f"PASV: {response}")
        assert "227" in response, "PASV command failed"
        
        # Parse PASV response to connect to the data channel
        pasv_info = response.split('(')[1].split(')')[0]
        p1, p2 = map(int, pasv_info.split(',')[-2:])
        data_port = p1 * 256 + p2
        
        data_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        data_sock.connect(('127.0.0.1', data_port))
        
        # Request the LIST command and receive the directory listing
        send_and_receive(sock, "LIST\r\n")
        data_response = data_sock.recv(1024).decode()
        print(f"LIST response: {data_response}")
        assert "FILE" in data_response or "DIR" in data_response, "LIST command failed"
        
        data_sock.close()
        
        # Test CWD command (change working directory)
        response = send_and_receive(sock, "CWD /testdir\r\n")
        print(f"CWD /testdir: {response}")
        assert "200" in response or "226" in response, "CWD command failed"
        
        # Test MKD command (make directory)
        response = send_and_receive(sock, "MKD /newdir\r\n")
        print(f"MKD /newdir: {response}")
        assert "257" in response or "200" in response, "MKD command failed"
        
        # Test RMD command (remove directory)
        response = send_and_receive(sock, "RMD /newdir\r\n")
        print(f"RMD /newdir: {response}")
        assert "257" in response or "200" in response, "RMD command failed"
        
    finally:
        # Close the connection
        sock.close()

# Run the test
if __name__ == "__main__":
    test_ftp_server()
