use std::time::Instant;

use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Backspace => {
            if key_event.modifiers.contains(KeyModifiers::ALT) {
                app.delete_last_word();
            } else {
                app.delete_last_character();
            }
            app.show_hint = false;
            app.last_keystroke = Instant::now();
        }
        KeyCode::Enter => {
            app.newline();
            app.show_hint = false;
            app.last_keystroke = Instant::now();
        }
        KeyCode::Char(c) => {
            if c == 'u' && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                app.clear_input();
                return Ok(())
            }
            if c == 'h' && key_event.modifiers.contains(KeyModifiers::CONTROL) {
                app.delete_last_character();
                return Ok(())
            }
            app.add_character(c);  // Add character to input buffer
            app.show_hint = false;
            app.last_keystroke = Instant::now();
        }
        _ => {}
    }
    Ok(())
}
