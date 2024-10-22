import os
from datetime import datetime
from dataclasses import dataclass
from typing import List, Dict
from PyQt5 import QtWidgets, QtCore, QtGui
from pathlib import Path
import subprocess

@dataclass
class FileInfo:
    name: str
    path: str
    type: str
    size: int
    modified: datetime
    is_directory: bool

class FileExplorer(QtWidgets.QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.current_path = Path("files")  # Default path
        self.setup_ui()
        self.load_files()

    def setup_ui(self):
        layout = QtWidgets.QVBoxLayout(self)
        
        # Path display
        self.path_label = QtWidgets.QLabel()
        self.path_label.setStyleSheet("color: #dadada;")
        layout.addWidget(self.path_label)
        
        # File list
        self.file_list = QtWidgets.QListWidget()
        self.file_list.setStyleSheet("""
            QListWidget {
                background-color: rgba(255, 255, 255, 0.1);
                border: none;
                color: #dadada;
            }
            QListWidget::item:hover {
                background-color: rgba(255, 255, 255, 0.2);
            }
            QListWidget::item:selected {
                background-color: rgba(255, 255, 255, 0.3);
            }
        """)
        self.file_list.itemDoubleClicked.connect(self.handle_item_double_click)
        layout.addWidget(self.file_list)

    def load_files(self):
        """Load files from current path"""
        self.file_list.clear()
        self.path_label.setText(str(self.current_path))
        
        try:
            items = []
            for entry in os.scandir(self.current_path):
                icon_name = self.get_icon_name(entry)
                item = QtWidgets.QListWidgetItem()
                item.setIcon(QtGui.QIcon(f"assets/{icon_name}"))
                item.setText(entry.name)
                item.setData(QtCore.Qt.UserRole, FileInfo(
                    name=entry.name,
                    path=str(entry.path),
                    type='directory' if entry.is_dir() else entry.name.split('.')[-1] if '.' in entry.name else 'file',
                    size=entry.stat().st_size,
                    modified=datetime.fromtimestamp(entry.stat().st_mtime),
                    is_directory=entry.is_dir()
                ))
                items.append(item)
            
            # Sort: directories first, then files
            items.sort(key=lambda x: (not x.data(QtCore.Qt.UserRole).is_directory, x.text().lower()))
            
            for item in items:
                self.file_list.addItem(item)
                
        except PermissionError:
            QtWidgets.QMessageBox.warning(self, "Error", "Permission denied accessing this folder")
        except Exception as e:
            QtWidgets.QMessageBox.warning(self, "Error", f"Error loading files: {str(e)}")

    def get_icon_name(self, entry) -> str:
        """Return appropriate icon name based on file type"""
        if entry.is_dir():
            return "folder-icon.png"
        
        ext = entry.name.split('.')[-1].lower() if '.' in entry.name else ''
        if ext in ['txt', 'md', 'py', 'json']:
            return "text-icon.png"
        elif ext == 'pdf':
            return "pdf-icon.png"
        else:
            return "file-icon.png"

    def handle_item_double_click(self, item):
        """Handle double click on file/folder"""
        file_info: FileInfo = item.data(QtCore.Qt.UserRole)
        if file_info.is_directory:
            self.current_path = Path(file_info.path)
            self.load_files()

    def refresh(self):
        """Refresh current directory"""
        self.load_files()

# Modified NetworkHandlers class
class NetworkHandlers:
    def __init__(self):
        self.is_connected = False
        self._ensure_files_directory()

    def _ensure_files_directory(self):
        """Ensure the 'files' directory exists"""
        Path("files").mkdir(exist_ok=True)

    def handle_connect(self, code_input: str, folder_path: str = "files") -> bool:
        """Handle connection with default path"""
        if self.is_connected:
            return False
            
        if not code_input:
            return False
        
        try:
            os.system(f"python ../cli/interface/main.py --host {code_input} --port 1234 --local-dir {folder_path} --remote-dir files sync")
            self.is_connected = True
            return True
        except subprocess.CalledProcessError as e:
            print("Error:", e.stderr)
            return False

    def handle_start_network(self) -> bool:
        """Handle network start"""
        if self.is_connected:
            return False
            
        # Implement network start logic
        self.is_connected = True
        return True

    def handle_stop_network(self) -> bool:
        """Handle network stop"""
        if not self.is_connected:
            return False
            
        # Implement network stop logic
        self.is_connected = False
        return True