extern crate gtk;

use gtk::{Window,Stack};
use gtk::WindowType::Toplevel;
use gtk::traits::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::signal::Inhibit;
use gtk::WindowPosition::Center;

fn main(){
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}	

	let window = Window::new(Toplevel).unwrap();
	window.set_title("Stack demo");
	let stack = Stack::new().unwrap();
	let label = gtk::Label::new("Stacked").unwrap();

	let check_btn = gtk::Button::new_with_label("Click me!").unwrap();
	stack.add_titled(&check_btn, "check", "Cechk Button");
	stack.add_titled(&label, "label", "A label");
	let vbox = gtk::Box::new(Vertical, 0).unwrap();
	window.add(&vbox);
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
let stack_switcher = gtk::StackSwitcher::new().unwrap();
	vbox.pack_start(&stack_switcher, true, true, 0);
	vbox.pack_start(&stack, true, true, 0);
	stack_switcher.set_stack(stack);
	window.set_size_request(800,600);
	window.set_window_position(Center);
	window.show_all();
	gtk::main();
}
