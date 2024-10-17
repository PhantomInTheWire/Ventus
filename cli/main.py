from test_runner import TestRunner
from ftp_utils import ConsoleUtils

if __name__ == "__main__":
    console = ConsoleUtils()
    console.display_ascii_art()
    runner = TestRunner()
    runner.run_all_tests()

