extern crate gtk;
extern crate glib;
extern crate curtain_gtk;

use gtk::{AboutDialog, Window, Menu, MenuItem, 
MenuToolButton, Frame, Button, ButtonBox, Entry, 
Label, IconSize, ReliefStyle, CheckButton, 
RadioButton,Toolbar,Image,ToolButton,
TreeView, TreeViewColumn,ListStore,CellRendererText,CellRendererToggle,
NoteBook, ScrolledWindow,Stack,StackSwitcher
};
use gtk::traits::{WidgetTrait, WindowTrait, BoxTrait, 
ToolItemTrait,DialogTrait,
ToolButtonTrait, ContainerTrait, EntryTrait,
MenuShellTrait, ButtonTrait
}; 

use gtk::WindowType::Toplevel;
use gtk::WindowPosition::Center;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::signal::{ButtonSignals,ToolButtonSignals,WidgetSignals,Inhibit};
use glib::Type;
use glib::Value;
use window::CurtainWindow;
use gtk::Dialog;
use gtk::ResponseType;
use gtk::DialogFlags;
use gtk::MessageDialog;
use gtk::MessageType;
use gtk::ButtonsType;


mod window;
mod tab;
mod connection;

fn main(){
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}

	let cwindow = CurtainWindow::new();
	cwindow.build_widget();
	gtk::main();

}
