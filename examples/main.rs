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
	let h_box = add_frame(&window);
	//add_tabs(&window);
	add_menu(&h_box);
	add_buttons(&h_box);
	add_text_fields(&h_box);
	add_checkboxes(&h_box);
	add_toggle_button(&h_box);
	add_radio_button(&h_box);
	add_switches(&h_box);
	window.show_all();
	window.set_window_position(Center);
	window.connect_delete_event(|_,_| {
		gtk::main_quit();
		Inhibit(false)
	});
	add_file_menu(&window);
	gtk::main();
}

fn add_menu(h_box: &gtk::Box){
	let menu_button = gtk::MenuButton::new().unwrap();
	let button = MenuItem::new_with_label("1. menu").unwrap();
	let button2 = MenuItem::new_with_label("menu2").unwrap();
	menu_button.add(&button);
	menu_button.add(&button2);
	h_box.add(&menu_button);
}

fn add_file_menu(window: &Window){
	println!("window: {:?}", window.get_name());
	let menu = Menu::new().unwrap();
	//window.append(&menu);
}

fn add_tabs(window:&Window){
       let close_image = gtk::Image::new_from_icon_name("window-close",
                                                         IconSize::Button as i32).unwrap();
 
	let tab = gtk::Box::new(Horizontal, 0).unwrap();
	let tab2 = gtk::Box::new(Horizontal, 0).unwrap();
	let notebook = gtk::NoteBook::new().unwrap();
	let sheet1 = Label::new("sheet1").unwrap();
	let content1 = Button::new_with_label("Content in page1").unwrap();
	let close_btn = gtk::Button::new().unwrap();
	close_btn.set_relief(ReliefStyle::None);
    close_btn.set_focus_on_click(false);
    close_btn.add(&close_image);
	tab.pack_start(&sheet1, false, false, 0);
	tab.pack_start(&close_btn, false, false, 0);
	tab.show_all();
	notebook.append_page(&content1, Some(&tab));
	window.add(&notebook);
	close_btn.connect_clicked(move|_|{
				notebook.remove_page(0);
		}
	);
}

fn add_frame(window: &Window)->gtk::Box{
	let frame = Frame::new(Some("Yep a frame")).unwrap();
 	frame.set_border_width(10);
	window.add(&frame);
	let h_box = gtk::Box::new(Vertical, 10).unwrap();
	frame.add(&h_box);
	h_box
}

fn add_buttons(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let button = Button::new_with_label(&format!("Button {}", i)).unwrap();
		button_box.add(&button);
	}
	h_box.add(&button_box);
}

fn add_text_fields(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let entry = Entry::new().unwrap();
		entry.set_placeholder("An Entry with a placeholder !");
		button_box.add(&entry);
	}
	h_box.add(&button_box);
}

fn add_checkboxes(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let check_btn = CheckButton::new_with_label(&format!("Checkbox {}", i)).unwrap();
		h_box.add(&check_btn);
	}
}

fn add_radio_button(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let check_btn = RadioButton::new_with_label(&format!("Checkbox {}", i)).unwrap();
		h_box.add(&check_btn);
	}
}


fn add_toggle_button(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let toggle_btn = gtk::ToggleButton::new_with_label(&format!("Checkbox {}", i)).unwrap();
		h_box.add(&toggle_btn);
	}
}

fn add_switches(h_box: &gtk::Box){
	let button_box = ButtonBox::new(Vertical).unwrap();
	for i in 0..3{
		let btn = gtk::Switch::new().unwrap();
		h_box.add(&btn);
	}
}
