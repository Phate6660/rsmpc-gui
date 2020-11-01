extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::{Box, Button, Orientation::{Horizontal, Vertical}, prelude::*};
use mpd::{Client, Song};
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

fn info() -> String {
    let mut c = Client::connect("127.0.0.1:6600").unwrap();
    let song: Song = c.currentsong().unwrap().unwrap();
    let tit = song.title.as_ref().unwrap();
    let art = song.tags.get("Artist").unwrap();
    format!("{} - {}", art, tit)
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

        // Boxes for organizing widgets
        let vbox = Box::new(Vertical, 0);
        let hbox = Box::new(Horizontal, 0);
        let infobox = Box::new(Horizontal, 0);

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

        // Cover art
        let image = gtk::Image::from_file("/tmp/cover.png");

        // Music info
        let info = info();
        let entry = gtk::Entry::new();
        entry.set_text(&info);
        entry.set_overwrite_mode(false);
        entry.set_has_frame(false);
        entry.set_activates_default(false);
        entry.set_alignment(0.5);

        // Actually use the boxes
        hbox.pack_start(&prev, true, false, 2);
        hbox.pack_start(&toggle, true, false, 2);
        hbox.pack_start(&next, true, false, 2);
        infobox.pack_start(&entry, true, true, 2);
        vbox.pack_start(&infobox, true, true, 2);
        vbox.pack_start(&image, true, true, 0);
        vbox.pack_start(&hbox, false, false, 0);

        // Display in window
        win.add(&vbox);
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
