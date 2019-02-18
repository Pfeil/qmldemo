import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import RustCode 1.0;
//import "folder/HelloTab.qml"

ApplicationWindow {
    id: window
    visible: true
    width: 1024
    height: 400
    title: "QML Demo"

    header: TabBar {
        id: bar
        width: parent.width
        TabButton {
            text: qsTr("Hello, World")
        }
        TabButton {
            text: qsTr("List")
        }
        TabButton {
            text: qsTr("Grid")
        }
        TabButton {
            text: qsTr("Interaction")
        }
        TabButton {
            text: qsTr("CustomWidget")
        }
    }

    // Our useless Rust struct.
    DataStructure {
        name: "Useless."
        color: "red" // default color palette, "implicit cast" to QColor type.
    }

    // Another useless Rust struct with default values. This is always allowed in QML.
    // This is why deriving Default is needed.
    DataStructure {}

    // Our Rust List Model.
    CheckList {
        id: items
        Component.onCompleted: {
            init()
        }
    }

    StackLayout {
        anchors.fill: parent
        currentIndex: bar.currentIndex // Connection to TabBar!
        HelloTab {}
        ListTab {}
        GridTab {}
        HelloTab {}
        PieChartTab {}
    }

}
