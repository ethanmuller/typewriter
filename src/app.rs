use std::{char, error, env, time::{Duration, Instant}};
use std::fs::File;
use escposify::printer::Printer;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

const MAX_LINE_LENGTH: usize = 32;

/// Application.
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub input: String,
    pub printed: String,
    pub history: String,
    pub show_hint: bool,
    pub last_keystroke: Instant,
    printer: Printer<File>,
}

impl Default for App {

    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let output_file_path = &args[1];
        let device_file = File::options()
            .append(true)
            .create(true)
            .open(output_file_path)
            .unwrap();
        let mut printer = Printer::new(device_file, None, None);
        printer.chain_hwinit().unwrap();

        Self {
            running: true,
            input: String::new(),
            printed: String::new(),
            history: String::new(),
            show_hint: true,
            last_keystroke: Instant::now(),
            printer,
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
    pub fn tick(&mut self) {
        if self.time_since_last_keystroke() > Duration::from_secs(3) {
            self.show_hint = true
        }
    }

    pub fn newline(&mut self) {
        self.history = self.printed.clone();
        // fs::write("/dev/serial0", self.printed.clone()).expect("Unable to print");
        self.printed = self.input.clone();
        self.input = String::from("");

        self.printer.chain_text(&self.printed).unwrap();
        self.printer.flush().unwrap();
    }

    pub fn add_character(&mut self, char: char) {
        // Append the new character to the `self.input`
        self.input.push(char);

        // Check if the length exceeds or equals the max limit
        if self.input.len() > MAX_LINE_LENGTH {
            // Perform word wrapping
            let (line_to_print, wrapped_word) = word_wrap(&self.input);

            // Shift the lines and update accordingly
            self.history = self.printed.clone();
            // fs::write("/dev/serial0", self.printed.clone()).expect("Unable to print");
            self.printed = line_to_print.trim_end().to_string();  // Trim any trailing spaces for clean line ends
            self.input = wrapped_word.trim_start().to_string(); // Trim leading spaces for clean starts

            self.printer.chain_text(&self.printed).unwrap();
            self.printer.flush().unwrap();
        }
    }

    pub fn time_since_last_keystroke(&self) -> std::time::Duration {
        self.last_keystroke.elapsed() // Calculate the time elapsed since the last keystroke
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.printer.text(&self.input).unwrap();
        self.printer.chain_feed(1).unwrap();
        self.printer.chain_cut(false).unwrap();
        self.printer.flush().unwrap();

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

        assert_eq!(app.input, "ttt");
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
        assert_eq!(app.printed, "this is a long line that will");
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
             
        assert_eq!(app.printed, "Welcome to your new text editor.");
        assert_eq!(app.input, "");
    }
}
