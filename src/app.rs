use crate::generator::*;
use gtk::prelude::*;
use adw::Application;
use gtk::{ApplicationWindow, Box, Orientation, Label, CheckButton, Button, SpinButton};

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

    let spin_button = SpinButton::with_range(1.0, 50.0, 1.0);

    let password_label = Label::new(Some("Your password: "));

    window_box.append(&weak_button);
    window_box.append(&avg_button);
    window_box.append(&strong_button);
    window_box.append(&spin_button);
    
    window_box.append(&password_label);

    let button = Button::with_label("Generate password");
    button.connect_clicked(move |_| {
        let strenth = match (weak_button.is_active(), avg_button.is_active(), strong_button.is_active()) {
            (true, false, false) => STRENTH::Weak,
            (false, true, false) => STRENTH::Average,
            _ => STRENTH::Strong
        };

        password_label.set_text(&genpwd(spin_button.value() as i32, strenth));
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