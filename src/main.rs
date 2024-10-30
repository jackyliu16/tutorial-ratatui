use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};
use std::io;

// 1. in alternate screen
// 2. enable raw mode to turn off input and output
// 3. app create backend and terminal to print characters to screen
fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = run(terminal);
    // ratatui.restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press q to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        // handle keyboard events
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}
