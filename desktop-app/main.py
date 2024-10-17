from PyQt5 import QtCore, QtGui, QtWidgets

class Ui_MainWindow(object):
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

    def initializations(self):
        self.is_connected = False

    def createWidget(self, widget_name, parent=None):
        if parent:
            newWidget = QtWidgets.QWidget(parent)
        else:
            newWidget = QtWidgets.QWidget()
        newWidget.setObjectName(widget_name)
        return newWidget

    def createStack(self, widget_name, parent, dimension):
        newWidget = QtWidgets.QStackedWidget(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setStyleSheet("#%s { background-color: rgba(255, 255, 255, 0); }" % widget_name)
        newWidget.setObjectName(widget_name)
        return newWidget
    
    def createLabel(self, widget_name, parent, dimension, alignment="left"):
        newWidget = QtWidgets.QLabel(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setMinimumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setMaximumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setStyleSheet("#%s {color: #dadada}" % widget_name)
        font = QtGui.QFont()
        font.setFamily("Montserrat SemiBold")
        font.setPointSize(14)
        newWidget.setFont(font)
        if alignment == "center":
            newWidget.setAlignment(QtCore.Qt.AlignCenter)
        newWidget.setText("")
        newWidget.setTextFormat(QtCore.Qt.PlainText)
        newWidget.setLineWidth(0)
        newWidget.setAutoFillBackground(False)
        newWidget.setObjectName(widget_name)
        return newWidget
    
    def createImgLabel(self, widget_name, parent, dimension, img_src):
        newWidget = QtWidgets.QLabel(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setMinimumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setMaximumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setText("")
        newWidget.setLineWidth(0)
        newWidget.setPixmap(QtGui.QPixmap(f"assets/{img_src}"))
        newWidget.setScaledContents(True)
        newWidget.setObjectName(widget_name)
        return newWidget

    def createSizePolicy(self, parent=None):
        newSizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Fixed, QtWidgets.QSizePolicy.Fixed)
        newSizePolicy.setHorizontalStretch(0)
        newSizePolicy.setVerticalStretch(0)
        newSizePolicy.setHeightForWidth(parent.sizePolicy().hasHeightForWidth())
        return newSizePolicy

    def createInput(self, widget_name, parent, dimension, size=10):
        newWidget = QtWidgets.QTextEdit(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))

        sizePolicy = self.createSizePolicy(newWidget)
        newWidget.setSizePolicy(sizePolicy)
        font = QtGui.QFont()
        font.setFamily("Montserrat")
        font.setPointSize(size)
        newWidget.setFont(font)
        newWidget.setFocusPolicy(QtCore.Qt.ClickFocus)
        newWidget.setAutoFillBackground(False)
        newWidget.setStyleSheet("#%s { background: transparent !important; color: #dadada; border: 0px #dadada !important;}" % widget_name)
        newWidget.setInputMethodHints(QtCore.Qt.ImhNone)
        newWidget.setVerticalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        newWidget.setHorizontalScrollBarPolicy(QtCore.Qt.ScrollBarAlwaysOff)
        newWidget.setLineWrapMode(QtWidgets.QTextEdit.NoWrap)
        newWidget.setTabStopWidth(4)
        newWidget.setAcceptRichText(False)
        newWidget.setObjectName(widget_name)
        return newWidget

    def createBtn(self, widget_name, parent, dimension, handleClick):
        newWidget = QtWidgets.QPushButton(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setCursor(QtGui.QCursor(QtCore.Qt.PointingHandCursor))
        newWidget.setStyleSheet("#%s { border-radius: 10; background-color: rgba(255, 255, 255, 0); }" % widget_name)
        newWidget.setText("")
        newWidget.setCheckable(False)
        newWidget.setChecked(False)
        newWidget.setAutoDefault(False)
        newWidget.setDefault(False)
        newWidget.setFlat(True)
        newWidget.setObjectName(widget_name)
        newWidget.clicked.connect(handleClick)
        return newWidget

    def setupUi(self, MainWindow):
        self.setupMainWindow(MainWindow)
        self.initializations()

        self.centralwidget = self.createWidget("centralwidget", MainWindow)
        self.pages = self.createStack("pages", self.centralwidget, [0, 0, 681, 511])

        # Connect Page
        self.connect_page = self.createWidget("connect_page")
        self.connectLabel = self.createLabel("connectLabel", self.connect_page, (110, 26, 91, 20))
        self.start_bg = self.createImgLabel("start_bg", self.connect_page, (460, 90, 121, 51), "Start-connection-btn.png")
        self.connectCard = self.createImgLabel("connectCard", self.connect_page, (110, 70, 220, 89), "Connect-card.png")
        self.codeInput = self.createInput("codeInput", self.connect_page, (150, 100, 111, 31))
        self.startNetworkLabel = self.createLabel("startNetworkLabel", self.connect_page, (410, 30, 161, 20))
        self.startNetworkBtn = self.createBtn("startNetworkBtn", self.connect_page, (460, 90, 121, 51), lambda: self.handleStartNetwork)
        self.startNetworkCard = self.createImgLabel("startNetworkCard", self.connect_page, (410, 70, 220, 89), "start-network-card.png")
        self.bg = self.createImgLabel("bg", self.connect_page, (0, 0, 681, 511), "Bg.png")
        self.connectBtn = self.createBtn("connectBtn", self.connect_page, (270, 98, 31, 31), self.handleConnect)
        self.settingsBtn = self.createBtn("settingsBtn", self.connect_page, (22, 468, 21, 21), lambda: self.changePage("settings"))
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
        self.pages.addWidget(self.connect_page)


        # <----- Home Page ----->
        self.home_page = self.createWidget("home_page")
        self.bg_2 = self.createImgLabel("bg_2", self.home_page, (0, 0, 681, 511), "Bg.png")
        self.stopBg = self.createImgLabel("stopBg", self.home_page, (110, 70, 215, 88), "stop-bg.png")
        self.networkInfoLabel = self.createLabel("networkInfoLabel", self.home_page, (110, 27, 221, 20))
        self.stopBtn = self.createBtn("connectBtn", self.home_page, (160, 90, 111, 41), self.handleStop)

        self.qrCodeLabel = self.createLabel("networkInfoLabel", self.home_page, (380, 27, 90, 20))
        self.qrCode = self.createImgLabel("qrCode", self.home_page, (420, 100, 181, 171), "qr-code.png")
        self.outputCode = self.createLabel("outputCode", self.home_page, (435, 310, 138, 30), "center")
        self.stopBg_2 = self.createImgLabel("stopBg_2", self.home_page, (380, 70, 258, 312), "qr-bg.png")
        self.connectionLabel = self.createLabel("connectionLabel", self.home_page, (380, 29, 221, 20))
        self.settingsBtn_1 = self.createBtn("settingsBtn_1", self.home_page, (22, 468, 21, 21), lambda: self.changePage("settings"))

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
        self.pages.addWidget(self.home_page)


        # <----- Settings page ----->
        self.settings_page = self.createWidget("settings_page")
        self.bg_3 = self.createImgLabel("bg_3", self.settings_page, (0, 0, 681, 511), "Bg.png")
        self.arrowLeft = self.createImgLabel("arrowLeft", self.settings_page, (100, 30, 23, 23), "arrowLeft.png")
        self.settingsTitle = self.createLabel("settingsTitle", self.settings_page, (140, 30, 211, 21))

        # General settings
        self.generalSettingsLabel = self.createLabel("generalLabel", self.settings_page, (100, 70, 81, 20))
        self.generalCard = self.createImgLabel("generalCard", self.settings_page, (100, 100, 236, 120), "GeneralSettings.png")
        self.deviceName = self.createInput("deviceName", self.settings_page, (150, 130, 111, 31), 8)
        self.folderName = self.createInput("folderName", self.settings_page, (150, 180, 131, 21), 8)

        # Connection settings
        self.connectionSettingsLabel = self.createLabel("connectionSettingsLabel", self.settings_page, (380, 70, 121, 20))
        # self.connectionTitle = self.createInput("connectionTitle", self.settings_page, (380, 70, 121, 20))
        self.connectionCard = self.createImgLabel("connectionCard", self.settings_page, (380, 100, 260, 172), "ConnectionSettings.png")
        self.maxConn = self.createInput("maxConn", self.settings_page, (430, 130, 111, 31), 8)
        self.maxRate = self.createInput("maxRate", self.settings_page, (430, 180, 111, 31), 8)
        self.sizeLimit = self.createInput("sizeLimit", self.settings_page, (430, 230, 111, 31), 8)
        
        # Network settings
        self.networkSettingsLabel = self.createLabel("networkSettingsLabel", self.settings_page, (100, 270, 88, 20))
        self.settingsBtn_2 = self.createBtn("settingsBtn_2", self.settings_page, (23, 468, 21, 21), lambda: self.changePage('settings'))
        self.backBtn = self.createBtn("backBtn", self.settings_page, (102, 31, 21, 21), self.handle_back_nav)
        self.networkSettingsCard = self.createImgLabel("networkSettingsCard", self.settings_page, (100, 300, 236, 188), "NetworkSettings.png")
        
        # if self.is_connected:
        self.pages.addWidget(self.settings_page)
        MainWindow.setCentralWidget(self.centralwidget)

        self.retranslateUi(MainWindow)
        self.settingsBtn.clicked.connect(self.pages.update)
        QtCore.QMetaObject.connectSlotsByName(MainWindow)

    def retranslateUi(self, MainWindow):
        _translate = QtCore.QCoreApplication.translate
        MainWindow.setWindowTitle(_translate("MainWindow", "Ventus"))

        self.set_init_text({
            self.connectLabel: "Connect",
            self.startNetworkLabel: "Start a network",
            self.networkInfoLabel: "Connection",
            self.qrCodeLabel: "QR Code",
            self.outputCode: "abc - defg - hij",
            self.settingsTitle: "Settings",
            self.generalSettingsLabel: "General",
            self.networkSettingsLabel: "Network",
            self.connectionSettingsLabel: "Connection",
        }, _translate)
        self.set_init_placeholder({
            self.codeInput: "Enter code",
            self.deviceName: "Inspiron 5508",
            self.folderName: "Downloads/Ventus",
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
        self.is_connected = new_status
    
    def handleConnect(self):
        # print(self.codeInput, type(self.codeInput))
        # print(self.codeInput.toPlainText())
        if not self.codeInput.toPlainText():
            return
        self.handle_network_connect(True)
        self.changePage('home')
    
    def handleStartNetwork(self):
        pass

    def handleStop(self):
        self.handle_network_connect(False)
        self.changePage('connect')
    
    def changePage(self, index):
        PAGES = {
            "connect": 0,
            "home": 1,
            "settings": 2
        }
        self.pages.setCurrentIndex(PAGES.get(index, 0))
    
    def handle_back_nav(self):
        self.changePage('home') if self.is_connected else self.changePage('connect')

if __name__ == "__main__":
    import sys
    app = QtWidgets.QApplication(sys.argv)
    MainWindow = QtWidgets.QMainWindow()
    ui = Ui_MainWindow()
    ui.setupUi(MainWindow)
    MainWindow.show()
    sys.exit(app.exec_())
