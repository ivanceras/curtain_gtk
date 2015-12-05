extern crate gtk;

use gtk::{Window,Stack};
use gtk::WindowType::Toplevel;
use gtk::traits::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::signal::Inhibit;

fn main(){
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}	

	let window = Window::new(Toplevel).unwrap();
	window.set_title("Header bar demo");
	let hb = gtk::HeaderBar::new().unwrap();
	hb.set_title("HeaderBar example");
	hb.set_show_close_button(true);
	let lbtn = gtk::Button::new().unwrap();
	lbtn.add(&gtk::Arrow::new(gtk::ArrowType::Left, gtk::ShadowType::None).unwrap());

	let rbtn = gtk::Button::new().unwrap();
	rbtn.add(&gtk::Arrow::new(gtk::ArrowType::Right, gtk::ShadowType::None).unwrap());
	let hbox = gtk::Box::new(Horizontal,2).unwrap();

	hbox.add(&lbtn);
	hbox.add(&rbtn);
	hb.pack_start(&hbox);
	window.set_titlebar(&hb);
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
	window.set_default_size(400, 400);
	window.show_all();
	window.set_decorated(true);
	gtk::main();
}
