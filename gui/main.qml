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

    StackLayout {
        anchors.fill: parent
        currentIndex: bar.currentIndex // Connection to TabBar!
        HelloTab {}
        ListTab {}
        HelloTab {}
        HelloTab {}
        PieChartTab {}
    }

}
