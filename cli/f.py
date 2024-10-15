import socket
from ftplib import FTP

def test_pasv_mode():
    # Connect to the FTP server
    ftp = FTP()
    ftp.connect('127.0.0.1', 1234)
    ftp.login()  # Anonymous login

    print("Connected to FTP server")

    # Test PASV command
    try:
        pasv_response = ftp.sendcmd('PASV')
        print(f"PASV response: {pasv_response}")

        # Parse the PASV response
        if pasv_response.startswith('227'):
            # Extract the part within parentheses
            start = pasv_response.find('(')
            end = pasv_response.find(')')
            if start != -1 and end != -1:
                numbers = pasv_response[start+1:end].split(',')
                if len(numbers) == 6:
                    ip = '.'.join(numbers[:4])
                    port = int(numbers[4]) * 256 + int(numbers[5])
                    print(f"Parsed PASV response - IP: {ip}, Port: {port}")

                    # Try to connect to the data port
                    data_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                    data_socket.connect((ip, port))
                    print("Successfully connected to data port")
                    data_socket.close()

                    # Test LIST command in passive mode
                    print("Listing directory contents:")
                    ftp.retrlines('LIST')
                else:
                    print(f"Invalid number of elements in PASV response: {len(numbers)}")
            else:
                print("Could not find parentheses in PASV response")
        else:
            print("Failed to enter passive mode")

    except Exception as e:
        print(f"Error during PASV test: {e}")

    finally:
        ftp.quit()
        print("Disconnected from FTP server")

if __name__ == "__main__":
    test_pasv_mode()
