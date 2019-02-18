import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;

Item {
    id: gridTab

    // This model was initialized in main.qml this way:
    //CheckList {
    //    id: items
    //    // Represents the model for the given List
    //    Component.onCompleted: {
    //        init()
    //    }
    //}

    GridView {
        id: grid
        anchors.fill: parent
        
        model: items
        cellHeight: 200
        cellWidth: 200
        delegate: Component {
            Rectangle {
                height: grid.cellHeight
                width: grid.cellWidth
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