#![warn(clippy::all, clippy::pedantic)]

use crossterm::{
    ExecutableCommand, event,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io;
use thiserror::Error;

// Re-export ratatui for use by applications
pub use ratatui;
// Re-export event types for use by applications
pub use crossterm::event::{Event, KeyCode, KeyModifiers};
// Re-export Widget trait
pub use ratatui::widgets::Widget;

// UI Components
mod animation;
mod layout;
pub mod widgets;

pub use animation::*;
pub use layout::*;
pub use widgets::*;

pub const GAEROS_ASCII: &str = r#"
▄▄ •  ▄▄▄· ▄▄▄ .▄▄▄        .▄▄ ·
▐█ ▀ ▪▐█ ▀█ ▀▄.▀·▀▄ █·▪     ▐█ ▀.
▄█ ▀█▄▄█▀▀█ ▐▀▀▪▄▐▀▀▄  ▄█▀▄ ▄▀▀▀█▄
▐█▄▪▐█▐█ ▪▐▌▐█▄▄▌▐█•█▌▐█▌.▐▌▐█▄▪▐█
·▀▀▀▀  ▀  ▀  ▀▀▀ .▀  ▀ ▀█▄▀▪ ▀▀▀▀
"#;

pub const KADE_ASCII: &str = r#"
 ▄ •▄  ▄▄▄· ·▄▄▄▄  ▄▄▄ .
█▌▄▌▪▐█ ▀█ ██▪ ██ ▀▄.▀·
▐▀▀▄·▄█▀▀█ ▐█· ▐█▌▐▀▀▪▄
▐█.█▌▐█ ▪▐▌██. ██ ▐█▄▄▌
·▀  ▀ ▀  ▀ ▀▀▀▀▀•  ▀▀▀
"#;

/// Error type for terminal operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Terminal error: {0}")]
    Terminal(#[from] anyhow::Error),
}

/// Terminal UI application trait
pub trait TerminalApp {
    /// Render the UI
    fn ui(&self, frame: &mut Frame);

    /// Handle terminal events
    ///
    /// # Errors
    /// Returns an error if event handling fails.
    /// Returns Ok(true) if the application should exit, Ok(false) otherwise.
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool>;
}

/// Setup the terminal for TUI application
///
/// # Errors
/// Returns an error if:
/// - Failed to enable raw mode
/// - Failed to enter alternate screen
/// - Failed to create terminal
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(io::stdout())).map_err(|e| Error::Terminal(e.into()))
}

/// Restore terminal to original state
///
/// # Errors
/// Returns an error if:
/// - Failed to disable raw mode
/// - Failed to leave alternate screen
pub fn restore_terminal() -> Result<(), Error> {
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

/// Run a terminal application
///
/// # Errors
/// Returns an error if:
/// - Failed to draw to terminal
/// - Failed to poll for events
/// - Failed to read events
/// - Application event handling failed
pub fn run_app<A: TerminalApp>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: A,
) -> Result<(), Error> {
    loop {
        terminal
            .draw(|f| app.ui(f))
            .map_err(|e| Error::Terminal(e.into()))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                if app.handle_event(Event::Key(key)).map_err(Error::Terminal)? {
                    break;
                }
            }
        }
    }
    Ok(())
}

/// Create a centered rectangle
///
/// # Arguments
/// * `percent_x` - Width of the rectangle as a percentage of the container
/// * `percent_y` - Height of the rectangle as a percentage of the container
/// * `r` - Container rectangle
#[must_use]
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
