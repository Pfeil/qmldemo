#![recursion_limit = "256"]

extern crate cpp;
extern crate qmetaobject;
mod piechart;
use piechart::PieChart;
use qmetaobject::*;
use std::ffi::CStr;

qrc!(gui,
    "demo/qml" {
        "gui/main.qml" as "main.qml",
        "gui/HelloTab.qml" as "HelloTab.qml",
        "gui/ListTab.qml" as "ListTab.qml",
        "gui/PieChart.qml" as "PieChart.qml",
    },
);

#[derive(QObject, Default)]
struct DataStructure {
    base: qt_base_class!(trait QObject),
}

fn register_all_types_in_qml() {
    qml_register_type::<DataStructure>(
        CStr::from_bytes_with_nul(b"RustCode\0").unwrap(), // qml module name
        1,                                                 // major version
        0,                                                 // minor version
        CStr::from_bytes_with_nul(b"DataStructure\0").unwrap(), // type name
    );
    qml_register_type::<PieChart>(
        CStr::from_bytes_with_nul(b"RustCode\0").unwrap(), // qml module name
        1,                                                 // major version
        0,                                                 // minor version
        CStr::from_bytes_with_nul(b"PieChart\0").unwrap(), // type name
    );
}

fn main() {
    gui();
    register_all_types_in_qml();
    QQuickStyle::set_style("Material"); // use googles material design
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/demo/qml/main.qml".into());
    engine.exec();
}
