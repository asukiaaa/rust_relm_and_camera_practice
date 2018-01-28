extern crate futures_glib;
extern crate gtk;
extern crate gdk_pixbuf;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate rscam;

use futures_glib::Timeout;
use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    Image,
    ImageExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::Vertical;
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use relm::{Relm, Update, Widget};
use rscam::{Camera, Config};
use std::time::Duration;

struct Model {
    relm: Relm<Win>,
    started_camera: Option<Camera>,
}

#[derive(Msg)]
enum Msg {
    ToggleCamera,
    Quit,
    UpdateCameraImage(()),
}

// Create the structure that holds the widgets used in the view.
struct Win {
    state_label: Label,
    image: Image,
    model: Model,
    window: Window,
}

fn jpeg_vec_to_pixbuf(jpeg_vec: &[u8]) -> Pixbuf {
    let loader = PixbufLoader::new();
    loader.loader_write(jpeg_vec).unwrap();
    loader.close().unwrap();
    loader.get_pixbuf().unwrap()
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            started_camera: None,
        }
    }

    fn update(&mut self, event: Msg) {
        let label = &self.state_label;
        let image = &self.image;

        match event {
            Msg::ToggleCamera => {
                match self.model.started_camera {
                    Some(_) => {
                        self.model.started_camera = None;
                        label.set_text("closed camera");
                    },
                    None => {
                        let mut camera = Camera::new("/dev/video0").unwrap();
                        camera.start(&Config {
                            interval: (1, 30), // 30 fps.
                            resolution: (640, 360),
                            format: b"MJPG",
                            ..Default::default()
                        }).unwrap();
                        self.model.started_camera = Some(camera);
                        label.set_text("opened camera");

                        let stream = Timeout::new(Duration::from_millis(10));
                        self.model.relm.connect_exec_ignore_err(stream, Msg::UpdateCameraImage);
                    },
                }
            },
            Msg::UpdateCameraImage(()) => {
                match self.model.started_camera {
                    Some(ref camera) => {
                        let frame = camera.capture().unwrap();
                        let pixbuf = jpeg_vec_to_pixbuf(&frame[..]);
                        image.set_from_pixbuf(&pixbuf);
                        while gtk::events_pending() {
                            gtk::main_iteration_do(true);
                        }

                        let stream = Timeout::new(Duration::from_millis(10));
                        self.model.relm.connect_exec_ignore_err(stream, Msg::UpdateCameraImage);
                    },
                    None => return,
                }
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(Vertical, 0);

        let state_label = Label::new("wait to toggle camera");
        vbox.add(&state_label);

        let toggle_camera_button = Button::new_with_label("toggle camera");
        vbox.add(&toggle_camera_button);

        let image = Image::new();
        vbox.add(&image);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);

        window.show_all();

        // Send the message Increment when the button is clicked.
        connect!(relm, toggle_camera_button, connect_clicked(_), Msg::ToggleCamera);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        Win {
            state_label: state_label,
            image: image,
            model,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
