
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

use gtk;

#[derive(Clone)]
pub struct DataToolbar{
	
	pub new: ToolButton,
	pub save: ToolButton,
	pub refresh: ToolButton,
	pub detail: ToolButton,
	pub delete: ToolButton,
	pub undo: ToolButton,
	pub redo: ToolButton,

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
		let detail_button = ToolButton::new::<Image>(Some(&detail_icon), Some("Detailed View")).unwrap();
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

		DataToolbar{
			new: new_button,
			save: save_button,
			refresh: refresh_button,
			detail: detail_button,
			delete: delete_button,
			undo: undo_button,
			redo: redo_button,
		}
	}

	pub fn build_widget(&self)->Toolbar{
		let toolbar = Toolbar::new().unwrap();
		toolbar.add(&self.new);
		toolbar.add(&self.save);
		toolbar.add(&self.refresh);
		toolbar.add(&self.detail);
		toolbar.add(&self.delete);
		toolbar.add(&self.undo);
		toolbar.add(&self.redo);
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
		let stack = Stack::new().unwrap();
		let stack_switcher = StackSwitcher::new().unwrap();
		stack.add_titled(&self.list_content, "list", "List View");
		stack.add_titled(&self.detail_content, "detail", "Detailed View");
		//vbox.add(&stack_switcher);
		vbox.add(&stack);
		stack_switcher.set_stack(stack);
		self.init_detail_toolbar();
		vbox
	}

	fn set_list_view(&self, list_view: bool){
		self.list_content.set_visible(list_view);
		self.detail_content.set_visible(!list_view);
		let (icon_name, label) = match list_view{
			true => ("view-fullscreen", "View Detail"),
			false => ("view-restore", "List View")
		};
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
			if let Some(path) = model.get_path(&iter){
				selection.select_path(&path);
			}
		}
		tree.set_size_request(200, 600);
		let scroll = ScrolledWindow::new(None, None).unwrap();
		scroll.add(&tree);
		let vbox = gtk::Box::new(Vertical, 0).unwrap();
		vbox.add(&scroll);
		let paging = Self::create_paging_buttons();
		vbox.pack_end(&paging, false, false, 0);
		vbox	
	}
	
	fn create_detail_content()->gtk::Box{
		let vbox = gtk::Box::new(Vertical,0).unwrap();
		let btn  = Button::new_with_label("Detail view").unwrap();
		vbox.pack_start(&btn,true,true,0);
		vbox
	}
	
	//paging
	fn create_paging_buttons()->Toolbar{
		let toolbar = Toolbar::new().unwrap();
		let prev_icon = Image::new_from_icon_name("go-previous", IconSize::SmallToolbar as i32).unwrap();
		let prev_button = ToolButton::new::<Image>(Some(&prev_icon), None).unwrap();
		//prev_button.set_is_important(true);
		toolbar.add(&prev_button);
		
		let next_icon = Image::new_from_icon_name("go-next", IconSize::SmallToolbar as i32).unwrap();
		let next_button = ToolButton::new::<Image>(Some(&next_icon), None).unwrap();
		//next_button.set_is_important(true);
		toolbar.add(&next_button);
		
		toolbar
	}
}
