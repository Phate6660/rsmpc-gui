extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Button, Orientation::Horizontal, prelude::*};
use mpd::Client;
use std::env;

fn control(function: &str) {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    if function == "toggle" {
        c.toggle_pause().unwrap();
    } else if function == "next" {
        c.next().unwrap();
    } else if function == "prev" {
        c.prev().unwrap();
    }
}

fn main() {
    let uiapp = gtk::Application::new(
        Some("whydoes.thisidentify.likeanapp"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);
        win.set_title("rsmpc-gui");

        let hbox = gtk::Box::new(Horizontal, 5);

        // Buttons
        let toggle = Button::with_label("toggle");
        toggle.connect_clicked(|_| {
            control("toggle");
        });
        let next = Button::with_label("next");
        next.connect_clicked(|_| {
            control("next");
        });
        let prev = Button::with_label("previous");
        prev.connect_clicked(|_| {
            control("prev");
        });
        hbox.pack_start(&prev, true, false, 2);
        hbox.pack_start(&toggle, true, false, 2);
        hbox.pack_start(&next, true, false, 2);
        win.add(&hbox);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
