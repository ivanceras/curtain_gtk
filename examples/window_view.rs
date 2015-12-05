extern crate gtk;
extern crate glib;
extern crate curtain_gtk;

use gtk::{AboutDialog, Window, Menu, MenuItem, 
MenuToolButton, Frame, Button, ButtonBox, Entry, 
Label, IconSize, ReliefStyle, CheckButton, 
RadioButton,Toolbar,Image,ToolButton,
TreeView, TreeViewColumn,ListStore,CellRendererText,CellRendererToggle,
NoteBook, ScrolledWindow
};
use gtk::WindowType::Toplevel;
use gtk::traits::widget::WidgetTrait;
use gtk::traits::window::WindowTrait;
use gtk::traits::_box::BoxTrait;
use gtk::WindowPosition::Center;
use gtk::traits::container::ContainerTrait;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::traits::entry::EntryTrait;
use gtk::traits::menu_shell::MenuShellTrait;
use gtk::traits::button::ButtonTrait;
use gtk::signal::{ButtonSignals,ToolButtonSignals,WidgetSignals,Inhibit};
use gtk::traits::tool_item::ToolItemTrait;
use gtk::traits::tool_button::ToolButtonTrait;
use glib::Type;
use glib::Value;
use curtain_gtk::window::CurtainWindow; 

fn main(){
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}

	let window = CurtainWindow::new();
	window.build_widget();
	gtk::main();

}
