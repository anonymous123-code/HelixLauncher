use cstr::cstr;
use qmetaobject::qtquickcontrols2::QQuickStyle;
use qmetaobject::{prelude::*, qml_register_singleton_type};
use std::env;

mod instances;

qrc!(register_resources,
     "qml" as "qml" {
         "main.qml",
         "InstancesPage.qml",
         "NewInstancePage.qml",
         "InstancePage.qml",
     },
);

fn main() {
    if env::var_os("QT_QUICK_CONTROLS_STYLE").is_none() {
        QQuickStyle::set_style("org.kde.desktop");
    }

    register_resources();

    qml_register_singleton_type::<instances::InstancesModel>(
        cstr!("dev.helixlauncher.qml"),
        1,
        0,
        cstr!("InstancesModel"),
    );
    let mut engine = QmlEngine::new();
    engine.load_file("qrc:/qml/main.qml".into());
    engine.exec();
}
