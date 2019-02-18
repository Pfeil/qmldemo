//! Implements the piechart [example].
//! [example]: https://doc.qt.io/qt-5/qtqml-tutorials-extending-qml-example.html
use crate::qmetaobject::qpainter::{QPainter, QPen};
use crate::qmetaobject::qtdeclarative::{QQuickItem, QQuickPaintedItem};
use crate::qmetaobject::qttypes::{QColor, QString};
use crate::qmetaobject::*;
//pub use self::pieslice::PieSlice;

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct PieChart {
    base: qt_base_class!(trait QQuickPaintedItem),

    // Q_PROPERTY(PieSlice* pieSlice READ pieSlice WRITE setPieSlice)
    //pieslice: qt_property!(PieSlice; WRITE set_pieslice),
    color: qt_property!(QColor; READ get_color WRITE set_color NOTIFY colorChanged),
    name: qt_property!(QString;),

    colorChanged: qt_signal!(),
}

impl PieChart {
    fn get_color(&self) -> QColor {
        return self.color;
    }
    fn set_color(&mut self, new_color: QColor) {
        if new_color != self.color {
            self.color = new_color;
            self.update(None); // paint again (None = everything)
            self.colorChanged(); // emit colorChanged signal, catch in QML with "onColorChanged"
        }
    }
    //fn set_pieslice(&mut self, pieslice: PieSlice) {
    //    // TODO pieslice.set_parent_item(self);
    //    let pie_obj = &pieslice.get_cpp_object();
    //    let self_obj = self.get_cpp_object();
    //
    //    //cpp!(unsafe [pie_obj as "QQuickItem*", self_obj as "QQuickItem"] {
    //    //    pie_obj->setParentItem(self_obj);
    //    //});
    //    self.pieslice = pieslice;
    //}
}

impl QQuickItem for PieChart {}

impl QQuickPaintedItem for PieChart {
    fn paint(&self, painter: &mut QPainter) {
        //let pen = QPen::from_color_and_width(self.color, 2i32);
        let pen = QPen::from_color(self.color);
        painter.set_pen(pen);
        // TODO cpp!(unsafe [painter as "QPainter*"] { painter->setRenderHints(1, true); });
        let rect = self.bounding_rect();
        painter.draw_pie_rect(rect, 90 * 16, 290 * 16)
        // TODO painter->setRenderHints(QPainter::Antialiasing, true);
        //painter->drawPie(boundingRect(). TODO adjusted(1, 1, -1, -1), 90 * 16, 290 * 16);
    }
}
