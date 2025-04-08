use std::{char, error, time::{Duration, Instant}};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const MAX_LINE_LENGTH: usize = 32;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: String,
    pub history: Vec<String>,
    pub disable_hints: bool,
    pub display_hints: bool,
    pub last_keystroke: Instant,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            input: String::new(),
            history: Vec::new(),
            disable_hints: false,
            display_hints: true,
            last_keystroke: Instant::now(),
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
    pub fn new(disable_hints: bool) -> Self {
        Self {
            disable_hints,
            display_hints: !disable_hints,
            ..Default::default()
        }
    }

    pub fn tick(&mut self) {
        if !self.disable_hints && self.time_since_last_keystroke() > Duration::from_secs(3) {
            self.display_hints = true
        }
    }

    pub fn newline(&mut self) {
        self.history.push(self.input.clone());
        self.input = String::from("");
    }

    pub fn add_character(&mut self, char: char) {
        // Append the new character to the `self.input`
        self.input.push(char);

        // Check if the length exceeds or equals the max limit
        if self.input.len() > MAX_LINE_LENGTH {
            let (latest_line, wrapped_word) = word_wrap(&self.input);
            self.history.push(latest_line);
            self.input = wrapped_word.trim_start().to_string(); // Trim leading spaces for clean starts
        }


    }

    pub fn delete_last_character(&mut self) {
        if !self.input.is_empty() {
            self.input = self.input[..self.input.len() - 1].to_string();
        }
    }

    pub fn clear_input(&mut self) {
        if !self.input.is_empty() {
            self.input = "".to_string();
        }
    }

    pub fn delete_last_word(&mut self) {
        let trimmed = self.input.trim_end();

        self.input = match trimmed.rfind(char::is_whitespace) {
            Some(pos) => trimmed[..pos + 1].to_string(),
            None => "".to_string(),
        }
    }

    pub fn time_since_last_keystroke(&self) -> std::time::Duration {
        self.last_keystroke.elapsed() // Calculate the time elapsed since the last keystroke
    }

    pub fn quit(&mut self) {
        if !self.input.is_empty() {
            self.history.push(self.input.clone());
        }

        self.running = false;
    }

    pub fn latest_line(&self) -> String {
        self.history.get(self.history.len().wrapping_sub(1)).cloned().unwrap_or_default()
    }

    pub fn second_latest_line(&self) -> String {
        self.history.get(self.history.len().wrapping_sub(2)).cloned().unwrap_or_default()
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

        assert_eq!(app.input, "ttt");
    }

    #[test]
    fn can_delete_char() {
        let mut app = App::default();
        app.add_character('a');
        app.add_character('b');
        app.add_character('c');
        app.delete_last_character();
    }

    #[test]
    fn can_delete_word() {
        let mut app = App::default();
        app.add_character('o');
        app.add_character('n');
        app.add_character('e');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('w');
        app.add_character('o');
        app.delete_last_word();
        assert_eq!(app.input, "one ");
    }

    #[test]
    fn can_delete_word_past_space() {
        let mut app = App::default();
        app.add_character('o');
        app.add_character('n');
        app.add_character('e');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('w');
        app.add_character('o');
        app.add_character(' ');
        app.delete_last_word();
        assert_eq!(app.input, "one ");
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
        assert_eq!(app.input, "this is a long line that will ge");
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
        assert_eq!(app.latest_line(), "this is a long line that will");
        assert_eq!(app.input, "get");
    }

    #[ignore]
    #[test]
    fn space_after_full_line() {
        let mut app = App::default();
        app.add_character('W');
        app.add_character('e');
        app.add_character('l');
        app.add_character('c');
        app.add_character('o');
        app.add_character('m');
        app.add_character('e');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('o');
        app.add_character(' ');
        app.add_character('y');
        app.add_character('o');
        app.add_character('u');
        app.add_character('r');
        app.add_character(' ');
        app.add_character('n');
        app.add_character('e');
        app.add_character('w');
        app.add_character(' ');
        app.add_character('t');
        app.add_character('e');
        app.add_character('x');
        app.add_character('t');
        app.add_character(' ');
        app.add_character('e');
        app.add_character('d');
        app.add_character('i');
        app.add_character('t');
        app.add_character('o');
        app.add_character('r');
        app.add_character('.');
        app.add_character(' ');
             
        assert_eq!(app.latest_line(), "Welcome to your new text editor.");
        assert_eq!(app.input, "");
    }

}
