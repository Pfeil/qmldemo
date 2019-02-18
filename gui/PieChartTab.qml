import RustCode 1.0
import QtQuick 2.9

// Implements the piechart [example].
// [example]: https://doc.qt.io/qt-5/qtqml-tutorials-extending-qml-example.html

Item {
    width: 300; height: 200

    Row {
        anchors.centerIn: parent
        spacing: 20
    
        PieChart {
            id: chartA
            width: 200; height: 200
            color: "red"
            MouseArea {
                anchors.fill: parent
                onClicked: {
                    chartA.color = "red"
                    console.log("color: red")
                }
            }
            Text {
                anchors { top: parent.bottom; horizontalCenter: parent.horizontalCenter; bottomMargin: 20 }
                text: "red"
            }
        }

        PieChart {
            id: chartB
            width: 200; height: 200
            color: chartA.color
            MouseArea {
                anchors.fill: parent
                onClicked: {
                    chartA.color = "blue"
                    console.log("color: blue")
                }
            }
            Text {
                anchors { top: parent.bottom; horizontalCenter: parent.horizontalCenter; bottomMargin: 20 }
                text: "blue"
            }
        }
    }
}