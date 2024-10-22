from PyQt5 import QtCore, QtGui, QtWidgets

class WidgetFactory:
    @staticmethod
    def create_widget(widget_name, parent=None):
        if parent:
            newWidget = QtWidgets.QWidget(parent)
        else:
            newWidget = QtWidgets.QWidget()
        newWidget.setObjectName(widget_name)
        return newWidget

    @staticmethod
    def create_stack(widget_name, parent, dimension):
        newWidget = QtWidgets.QStackedWidget(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setStyleSheet(f"#{widget_name} {{ background-color: rgba(255, 255, 255, 0); }}")
        newWidget.setObjectName(widget_name)
        return newWidget
    
    @staticmethod
    def create_label(widget_name, parent, dimension, alignment="left"):
        newWidget = QtWidgets.QLabel(parent)
        newWidget.setGeometry(QtCore.QRect(*dimension))
        newWidget.setMinimumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setMaximumSize(QtCore.QSize(*dimension[2:]))
        newWidget.setStyleSheet(f"#{widget_name} {{color: #dadada}}")
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

    @staticmethod
    def create_img_label(widget_name, parent, dimension, img_src):
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

    @staticmethod
    def create_size_policy(parent=None):
        newSizePolicy = QtWidgets.QSizePolicy(QtWidgets.QSizePolicy.Fixed, QtWidgets.QSizePolicy.Fixed)
        newSizePolicy.setHorizontalStretch(0)
        newSizePolicy.setVerticalStretch(0)
        newSizePolicy.setHeightForWidth(parent.sizePolicy().hasHeightForWidth())
        return newSizePolicy
