mod app;
mod db;
mod models;

fn main() {
    let mut settings = iced::Settings::default();
    settings.default_text_size = iced::Pixels(16.0);

    let font_data = std::fs::read("Kruti Dev 010 Regular.ttf").expect("Failed to read font file");
    settings.fonts = vec![std::borrow::Cow::Owned(font_data)];

    iced::application("Kruti Dev Typing", app::App::update, app::App::view)
        .settings(settings)
        .run()
        .expect("Error running application");
}
