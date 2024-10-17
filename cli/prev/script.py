import socket
import re

# Connect to the FTP server
HOST = '127.0.0.1'
PORT = 1234

def send_cmd(sock, command):
    cmd = f"{command}\r\n"
    sock.sendall(cmd.encode('utf-8'))
    response = sock.recv(1024).decode('utf-8')
    print(f"Server response: {response}")
    return response

def connect_data_channel(host, port):
    data_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    data_socket.connect((host, port))
    return data_socket

def parse_pasv_response(response):
    # Extract IP and port from the PASV response
    match = re.search(r"(\d+),(\d+),(\d+),(\d+),(\d+),(\d+)", response)
    if match:
        numbers = list(map(int, match.groups()))
        ip_address = ".".join(map(str, numbers[:4]))
        port = numbers[4] * 256 + numbers[5]
        return ip_address, port
    raise ValueError("Failed to parse PASV response")

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        
        # Expect the welcome message from the server
        welcome = s.recv(1024).decode('utf-8')
        print(f"Connected: {welcome}")

        # Send commands to the server
        send_cmd(s, "USER anonymous")
        send_cmd(s, "PWD")
        send_cmd(s, "SYST")
        send_cmd(s, "AUTH")
        
        # Enter passive mode
        pasv_response = send_cmd(s, "PASV")
        ip_address, data_port = parse_pasv_response(pasv_response)
        print(f"Passive mode IP: {ip_address}, port: {data_port}")
        
        # Connect to the data channel
        with connect_data_channel(ip_address, data_port) as data_socket:
            print("Data connection established")
            
            # Send LIST command and receive the data
            send_cmd(s, "LIST")
            data_response = data_socket.recv(1024).decode('utf-8')
            print(f"Data response: {data_response}")

        # Close the main connection gracefully
        send_cmd(s, "QUIT")

if __name__ == "__main__":
    main()

