//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gtk;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("File Menu");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);
    
    let menu_bar = gtk::MenuBar

    let menu_box = gtk::Box(gtk::Orientation::Vertical);
    menu_box.pack_start(&menu_bar);
    window.add(&menu_box);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
