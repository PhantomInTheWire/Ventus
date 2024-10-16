import unittest
import socket
import os
import time

class FTPServerTest(unittest.TestCase):

    HOST = '127.0.0.1'
    PORT = 1234

    def setUp(self):
        # Give the server time to start
        time.sleep(1)

    def tearDown(self):
        pass
    
    def send_cmd(self, command):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((self.HOST, self.PORT))
            s.sendall(command.encode() + b'\r\n')
            data = s.recv(1024)
            return data.decode()

    def test_connect(self):
        response = self.send_cmd("")
        self.assertTrue("220 Welcome to this FTP server!\r\n" in response)

    def test_syst(self):
        response = self.send_cmd("SYST")
        self.assertTrue("215 UNIX Type: L8\r\n" in response)

    def test_user(self):
        response = self.send_cmd("USER anonymous")
        self.assertTrue("230 Welcome anonymous\r\n" in response)

    def test_user_invalid(self):
        response = self.send_cmd("USER")
        self.assertTrue("501 Invalid username\r\n" in response)

    def test_pwd_root(self):
        self.send_cmd("USER anonymous")
        response = self.send_cmd("PWD")
        self.assertTrue('257 "/"\r\n' in response)

    def test_cwd(self):
        self.send_cmd("USER anonymous")
        response = self.send_cmd("CWD test") 
        self.assertTrue("250 Directory changed to \"test\"\r\n" in response)

    def test_cwd_invalid(self):
        self.send_cmd("USER anonymous")
        response = self.send_cmd("CWD invalid_dir")
        self.assertTrue("501 No such file or directory\r\n" in response)

    def test_cdup(self):
        self.send_cmd("USER anonymous")
        self.send_cmd("CWD test")
        response = self.send_cmd("CDUP")
        self.assertTrue("200 Ok Directory changed to \"..\"\r\n" in response)

    def test_mkd(self):
        self.send_cmd("USER anonymous")
        response = self.send_cmd("MKD new_dir")
        self.assertTrue("250 Requested file action okay, completed.\r\n" in response)
        self.assertTrue(os.path.exists("new_dir"))
        os.rmdir("new_dir") 

    def test_mkd_invalid(self):
        self.send_cmd("USER anonymous")
        self.send_cmd("CWD test")
        response = self.send_cmd("MKD ../new_dir")
        self.assertTrue("501 Permission denied\r\n" in response)
        self.assertFalse(os.path.exists("../new_dir"))

    def test_rmd(self):
        self.send_cmd("USER anonymous")
        os.mkdir("dir_to_remove")
        response = self.send_cmd("RMD dir_to_remove")
        self.assertTrue("250 Requested file action okay, completed.\r\n" in response)
        self.assertFalse(os.path.exists("dir_to_remove"))

    def test_rmd_invalid(self):
        self.send_cmd("USER anonymous")
        self.send_cmd("CWD test")
        response = self.send_cmd("RMD ../dir_to_remove")
        self.assertTrue("501 Permission denied\r\n" in response)

    def test_type(self):
        response = self.send_cmd("TYPE I")
        self.assertTrue("200 Switching to Binary mode.\r\n" in response)

    def test_list(self):
        self.send_cmd("USER anonymous")
        response = self.send_cmd("LIST")
        self.assertTrue("125 Data connection already open; transfer starting.\r\n" in response)
        self.assertTrue("226 Closing data connection.\r\n" in response)

    def test_unknown_command(self):
        response = self.send_cmd("XXXX")
        self.assertTrue("500 Unknown command: XXXX\r\n" in response)

if __name__ == '__main__':
    unittest.main()
