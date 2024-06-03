import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ImageConverter 1.0

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "DatabendingUtils-RS"

    ImageConverter {
        id: converter
    }

    Column {
        anchors.centerIn: parent
        spacing: 20
        
        Text {
            text: "DatabendingUtils-RS"
            font.pointSize: 24
            anchors.horizontalCenter: parent.horizontalCenter
        }
        
        Text {
            text: "2.0.0"
            font.pointSize: 16
            anchors.horizontalCenter: parent.horizontalCenter
        }

        RowLayout {
            spacing: 20
            anchors.horizontalCenter: parent.horizontalCenter

            ColumnLayout {
                spacing: 10

                TextField {
                    id: inputField
                    placeholderText: "Input File Path"
                    text: converter.input_file
                    onTextChanged: converter.input_file = text
                    Layout.alignment: Qt.AlignHCenter
                }

                TextField {
                    id: outputField
                    placeholderText: "Output File Path"
                    text: converter.output_file
                    onTextChanged: converter.output_file = text
                    Layout.alignment: Qt.AlignHCenter
                }

                TextField {
                    id: headerField
                    placeholderText: "Header File Path"
                    text: converter.header_file
                    onTextChanged: converter.header_file = text
                    Layout.alignment: Qt.AlignHCenter
                }
            }

            ColumnLayout {
                spacing: 10

                Text {
                    text: "Pre Databending"
                    font.pointSize: 16
                    Layout.alignment: Qt.AlignHCenter
                }

                Button {
                    text: "Convert to SBR"
                    onClicked: converter.convert_to_sbr()
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                }

                Button {
                    text: "Convert to Headless BMP"
                    onClicked: converter.convert_to_hbmp()
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                }
            }

            ColumnLayout {
                spacing: 10

                Text {
                    text: "Post Databending"
                    font.pointSize: 16
                    Layout.alignment: Qt.AlignHCenter
                }

                Button {
                    text: "Convert from SBR"
                    onClicked: converter.convert_from_sbr()
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                }

                Button {
                    text: "Convert from Headless BMP"
                    onClicked: converter.convert_from_hbmp()
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                }
            }
        }
        
        Text {
            text: "General Instructions:"
            font.pointSize: 16
            anchors.horizontalCenter: parent.horizontalCenter
        }
        
        RowLayout{
            spacing: 20
            anchors.horizontalCenter: parent.horizontalCenter
            
            ColumnLayout {
                spacing: 10
                
                Text {
                    text: "If converting to SBR or HBMP:"
                    font.pointSize: 12
                    Layout.alignment: Qt.AlignHCenter
                }
                Text {
                    text: "/path/to/commonimage.png goes in Input"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
                Text {
                    text: "/path/to/bendingready.sbr/.hbmp goes in Output"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
                Text {
                    text: "/path/to/header.sbrhead/.hbmphead goes in Header"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
            }
                
            ColumnLayout {
                spacing: 10
                
                Text {
                    text: "If converting from SBR or HBMP:"
                    font.pointSize: 12
                    Layout.alignment: Qt.AlignHCenter
                }
                Text {
                    text: "/path/to/commonimage_out.png goes in Output"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
                Text {
                    text: "/path/to/bendingready.sbr/.hbmp goes in Input"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
                Text {
                    text: "/path/to/header.sbrhead/.hbmphead goes in Header"
                    font.pointSize: 11
                    Layout.alignment: Qt.AlignHLeft
                }
            }
        }
    }
}
