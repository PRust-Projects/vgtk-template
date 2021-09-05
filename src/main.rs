#![windows_subsystem = "windows"]
use vgtk::ext::*;
use vgtk::lib::gdk;
use vgtk::lib::gdk_pixbuf::Pixbuf;
use vgtk::lib::gio::{ApplicationExt, ApplicationFlags, Cancellable, MemoryInputStream};
use vgtk::lib::glib::Bytes;
use vgtk::lib::gtk::*;
use vgtk::{gtk, run, Component, UpdateAction, VNode};

use appres::Resources;

const CSS_PATH: &str = "styles/styles.css";
const ICON: &[u8] = include_bytes!("../assets/icons/${ICON_NAME}");

#[derive(Clone, Debug)]
struct Model {
    resources: Resources,
}

impl Default for Model {
    fn default() -> Self {
        if let Ok(resources) = Resources::new_relative_to_executable() {
            Self {
                resources,
            }
        } else {
            panic!("Cannot access the directory where the executable is located!");
        }
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    LoadCSS,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::LoadCSS => {
                if let Ok(css) = self.resources.load_from_file(CSS_PATH) {
                    let provider = CssProvider::new();
                    provider
                        .load_from_data(css.as_bytes())
                        .expect("Failed to load CSS");
                    StyleContext::add_provider_for_screen(
                        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
                        &provider,
                        STYLE_PROVIDER_PRIORITY_APPLICATION,
                    )

                }
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        let data_stream = MemoryInputStream::from_bytes(&Bytes::from_static(ICON));
        let icon = Pixbuf::from_stream(&data_stream, None as Option<&Cancellable>).unwrap();

        gtk! {
            <Application::new_unwrap(Some("org.ces.task-runner"), ApplicationFlags::empty()) on startup=|_| Message::LoadCSS>
                <Window default_width=600 default_height=800 border_width=20 icon=Some(icon) on destroy=|_| Message::Exit>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
