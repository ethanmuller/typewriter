use typewriter::app::{App, AppResult};
use typewriter::event::{Event, EventHandler};
use typewriter::handler::handle_key_events;
use typewriter::tui::Tui;
use std::io;
use std::env;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> AppResult<()> {

    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let output_file_path = &args[1];
        let output = app.history.join("\n");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file_path)?;

        writeln!(file, "{}", output)?;
        println!("Text saved in {}", output_file_path);
    } else {
        let output = app.history.join("\n");
        println!("{}", output);
    }

    Ok(())
}
