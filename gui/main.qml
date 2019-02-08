import QtQuick 2.9
import QtQuick.Controls 2.2
//import QtQuick.Layouts 1.3
import RustCode 1.0;

ApplicationWindow {
    id: window
    visible: true
    width: 400
    height: 200
    title: "Qml Demo"
    Rectangle {
        anchors.fill: parent
        color: "steelblue"

        Text {
            text: "Hello, World!"
            anchors.centerIn: parent
            font.pointSize: 38
        }

        // Instance of our Rust struct!
        DataStructure {}
    }
}