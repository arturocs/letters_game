use crate::{
    game_core::{Game, GameResult, Language},
    style,
    ui_text::UiText,
};
use iced::{
    button, scrollable,
    text_input::{self, TextInput},
    alignment,
    Alignment, Button, Column, Container, Element, Length, Row, Sandbox,
    Scrollable, Text,
};
use std::mem::swap;

enum Screen {
    Play,
    LanguageSelection,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::LanguageSelection
    }
}

#[derive(Default)]
pub(crate) struct GameUi<'a> {
    text_input_state: text_input::State,
    text_input_value: String,
    scroll: scrollable::State,
    increment_button: button::State,
    decrement_button: button::State,
    english_button: button::State,
    spanish_button: button::State,
    play_button: button::State,
    game_core: Game<'a>,
    ui_text: UiText<'a>,
    language: Screen,
    game_messages: (&'a str, String, &'a str),
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    EnglishPressed,
    SpanishPressed,
    PlayPressed,
    Input(String),
}

impl Sandbox for GameUi<'_> {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }

    fn title(&self) -> String {
        String::from("Letters")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                let size = self.game_core.available_letters.len().saturating_add(1);
                self.game_core.generate_available_letters(size);
            }
            Message::DecrementPressed => {
                let size = self.game_core.available_letters.len().saturating_sub(1);
                self.game_core.generate_available_letters(size);
            }
            Message::Input(s) => self.text_input_value = s.to_lowercase(),
            Message::PlayPressed => {
                if !self.text_input_value.is_empty() {
                    self.play()
                }
            }
            Message::EnglishPressed => {
                self.game_core = Game::new(Language::English, 10);
                self.ui_text = UiText::english();
                self.language = Screen::Play;
            }
            Message::SpanishPressed => {
                self.game_core = Game::new(Language::Spanish, 10);
                self.ui_text = UiText::spanish();
                self.language = Screen::Play;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let screen: Element<_> = match self.language {
            Screen::Play => self.play_screen(),
            Screen::LanguageSelection => self.language_selection().into(),
        };

        Container::new(screen)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl GameUi<'_> {
    fn language_selection(&mut self) -> Column<Message> {
        Column::new()
            .max_width(720)
            .spacing(100)
            .align_items(Alignment::Center)
            .push(
                Button::new(
                    &mut self.english_button,
                    Text::new("English").horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(style::Button::Primary)
                .padding(20)
                .on_press(Message::EnglishPressed),
            )
            .push(
                Button::new(
                    &mut self.spanish_button,
                    Text::new("Español").horizontal_alignment(alignment::Horizontal::Center),
                )
                .style(style::Button::Primary)
                .padding(20)
                .on_press(Message::SpanishPressed),
            )
    }

    fn play_screen(&mut self) -> Element<Message> {
        let content: Element<Message> = Column::new()
            .max_width(720)
            .spacing(15)
            .align_items(Alignment::Center)
            .push(
                Text::new(self.ui_text.letter_game)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(80),
            )
            .push(
                Text::new(self.game_messages.0)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(40),
            )
            .push(
                Text::new(&self.game_messages.1)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(40),
            )
            .push(
                Text::new(self.game_messages.2)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(40),
            )
            .push(
                Text::new(self.ui_text.available_letters)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .push(
                Text::new(self.game_core.format_available_letters())
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(40),
            )
            .push(
                Container::new(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.increment_button,
                                Text::new("+").horizontal_alignment(alignment::Horizontal::Center),
                            )
                            .padding(15)
                            .style(style::Button::Secondary)
                            .width(iced::Length::Units(50))
                            .on_press(Message::IncrementPressed),
                        )
                        .push(
                            Text::new(&self.game_core.available_letters.len().to_string()).size(50),
                        )
                        .push(
                            Button::new(
                                &mut self.decrement_button,
                                Text::new("-").horizontal_alignment(alignment::Horizontal::Center),
                            )
                            .padding(15)
                            .style(style::Button::Secondary)
                            .width(iced::Length::Units(50))
                            .on_press(Message::DecrementPressed),
                        ),
                )
                .center_x(),
            )
            .push(
                TextInput::new(
                    &mut self.text_input_state,
                    self.ui_text.input_placeholder,
                    &self.text_input_value,
                    Message::Input,
                )
                .width(Length::Fill)
                .size(30)
                .padding(15),
            )
            .push(
                Button::new(
                    &mut self.play_button,
                    Text::new(self.ui_text.play)
                        .horizontal_alignment(alignment::Horizontal::Center)
                        .vertical_alignment(alignment::Vertical::Center),
                )
                .style(style::Button::Secondary)
                .padding(15)
                .on_press(Message::PlayPressed),
            )
            .into();

        let scrollable = Scrollable::new(&mut self.scroll)
            .push(Container::new(content).width(Length::Fill).center_x());

        Container::new(scrollable)
            .height(Length::Fill)
            .center_y()
            .into()
    }

    fn play(&mut self) {
        if self.game_messages.0.is_empty() {
            self.game_messages = match self.game_core.play(&self.text_input_value) {
                GameResult::Tie(best) => (self.ui_text.my_word, best, self.ui_text.tie),
                GameResult::YouLose(best) => (self.ui_text.my_word, best, self.ui_text.you_lose),
                GameResult::DoesntExist => (self.ui_text.doesnt_exist, String::new(), ""),
                GameResult::CantForm => (self.ui_text.cant_form, String::new(), ""),
            };
            swap(&mut self.ui_text.play, &mut self.ui_text.play_again);
        } else {
            swap(&mut self.ui_text.play, &mut self.ui_text.play_again);
            self.game_core
                .generate_available_letters(self.game_core.available_letters.len());
            self.game_messages = Default::default()
        }
    }
}
