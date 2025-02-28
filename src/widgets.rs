#![warn(clippy::all, clippy::pedantic)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Status indicator colors
#[derive(Debug, Clone, Copy)]
pub enum StatusColor {
    Success,
    Warning,
    Error,
    Info,
    Custom(Color),
}

impl From<StatusColor> for Color {
    fn from(status: StatusColor) -> Self {
        match status {
            StatusColor::Success => Color::Green,
            StatusColor::Warning => Color::Yellow,
            StatusColor::Error => Color::Red,
            StatusColor::Info => Color::Blue,
            StatusColor::Custom(color) => color,
        }
    }
}

/// A widget that displays a status indicator with an optional label
#[derive(Debug, Clone)]
pub struct StatusIndicator<'a> {
    status: StatusColor,
    label: Option<&'a str>,
    style: Style,
}

impl<'a> StatusIndicator<'a> {
    /// Create a new status indicator
    #[must_use]
    pub fn new(status: StatusColor) -> Self {
        Self {
            status,
            label: None,
            style: Style::default(),
        }
    }

    /// Add a label to the status indicator
    #[must_use]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the style of the status indicator
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for StatusIndicator<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let status_color: Color = self.status.into();
        let status_dot = "●";

        let mut content = vec![Span::styled(status_dot, self.style.fg(status_color))];

        if let Some(label) = self.label {
            content.push(Span::raw(" "));
            content.push(Span::styled(label, self.style));
        }

        Paragraph::new(Line::from(content)).render(area, buf);
    }
}

/// A widget that displays a bordered box with a title and content
#[derive(Debug, Clone)]
pub struct Card<'a> {
    title: Option<&'a str>,
    content: Vec<Line<'a>>,
    style: Style,
    border_style: Style,
}

impl<'a> Card<'a> {
    /// Create a new card
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: None,
            content: Vec::new(),
            style: Style::default(),
            border_style: Style::default(),
        }
    }

    /// Set the title of the card
    #[must_use]
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Add a line of content to the card
    #[must_use]
    pub fn add_line(mut self, line: Line<'a>) -> Self {
        self.content.push(line);
        self
    }

    /// Set multiple lines of content at once
    #[must_use]
    pub fn content(mut self, content: Vec<Line<'a>>) -> Self {
        self.content = content;
        self
    }

    /// Set the style of the card content
    #[must_use]
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the style of the card border
    #[must_use]
    pub fn border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }
}

impl Widget for Card<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(self.border_style);

        let block = if let Some(title) = self.title {
            block.title(title)
        } else {
            block
        };

        let inner_area = block.inner(area);
        block.render(area, buf);

        if !self.content.is_empty() {
            Paragraph::new(Text::from(self.content))
                .style(self.style)
                .render(inner_area, buf);
        }
    }
}

impl Default for Card<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_indicator() {
        let indicator = StatusIndicator::new(StatusColor::Success).label("Running");

        // Create a test buffer and render the indicator
        let area = Rect::new(0, 0, 10, 1);
        let mut buffer = Buffer::empty(area);
        indicator.render(area, &mut buffer);

        // Check that the buffer contains the expected content
        assert_eq!(buffer[(0, 0)].symbol(), "●");
    }

    #[test]
    fn test_card() {
        let card = Card::new()
            .title("Test Card")
            .add_line(Line::from("Test content"));

        // Create a test buffer and render the card
        let area = Rect::new(0, 0, 20, 5);
        let mut buffer = Buffer::empty(area);
        card.render(area, &mut buffer);

        // Check that the buffer contains the title
        let title_found = (0..area.width).any(|x| buffer[(x, 0)].symbol() == "T");
        assert!(title_found, "Title not found in buffer");
    }
}
