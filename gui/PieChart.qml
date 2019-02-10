import RustCode 1.0
import QtQuick 2.9

Item {
    id: container

    PieChart {
        id: aPieChart
        anchors.centerIn: container
        width: 100; height: 100
        //name: "A simple pie chart"
        //color: "red"
    }

    //Text {
    //    anchors { bottom: parent.bottom; horizontalCenter: parent.horizontalCenter; bottomMargin: 20 }
    //    text: aPieChart.name
    //}
}