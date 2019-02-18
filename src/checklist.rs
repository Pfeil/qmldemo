use crate::qmetaobject::qttypes::QString;
use crate::qmetaobject::*;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Default, Clone)]
struct ListItem {
    name: String,
    isChecked: bool,
}


#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct CheckList {
    base: qt_base_class!(trait QAbstractListModel),

    list: Vec<ListItem>,
    init: qt_method!(fn(&mut self)),
    setCompleted: qt_method!(fn(&mut self, index: usize, status: bool) -> bool),}

impl CheckList {
    fn init(&mut self) {
        // qt pre-boilerplate
        // qt doc: https://doc.qt.io/qt-5/qabstractlistmodel-members.html
        (self as &mut QAbstractListModel).begin_reset_model();

        self.list = vec![
            ListItem {
                name: "foo".into(),
                isChecked: false,
            },
            ListItem {
                name: "bar".into(),
                isChecked: false,
            },
            ListItem {
                name: "Rust".into(),
                isChecked: true,
            },
            ListItem {
                name: "Meetup".into(),
                isChecked: true,
            },
            ListItem {
                name: "Karlsruhe".into(),
                isChecked: false,
            },
            ListItem {
                name: "Safety".into(),
                isChecked: false,
            },
            ListItem {
                name: "Concurrency".into(),
                isChecked: false,
            },
            ListItem {
                name: "Speed".into(),
                isChecked: false,
            },
        ];

        // qt post-boilerplate
        (self as &mut QAbstractListModel).end_reset_model();
    }
    #[allow(non_snake_case)]
    fn setCompleted(&mut self, index: usize, status: bool) -> bool {
        if index >= self.list.len() {
            return false;
        }
        self.list[index].isChecked = status;
        let idx = (self as &mut QAbstractListModel).row_index(index as i32);
        (self as &mut QAbstractListModel).data_changed(idx.clone(), idx);
        println!("Rust: Updated Index {} to {}", index, status);
        true
    }
}

impl QAbstractListModel for CheckList {
    fn row_count(&self) -> i32 {
        self.list.len() as i32
    }
    fn data(&self, index: QModelIndex, role: i32) -> QVariant {
        let idx = index.row() as usize;
        if idx < self.list.len() {
            if role == USER_ROLE {
                QString::from(self.list[idx].name.clone()).into()
            } else if role == USER_ROLE + 1 {
                QVariant::from(self.list[idx].isChecked)
            } else {
                QVariant::default()
            }
        } else {
            QVariant::default()
        }
    }
    fn role_names(&self) -> HashMap<i32, QByteArray> {
        let mut map = HashMap::new();
        map.insert(USER_ROLE, "name".into());
        map.insert(USER_ROLE + 1, "isChecked".into());
        map
    }
}
