use iced::{window, Application, Settings};
use image::ImageFormat;
mod gui;
mod view;
mod component;
mod plot;
mod adapt;

fn main() -> iced::Result {
    const ICON_BYTES: &'static [u8] = include_bytes!("image/profile.png");
    let profile_icon = window::icon::Icon::from_file_data(ICON_BYTES, Some(ImageFormat::Png));
    let setting: iced::Settings<()> = Settings {
        window: window::Settings {
            size: (1400, 800),
            position: window::Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: true,
            icon: profile_icon.ok(),
        },
        ..Default::default()
    };
    gui::States::run(setting)
}
