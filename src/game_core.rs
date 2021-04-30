#![allow(dead_code)]
use crate::game_data::{ENGLISH_DICTIONARY, ENGLISH_LETTERS, SPANISH_DICTIONARY, SPANISH_LETTERS};
use rand::{prelude::SliceRandom, thread_rng};
pub enum Language {
    English,
    Spanish,
}

pub struct Game<'a> {
    dictionary: Vec<(&'a str, usize)>,
    letter_count: &'a [(char, u32)],
    available_letters: Vec<char>,
}

pub enum GameResult {
    Tie(String),
    YouLose(String),
    DoesntExist,
    CantForm,
}

impl Default for Game<'_> {
    fn default() -> Self {
        Self::new(Language::English, 10)
    }
}

impl<'a> Game<'a> {
    pub fn parse_dictionary(dict_str: &str) -> Vec<(&str, usize)> {
        dict_str
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| (s, s.chars().count()))
            .collect()
    }

    fn remove_accents(c: char) -> char {
        match c {
            'á' => 'a',
            'é' => 'e',
            'í' => 'i',
            'ó' => 'o',
            'ú' | 'ü' => 'u',
            a => a,
        }
    }

    pub fn generate_available_letters(&mut self, size: usize) {
        let rng = &mut thread_rng();
        let mut letters: Vec<_> = (0..size)
            .map(|_| self.letter_count.choose_weighted(rng, |p| p.1 as f64))
            .map(|r| r.unwrap().0)
            .collect();
        letters.sort_unstable();
        self.available_letters = letters;
    }

    pub fn set_available_letters(&mut self, letters: Vec<char>) {
        self.available_letters = letters
    }

    pub fn new(language: Language, size: usize) -> Self {
        let mut game = match language {
            Language::English => Self {
                dictionary: Self::parse_dictionary(ENGLISH_DICTIONARY),
                letter_count: &ENGLISH_LETTERS,
                available_letters: Vec::new(),
            },
            Language::Spanish => Self {
                dictionary: Self::parse_dictionary(SPANISH_DICTIONARY),
                letter_count: &SPANISH_LETTERS,
                available_letters: Vec::new(),
            },
        };
        game.generate_available_letters(size);
        game
    }

    pub fn exist(&self, word: &str) -> bool {
        let size = word.chars().count();
        self.dictionary.binary_search(&(word, size)).is_ok()
    }

    pub fn get_available_letters(&self) -> String {
        self.available_letters
            .iter()
            .map(|&c| format!("{} ", c.to_uppercase()))
            .collect()
    }

    pub fn is_formable(&self, word: &str) -> bool {
        let mut letters = self.available_letters.clone();
        word.chars().map(Self::remove_accents).all(|c| {
            if let Ok(p) = letters.binary_search(&c) {
                letters.remove(p);
                true
            } else {
                false
            }
        })
    }

    pub fn find_longest_word(&self) -> (&str, usize) {
        self.dictionary
            .iter()
            .filter(|&&(_, len)| len <= self.available_letters.len())
            .fold(("", 0), |(best, best_len), &(curr, curr_len)| {
                if curr_len > best_len && self.is_formable(curr) {
                    (curr, curr_len)
                } else {
                    (best, best_len)
                }
            })
    }

    pub fn play(&self, user_input: &str) -> GameResult {
        if !self.is_formable(user_input) {
            return GameResult::CantForm;
        }
        if !self.exist(user_input) {
            return GameResult::DoesntExist;
        }
        let (best, best_len) = self.find_longest_word();
        let input_len = user_input.chars().count();
        let best = best.to_string();
        if input_len == best_len {
            GameResult::Tie(best)
        } else {
            GameResult::YouLose(best)
        }
    }
}
