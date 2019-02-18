import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

Item {
    id: listTab

    CheckList {
        id: items
        // Represents the model for the given List
        Component.onCompleted: {
            init()
        }
    }

    ListView {
        anchors.fill: parent
        orientation: ListView.Horizontal
        model: items
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
                    text: name
                }
            }
        }
    }
}