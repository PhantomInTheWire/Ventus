# ui_main_window.py
from PyQt5 import QtCore, QtGui, QtWidgets
from utils import WidgetFactory
from handlers import NetworkHandlers
from file_explorer import FileExplorer

class Ui_MainWindow(object):
    def __init__(self):
        self.is_connected = False
        self.widget_factory = WidgetFactory()
        self.network_handlers = NetworkHandlers()

    def setupUi(self, MainWindow):
        """Main setup method for the UI"""
        self.setupMainWindow(MainWindow)
        self.create_central_widget(MainWindow)
        self.create_pages()
        self.create_connect_page()
        self.create_home_page()
        self.create_settings_page()
        self.setup_translations(MainWindow)
        self.setup_event_connections()
        self.load_settings()
        
        MainWindow.setCentralWidget(self.centralwidget)
        QtCore.QMetaObject.connectSlotsByName(MainWindow)

    def setupMainWindow(self, MainWindow):
        MainWindow.setObjectName("MainWindow")
        MainWindow.resize(683, 512)
        MainWindow.setMinimumSize(QtCore.QSize(683, 512))
        MainWindow.setMaximumSize(QtCore.QSize(683, 512))
        MainWindow.setBaseSize(QtCore.QSize(1366, 1024))
        MainWindow.setStyleSheet("#MainWindow {background: #000}")
        font = QtGui.QFont()
        font.setFamily("Montserrat Medium")
        MainWindow.setFont(font)

    def create_central_widget(self, MainWindow):
        self.centralwidget = self.widget_factory.create_widget("centralwidget", MainWindow)
        self.pages = self.widget_factory.create_stack("pages", self.centralwidget, [0, 0, 681, 511])

    def create_pages(self):
        self.connect_page = self.widget_factory.create_widget("connect_page")
        self.home_page = self.widget_factory.create_widget("home_page")
        self.settings_page = self.widget_factory.create_widget("settings_page")

    def create_connect_page(self):
        # Create connect page widgets
        self.connectLabel = self.widget_factory.create_label("connectLabel", self.connect_page, (110, 26, 91, 20))
        self.start_bg = self.widget_factory.create_img_label("start_bg", self.connect_page, (460, 90, 121, 51), "Start-connection-btn.png")
        self.connectCard = self.widget_factory.create_img_label("connectCard", self.connect_page, (110, 70, 220, 89), "Connect-card.png")
        self.codeInput = self.create_input("codeInput", self.connect_page, (150, 100, 111, 31))
        self.startNetworkLabel = self.widget_factory.create_label("startNetworkLabel", self.connect_page, (410, 30, 161, 20))
        self.startNetworkBtn = self.create_btn("startNetworkBtn", self.connect_page, (460, 90, 121, 51), self.handle_start_network)
        self.startNetworkCard = self.widget_factory.create_img_label("startNetworkCard", self.connect_page, (410, 70, 220, 89), "start-network-card.png")
        self.bg = self.widget_factory.create_img_label("bg", self.connect_page, (0, 0, 681, 511), "Bg.png")
        self.connectBtn = self.create_btn("connectBtn", self.connect_page, (270, 98, 31, 31), self.handle_connect)
        self.settingsBtn = self.create_btn("settingsBtn", self.connect_page, (22, 468, 21, 21), lambda: self.change_page("settings"))

        # Raise widgets in correct order
        self.raise_connect_page_widgets()
        self.pages.addWidget(self.connect_page)

    def create_home_page(self):
        # Create home page widgets
        self.bg_2 = self.widget_factory.create_img_label("bg_2", self.home_page, (0, 0, 681, 511), "Bg.png")
        self.stopBg = self.widget_factory.create_img_label("stopBg", self.home_page, (110, 70, 215, 88), "stop-bg.png")
        self.networkInfoLabel = self.widget_factory.create_label("networkInfoLabel", self.home_page, (110, 27, 221, 20))
        self.stopBtn = self.create_btn("stopBtn", self.home_page, (160, 90, 111, 41), self.handle_stop)
        
        # Add directory button (above settings button)
        self.directoryBtn = self.create_btn("directoryBtn", self.home_page, (22, 430, 21, 21), self.show_file_explorer)
        
        # Create file explorer (initially hidden)
        self.file_explorer = FileExplorer(self.home_page)
        self.file_explorer.setGeometry(QtCore.QRect(100, 70, 538, 380))
        self.file_explorer.hide()

        self.qrCodeLabel = self.widget_factory.create_label("qrCodeLabel", self.home_page, (380, 27, 90, 20))
        self.qrCode = self.widget_factory.create_img_label("qrCode", self.home_page, (420, 100, 181, 171), "qr-code.png")
        self.outputCode = self.widget_factory.create_label("outputCode", self.home_page, (435, 310, 138, 30), "center")
        self.stopBg_2 = self.widget_factory.create_img_label("stopBg_2", self.home_page, (380, 70, 258, 312), "qr-bg.png")
        self.connectionLabel = self.widget_factory.create_label("connectionLabel", self.home_page, (380, 29, 221, 20))
        self.settingsBtn_1 = self.create_btn("settingsBtn_1", self.home_page, (22, 468, 21, 21), lambda: self.change_page("settings"))

        # Raise widgets in correct order
        self.raise_home_page_widgets()
        self.pages.addWidget(self.home_page)

    def show_file_explorer(self):
        """Toggle file explorer visibility"""
        if self.file_explorer.isVisible():
            self.file_explorer.hide()
            # Show other widgets
            self.stopBg.show()
            self.stopBg_2.show()
            self.qrCode.show()
            self.outputCode.show()
        else:
            self.file_explorer.show()
            # Hide other widgets that overlap
            self.stopBg.hide()
            self.stopBg_2.hide()
            self.qrCode.hide()
            self.outputCode.hide()
            # Refresh file explorer
            self.file_explorer.refresh()

    def create_settings_page(self):
        # Create settings page widgets
        self.bg_3 = self.widget_factory.create_img_label("bg_3", self.settings_page, (0, 0, 681, 511), "Bg.png")
        self.arrowLeft = self.widget_factory.create_img_label("arrowLeft", self.settings_page, (100, 30, 23, 23), "arrowLeft.png")
        self.settingsTitle = self.widget_factory.create_label("settingsTitle", self.settings_page, (140, 30, 211, 21))

        # Create settings sections
        self.create_general_settings()
        self.create_connection_settings()
        self.create_network_settings()

        self.settingsBtn_2 = self.create_btn("settingsBtn_2", self.settings_page, (23, 468, 21, 21), lambda: self.change_page('settings'))
        self.backBtn = self.create_btn("backBtn", self.settings_page, (102, 31, 21, 21), self.handle_back_nav)

        self.pages.addWidget(self.settings_page)

    def create_general_settings(self):
        self.generalSettingsLabel = self.widget_factory.create_label("generalLabel", self.settings_page, (100, 70, 81, 20))
        self.generalCard = self.widget_factory.create_img_label("generalCard", self.settings_page, (100, 100, 236, 120), "GeneralSettings.png")
        self.deviceName = self.create_input("deviceName", self.settings_page, (150, 130, 111, 31), 8)
        self.folderName = self.create_input("folderName", self.settings_page, (150, 180, 131, 21), 8)

    def create_connection_settings(self):
        self.connectionSettingsLabel = self.widget_factory.create_label("connectionSettingsLabel", self.settings_page, (380, 70, 121, 20))
        self.connectionCard = self.widget_factory.create_img_label("connectionCard", self.settings_page, (380, 100, 260, 172), "ConnectionSettings.png")
        self.maxConn = self.create_input("maxConn", self.settings_page, (430, 130, 111, 31), 8)
        self.maxRate = self.create_input("maxRate", self.settings_page, (430, 180, 111, 31), 8)
        self.sizeLimit = self.create_input("sizeLimit", self.settings_page, (430, 230, 111, 31), 8)

    def create_network_settings(self):
        self.networkSettingsLabel = self.widget_factory.create_label("networkSettingsLabel", self.settings_page, (100, 270, 88, 20))
        self.networkSettingsCard = self.widget_factory.create_img_label("networkSettingsCard", self.settings_page, (100, 300, 236, 188), "NetworkSettings.png")

    def create_input(self, widget_name, parent, dimension, size=10):
        newWidget = QtWidgets.QTextEdit(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        sizePolicy = self.widget_factory.create_size_policy(newWidget)
        newWidget.setSizePolicy(sizePolicy)
        
        font = QtGui.QFont()
        font.setFamily("Montserrat")
        font.setPointSize(size)
        newWidget.setFont(font)
        
        newWidget.setFocusPolicy(QtCore.Qt.ClickFocus)
        newWidget.setAutoFillBackground(False)
        newWidget.setStyleSheet(f"#{widget_name} {{ background: transparent !important; color: #dadada; border: 0px #dadada !important;}}")
        newWidget.setInputMethodHints(QtCore.Qt.ImhNone)
        newWidget.setVerticalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        newWidget.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        newWidget.setLineWrapMode(QtWidgets.QTextEdit.NoWrap)
        newWidget.setTabStopWidth(4)
        newWidget.setAcceptRichText(False)
        newWidget.setObjectName(widget_name)
        return newWidget

    def create_btn(self, widget_name, parent, dimension, handle_click):
        newWidget = QtWidgets.QPushButton(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setCursor(QtGui.QCursor(QtCore.Qt.PointingHandCursor))
        newWidget.setStyleSheet(f"#{widget_name} {{ border-radius: 10; background-color: rgba(255, 255, 255, 0); }}")
        newWidget.setText("")
        newWidget.setCheckable(False)
        newWidget.setChecked(False)
        newWidget.setAutoDefault(False)
        newWidget.setDefault(False)
        newWidget.setFlat(True)
        newWidget.setObjectName(widget_name)
        newWidget.clicked.connect(handle_click)
        return newWidget

    def raise_connect_page_widgets(self):
        self.bg.raise_()
        self.connectLabel.raise_()
        self.connectCard.raise_()
        self.codeInput.raise_()
        self.startNetworkLabel.raise_()
        self.startNetworkCard.raise_()
        self.start_bg.raise_()
        self.connectBtn.raise_()
        self.startNetworkBtn.raise_()
        self.settingsBtn.raise_()

    def raise_home_page_widgets(self):
        self.bg_2.raise_()
        self.stopBg.raise_()
        self.networkInfoLabel.raise_()
        self.stopBtn.raise_()
        self.qrCodeLabel.raise_()
        self.stopBg_2.raise_()
        self.connectionLabel.raise_()
        self.qrCode.raise_()
        self.outputCode.raise_()
        self.settingsBtn_1.raise_()
        self.directoryBtn.raise_()

    def setup_translations(self, MainWindow):
        _translate = QtCore.QCoreApplication.translate
        MainWindow.setWindowTitle(_translate("MainWindow", "Ventus"))

        # Set initial text
        self.set_init_text({
            self.connectLabel: "Connect",
            self.startNetworkLabel: "Start a network",
            self.networkInfoLabel: "Connection",
            self.qrCodeLabel: "",
            self.outputCode: "abc - defg - hij",
            self.settingsTitle: "Settings",
            self.generalSettingsLabel: "General",
            self.networkSettingsLabel: "Network",
            self.connectionSettingsLabel: "Connection",
        }, _translate)

        # Set placeholder text
        self.set_init_placeholder({
            self.codeInput: "Enter code",
            self.deviceName: "Legion slim 5",
            self.folderName: "files",  # Changed to default files directory
            self.maxConn: "5 devices",
            self.maxRate: "5 GB/s",
            self.sizeLimit: "10 GB",
        }, _translate)

    def set_init_text(self, data, _translate):
        for (prop, value) in data.items():
            prop.setText(_translate("MainWindow", value))
    
    def set_init_placeholder(self, data, _translate):
        for (prop, value) in data.items():
            prop.setPlaceholderText(_translate("MainWindow", value))

    def handle_network_connect(self, new_status=False):
        """Update connection status"""
        if self.is_connected != new_status:  # Only update if status actually changes
            self.is_connected = new_status
            self.update_connection_status(new_status)

    def handle_start_network(self):
        """Handle network start"""
        if not self.is_connected:  # Only proceed if not already connected
            success = self.network_handlers.handle_start_network()
            if success:
                self.handle_network_connect(True)
                self.change_page("home") 
                self.show_success_message("Connected", "Successfully connected to network")
            else:
                self.show_error_message("Connection Failed", "Unable to establish connection")
        
        # Change to home page after successful connection
                return True
        return False
    
    def setup_event_connections(self):
        """Setup event handlers for various UI elements"""
        # Text change handlers
        self.deviceName.textChanged.connect(self.save_settings)
        self.folderName.textChanged.connect(self.save_settings)
        self.maxConn.textChanged.connect(self.save_settings)
        self.maxRate.textChanged.connect(self.save_settings)
        self.sizeLimit.textChanged.connect(self.save_settings)

    def load_settings(self):
        """Load saved settings from QSettings"""
        settings = QtCore.QSettings('Ventus', 'Desktop')
        
        # Load text values
        self.deviceName.setText(settings.value('deviceName', 'Legion slim 5'))
        self.folderName.setText(settings.value('folderName', 'files'))
        self.maxConn.setText(settings.value('maxConn', '5 devices'))
        self.maxRate.setText(settings.value('maxRate', '5 GB/s'))
        self.sizeLimit.setText(settings.value('sizeLimit', '10 GB'))

    def save_settings(self):
        """Save current settings to QSettings"""
        settings = QtCore.QSettings('Ventus', 'Desktop')
        # Save text values
        settings.setValue('deviceName', self.deviceName.toPlainText())
        settings.setValue('folderName', self.folderName.toPlainText())
        settings.setValue('maxConn', self.maxConn.toPlainText())
        settings.setValue('maxRate', self.maxRate.toPlainText())
        settings.setValue('sizeLimit', self.sizeLimit.toPlainText())

    def handle_connect(self):
        """Handle connection attempt"""
        if not self.is_connected:  # Only proceed if not already connected
            code = self.codeInput.toPlainText().strip()
            folder_path = self.folderName.toPlainText().strip() or "files"  # Use files as default
            
            success = self.network_handlers.handle_connect(code, folder_path)
            if success:
                self.handle_network_connect(True)
                self.change_page("home")

    def handle_stop(self):
        """Enhanced stop handler"""
        try:
            self.network_handlers.handle_stop_network()
            self.handle_network_connect(False)
            self.update_connection_status(False)
            self.change_page('connect')
            self.show_success_message("Network Stopped", "Network successfully stopped")
        except Exception as e:
            self.show_error_message("Stop Error", f"An error occurred: {str(e)}")


    def handle_back_nav(self):
        """Handle back navigation from settings"""
        if self.is_connected:
            self.change_page("home")
        else:
            self.change_page("connect")

    def change_page(self, page_name):
        """Change current page based on name"""
        page_index = {
            "connect": 0,
            "home": 1,
            "settings": 2
        }.get(page_name, 0)
        
        self.pages.setCurrentIndex(page_index)

    def update_connection_status(self, is_connected):
        """Update UI elements based on connection status"""
        # Update QR code and connection info when connected
        if is_connected:
            # Here you would update the QR code image and connection info
            self.qrCodeLabel.setText("Connected")
            self.networkInfoLabel.setText("Network Active")
        else:
            # Reset labels when disconnected
            self.outputCode.setText("")
            self.connectionLabel.setText("Disconnected")

        # Clear input when disconnecting
        if not is_connected:
            self.codeInput.setText("")
            
        # Update internal state
        self.is_connected = is_connected
        
    def show_error_message(self, title, message):
        """Display error message dialog"""
        msg = QtWidgets.QMessageBox()
        msg.setIcon(QtWidgets.QMessageBox.Critical)
        msg.setWindowTitle(title)
        msg.setText(message)
        msg.exec_()

    def show_success_message(self, title, message):
        """Display success message dialog"""
        msg = QtWidgets.QMessageBox()
        msg.setIcon(QtWidgets.QMessageBox.Information)
        msg.setWindowTitle(title)
        msg.setText(message)
        msg.exec_()
    
