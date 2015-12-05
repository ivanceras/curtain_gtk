extern crate gtk;
extern crate glib;
use gtk::{AboutDialog, Window, Menu, MenuItem, MenuToolButton, Frame, Button, ButtonBox, Entry, Label, IconSize, ReliefStyle, CheckButton, RadioButton};
use gtk::WindowType::Toplevel;
use gtk::traits::widget::WidgetTrait;
use gtk::traits::window::WindowTrait;
use gtk::traits::_box::BoxTrait;
use gtk::WindowPosition::Center;
use gtk::signal::WidgetSignals;
use gtk::signal::Inhibit;
use gtk::traits::container::ContainerTrait;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::traits::entry::EntryTrait;
use gtk::traits::menu_shell::MenuShellTrait;
use gtk::traits::button::ButtonTrait;
use gtk::signal::ButtonSignals;

fn main() {
	
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}

	let window = Window::new(Toplevel).unwrap();
	window.set_title("Curtain - GTK");
	window.set_default_size(400,600);
	let store = gtk::ListStore::new(&[glib::types::Type::String, glib::types::Type::String, glib::types::Type::String]).unwrap();
	let model = store.get_model().unwrap();
	let treeview = gtk::TreeView::new_with_model(&model);	
	window.set_window_position(Center);
	window.show_all();
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
	gtk::main();
}
