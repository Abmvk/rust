use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, WindowPosition};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window1 = ApplicationWindow::builder()
            .application(app)
            .title("EEN")
            .default_width(350)
            .default_height(70)
            .build();
        window1.set_position(WindowPosition::None);
        window1.move_(10, 100);

        let window2 = ApplicationWindow::builder()
            .application(app)
            .title("TWEE")
            .default_width(350)
            .default_height(70)
            .build();

        window2.set_transient_for(Some(&window1));

        window2.set_position(WindowPosition::None);
        window2.move_(320, 100);

        let button1 = Button::with_label("1");
        button1.connect_clicked(|_| {
            eprintln!("een");
        });
        window1.add(&button1);

        let button2 = Button::with_label("2");
        button2.connect_clicked(|_| {
            eprintln!("twee");
        });
        window2.add(&button2);

        window1.show_all();
        window2.show_all();
    });

    application.run();
}
