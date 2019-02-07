extern crate qmetaobject;
use qmetaobject::*;

use std::ffi::CStr;

qrc!(gui,
    "demo/qml" {
        "gui/main.qml" as "main.qml",
    },
);

#[derive(QObject, Default)]
struct Todos {
    base: qt_base_class!(trait QObject),
}

fn register_all_types_in_qml() {
    qml_register_type::<Todos>(
        CStr::from_bytes_with_nul(b"RustCode\0").unwrap(), // qml module name
        1, // major version
        0, // minor version
        CStr::from_bytes_with_nul(b"Todos\0").unwrap(), // type name
    );
}

fn main() {
    gui();
    register_all_types_in_qml();
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/demo/qml/main.qml".into());
    engine.exec();
}
