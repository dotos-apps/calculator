extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, Entry};

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
                                        .application(application)
                                        .title("Calculator")
                                        .border_width(20)
                                        .window_position(gtk::WindowPosition::Center)
                                        .default_width(400)
                                        .default_height(550)
                                        .build();

    let entry = Entry::new();

    window.add(&entry);

    window.show_all();
}

fn main() {
    let application = Application::new(
        Some("co.dothq.gtk-demo"),
        Default::default()
    ).expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        build_ui(app);
    });

    application.run(&[]);
}