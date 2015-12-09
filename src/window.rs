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


#[derive(Clone)]
pub struct CurtainWindow{
	window_list: TreeView,
	list_widget: ScrolledWindow,
	toolbar: Toolbar,
	open_tabs: NoteBook,
	pub window: Window,
}

impl CurtainWindow{

	pub fn new()->Self{
		let (window_list, list_widget) = Self::create_window_list();	
		let toolbar = Self::create_toolbar();
		let open_tabs = Self::create_tabs();
		let window = Window::new(Toplevel).unwrap();
		
		CurtainWindow{
			window_list: window_list,
			list_widget: list_widget,
			toolbar: toolbar,
			open_tabs: open_tabs,
			window: window
		}
	}

	pub fn build_widget<'a>(&'a self){
		self.window.set_title("Curtain - GTK");
		self.window.set_default_size(1024,768);
		let vbox = gtk::Box::new(Vertical, 0).unwrap();
		vbox.add(&self.toolbar);
		self.window.add(&vbox);
		let hbox = gtk::Box::new(Horizontal,0).unwrap();
		vbox.add(&hbox);
		hbox.add(&self.list_widget);
		hbox.add(&self.open_tabs);
		self.window.set_window_position(Center);
		for i in 0..3{
			self.add_curtain_tab(&format!("Window {}",i));
		}
		self.init_window_list_selection();
		let this = self.clone();
		self.window.show_all();
		self.window.connect_delete_event(|_,_| {
			gtk::main_quit();
			Inhibit(false)
		});
	}

	// this is a list of the items to be viewed
	fn create_window_list()->(TreeView, ScrolledWindow){
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
		for i in 0..50{
			let iter = store.append();
			store.set_string(&iter, 0, &format!("Window {}", i));
		}
		let scroll = ScrolledWindow::new(None, None).unwrap();
		scroll.add(&tree);
		scroll.set_size_request(200, 600);
		(tree, scroll)
	}

	fn init_window_list_selection(&self){
		let this = self.clone();
		self.window_list.get_selection().as_ref().unwrap().connect_changed(move| _ |{
			let (model, iter) = this.window_list.get_selection().as_ref().unwrap().get_selected().unwrap();
			if let Some(path) = model.get_path(&iter) {
				let value = unsafe {model.get_value(&iter,0).get_string().unwrap()};
				println!("open window {} {}", path.to_string().unwrap(), value);
				this.add_curtain_tab(&value);
				//TODO: show the newly open tab
				this.window.show_all();//Notes: Important to call show all in order to redraw the added tabs
			}	
		});

	}

	// add a toolbar to the vertical box of the window
	fn create_toolbar()->Toolbar{
		let toolbar = Toolbar::new().unwrap();
		let open_icon = Image::new_from_icon_name("network-server", IconSize::LargeToolbar as i32).unwrap();
		let open_button = ToolButton::new::<Image>(Some(&open_icon), Some("Connect to Server")).unwrap();
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
		notebook
	}

	pub fn add_curtain_tab(&self, title:&str){
		println!("Adding {}", title);
		let tab = gtk::Box::new(Horizontal, 0).unwrap();
		let sheet = Label::new(title).unwrap();
		let close_btn = Button::new().unwrap();
		
		close_btn.set_relief(ReliefStyle::None);
		close_btn.set_focus_on_click(false);
		let close_image = Image::new_from_icon_name("window-close",IconSize::Menu as i32).unwrap();
		close_btn.add(&close_image);
		tab.add(&sheet);
		tab.add(&close_btn);
		let curtain_tab = CurtainTab::new();
		let curtain_widget = curtain_tab.build_widget();
		curtain_widget.show_all();// needed in order for setting current page works
		self.open_tabs.append_page(&curtain_widget, Some(&tab));
		self.open_tabs.set_tab_reorderable(&curtain_widget, true);
        self.open_tabs.set_tab_detachable(&curtain_widget, true);
		self.open_tabs.popup_enable();
		let page_num = self.open_tabs.page_num(&curtain_widget).unwrap();
		println!("page num: {}", page_num);
		//curtain_widget.set_visible(true);//NEEDED or else set_current_page wont work
		self.open_tabs.set_current_page(page_num as i32);
		let notebook_clone = self.open_tabs.clone();
		close_btn.connect_clicked(move|_|{
					let index = notebook_clone.page_num(&curtain_widget).unwrap();
					notebook_clone.remove_page(index as i32);
			}
		);
		tab.show_all();
	}

}
