//! Trying to implement the freaking piechart [example].
//! [example]: https://doc.qt.io/qt-5/qtqml-tutorials-extending-qml-example.html
use crate::cpp::*;
use crate::qmetaobject::qtdeclarative::{QQuickItem, QQuickPaintedItem};
use crate::qmetaobject::qttypes::{QPen, QPainter};
use crate::qmetaobject::*;

use qmetaobject::qttypes::QColor;

#[derive(Default, QObject)]
pub struct PieChart {
    base: qt_base_class!(trait QQuickPaintedItem),
    //name: qt_property!(QColor),
}

impl QQuickItem for PieChart {}

impl QQuickPaintedItem for PieChart {
    
}

//painter->setPen(pen);
//painter.setRenderHints(QPainter::Antialiasing, true);
//painter.drawPie(boundingRect().adjusted(1, 1, -1, -1), 90 * 16, 290 * 16);