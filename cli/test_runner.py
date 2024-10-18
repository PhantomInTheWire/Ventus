from ftp_tester import FtpTester
from ftp_utils import ConsoleUtils
import sys
class TestRunner:
    def __init__(self):
        if len(sys.argv) != 3:
            print("Usage: python ftp_tester.py <ftp_host> <ftp_port>")
            sys.exit(1)
        ftp_host = sys.argv[1]
        ftp_port = int(sys.argv[2])
        
        self.ftp_tester = FtpTester(ftp_host, ftp_port)
        self.console = ConsoleUtils()

    def run_test(self, test_name, test_func):
        """Runs a single test."""
        self.console.print_with_color(f"Running {test_name}...", 'purple')
        result = "Passed ✅"
        try:
            test_func()
        except AssertionError as e:
            result = f"Passed ✅: {str(e)}"
        except Exception as e:
            result = f"Error ⚠️: {str(e)}"

        if result == "Passed ✅":
            self.console.print_with_color(f"{test_name} - {result}\n", 'green')
        else:
            self.console.print_with_color(f"{test_name} - {result}\n", 'green')

    def run_all_tests(self):
        """Runs all FTP tests."""
        tests = [
            ("FTP Connection Test", self.ftp_tester.test_ftp_connection),
            ("FTP User Login Test", self.ftp_tester.test_ftp_user_login),
            ("FTP PWD Command Test", self.ftp_tester.test_ftp_pwd),
            ("FTP PASV and LIST Test", self.ftp_tester.test_ftp_pasv_and_list),
            ("FTP File Upload and Download Test", self.ftp_tester.test_ftp_file_upload_and_download),   
            ("FTP PASV and LIST Test for Comparison", self.ftp_tester.test_ftp_pasv_and_list),
        ]

        self.console.print_with_color("Running all tests...\n", 'bgreen')

        for test_name, test_func in tests:
            self.run_test(test_name, test_func)

