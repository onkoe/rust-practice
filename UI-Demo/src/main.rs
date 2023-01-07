use std::f32::consts::E;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use rand::random;

fn main() {
    let app = Application::builder()
        .application_id("coop.omni.uidemo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("Click 'Flip Coin' to begin!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("Flip Coin")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("UI Demo")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| coin_toss(&label));

    window.show();
}

fn coin_toss(label: &Label) {
    let mut heads_counter: i32 = 0;
    let mut tails_counter: i32 = 0;

    if random() {
        heads_counter += 1;
        label.set_text("Heads" += heads_counter);
    } else {
        tails_counter+= 1;
        label.set_text("Tails" += tails_counter);
    }
    
}
