extern crate gtk;
extern crate glib;

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

mod tab;
mod window;

fn main() {
	
	if gtk::init().is_err(){
		println!("Failed to initialize GTK");
		return;
	}

	let window = Window::new(Toplevel).unwrap();
	window.set_title("Curtain - GTK");
	window.set_default_size(1024,768);
    //vertical box that contains the toolbar and the rest of the widgets
    let vbox = gtk::Box::new(Vertical, 0).unwrap();
	
    let toolbar = create_toolbar();
    vbox.pack_start(&toolbar, false, true, 0);
    //hbox is a horizontal box that contains the window listing and the tab contents
	let hbox = gtk::Box::new(Horizontal,0).unwrap();
	vbox.pack_start(&hbox,true, true,0);
	let list = create_window_list();
	hbox.pack_start(&list, false,true,10);
	let tabs = create_tabs();
    hbox.pack_start(&tabs, true, true, 0);
	window.add(&vbox);
	window.set_size_request(800,600);
	window.set_window_position(Center);
	window.show_all();
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
	gtk::main();
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
    toolbar.add(&open_button);
	toolbar
}
// add tabs to the horizontal box that divites the list from the contents of that list
fn create_tabs()->NoteBook{
	let notebook = NoteBook::new().unwrap();
    notebook.set_scrollable(true);
    for i in 0..8{
        let close_image = Image::new_from_icon_name("window-close",
                                                             IconSize::Button as i32).unwrap();
     
        let tab = gtk::Box::new(Horizontal, 0).unwrap();
        let sheet = Label::new(&format!("sheet {}", i)).unwrap();
        //let content = Button::new_with_label(&format!("Content in page {}", i)).unwrap();
        let close_btn = Button::new().unwrap();
        
        close_btn.set_relief(ReliefStyle::None);
        close_btn.set_focus_on_click(false);
        close_btn.add(&close_image);
        tab.pack_start(&sheet, false, false, 0);
        tab.pack_start(&close_btn, false, false, 0);
        let vbox = gtk::Box::new(Vertical, 0).unwrap();//contains the data toolbar and the list
        let list_content = create_list_content();
		let detail_content = create_detail_content();
        //let data_toolbar = create_data_toolbar(&list_content, &detail_content);//content can be manipulated with the zoom button
        let data_toolbar = DataToolbar::new();
		vbox.add(&data_toolbar.build_widget());
        vbox.add(&list_content);
		//detail_content.set_no_show_all(true);
		detail_content.set_visible(false);
		vbox.add(&detail_content);
        notebook.append_page(&vbox, Some(&tab));
        notebook.set_tab_reorderable(&list_content, true);
        notebook.set_tab_detachable(&list_content, true);

         let close_image2 = gtk::Image::new_from_icon_name("window-close",
                                                             IconSize::Button as i32).unwrap();
     
        let notebook_clone = notebook.clone();
        close_btn.connect_clicked(move|_|{
                    let index = notebook_clone.page_num(&vbox).unwrap();
                    notebook_clone.remove_page(index as i32);
            }
        );
		
        tab.show_all();
		//init_list_view(&list_content, &detail_content);
    }
    notebook
}

fn create_detail_content()->gtk::Box{
	let vbox = gtk::Box::new(Horizontal,0).unwrap();
	let btn  = Button::new_with_label("Detail view").unwrap();
	vbox.add(&btn);
	vbox
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
	let paging = create_paging_buttons();
	vbox.add(&paging);
	vbox
}


fn set_list_view(list_view: bool, list_content:&gtk::Box, detail_content: &gtk::Box, 
		detail_button:Option<&ToolButton>, 
		detail_icon: Option<&Image>)
		
		{
		list_content.set_visible(list_view);
		detail_content.set_visible(!list_view);
		let (icon_name, label) = match list_view{
			true => ("view-fullscreen", "View Detail"),
			false => ("view-restore", "List View  ")
		};
		match detail_icon{
			Some(detail_icon) => {
				detail_icon.set_from_icon_name(&icon_name, IconSize::SmallToolbar as i32);
				match detail_button{
					Some(detail_button) =>  {
						detail_button.set_label(label)
					},
					None => ()
				}
			},
			None => ()
		}
}

fn toggle_list_view(list_content:&gtk::Box, detail_content: &gtk::Box, detail_button:&ToolButton, detail_icon: &Image){
		let list_view = !list_content.is_visible();
		set_list_view(list_view, list_content, detail_content, Some(detail_button), Some(detail_icon));
}

fn init_list_view(list_content:&gtk::Box, detail_content: &gtk::Box){
		set_list_view(true, list_content, detail_content, None, None);
}
//add toolbars to the data tab
fn create_data_toolbar(list_content: &gtk::Box, detail_content: &gtk::Box)->Toolbar{
	let toolbar = Toolbar::new().unwrap();

    // new
    let new_icon = Image::new_from_icon_name("list-add", IconSize::SmallToolbar as i32).unwrap();
    let new_button = ToolButton::new::<Image>(Some(&new_icon), Some("New")).unwrap();
    new_button.set_is_important(true);
    toolbar.add(&new_button);
   
    // save button
    let save_icon = Image::new_from_icon_name("document-save", IconSize::SmallToolbar as i32).unwrap();
    let save_button = ToolButton::new::<Image>(Some(&save_icon), Some("Save")).unwrap();
    save_button.set_is_important(true);
    toolbar.add(&save_button);
	
	// refresh button
    let refresh_icon = Image::new_from_icon_name("view-refresh", IconSize::SmallToolbar as i32).unwrap();
    let refresh_button = ToolButton::new::<Image>(Some(&refresh_icon), Some("Refresh")).unwrap();
    refresh_button.set_is_important(true);
    toolbar.add(&refresh_button);

    let detail_icon = Image::new().unwrap();
    let detail_button = ToolButton::new::<Image>(Some(&detail_icon), Some("Detailed View")).unwrap();
    detail_button.set_is_important(true);
	
    toolbar.add(&detail_button);
	let list_content_clone = list_content.clone();
	let detail_button_clone = detail_button.clone();
	let detail_content_clone = detail_content.clone();
	set_list_view(true, list_content, detail_content, Some(&detail_button), Some(&detail_icon));
	detail_button.connect_clicked(move|_|{
		toggle_list_view(&list_content_clone, &detail_content_clone, &detail_button_clone, &detail_icon);
	});

    // delete button
    let delete_icon = Image::new_from_icon_name("list-remove", IconSize::SmallToolbar as i32).unwrap();
    let delete_button = ToolButton::new::<Image>(Some(&delete_icon), Some("Delete")).unwrap();
    delete_button.set_is_important(true);
    toolbar.add(&delete_button);


    
    //undo
    let undo_icon = Image::new_from_icon_name("edit-undo", IconSize::SmallToolbar as i32).unwrap();
    let undo_button = ToolButton::new::<Image>(Some(&undo_icon), Some("Undo")).unwrap();
    undo_button.set_is_important(true);
    toolbar.add(&undo_button);
    
    //redo
    let redo_icon = Image::new_from_icon_name("edit-redo", IconSize::SmallToolbar as i32).unwrap();
    let redo_button = ToolButton::new::<Image>(Some(&redo_icon), Some("Redo")).unwrap();
    redo_button.set_is_important(true);
    toolbar.add(&redo_button);
    toolbar
}

//paging
fn create_paging_buttons()->Toolbar{
	let toolbar = Toolbar::new().unwrap();
    let prev_icon = Image::new_from_icon_name("go-previous", IconSize::Invalid as i32).unwrap();
    let prev_button = ToolButton::new::<Image>(Some(&prev_icon), None).unwrap();
    prev_button.set_is_important(true);
    toolbar.add(&prev_button);
    
	let next_icon = Image::new_from_icon_name("go-next", IconSize::Invalid as i32).unwrap();
    let next_button = ToolButton::new::<Image>(Some(&next_icon), None).unwrap();
    next_button.set_is_important(true);
    toolbar.add(&next_button);
    
	toolbar
}
