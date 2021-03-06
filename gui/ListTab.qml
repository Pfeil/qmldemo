import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

Item {
    id: listTab

    // This model was initialized in main.qml this way:
    //CheckList {
    //    id: items
    //    // Represents the model for the given List
    //    Component.onCompleted: {
    //        init()
    //    }
    //}

    ListView {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        width: 200
        
        orientation: ListView.Vertical
        model: items
        delegate: Component {
            Rectangle {
                height: 100
                width: 200
                color: "steelblue"
                CheckBox {
                    anchors.fill: parent
                    text: name
                    checked: isChecked
                    onToggled: {
                        console.log("Checkbox: toggled " + index + " to " + checked)
                        items.setCompleted(index, checked)
                    }
                    
                }
            }
        }
    }
}