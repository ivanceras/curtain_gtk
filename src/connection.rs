//! database connection url dialogbox

use gtk::{AboutDialog, Window, Menu, MenuItem, 
MenuToolButton, Frame, Button, ButtonBox, Entry, 
Label, IconSize, ReliefStyle, CheckButton, 
RadioButton,Toolbar,Image,ToolButton,
TreeView, TreeViewColumn,ListStore,CellRendererText,CellRendererToggle,
NoteBook, ScrolledWindow,Stack,StackSwitcher,
ComboBoxText, InputPurpose,Grid,
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

use gtk;

pub enum Connection{
	Url(String),
	Scheme(ConnectionScheme),
}


pub struct ConnectionScheme{
	platform: String,
	host: String,
	port: u16,
	database: String,
	user: String,
	password: String,
}

impl Connection{	
	
	pub fn new()->Self{
	
		Connection::Url("postgres://postgre".to_owned())
	}

	pub fn build_widget(&self)->Dialog{
		let title = "New connection";
		let flags = DialogFlags::all();
		let dialog = Dialog::with_buttons(title, None, flags,
                     [("Ok", ResponseType::Accept), ("Cancel", ResponseType::Cancel)]);

		let area = dialog.get_content_area();
		let stack = Stack::new().unwrap();
		let stack_switcher = StackSwitcher::new().unwrap();
		let simple = Self::connection_scheme_widget();
		let advance = Self::connection_url_widget();
		let vbox = gtk::Box::new(Vertical,10).unwrap();
		stack.add_titled(&simple,"simple", "Simple");
		stack.add_titled(&advance, "advance", "Advance");
		vbox.pack_start(&stack_switcher, true, true, 10);
		vbox.pack_start(&stack, true, true, 10);
		stack_switcher.set_stack(stack);
		area.pack_start(&vbox, true, true, 10);

		dialog.show_all();
		dialog
	}

	fn connection_url_widget()->ButtonBox{
		let bbox = ButtonBox::new(Horizontal).unwrap();
		let label = Label::new("Connection Url: ").unwrap();
		let entry = Entry::new().unwrap();
		bbox.pack_start(&label, true, true, 10);
		bbox.pack_start(&entry, true, true, 10);
		bbox
	}

	fn connection_scheme_widget()->Grid{
		let grid = Grid::new().unwrap();

		let platform_lbl = Label::new("platform").unwrap();
		let platform_cb = ComboBoxText::new().unwrap();
		platform_cb.append("pg", "postgres");
		platform_cb.append("my", "mysql");
		platform_cb.append("sq", "sqlite");
		grid.attach(&platform_lbl, 0, 0, 1, 1);
		grid.attach(&platform_cb, 1, 0, 1, 1);
		
		let host_lbl = Label::new("host").unwrap();
		let host_entry = Entry::new().unwrap();
		grid.attach(&host_lbl, 0, 1, 1, 1);
		grid.attach(&host_entry, 1, 1, 1, 1);

		let port_lbl = Label::new("port").unwrap();
		let port_entry = Entry::new().unwrap();
		grid.attach(&port_lbl, 0, 2, 1, 1);
		grid.attach(&port_entry, 1, 2, 1, 1);

		let db_lbl = Label::new("database").unwrap();
		let db_entry = Entry::new().unwrap();
		grid.attach(&db_lbl, 0, 3, 1, 1);
		grid.attach(&db_entry, 1, 3, 1, 1);
		
		let user_lbl = Label::new("user").unwrap();
		let user_entry = Entry::new().unwrap();
		grid.attach(&user_lbl, 0, 4, 1, 1);
		grid.attach(&user_entry, 1, 4, 1, 1);
		
		let pwd_lbl = Label::new("password").unwrap();
		let pwd_entry = Entry::new().unwrap();
		pwd_entry.set_input_purpose(InputPurpose::Password);
		grid.attach(&pwd_lbl, 0, 5, 1, 1);
		grid.attach(&pwd_entry, 1, 5, 1, 1);
		
		grid
	}

}
