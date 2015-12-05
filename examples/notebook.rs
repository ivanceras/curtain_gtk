extern crate gtk;
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
	let menu = Menu::new().unwrap();
	let item = MenuItem::new_with_label("SuperPower").unwrap();
	add_tabs(&window);
    window.set_size_request(400,600);
	window.show_all();
	window.set_window_position(Center);
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
	gtk::main();
}


fn add_tabs(window:&Window){
	let notebook = gtk::NoteBook::new().unwrap();
    notebook.set_scrollable(true);
    for i in 0..8{
        let close_image = gtk::Image::new_from_icon_name("window-close",
                                                             IconSize::Button as i32).unwrap();
     
        let tab = gtk::Box::new(Horizontal, 0).unwrap();
        let sheet = Label::new(&format!("sheet {}", i)).unwrap();
        let content = Button::new_with_label(&format!("Content in page {}", i)).unwrap();
        let close_btn = gtk::Button::new().unwrap();
        
        close_btn.set_relief(ReliefStyle::None);
        close_btn.set_focus_on_click(false);
        close_btn.add(&close_image);
        tab.pack_start(&sheet, false, false, 0);
        tab.pack_start(&close_btn, false, false, 0);
        tab.show_all();
        
        notebook.append_page(&content, Some(&tab));
        notebook.set_tab_reorderable(&content, true);
        notebook.set_tab_detachable(&content, true);

         let close_image2 = gtk::Image::new_from_icon_name("window-close",
                                                             IconSize::Button as i32).unwrap();
     
        window.add(&notebook);
        let notebook_clone = notebook.clone();
        close_btn.connect_clicked(move|_|{
                    let index = notebook_clone.page_num(&content).unwrap();
                    notebook_clone.remove_page(index as i32);
            }
        );
    }
}

