use iced::{Sandbox, Settings};
mod game_core;
mod ui_text;
mod style;
mod game_ui;


pub fn main() -> iced::Result {
    game_ui::GameUI::run(Settings::default())
}
