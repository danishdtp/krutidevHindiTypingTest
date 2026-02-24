#![windows_subsystem = "windows"]

mod app;
mod db;
mod models;

static FONT_DATA: &[u8] = include_bytes!("../Kruti Dev 010 Regular.ttf");

fn main() {
    let mut settings = iced::Settings::default();
    settings.default_text_size = iced::Pixels(16.0);
    settings.fonts = vec![std::borrow::Cow::Borrowed(FONT_DATA)];

    iced::application("Kruti Dev Typing", app::App::update, app::App::view)
        .settings(settings)
        .run()
        .expect("Error running application")
}
