import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

Item {
    id: listTab
    ListView {
        anchors.fill: parent
        orientation: ListView.Horizontal
        model: ["red", "green", "blue", "yellow", "pink", "lightgray", "violet", "red", "green", "blue", "yellow", "pink", "lightgray", "violet", "red", "green", "blue", "yellow", "pink", "lightgray", "violet", "red", "green", "blue", "yellow", "pink", "lightgray", "violet", "red", "green", "blue", "yellow", "pink", "lightgray", "violet"]
        delegate: Component {
            Rectangle {
                height: 50
                width: 50
                //anchors {
                //    left: parent.left
                //    right: parent.right
                //}
                color: modelData
                Text {
                    anchors.centerIn: parent
                    text: index
                }
            }
        }
    }
}