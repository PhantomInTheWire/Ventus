import subprocess
from os import system

class NetworkHandlers:
    def __init__(self):
        self.is_connected = False
        self._ensure_files_directory()

    def _ensure_files_directory(self):
        """Ensure the 'files' directory exists"""
        from pathlib import Path
        Path("files").mkdir(exist_ok=True)

    def handle_connect(self, code_input: str, folder_path: str = "files") -> bool:
        """Handle connection with default path"""
        if self.is_connected:
            return False
            
        if not code_input:
            return False
        
        try:
            system(f"python ../cli/interface/main.py --host {code_input} --port 1234 --local-dir {folder_path} --remote-dir files sync")
            self.is_connected = True
            return True
        except subprocess.CalledProcessError as e:
            print("Error:", e.stderr)
            return False

    def handle_start_network(self) -> bool:
        """Handle network start"""
        if self.is_connected:
            return False
            
        self.is_connected = True
        return True

    def handle_stop_network(self) -> bool:
        """Handle network stop"""
        if not self.is_connected:
            return False
            
        self.is_connected = False
        return True

    @staticmethod
    def handle_start_network():
        # Implement network start logic
        pass

    @staticmethod
    def handle_stop_network():
        # Implement network stop logic
        pass