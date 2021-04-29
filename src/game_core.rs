use rand::{prelude::SliceRandom, thread_rng};
use rustc_hash::FxHashMap;

pub static ENGLISH_DICTIONARY: &str = include_str!("../dictionaries/eng.txt");
pub static SPANISH_DICTIONARY: &str = include_str!("../dictionaries/spa.txt");

pub struct Game<'a> {
    dictionary: Vec<(&'a str, u8)>,
    letter_count: Vec<(char, u32)>,
    available_letters: Vec<char>,
}

impl Default for Game<'_> {
    fn default() -> Self {
        Self::new(ENGLISH_DICTIONARY, 10)
    }
}

impl<'a> Game<'a> {
    fn parse_dictionary(dict_str: &str) -> Vec<(&str, u8)> {
        dict_str
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| (s, s.chars().count() as u8))
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

    fn count_letters(dictionary: &[(&str, u8)]) -> Vec<(char, u32)> {
        let mut char_count = FxHashMap::default();
        char_count.reserve(27);
        for (word, _) in dictionary.iter() {
            for c in word.chars() {
                *char_count.entry(Self::remove_accents(c)).or_insert(0) += 1;
            }
        }
        char_count.into_iter().collect()
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

    pub fn new(dict_str: &'a str, size: usize) -> Self {
        let dictionary = Self::parse_dictionary(dict_str);
        let letter_count = Self::count_letters(&dictionary);
        let mut game = Self {
            dictionary,
            letter_count,
            available_letters: Vec::new()
        };
        game.generate_available_letters(size);
        game
    }

    pub fn exist(&self, word: &str) -> bool {
        //It doesn't matter if it overflows, because if it does, the word doesn't exist.
        let size = word.chars().count() as u8;
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

    pub fn find_longest_word(&self) -> (&str, u8) {
        self.dictionary
            .iter()
            .filter(|&&(_, len)| len as usize <= self.available_letters.len())
            .fold(("", 0), |(best, best_len), &(curr, curr_len)| {
                if curr_len > best_len && self.is_formable(curr) {
                    (curr, curr_len)
                } else {
                    (best, best_len)
                }
            })
    }
}
