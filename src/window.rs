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
use tab::DataToolbar;
use tab::CurtainTab;
use connection::Connection;


use gtk;


pub struct CurtainWindow{
	window_list: ScrolledWindow,
	toolbar: Toolbar,
	open_tabs: NoteBook,
}

impl CurtainWindow{

	pub fn new()->Self{
		let window_list = Self::create_window_list();	
		let toolbar = Self::create_toolbar();
		let open_tabs = Self::create_tabs();
		CurtainWindow{
			window_list: window_list,
			toolbar: toolbar,
			open_tabs: open_tabs,
		}
	}

	pub fn build_widget(&self)->Window{
		let window = Window::new(Toplevel).unwrap();
		window.set_title("Curtain - GTK");
		//window.set_default_size(1024,768);
		let vbox = gtk::Box::new(Vertical, 0).unwrap();
		vbox.add(&self.toolbar);
		window.add(&vbox);
		let hbox = gtk::Box::new(Horizontal,0).unwrap();
		vbox.add(&hbox);
		hbox.add(&self.window_list);
		hbox.add(&self.open_tabs);
		window.set_window_position(Center);
		window.show_all();
		window.connect_delete_event(|_,_| {
			gtk::main_quit();
			Inhibit(false)
		});
		window
	}

	// this is a list of the items to be viewed
	fn create_window_list()->ScrolledWindow{
		let tree = TreeView::new().unwrap();
		let column_types = [Type::String];
		let store = ListStore::new(&column_types).unwrap();
		let model = store.get_model().unwrap();
		tree.set_model(&model);
		tree.set_headers_visible(false);
		let column = TreeViewColumn::new().unwrap();
		let cell = CellRendererText::new().unwrap();
		column.pack_start(&cell, true);
		column.add_attribute(&cell, "text", 0);
		tree.append_column(&column);
		let selection = tree.get_selection().unwrap();
		selection.connect_changed(| tree_selection |{
				let (model, iter) = tree_selection.get_selected().unwrap();
				if let Some(path) = model.get_path(&iter) {
					let value = unsafe {model.get_value(&iter,0).get_string().unwrap()};
					println!("selected row {} {}", path.to_string().unwrap(), value);
				}	
			}
		);
		for i in 0..50{
			let iter = store.append();
			store.set_string(&iter, 0, &format!("Window {}", i));
			if let Some(path) = model.get_path(&iter){
				selection.select_path(&path);
			}
		}
		let scroll = ScrolledWindow::new(None, None).unwrap();
		scroll.add(&tree);
		scroll.set_size_request(200, 600);
		scroll
	}

	// add a toolbar to the vertical box of the window
	fn create_toolbar()->Toolbar{
		let toolbar = Toolbar::new().unwrap();
		//open button
		let open_icon = Image::new_from_icon_name("document-open", IconSize::LargeToolbar as i32).unwrap();
		let open_button = ToolButton::new::<Image>(Some(&open_icon), Some("Open Connection")).unwrap();
		open_button.set_is_important(true);
		open_button.connect_clicked(move|_|{
			println!("Openning connection box");
			let connection = Connection::new();
			connection.build_widget();
		});
		toolbar.add(&open_button);
		toolbar
	}


	// add tabs to the horizontal box that divites the list from the contents of that list
	fn create_tabs()->NoteBook{
		let notebook = NoteBook::new().unwrap();
		notebook.set_scrollable(true);
		for i in 0..8{
			Self::create_curtain_sheet(&notebook, &format!("Curtain {}", i+1));
		}
		notebook
	}

	fn create_curtain_sheet(notebook: &NoteBook, title:&str){
		let tab = gtk::Box::new(Horizontal, 0).unwrap();
		let sheet = Label::new(title).unwrap();
		let close_btn = Button::new().unwrap();
		
		close_btn.set_relief(ReliefStyle::None);
		close_btn.set_focus_on_click(false);
		let close_image = Image::new_from_icon_name("window-close",IconSize::Button as i32).unwrap();
		close_btn.add(&close_image);
		tab.add(&sheet);
		tab.add(&close_btn);
		let curtain_tab = CurtainTab::new();
		let curtain_widget = curtain_tab.build_widget();
		notebook.append_page(&curtain_widget, Some(&tab));
		let notebook_clone = notebook.clone();
		close_btn.connect_clicked(move|_|{
					let index = notebook_clone.page_num(&curtain_widget).unwrap();
					notebook_clone.remove_page(index as i32);
			}
		);
		tab.show_all();
	}

}
