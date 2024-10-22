
# Ventus Desktop App Documentation

## Overview

Ventus is a cross-platform desktop application designed to facilitate easy and secure file transfers within a local network. It leverages a custom network protocol and a user-friendly interface to streamline the process of sharing files between devices.

## Screenshots

**Connect Screen:**

![Connect Screen](https://github.com/PhantomInTheWire/Ventus/tree/main/screenshots/connect_screen.png)

**Home Screen (Active Network):**

![Home Screen](https://github.com/PhantomInTheWire/Ventus/tree/main/screenshots/home_screen.png)

**Settings Screen:**

![Settings Screen](https://github.com/PhantomInTheWire/Ventus/tree/main/screenshots/settings_screen.png)

## Features

* **Easy Network Creation:** Start a new file-sharing network with a single click.
* **Simple Connection:** Join an existing network by entering a unique connection code.
* **Secure File Transfer:** Files are transferred directly between devices within the local network, enhancing security.
* **QR Code Sharing:** Easily share the network connection code via QR code.
* **Customizable Settings:** Adjust device name, shared folder, and network parameters.
* **Cross-Platform Compatibility:** Developed using PyQt5, ensuring compatibility across Windows, macOS, and Linux.


## Components

### 1. `ui_main_window.py`

This file defines the user interface of the application using PyQt5. It manages the layout, widgets, and event handling for different screens (Connect, Home, Settings).

**Key Classes:**

* `Ui_MainWindow`:  
    *  Sets up the main window, including layout, widgets, and event connections.
    *  Handles navigation between different pages (Connect, Home, Settings).
    *  Manages UI updates based on network connection status.
    *  Provides methods for displaying error and success messages.
    *  Implements settings validation and storage.

### 2. `handlers.py`

This file contains the logic for handling network-related operations.

**Key Classes:**

* `NetworkHandlers`:
    *  `handle_connect`:  Initiates a connection to an existing network using a connection code.
    *  `handle_start_network`: Creates a new network and manages its state.
    *  `handle_stop_network`: Stops the currently active network.

### 3. `utils.py`

This file provides utility functions for creating and managing UI elements.

**Key Classes:**

* `WidgetFactory`:
    *  Provides static methods for creating common PyQt5 widgets like labels, buttons, and stacked widgets.
    *  Simplifies UI creation within `ui_main_window.py`.


## Requirements

* **Python 3.7+**
* **PyQt5:** `pip install PyQt5`

## Installation

1. **Clone the Repository:**
   ```bash
   git clone <repository_url>
   cd desktop-app 
   ```

2. **Install Dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

## Running the Application

```bash
python main.py 
```

## Usage

1. **Start a Network:**
   * On the "Connect" screen, click the "Start a Network" button. 
   * This will create a new local network and generate a unique connection code.
   * The connection code will be displayed along with a QR code on the "Home" screen.

2. **Connect to a Network:**
   * On the "Connect" screen, enter the connection code of the network you want to join.
   * Click the "Connect" button.
   * The application will attempt to connect to the specified network.

3. **Transfer Files:**
   * Once connected, files can be transferred using the functionalities provided by the network protocol (not implemented in this example).

4. **Settings:**
   * Access the settings screen to change the device name, shared folder path, and network parameters.


## Future Enhancements

* **File Transfer Implementation:** Integrate the actual file transfer functionality using the custom network protocol.
* **Improved Network Management:** Add features for managing connected devices, monitoring transfer progress, and handling disconnections.
* **Enhanced Security:** Implement encryption and authentication mechanisms to secure file transfers.
* **User Interface Improvements:** Refine the UI for better user experience and add features like drag-and-drop file transfer.
* **Error Handling and Logging:** Improve error handling and provide detailed logging for debugging.
