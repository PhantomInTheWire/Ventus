# File System Monitor Daemon

## Overview
This daemon provides a robust file system monitoring solution with scheduled processing capabilities. It combines real-time file system monitoring with scheduled execution, making it ideal for automated file processing workflows.

## Features
- Real-time monitoring of file system changes (create, modify, delete)
- Scheduled processing at 2-hour intervals
- Comprehensive logging system
- Graceful error handling and shutdown
- Cross-platform compatibility
- Configurable monitoring paths and processing logic
- Systemd service integration (Linux)

## Prerequisites
- Python 3.7 or higher
- pip (Python package manager)
- Administrative privileges (for system service setup)

## Installation

1. Clone or download the project to your desired location:
```bash
git clone <repository-url>
cd filesystem-monitor-daemon
```

2. Create and activate a virtual environment (recommended):
```bash
# On Windows
python -m venv venv
venv\Scripts\activate

# On Linux/MacOS
python3 -m venv venv
source venv/bin/activate
```

3. Install required dependencies:
```bash
pip install -r requirements.txt
```

## Configuration

### Basic Configuration
Edit the following variables in `daemon_script.py`:

```python
WATCH_PATH = "/path/to/your/directory"  # Directory to monitor
```

### Advanced Configuration Options
The daemon can be customized by modifying these parameters:

1. Logging Configuration:
```python
logging.basicConfig(
    level=logging.INFO,  # Can be changed to DEBUG for more verbose logging
    format='%(asctime)s - %(message)s',
    handlers=[
        logging.FileHandler('filesystem_monitor.log'),
        logging.StreamHandler()
    ]
)
```

2. Scheduler Interval:
```python
scheduler = ScheduledProcessor(process_function, interval_hours=2)  # Modify interval_hours as needed
```

## Usage

### Running as a Python Script

1. Start the daemon:
```bash
python daemon_script.py
```

2. Stop the daemon:
- Press `Ctrl+C` for graceful shutdown
- The daemon will properly clean up resources and stop all monitoring threads

### Running as a System Service (Linux)

1. Create a systemd service file:
```bash
sudo nano /etc/systemd/system/file-monitor.service
```

2. Add the following configuration:
```ini
[Unit]
Description=File System Monitor Daemon
After=network.target

[Service]
Type=simple
User=your_username
ExecStart=/usr/bin/python3 /full/path/to/daemon_script.py
Restart=always
RestartSec=10
WorkingDirectory=/full/path/to/daemon/directory

[Install]
WantedBy=multi-user.target
```

3. Enable and start the service:
```bash
sudo systemctl enable file-monitor
sudo systemctl start file-monitor
```

4. Service management commands:
```bash
# Check status
sudo systemctl status file-monitor

# Stop service
sudo systemctl stop file-monitor

# Restart service
sudo systemctl restart file-monitor

# View logs
sudo journalctl -u file-monitor
```

## Customizing Process Function

The `example_process_function` can be customized to handle files according to your needs:

```python
def example_process_function(file_path, trigger_type):
    if trigger_type == "scheduled":
        # Add scheduled processing logic
        pass
    else:  # trigger_type == "file_change"
        # Add file change processing logic
        pass
```

### Example Custom Implementation:
```python
def example_process_function(file_path, trigger_type):
    if trigger_type == "scheduled":
        logging.info("Running scheduled batch processing")
        for file in Path(WATCH_PATH).glob('*.txt'):
            process_file(file)
    else:
        logging.info(f"Processing changed file: {file_path}")
        process_file(Path(file_path))

def process_file(file_path):
    # Add your file processing logic here
    pass
```

## Logging

The daemon maintains two types of logs:
1. Console output
2. File log (`filesystem_monitor.log`)

Log entries include:
- Timestamp
- Event type (file creation, modification, deletion, scheduled run)
- File paths
- Error messages and stack traces
- Service start/stop notifications

## Error Handling

The daemon includes comprehensive error handling:
- File system errors
- Permission issues
- Invalid paths
- Processing errors
- Resource cleanup on shutdown

## Troubleshooting

Common issues and solutions:

1. Permission Denied
```bash
sudo chown -R your_username:your_group /path/to/watch
chmod 755 /path/to/watch
```

2. Service Won't Start
- Check logs: `sudo journalctl -u file-monitor`
- Verify paths in service file
- Ensure Python environment is accessible

3. High CPU Usage
- Increase scheduling interval
- Optimize processing function
- Reduce monitoring scope

4. Memory Leaks
- Implement proper resource cleanup in processing function
- Monitor memory usage
- Restart service periodically if needed

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Create a Pull Request

## License
[Specify your license here]

## Support
[Specify support contact information]

## Changelog

### Version 1.0.0
- Initial release
- File system monitoring
- Scheduled processing
- Systemd service support

## Security Considerations
- Runs with limited user privileges
- No external network access required
- File permissions should be properly configured
- Logging contains no sensitive information