import time
import logging
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
import sys
import os
from pathlib import Path
from datetime import datetime, timedelta
import threading

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(message)s',
    handlers=[
        logging.FileHandler('filesystem_monitor.log'),
        logging.StreamHandler()
    ]
)


class ChangeHandler(FileSystemEventHandler):
    """Handles file system events"""

    def __init__(self, process_function):
        self.process_function = process_function
        super().__init__()

    def on_created(self, event):
        if not event.is_directory:
            logging.info(f"File created: {event.src_path}")
            self.process_function(event.src_path, trigger_type="file_change")

    def on_modified(self, event):
        if not event.is_directory:
            logging.info(f"File modified: {event.src_path}")
            self.process_function(event.src_path, trigger_type="file_change")

    def on_deleted(self, event):
        if not event.is_directory:
            logging.info(f"File deleted: {event.src_path}")
            self.process_function(event.src_path, trigger_type="file_change")


class ScheduledProcessor:
    """Handles scheduled processing"""

    def __init__(self, process_function, interval_hours=2):
        self.process_function = process_function
        self.interval_hours = interval_hours
        self.stop_flag = False
        self.thread = threading.Thread(target=self._run)
        self.thread.daemon = True

    def start(self):
        self.thread.start()

    def stop(self):
        self.stop_flag = True
        self.thread.join()

    def _run(self):
        while not self.stop_flag:
            logging.info("Running scheduled process")
            self.process_function(None, trigger_type="scheduled")

            # Sleep until next interval, but check stop_flag every minute
            for _ in range(self.interval_hours * 60):
                if self.stop_flag:
                    break
                time.sleep(60)


class FileSystemMonitor:
    def __init__(self, path_to_watch, process_function):
        self.path_to_watch = path_to_watch
        self.process_function = process_function
        self.observer = Observer()
        self.scheduler = ScheduledProcessor(process_function)

    def start(self):
        """Start monitoring the specified directory and scheduled processing"""
        try:
            path = Path(self.path_to_watch).resolve()
            if not path.exists():
                raise FileNotFoundError(f"Directory not found: {path}")

            # Start the file system observer
            event_handler = ChangeHandler(self.process_function)
            self.observer.schedule(event_handler, str(path), recursive=False)
            self.observer.start()
            logging.info(f"Started monitoring directory: {path}")

            # Start the scheduled processor
            self.scheduler.start()
            logging.info("Started scheduled processor (2-hour interval)")

            try:
                while True:
                    time.sleep(1)
            except KeyboardInterrupt:
                self.cleanup()

        except Exception as e:
            logging.error(f"Error occurred: {str(e)}")
            self.cleanup()
            sys.exit(1)

    def cleanup(self):
        """Clean up resources"""
        self.observer.stop()
        self.scheduler.stop()
        self.observer.join()
        logging.info("Monitoring stopped")


def example_process_function(file_path, trigger_type):
    """Example function to be called when file changes are detected or on schedule"""
    if trigger_type == "scheduled":
        logging.info("Running scheduled process")
        # Add your scheduled processing logic here
        # For example, process all files in the directory:
        for file in Path(WATCH_PATH).glob('*'):
            if file.is_file():
                logging.info(f"Processing file (scheduled): {file}")
                # Your processing code here
    else:  # trigger_type == "file_change"
        logging.info(f"Processing file (change triggered): {file_path}")
        # Add your file change processing logic here


if __name__ == "__main__":
    # Replace with your directory path
    WATCH_PATH = "/path/to/your/directory"

    # Create and start the monitor
    monitor = FileSystemMonitor(WATCH_PATH, example_process_function)
    monitor.start()