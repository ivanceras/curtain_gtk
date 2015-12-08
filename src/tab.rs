
use gtk::{AboutDialog, Window, Menu, MenuItem, 
MenuToolButton, Frame, Button, ButtonBox, Entry, 
Label, IconSize, ReliefStyle, CheckButton, 
RadioButton,Toolbar,Image,ToolButton,
TreeView, TreeViewColumn,ListStore,CellRendererText,CellRendererToggle,
NoteBook, ScrolledWindow,Stack,StackSwitcher, Grid,
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

static VIEW_DETAIL: &'static str = "View detail";
static LIST_VIEW: &'static str = "List view";

#[derive(Clone)]
pub struct DataToolbar{
	
	pub new: ToolButton,
	pub save: ToolButton,
	pub refresh: ToolButton,
	pub detail: ToolButton,
	pub detail_icon: Image,
	pub delete: ToolButton,
	pub undo: ToolButton,
	pub redo: ToolButton,
	pub find: ToolButton,

}

impl DataToolbar{

	pub fn new()->Self{
		
		// new
		let new_icon = Image::new_from_icon_name("list-add", IconSize::SmallToolbar as i32).unwrap();
		let new_button = ToolButton::new::<Image>(Some(&new_icon), Some("New")).unwrap();
		new_button.set_is_important(true);
	   
		// save button
		let save_icon = Image::new_from_icon_name("document-save", IconSize::SmallToolbar as i32).unwrap();
		let save_button = ToolButton::new::<Image>(Some(&save_icon), Some("Save")).unwrap();
		save_button.set_is_important(true);
		
		// refresh button
		let refresh_icon = Image::new_from_icon_name("view-refresh", IconSize::SmallToolbar as i32).unwrap();
		let refresh_button = ToolButton::new::<Image>(Some(&refresh_icon), Some("Refresh")).unwrap();
		refresh_button.set_is_important(true);

		let detail_icon = Image::new_from_icon_name("view-fullscreen", IconSize::SmallToolbar as i32).unwrap();
		let detail_button = ToolButton::new::<Image>(Some(&detail_icon), Some(VIEW_DETAIL)).unwrap();
		detail_button.set_is_important(true);
		
		// delete button
		let delete_icon = Image::new_from_icon_name("list-remove", IconSize::SmallToolbar as i32).unwrap();
		let delete_button = ToolButton::new::<Image>(Some(&delete_icon), Some("Delete")).unwrap();
		delete_button.set_is_important(true);

		
		//undo
		let undo_icon = Image::new_from_icon_name("edit-undo", IconSize::SmallToolbar as i32).unwrap();
		let undo_button = ToolButton::new::<Image>(Some(&undo_icon), Some("Undo")).unwrap();
		undo_button.set_is_important(true);
		
		//redo
		let redo_icon = Image::new_from_icon_name("edit-redo", IconSize::SmallToolbar as i32).unwrap();
		let redo_button = ToolButton::new::<Image>(Some(&redo_icon), Some("Redo")).unwrap();
		redo_button.set_is_important(true);

		//find
		let find_icon = Image::new_from_icon_name("edit-find", IconSize::SmallToolbar as i32).unwrap();
		let find_button = ToolButton::new::<Image>(Some(&find_icon), Some("Search")).unwrap();
		find_button.set_is_important(true);
		
		DataToolbar{
			new: new_button,
			save: save_button,
			refresh: refresh_button,
			detail: detail_button,
			detail_icon: detail_icon,
			delete: delete_button,
			undo: undo_button,
			redo: redo_button,
			find: find_button,
		}
	}

	pub fn build_widget(&self)->Toolbar{
		let toolbar = Toolbar::new().unwrap();
		toolbar.add(&self.new);
		toolbar.add(&self.save);
		toolbar.add(&self.refresh);
		toolbar.add(&self.delete);
		toolbar.add(&self.undo);
		toolbar.add(&self.redo);
		toolbar.add(&self.find);
		toolbar.add(&self.detail);
		toolbar
	}


}

#[derive(Clone)]
pub struct CurtainTab{
	pub toolbar: DataToolbar,
	pub list_content: gtk::Box,
	pub detail_content: gtk::Box,
}

impl CurtainTab{

	pub fn new()->Self{
		let toolbar = DataToolbar::new();
		let list_content = Self::create_list_content();
		let detail_content = Self::create_detail_content();
			

		let ctab = CurtainTab{
			toolbar: toolbar,
			list_content: list_content,
			detail_content: detail_content,
		};
		ctab
	}

	
	fn init_detail_toolbar(&self){
		self.list_content.set_visible(true);
		self.detail_content.set_visible(false);
		let this = self.clone();
		self.toolbar.detail.connect_clicked(move|_|{
			println!("detail button clicked!..");
			this.toggle_list_view();
		});
	}

	pub fn build_widget(&self)->gtk::Box{
		let vbox = gtk::Box::new(Vertical,0).unwrap();
		vbox.add(&self.toolbar.build_widget());
		vbox.add(&self.list_content);
		vbox.add(&self.detail_content);
		self.detail_content.set_no_show_all(true);
		self.init_detail_toolbar();
		vbox
	}

	fn set_list_view(&self, list_view: bool){
		self.detail_content.set_no_show_all(false);
		self.detail_content.show_all();
		self.list_content.set_visible(list_view);
		self.detail_content.set_visible(!list_view);
		let (icon_name, label) = match list_view{
			true => ("view-fullscreen", VIEW_DETAIL),
			false => ("view-restore", LIST_VIEW)
		};
		println!("icon name: {:#?}", self.toolbar.detail.get_icon_name());
		self.toolbar.detail.set_icon_name(icon_name);//FIXME doesn't change the icon
		self.toolbar.detail_icon.set_from_icon_name(icon_name, IconSize::SmallToolbar as i32);
		self.toolbar.detail.set_label(label);
		println!("now icon name: {:#?}", self.toolbar.detail.get_icon_name());
	}
	
	fn toggle_list_view(&self){
		let list_view = self.list_content.is_visible();
		self.set_list_view(!list_view);
	}

	/// a contents to the notebook
	fn create_list_content()->gtk::Box{
		let tree = TreeView::new().unwrap();
		tree.set_resize_mode(gtk::ResizeMode::Immediate);
		tree.set_hexpand(true);
		tree.set_vexpand(true);
		let column_types = [Type::String, Type::String, Type::Bool];
		let store = ListStore::new(&column_types).unwrap();
		let model = store.get_model().unwrap();
		tree.set_model(&model);
		tree.set_headers_visible(true);
		let column1 = TreeViewColumn::new().unwrap();
		let cell1 = CellRendererText::new().unwrap();
		let sort_icon = Image::new_from_icon_name("window-close",                                                         IconSize::Button as i32).unwrap();
		let sort_button = Button::new().unwrap();
		sort_button.add(&sort_icon);
		column1.pack_start(&cell1, true);
		tree.add(&sort_button);
		column1.set_min_width(1);//1pixel
		column1.set_max_width(500);
		column1.add_attribute(&cell1, "text", 0);
		column1.set_reorderable(true);
		column1.set_resizable(true);
		column1.set_title("Column1");
		tree.append_column(&column1);
		
		let column2 = TreeViewColumn::new().unwrap();
		let cell2 = CellRendererText::new().unwrap();
		column2.set_title("Column2");
		column2.set_resizable(true);
		column2.pack_start(&cell2, true);
		column2.add_attribute(&cell2, "text", 1);
		column2.set_reorderable(true);
		tree.append_column(&column2);
		
		let column3 = TreeViewColumn::new().unwrap();
		let cell3 = CellRendererToggle::new().unwrap();
		column3.set_title("Active");
		column3.set_resizable(true);
		column3.pack_start(&cell3, true);
		column3.add_attribute(&cell3, "activatable", 2);
		column3.set_reorderable(true);
		tree.append_column(&column3);
		
		let selection = tree.get_selection().unwrap();
		selection.connect_changed(| tree_selection |{
				let (model, iter) = tree_selection.get_selected().unwrap();
				if let Some(path) = model.get_path(&iter) {
					let value = unsafe {model.get_value(&iter,1).get_string().unwrap()};
					println!("selected row {} {}", path.to_string().unwrap(), value);
				}	
			}
		);
		for i in 0..50{
			let iter = store.append();
			store.set_string(&iter, 0, &format!("I am a content in a tab{}", i));
			store.set_string(&iter, 1, &format!("Cell 2 tab {}", i));
			let mut bvalue = unsafe {Value::new()};
			bvalue.init(Type::Bool);
			unsafe{bvalue.set_boolean(true)};
			store.set_value(&iter, 2, &bvalue);
			/*if let Some(path) = model.get_path(&iter){
				selection.select_path(&path);
			}
			*/
		}
		//tree.set_size_request(200, 600);
		let scroll = ScrolledWindow::new(None, None).unwrap();
		scroll.add(&tree);
		//scroll.set_min_content_height(600);
		let vbox = gtk::Box::new(Vertical, 0).unwrap();
		//vbox.pack_start(&scroll, true, true, 10);
		let paging = Self::create_paging_buttons_for_list();
		//vbox.pack_start(&paging, true, true, 10);
		vbox.add(&scroll);
		vbox.pack_start(&paging, false, false, 10);
		vbox	
	}
	
	fn create_detail_content()->gtk::Box{
		let vbox = gtk::Box::new(Vertical,0).unwrap();
		let hbox = gtk::Box::new(Horizontal,0).unwrap();
		let close_btn  = Button::new().unwrap();
		let close_image = Image::new_from_icon_name("window-close",IconSize::Menu as i32).unwrap();
		close_btn.add(&close_image);

		vbox.add(&hbox);
		hbox.pack_end(&close_btn,false,false,0);
		let nav_records = Self::create_record_nav_buttons_for_detail();
		hbox.pack_end(&nav_records, false, false, 5);
		let grid = Grid::new().unwrap();
		


		for i in 0..5{
				let label = Label::new(&format!("column {}",i+1)).unwrap();
				let entry = Entry::new().unwrap();
				entry.set_text(&format!("value {}", (i+1)*100));
				grid.attach(&label, 0, i*2, 1, 2);
				grid.attach(&entry, 2, i*2, 2, 1);
		}
		vbox.pack_start(&grid, true, true, 10);
		vbox
	}
	
	fn create_record_nav_buttons_for_detail()->gtk::Box{
		let paging = gtk::Box::new(Horizontal, 0).unwrap();
		let prev_icon = Image::new_from_icon_name("go-previous", IconSize::SmallToolbar as i32).unwrap();
		let prev_button = Button::new().unwrap();
		prev_button.add(&prev_icon);
		paging.pack_start(&prev_button, false, false, 0);
		
		let next_icon = Image::new_from_icon_name("go-next", IconSize::SmallToolbar as i32).unwrap();
		let next_button = Button::new().unwrap();
		next_button.add(&next_icon);
		paging.pack_start(&next_button, false, false, 0);
		
		paging
	}
	//paging
	fn create_paging_buttons_for_list()->gtk::Box{
		let paging = gtk::Box::new(Horizontal, 0).unwrap();
		let prev_icon = Image::new_from_icon_name("go-previous", IconSize::SmallToolbar as i32).unwrap();
		let prev_button = Button::new().unwrap();
		prev_button.add(&prev_icon);
		paging.pack_start(&prev_button, false, false, 0);
		
		let next_icon = Image::new_from_icon_name("go-next", IconSize::SmallToolbar as i32).unwrap();
		let next_button = Button::new().unwrap();
		next_button.add(&next_icon);
		paging.pack_start(&next_button, false, false, 0);
		
		paging
	}
}
