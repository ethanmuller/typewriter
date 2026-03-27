## Typewriter

The problem with text editors is that you can edit your text. This allows you to overthink. Sometimes you just want to write to get thoughts out of your head, you know?

Typewriter is a virtual typewriter built for focus and flow. As you write, your text gradually fades out of view. You can delete the last few words you wrote, but anything further back is uneditable. Remember: this is a tool for writing, not editing.

If you were to download this project and `cargo build` it, you would receive a fresh new `typewriter` binary in `target/release`. Launch it and you will be presented with a typewriter interface inside of your terminal. Press `esc` when you're done.

By default, when you exit, your writing will be spit back to you. 

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```
