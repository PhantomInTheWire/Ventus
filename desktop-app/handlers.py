import subprocess
from os import system

class NetworkHandlers:
    @staticmethod
    def handle_connect(code_input, folder_path):
        if not code_input:
            return False
        
        try:
            system(f"python ../cli/interface/main.py --host {code_input} --port 1234 --local-dir {folder_path} --remote-dir files sync")
            return True
        except subprocess.CalledProcessError as e:
            print("Error:", e.stderr)
            return False

    @staticmethod
    def handle_start_network():
        # Implement network start logic
        pass

    @staticmethod
    def handle_stop_network():
        # Implement network stop logic
        pass