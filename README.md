## Typewriter

A virtual typewriter built for focus and flow. As you write, your text gradually fades out of view. Built for writing, not editing. You can delete the last few words, but anything further back is uneditable.

The problem with text editors is that you can edit your text. This allows you to overthink. Sometimes you just want to write to get thoughts out of your head, you know?

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
