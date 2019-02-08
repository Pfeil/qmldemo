import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

Item {
    id: helloTab
    Rectangle {
        anchors.fill: parent
        color: "steelblue"
        Text {
            anchors.centerIn: parent
            text: "Hello, World"
            font.pointSize: 30
        }
    }
}