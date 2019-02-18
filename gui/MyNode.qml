import QtQuick 2.9

Rectangle {
	id: cursor

	height: 50; width: 50
	x: (window.width / 2) - (width / 2);
	y: (window.height / 2) - (height / 2);
	color: "lightblue"
	border.color: color
	border.width: 3

	focus: true
	Keys.onUpPressed: if(smoothy.animation == null || smoothy.animation.running == false) {y -= 50}
	Keys.onDownPressed: if(smoothy.animation == null || smoothy.animation.running == false) {y += 50}
	Keys.onRightPressed: if(smoothx.animation == null || smoothx.animation.running == false) {x += 50}
	Keys.onLeftPressed: if(smoothx.animation == null || smoothx.animation.running == false) {x -= 50}
	Keys.onReturnPressed: Qt.quit();

	Behavior on x { id: smoothx; enabled: false; PropertyAnimation { duration: 200; alwaysRunToEnd: true } }
	Behavior on y { id: smoothy; enabled: false; PropertyAnimation { duration: 200; alwaysRunToEnd: true } }

	SequentialAnimation { id: blink;
		PropertyAnimation { target: border; properties: "color"; to: "black"; duration: 25 }
		PropertyAnimation { target: border; easing.type: Easing.OutQuad; properties: "color"; to: color; duration: 250 }
	}

	MouseArea {
		anchors.fill: parent
		drag.target: parent
		drag.axis: Drag.XandYAxis
		onReleased: blink.start()
		onClicked: {
			parent.focus = true;
		}
	}

	Component.onCompleted: {
		x = (window.width / 2) - (width / 2);
		y = (window.height / 2) - (height / 2);
		smoothx.enabled = true;
		smoothy.enabled = true;
	}
}
