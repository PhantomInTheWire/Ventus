import daemon
import signal
import lockfile
from pathlib import Path

class DirectoryMonitorDaemon:
    def __init__(self, watch_path, host, port, remote_dir, pid_file):
        self.watch_path = watch_path
        self.host = host
        self.port = port
        self.remote_dir = remote_dir
        self.pid_file = pid_file
        self.monitor = None

    def run(self):
        # Setup signal handlers
        signal.signal(signal.SIGTERM, self.handle_signal)
        signal.signal(signal.SIGHUP, self.handle_signal)

        try:
            self.monitor = DirectoryMonitor(self.watch_path, self.host, self.port, self.remote_dir)
            observer = Observer()
            observer.schedule(self.monitor, self.watch_path, recursive=False)
            observer.start()
            
            while True:
                time.sleep(1)
                
        except Exception as e:
            logging.error(f"Daemon error: {str(e)}")
            observer.stop()
        finally:
            observer.join()

    def handle_signal(self, signo, stack_frame):
        logging.info(f"Received signal {signo}")
        sys.exit(0)

def start_daemon(watch_path, host, port, remote_dir):
    pid_file = '/var/run/ftpmonitor.pid'
    log_file = '/var/log/ftpmonitor.log'
    
    # Configure daemon context
    context = daemon.DaemonContext(
        working_directory='/var/lib/ftpmonitor',
        umask=0o002,
        pidfile=lockfile.FileLock(pid_file),
        files_preserve=[
            logging.getLogger().handlers[0].stream,
        ],
    )
    
    # Initialize logging before daemonizing
    logging.basicConfig(
        filename=log_file,
        level=logging.INFO,
        format='%(asctime)s - %(message)s'
    )

    logging.info("Starting FTP monitor daemon")
    
    # Start the daemon
    with context:
        daemon = DirectoryMonitorDaemon(watch_path, host, port, remote_dir, pid_file)
        daemon.run()

if __name__ == "__main__":
    if len(sys.argv) != 5:
        print("Usage: python daemon_monitor.py <watch_path> <host> <port> <remote_dir>")
        sys.exit(1)
        
    watch_path = sys.argv[1]
    host = sys.argv[2]
    port = sys.argv[3]
    remote_dir = sys.argv[4]
    
    start_daemon(watch_path, host, port, remote_dir)
