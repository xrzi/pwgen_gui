use crate::generator::*;
use gtk::prelude::*;
use adw::Application;
use gtk::{ApplicationWindow, Box, Orientation, Label, CheckButton, Button, SpinButton, Entry};

fn gui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(700)
        .default_height(700)
        .build();
    
    let content = Box::new(Orientation::Vertical, 20);
    window.set_child(Some(&content));

    let weak_button = CheckButton::with_label("Weak");
    let avg_button = CheckButton::with_label("Average");
    let strong_button = CheckButton::with_label("Strong");
    avg_button.set_group(Some(&weak_button));
    strong_button.set_group(Some(&weak_button));
    strong_button.set_active(true);

    let buttons_layout = Box::new(Orientation::Horizontal, 20);
    buttons_layout.append(&weak_button);
    buttons_layout.append(&avg_button);
    buttons_layout.append(&strong_button);

    content.append(&buttons_layout);

    let len_label = Label::new(Some(&"Choose len of password"));
    let spin_button = SpinButton::with_range(1.0, 50.0, 1.0);
    let len_layout = Box::new(Orientation::Horizontal, 20);
    spin_button.set_hexpand(true);

    len_layout.append(&spin_button);
    len_layout.append(&len_label);
    content.append(&len_layout);

    let password_entry = Entry::new();
    password_entry.set_text("Your password will be here");

    content.append(&password_entry);

    let button = Button::with_label("Generate password");
    button.connect_clicked(move |_| {
        let strenth = match (weak_button.is_active(), avg_button.is_active(), strong_button.is_active()) {
            (true, false, false) => STRENTH::Weak,
            (false, true, false) => STRENTH::Average,
            _ => STRENTH::Strong
        };

        password_entry.set_text(&genpwd(spin_button.value() as i32, strenth));
    });

    content.append(&button);
    window.present();
}

pub fn run() {
    let app = Application::builder()
        .application_id("ID")
        .build();
    app.connect_activate(gui);
    app.run();
}