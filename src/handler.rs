use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC`
        KeyCode::Esc => {
            app.quit();
        }
        // Capture other characters and add to input buffer
        KeyCode::Char(c) => {
            app.add_character(c);  // Add character to input buffer
        }
        _ => {}
    }
    Ok(())
}
