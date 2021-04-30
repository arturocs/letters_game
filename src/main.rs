use iced::{Sandbox, Settings};
mod game_core;
mod game_data;
mod game_ui;
mod style;
mod ui_text;

fn main() -> iced::Result {
    game_ui::GameUI::run(Settings::default())
}
