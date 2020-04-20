use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

#[macro_use]
mod utils;
mod config;
mod window;
mod timer_display;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("dhyana", config::LOCALEDIR);
    textdomain("dhyana");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/dhyana.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("dev.rdpi.Builder"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();
        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
