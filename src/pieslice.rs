use crate::qmetaobject::qtdeclarative::{QQuickItem, QQuickPaintedItem};
use crate::qmetaobject::qttypes::{QColor, QString};
use crate::qmetaobject::qpainter::{QPainter, QPen};
use crate::qmetaobject::*;

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct PieSlice {
    base: qt_base_class!(trait QQuickPaintedItem),
    // Q_PROPERTY(QColor color READ color WRITE setColor)
    color: qt_property!(QColor;)
}

impl Clone for PieSlice {
    fn clone(&self) -> PieSlice {
        PieSlice {
            base: self.base.clone(), // FIXME clone does not exist.
            color: QColor::from_rgb_f(0., 0., 0.),
        }
    }
}

impl QMetaType for PieSlice {}

impl QQuickItem for PieSlice {}

impl QQuickPaintedItem for PieSlice {
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