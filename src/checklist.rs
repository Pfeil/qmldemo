use crate::qmetaobject::qttypes::QString;
use crate::qmetaobject::*;
use std::collections::HashMap;

#[derive(Default, Clone)]
struct ListItem {
    name: String,
    checked: bool,
}

#[allow(non_snake_case)]
#[derive(Default, QObject)]
pub struct CheckList {
    base: qt_base_class!(trait QAbstractListModel),

    list: Vec<ListItem>,
    init: qt_method!(fn(&mut self)),
}

impl CheckList {
    fn init(&mut self) {
        // qt pre-boilerplate
        // qt doc: https://doc.qt.io/qt-5/qabstractlistmodel-members.html
        (self as &mut QAbstractListModel).begin_reset_model();

        self.list = vec![
            ListItem {
                name: "foo".into(),
                checked: false,
            },
            ListItem {
                name: "bar".into(),
                checked: false,
            },
            ListItem {
                name: "Rust".into(),
                checked: true,
            },
            ListItem {
                name: "Meetup".into(),
                checked: false,
            },
            ListItem {
                name: "Karlsruhe".into(),
                checked: false,
            },
            ListItem {
                name: "Safety".into(),
                checked: false,
            },
            ListItem {
                name: "Concurrency".into(),
                checked: false,
            },
            ListItem {
                name: "Speed".into(),
                checked: false,
            },
        ];

        // qt post-boilerplate
        (self as &mut QAbstractListModel).end_reset_model();
        //self.activeCount = transactions.len();
        //self.active_count_changed();
        //self.count_changed();
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
                QVariant::from(self.list[idx].checked.clone()).into()
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
        map.insert(USER_ROLE + 1, "checked".into());
        map
    }
}
