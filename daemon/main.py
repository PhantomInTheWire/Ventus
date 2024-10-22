import time
import os
import sys

def foo(host, port, dir):
  """
  This function will be called whenever a change is detected in the watched directory.
  """
  print("Change detected! calling sync function")
  os.system(f"../android-bin/target/debug/android-bin sync --host {host} --port {port} --local-dir {dir} --remote-dir files")
  time.sleep(5)

def watch_directory(path, host, port):
  """
  Monitors the specified directory for changes and calls foo() when detected.
  """
  before = dict([(f, None) for f in os.listdir(path)])
  while True:
    time.sleep(1)  # Check every 1 second, adjust as needed
    after = dict([(f, None) for f in os.listdir(path)])
    added = [f for f in after if not f in before]
    removed = [f for f in before if not f in after]
    if added or removed:
      foo(host, port, path)  # Pass path as directory to sync
    before = after

if __name__ == "__main__":
  if len(sys.argv) != 4:
    print("Usage: python main.py <directory_path> <host> <port>")
    sys.exit(1)

  directory_path = sys.argv[1]
  host = sys.argv[2]
  port = sys.argv[3]

  if not os.path.isdir(directory_path):
    print("Error: Invalid directory path")
    sys.exit(1)

  # Run the monitoring process as a daemon
  pid = os.fork()
  if pid > 0:
    # Exit the parent process
    sys.exit(0)
  # Become a session leader and detach from the controlling terminal
  os.setsid()
  os.umask(0)
  # Redirect standard file descriptors to a file
  sys.stdin.flush()
  sys.stdout.flush()
  sys.stderr.flush()
  si = open(os.devnull, 'r')
  so = open("daemon_output.txt", 'a+')  # Redirect stdout to daemon_output.txt
  se = open("daemon_error.txt", 'a+')  # Redirect stderr to daemon_error.txt
  os.dup2(si.fileno(), sys.stdin.fileno())
  os.dup2(so.fileno(), sys.stdout.fileno())
  os.dup2(se.fileno(), sys.stderr.fileno())

  # Start watching the directory
  watch_directory(directory_path, host, port) 
