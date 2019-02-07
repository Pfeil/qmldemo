extern crate qmetaobject;
use qmetaobject::*;

use std::ffi::CStr;

qrc!(my_ressource,
    "demo/" {
        "gui/main.qml",
    },
);

fn main() {
    my_ressource();
    //qml_register_type::<implementation::Todos>(
    //    CStr::from_bytes_with_nul(b"RustCode\0").unwrap(), // qml module name
    //    1, // major version
    //    0, // minor version
    //    CStr::from_bytes_with_nul(b"Todos\0").unwrap(), 
    //);
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/demo/gui/main.qml".into());
    engine.exec();
}
