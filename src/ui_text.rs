pub struct UIText<'a> {
    pub letter_game: &'a str,
    pub available_letters: &'a str,
    pub input_placeholder: &'a str,
    pub my_word: &'a str,
    pub tie: &'a str,
    pub you_lose: &'a str,
    pub doesnt_exist: &'a str,
    pub cant_form: &'a str,
    pub play: &'a str,
    pub play_again: &'a str,
}

impl Default for UIText<'_> {
    fn default() -> Self {
        UIText::english()
    }
}

impl UIText<'_> {
    pub fn spanish() -> Self {
        Self {
            letter_game: "Juego de Letras",
            available_letters: "Letras disponibles",
            input_placeholder: "Palabra mas larga que puedas formar",
            my_word: "Mi palabra:",
            tie: "Empate",
            you_lose: "Has perdido",
            doesnt_exist: "La palabra no existe en el diccionario",
            cant_form: "La palabra no puede ser formada con las letras disponibles",
            play: "Jugar",
            play_again: "Jugar de nuevo",
        }
    }

    pub fn english() -> Self {
        Self {
            letter_game: "Letters Game",
            available_letters: "Available letters",
            input_placeholder: "Longest word you can find using the letters above",
            my_word: "My word:",
            tie: "Tie",
            you_lose: "You have lost",
            doesnt_exist: "The word does not exist in the dictionary",
            cant_form: "The word cannot be built with the available letters",
            play: "Play",
            play_again: "Play again",
        }
    }
}
