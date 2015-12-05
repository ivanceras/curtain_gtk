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
    window.set_title("Text File Viewer");
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    let toolbar = gtk::Toolbar::new().unwrap();

    let open_icon = gtk::Image::new_from_icon_name("document-open",
                                                   gtk::IconSize::SmallToolbar as i32).unwrap();
    let text_view = gtk::TextView::new().unwrap();

    let open_button = gtk::ToolButton::new::<gtk::Image>(Some(&open_icon), Some("Open")).unwrap();
    open_button.set_is_important(true);

    toolbar.add(&open_button);

    let scroll = gtk::ScrolledWindow::new(None, None).unwrap();
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0).unwrap();
    vbox.pack_start(&toolbar, false, true, 0);
    vbox.pack_start(&scroll, true, true, 0);

    window.add(&vbox);

    open_button.connect_clicked(move |_| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            "Open File", None, gtk::FileChooserAction::Open,
            [("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)]);
        if file_chooser.run() == gtk::ResponseType::Ok as i32 {
            let filename = file_chooser.get_filename().unwrap();
            let file = File::open(&filename).unwrap();

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            text_view.get_buffer().unwrap().set_text(&contents);
        }

        file_chooser.destroy();
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
