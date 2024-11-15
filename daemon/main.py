import sys
import time
import logging
import subprocess
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler
import os
from typing import Set
import argparse

class SyncHandler(FileSystemEventHandler):
    def __init__(self, remote_dir: str, local_dir: str, host: str, port: int, cooldown: int = 5):
        self.remote_dir = remote_dir
        self.local_dir = local_dir
        self.host = host
        self.port = port
        self.cooldown = cooldown
        self.last_sync = 0
        self.pending_changes: Set[str] = set()
        self.setup_logging()

    def setup_logging(self):
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(message)s',
            handlers=[
                logging.FileHandler('sync_daemon.log'),
                logging.StreamHandler()
            ]
        )

    def on_modified(self, event):
        if event.is_directory:
            return
        self.pending_changes.add(event.src_path)
        self._check_sync()

    def on_created(self, event):
        if event.is_directory:
            return
        self.pending_changes.add(event.src_path)
        self._check_sync()

    def on_deleted(self, event):
        if event.is_directory:
            return
        self.pending_changes.add(event.src_path)
        self._check_sync()

    def _check_sync(self):
        current_time = time.time()
        if current_time - self.last_sync >= self.cooldown:
            self._perform_sync()

    def _perform_sync(self):
        if not self.pending_changes:
            return

        try:
            logging.info(f"Running sync for changes in: {', '.join(self.pending_changes)}")
            sync_command = f"./artifact sync --remote-dir {self.remote_dir} --local-dir {self.local_dir} --host {self.host} --port {self.port}"
            
            result = os.system(sync_command)
            if result == 0:
                logging.info("Sync completed successfully")
                self.pending_changes.clear()
                self.last_sync = time.time()
            else:
                logging.error(f"Sync failed with exit code: {result}")
                
        except Exception as e:
            logging.error(f"Error running sync: {str(e)}")

def main():
    parser = argparse.ArgumentParser(description='File Sync Daemon')
    parser.add_argument('--remote-dir', required=True, help='Remote directory to sync')
    parser.add_argument('--local-dir', required=True, help='Local directory to sync')
    parser.add_argument('--host', required=True, help='Remote host address')
    parser.add_argument('--port', type=int, required=True, help='Remote host port')
    parser.add_argument('--cooldown', type=int, default=5, help='Minimum seconds between syncs')
    args = parser.parse_args()

    if not os.path.exists(args.local_dir):
        print(f"Error: Local directory {args.local_dir} does not exist")
        sys.exit(1)

    event_handler = SyncHandler(
        remote_dir=args.remote_dir,
        local_dir=args.local_dir,
        host=args.host,
        port=args.port,
        cooldown=args.cooldown
    )
    
    observer = Observer()
    observer.schedule(event_handler, args.local_dir, recursive=True)
    
    logging.info(f"Starting file sync daemon monitoring: {args.local_dir}")
    logging.info(f"Remote sync target: {args.host}:{args.port} {args.remote_dir}")
    observer.start()

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
        logging.info("Stopping file sync daemon")

    observer.join()

if __name__ == "__main__":
    main()
