use std::{char, error};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const MAX_LINE_LENGTH: usize = 32;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub one: String,
    pub two: String,
    pub three: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            one: String::new(),
            two: String::new(),
            three: String::new(),
        }
    }
}

fn word_wrap(text: &str) -> (String, String) {
    // If the text is shorter than or equal to MAX_LINE_LENGTH, no need to wrap
    if text.len() <= MAX_LINE_LENGTH {
        return (text.to_string(), String::new());
    }

    if let Some(last_space_index) = text[..MAX_LINE_LENGTH].rfind(' ') {
        let first_part = &text[..last_space_index];  // Include everything before the last space
        let second_part = &text[last_space_index + 1..]; // Everything after the last space
        return (first_part.to_string(), second_part.to_string());
    }

    // If no space is found within the limit, move the entire word to the second part
    // This is the case where the first word itself is longer than the max length
    let first_part = &text[..MAX_LINE_LENGTH]; // This will cut the string
    let second_part = &text[MAX_LINE_LENGTH..]; // Remaining string including any part of a word

    (first_part.to_string(), second_part.to_string())
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}


    pub fn add_character(&mut self, char: char) {
        // Append the new character to the `self.one`
        self.one.push(char);

        // Check if the length exceeds or equals the max limit
        if self.one.len() > MAX_LINE_LENGTH {
            // Perform word wrapping
            let (line_to_print, wrapped_word) = word_wrap(&self.one);

            // Shift the lines and update accordingly
            self.three = self.two.clone();
            self.two = line_to_print.trim_end().to_string();  // Trim any trailing spaces for clean line ends
            self.one = wrapped_word.trim_start().to_string(); // Trim leading spaces for clean starts
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_wrap_for_32char_lines() {
        let result = word_wrap("this is a long line that will ge");
        assert_eq!(result,
            (
                String::from("this is a long line that will ge"),
                String::from(""),
            ));
    }

    #[test]
    fn not_wrap_for_33char_lines() {
        let result = word_wrap("this is a long line that will get");
        assert_eq!(result,
            (
                String::from("this is a long line that will"),
                String::from("get"),
            ));
    }

    #[test]
    fn wraps_longer_lines() {
        let result = word_wrap("this is a long line that will get wrapped");
        assert_eq!(result,
            (
                String::from("this is a long line that will"),
                String::from("get wrapped"),
            ));
    }

    #[test]
    fn app_character_by_character() {
        let mut app = App::default();
        app.add_character('t');
        app.add_character('t');
        app.add_character('t');

        assert_eq!(app.one, "ttt");
    }

    #[test]
    fn char_by_char_32() {
        let mut app = App::default();
        app.add_character('t');
        app.add_character('h');
        app.add_character('i');
        app.add_character('s');
        app.add_character(' ');
        app.add_character('i');
        app.add_character('s');
        app.add_character(' ');
        app.add_character('a');
        app.add_character(' ');
        app.add_character('l');
        app.add_character('o');
        app.add_character('n');
        app.add_character('g');
        app.add_character(' ');
        app.add_character('l');
        app.add_character('i');
        app.add_character('n');
        app.add_character('e');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('h');
        app.add_character('a');
        app.add_character('t');
        app.add_character(' ');
        app.add_character('w');
        app.add_character('i');
        app.add_character('l');
        app.add_character('l');
        app.add_character(' ');
        app.add_character('g');
        app.add_character('e');
        assert_eq!(app.one, "this is a long line that will ge");
    }

    #[test]
    fn char_by_char_33() {
        let mut app = App::default();
        app.add_character('t');
        app.add_character('h');
        app.add_character('i');
        app.add_character('s');
        app.add_character(' ');
        app.add_character('i');
        app.add_character('s');
        app.add_character(' ');
        app.add_character('a');
        app.add_character(' ');
        app.add_character('l');
        app.add_character('o');
        app.add_character('n');
        app.add_character('g');
        app.add_character(' ');
        app.add_character('l');
        app.add_character('i');
        app.add_character('n');
        app.add_character('e');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('h');
        app.add_character('a');
        app.add_character('t');
        app.add_character(' ');
        app.add_character('w');
        app.add_character('i');
        app.add_character('l');
        app.add_character('l');
        app.add_character(' ');
        app.add_character('g');
        app.add_character('e');
        app.add_character('t');
        assert_eq!(app.two, "this is a long line that will");
        assert_eq!(app.one, "get");
    }
}
