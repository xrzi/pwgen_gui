use crate::generator::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, Label, CheckButton, Button};

fn gui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(700)
        .default_height(700)
        .build();
    
    let window_box = Box::new(Orientation::Vertical, 20);
    window.set_child(Some(&window_box));

    let weak_button = CheckButton::with_label("Weak");
    let avg_button = CheckButton::with_label("Average");
    let strong_button = CheckButton::with_label("Strong");
    avg_button.set_group(Some(&weak_button));
    strong_button.set_group(Some(&weak_button));
    strong_button.set_active(true);

    let password_label = Label::new(Some("Your password: "));

    window_box.append(&weak_button);
    window_box.append(&avg_button);
    window_box.append(&strong_button);
    window_box.append(&password_label);

    let button = Button::with_label("Generate password");
    button.connect_clicked(move |_| {
        let mut strenth: STRENTH = STRENTH::Strong;

        if weak_button.is_active() {strenth = STRENTH::Weak;}
        if avg_button.is_active() {strenth = STRENTH::Average;}
        else {strenth = STRENTH::Strong;}
        password_label.set_text(&genpwd(50, strenth));
    });

    window_box.append(&button);
    window.present();
}

pub fn run() {
    let app = Application::builder()
        .application_id("ID")
        .build();
    app.connect_activate(gui);
    app.run();

}