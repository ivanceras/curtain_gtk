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
     let close_image1 = gtk::Image::new_from_icon_name("window-close",
                                                         IconSize::Button as i32).unwrap();
 
	let tab1 = gtk::Box::new(Horizontal, 0).unwrap();
	let sheet1 = Label::new("sheet1").unwrap();
	let content1 = Button::new_with_label("Content in page1").unwrap();
	let close_btn1 = gtk::Button::new().unwrap();
	
    close_btn1.set_relief(ReliefStyle::None);
    close_btn1.set_focus_on_click(false);
    close_btn1.add(&close_image1);
	tab1.pack_start(&sheet1, false, false, 0);
	tab1.pack_start(&close_btn1, false, false, 0);
	tab1.show_all();
	
    notebook.append_page(&content1, Some(&tab1));


     let close_image2 = gtk::Image::new_from_icon_name("window-close",
                                                         IconSize::Button as i32).unwrap();
 
	let tab2 = gtk::Box::new(Horizontal, 0).unwrap();
	let sheet2 = Label::new("  2nd here!....  ").unwrap();
	let content2 = Button::new_with_label("Content in page2").unwrap();
	let close_btn2 = gtk::Button::new().unwrap();
	
    close_btn2.set_relief(ReliefStyle::None);
    close_btn2.set_focus_on_click(false);
    close_btn2.add(&close_image2);
	tab2.pack_start(&sheet2, false, false, 0);
	tab2.pack_start(&close_btn2, false, false, 0);
	tab2.show_all();
	
    notebook.append_page(&content2, Some(&tab2));
    window.add(&notebook);
	close_btn1.connect_clicked(move|_|{
				notebook.remove_page(0);
		}
	);

	close_btn2.connect_clicked(move|_|{
                println!("close button2 clicked...!");
				notebook.remove_page(1);
		}
	);
}

