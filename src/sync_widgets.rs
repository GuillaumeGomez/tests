//! # Synchronizing Widgets
//!
//! You can use signals in order to synchronize the values of widgets. In this example a spin button and a horizontal scale will get interlocked.

extern crate gtk;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Enter your age");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(300, 20);

    let spin_button = gtk::SpinButton::new_with_range(0.0, 130.0, 1.0);
    let slider = gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 130.0, 1.0);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    hbox.set_homogeneous(true);
    hbox.add(&spin_button);
    hbox.add(&slider);

    window.add(&hbox);
    window.show_all();

    let slider_adj = slider.get_adjustment();
    spin_button.get_adjustment().connect_value_changed(move |adj| {
        slider_adj.set_value(adj.get_value());
    });

    let spin_button_adj = spin_button.get_adjustment();
    slider.get_adjustment().connect_value_changed(move |adj| {
        spin_button_adj.set_value(adj.get_value());
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
