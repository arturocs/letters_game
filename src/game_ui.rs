use iced::{
    button, scrollable, Align, Button, Column, Container, Element, HorizontalAlignment, Length,
    Sandbox, Scrollable, Text, VerticalAlignment,
};
use iced::{
    text_input::{self, TextInput},
    Row,
};

use crate::ui_text::UIText;
use crate::{
    game_core::{self, Game},
    style,
};

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
pub(crate) struct GameUI<'a> {
    value: u8,
    text_input_state: text_input::State,
    text_input_value: String,
    scroll: scrollable::State,
    increment_button: button::State,
    decrement_button: button::State,
    english_button: button::State,
    spanish_button: button::State,
    play_button: button::State,
    game_core: Game<'a>,
    ui_text: UIText<'a>,
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

impl Sandbox for GameUI<'_> {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: 10,
            scroll: scrollable::State::new(),
            ..Default::default()
        }
    }

    fn title(&self) -> String {
        String::from("Letters")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value = self.value.saturating_add(1);
                self.game_core.generate_available_letters(self.value.into());
            }
            Message::DecrementPressed => {
                self.value = self.value.saturating_sub(1);
                self.game_core.generate_available_letters(self.value.into());
            }
            Message::Input(s) => self.text_input_value = s.to_lowercase(),
            Message::PlayPressed => self.play(),
            Message::EnglishPressed => self.language = Screen::Play,
            Message::SpanishPressed => {
                self.language = Screen::Play;
                self.game_core = Game::new(game_core::SPANISH_DICTIONARY, 10);
                self.ui_text = UIText::spanish();
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let screen: Element<_> = match self.language {
            Screen::Play => self.play_screen().into(),
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

impl GameUI<'_> {
    fn language_selection(&mut self) -> Column<Message> {
        Column::new()
            .max_width(720)
            .spacing(100)
            .align_items(Align::Center)
            .push(
                Button::new(
                    &mut self.english_button,
                    Text::new("English").horizontal_alignment(HorizontalAlignment::Center),
                )
                .style(style::Button::Primary)
                .padding(20)
                .on_press(Message::EnglishPressed),
            )
            .push(
                Button::new(
                    &mut self.spanish_button,
                    Text::new("Español").horizontal_alignment(HorizontalAlignment::Center),
                )
                .style(style::Button::Primary)
                .padding(20)
                .on_press(Message::SpanishPressed),
            )
            .into()
    }

    fn play_screen(&mut self) -> Element<Message> {
        let content: Element<Message> = Column::new()
            .max_width(720)
            .spacing(15)
            .align_items(Align::Center)
            .push(
                Text::new(self.ui_text.letter_game)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(80),
            )
            .push(
                Text::new(self.game_messages.0)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(40),
            )
            .push(
                Text::new(&self.game_messages.1)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(40),
            )
            .push(
                Text::new(self.game_messages.2)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(40),
            )
            .push(
                Text::new(self.ui_text.available_letters)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(50),
            )
            .push(
                Text::new(self.game_core.get_available_letters())
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(40),
            )
            .push(
                Container::new(
                    Row::new()
                        .spacing(10)
                        .push(
                            Button::new(
                                &mut self.increment_button,
                                Text::new("+").horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .padding(15)
                            .style(style::Button::Secondary)
                            .min_width(50)
                            .on_press(Message::IncrementPressed),
                        )
                        .push(Text::new(&self.value.to_string()).size(50))
                        .push(
                            Button::new(
                                &mut self.decrement_button,
                                Text::new("-").horizontal_alignment(HorizontalAlignment::Center),
                            )
                            .padding(15)
                            .style(style::Button::Secondary)
                            .min_width(50)
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
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
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
        if self.game_messages.0 == "" {
            if self.text_input_value != "" {
                let input_len = self.text_input_value.chars().count();
                self.game_messages = if self.game_core.is_formable(&self.text_input_value) {
                    if self.game_core.exist(&self.text_input_value) {
                        let (best, best_len) = self.game_core.find_longest_word();
                        let best = best.to_string();
                        if input_len == best_len.into() {
                            (self.ui_text.my_word, best, self.ui_text.tie)
                        } else {
                            (self.ui_text.my_word, best, self.ui_text.you_lose)
                        }
                    } else {
                        (self.ui_text.doesnt_exist, String::new(), "")
                    }
                } else {
                    (self.ui_text.cant_form, String::new(), "")
                };
                std::mem::swap(&mut self.ui_text.play, &mut self.ui_text.play_again);
            }
        } else {
            std::mem::swap(&mut self.ui_text.play, &mut self.ui_text.play_again);
            self.game_core.generate_available_letters(self.value.into());
            self.game_messages = Default::default()
        }
    }
}
