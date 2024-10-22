# FTP Directory Monitor Documentation

## Overview
The FTP Directory Monitor is a daemon process that watches specified directories for changes and automatically syncs them with a remote FTP server using a custom FTP client.

## Components
- Directory Monitor Daemon (`ftpmonitor_daemon.py`)
- Custom FTP Client (`ftp_client.py`)
- System Service Files (Linux/Windows/macOS)

## Requirements
- Python 3.6+
- Required packages:
  ```bash
  pip install watchdog python-daemon
  ```
- Running instance of custom FTP server
- Root/sudo access for daemon operations

## Installation

1. Clone the repository:
```bash
git clone <repository_url>
cd <project_directory>
```

2. Install dependencies:
```bash
pip install watchdog python-daemon
```

3. Set up system service (choose based on OS):

### Linux (systemd)
1. Create service file:
```bash
sudo nano /etc/systemd/system/ftpmonitor.service
```

2. Add configuration:
```ini
[Unit]
Description=FTP Directory Monitor Service
After=network.target

[Service]
Type=simple
User=YOUR_USERNAME
ExecStart=/usr/bin/python3 /path/to/monitor.py /path/to/watch host port /remote/dir
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

3. Enable and start service:
```bash
sudo systemctl enable ftpmonitor
sudo systemctl start ftpmonitor
```

### Windows
1. Create batch file:
```batch
@echo off
python "C:\path\to\monitor.py" "C:\path\to\watch" host port /remote/dir
```

2. Place in startup folder (`shell:startup`)

### macOS
1. Create launch agent:
```bash
nano ~/Library/LaunchAgents/com.user.ftpmonitor.plist
```

2. Add configuration:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.user.ftpmonitor</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/bin/python3</string>
        <string>/path/to/monitor.py</string>
        <string>/path/to/watch</string>
        <string>host</string>
        <string>port</string>
        <string>/remote/dir</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```

3. Load launch agent:
```bash
launchctl load ~/Library/LaunchAgents/com.user.ftpmonitor.plist
```

## Usage

### Manual Start
```bash
sudo python3 ftpmonitor_daemon.py /path/to/watch host port /remote/dir
```

### Check Status
```bash
# Check process
ps aux | grep ftpmonitor

# View PID
cat /var/run/ftpmonitor.pid

# Check logs
tail -f /var/log/ftpmonitor.log
```

### Stop Daemon
```bash
sudo kill $(cat /var/run/ftpmonitor.pid)
```

## Features
- Real-time directory monitoring
- Automatic file synchronization
- System startup integration
- Logging to `/var/log/ftpmonitor.log`
- PID management via `/var/run/ftpmonitor.pid`
- Signal handling (SIGTERM, SIGHUP)
- Cooldown period between syncs (5 seconds)

## Configuration

### Logging
- Location: `/var/log/ftpmonitor.log`
- Format: `timestamp - message`
- Level: INFO

### Daemon Settings
- Working Directory: `/var/lib/ftpmonitor`
- PID File: `/var/run/ftpmonitor.pid`
- Umask: 0o002

## Troubleshooting

### Common Issues
1. Permission Denied
   - Solution: Run with sudo/root permissions
   
2. Port in Use
   - Solution: Check if another instance is running
   
3. Cannot Create PID File
   - Solution: Ensure /var/run is writable

### Debug Steps
1. Check logs:
```bash
tail -f /var/log/ftpmonitor.log
```

2. Verify process:
```bash
ps aux | grep ftpmonitor
```

3. Check file permissions:
```bash
ls -l /var/run/ftpmonitor.pid
ls -l /var/log/ftpmonitor.log
```

## Error Handling
- Failed syncs are logged and retried on next file change
- Daemon automatically restarts on crash (when using system service)
- Graceful shutdown on SIGTERM/SIGHUP signals

## Limitations
- No recursive directory monitoring
- Basic file comparison (filename only)
- Fixed 5-second cooldown between syncs
- Requires root/sudo access

## Future Improvements
- Add configuration file support
- Implement recursive directory monitoring
- Add checksum-based file comparison
- Add user-configurable sync cooldown
- Implement retry mechanism for failed syncs
