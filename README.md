# ratui_lib

A high-level wrapper around [ratatui](https://github.com/ratatui-org/ratatui) providing reusable components and utilities for building terminal user interfaces in Rust.

## Features

- 🎨 **Rich Widget Library**: Pre-built, customizable widgets for common UI patterns
- 📐 **Layout Utilities**: Helper functions for creating responsive terminal layouts
- 🎬 **Animation Support**: Built-in animation system for creating dynamic UIs
- 🛠️ **Application Framework**: Simple trait-based framework for building TUI applications
- ⚡ **Event Handling**: Streamlined event handling with crossterm integration
- 🔧 **Error Handling**: Comprehensive error types and handling utilities

## Installation

```bash
cargo add ratui_lib
```

## Quick Start

```rust
use ratui_lib::{TerminalApp, Frame, Event, setup_terminal, restore_terminal, run_app};
use anyhow::Result;

struct MyApp {
    counter: u32,
}

impl TerminalApp for MyApp {
    fn ui(&self, frame: &mut Frame) {
        // Your UI rendering code here
    }

    fn handle_event(&mut self, event: Event) -> Result<bool> {
        // Your event handling code here
        Ok(false)
    }
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let app = MyApp { counter: 0 };
    
    let result = run_app(&mut terminal, app);
    restore_terminal()?;
    
    result.map_err(Into::into)
}
```

## Core Components

### Terminal Setup and Management

```rust
let mut terminal = setup_terminal()?;
// ... your application code ...
restore_terminal()?;
```

### Application Framework

The `TerminalApp` trait provides the foundation for building TUI applications:

```rust
pub trait TerminalApp {
    fn ui(&self, frame: &mut Frame);
    fn handle_event(&mut self, event: Event) -> anyhow::Result<bool>;
}
```

### Layout Utilities

Create centered rectangles and complex layouts:

```rust
let centered = ratui_lib::centered_rect(80, 60, frame.size());
```

### Widgets

The library includes various pre-built widgets in the `widgets` module:

- Animated components
- Custom styled widgets
- Layout-aware components

### Animation System

Built-in support for creating animated UI components:

```rust
use ratui_lib::animation::{Animation, AnimationState};
// ... animation implementation ...
```

## Examples

Check out the `examples/` directory for complete working examples:

- Basic application setup
- Widget demonstrations
- Animation examples
- Layout patterns

## Error Handling

The library provides its own error type that wraps common terminal and I/O errors:

```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Terminal error: {0}")]
    Terminal(#[from] anyhow::Error),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built on top of the excellent [ratatui](https://github.com/ratatui-org/ratatui) library
- Uses [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
