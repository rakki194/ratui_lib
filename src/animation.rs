#![warn(clippy::all, clippy::pedantic)]

use ratatui::prelude::*;
use std::time::{Duration, Instant};

/// A trait for animated patterns that can be rendered to a buffer
pub trait Pattern {
    /// Update the pattern state
    fn update(&mut self, delta: Duration);

    /// Render the pattern to a buffer
    fn render(&self, area: Rect, buf: &mut Buffer);
}

/// A simple animation timer that tracks time and delta time
#[derive(Debug)]
pub struct AnimationTimer {
    start_time: Instant,
    last_update: Instant,
}

impl AnimationTimer {
    /// Create a new animation timer
    #[must_use]
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
        }
    }

    /// Get the time elapsed since the animation started
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get the time elapsed since the last update and update the last update time
    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update);
        self.last_update = now;
        delta
    }

    /// Reset the animation timer
    pub fn reset(&mut self) {
        let now = Instant::now();
        self.start_time = now;
        self.last_update = now;
    }
}

impl Default for AnimationTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// A wave pattern that creates animated waves using ASCII characters
pub struct WavePattern {
    time: f64,
    speed: f64,
    chars: Vec<char>,
}

impl WavePattern {
    /// Create a new wave pattern with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 2.0,
            chars: vec!['░', '▒', '▓', '█'],
        }
    }

    /// Set the animation speed
    #[must_use]
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Set the characters used for the wave pattern
    #[must_use]
    pub fn chars(mut self, chars: Vec<char>) -> Self {
        self.chars = chars;
        self
    }
}

impl Pattern for WavePattern {
    fn update(&mut self, delta: Duration) {
        self.time += delta.as_secs_f64() * self.speed;
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let wave = ((f64::from(x) * 0.2 - self.time * 2.0).sin() * 5.0)
                    + ((f64::from(y) * 0.1 + self.time).cos() * 3.0)
                    + ((f64::from(x) + f64::from(y)) * 0.1 - self.time * 1.5).sin() * 2.0;

                let char_index = {
                    let normalized = (wave + 10.0)
                        * (f64::from(u32::try_from(self.chars.len()).unwrap_or(1)) / 20.0);
                    let index = normalized.abs().floor();
                    if index.is_nan() {
                        0
                    } else {
                        (index as usize) % self.chars.len()
                    }
                };
                buf[(x, y)].set_char(self.chars[char_index]);
            }
        }
    }
}

impl Default for WavePattern {
    fn default() -> Self {
        Self::new()
    }
}

/// A rain pattern that creates animated rain drops
pub struct RainPattern {
    time: f64,
    speed: f64,
    drops: Vec<(f64, f64)>, // x, y positions
    chars: Vec<char>,
    drop_chance: f64,
}

impl RainPattern {
    /// Create a new rain pattern with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 1.0,
            drops: Vec::new(),
            chars: vec!['│', '╵', '·'],
            drop_chance: 0.3,
        }
    }

    /// Set the animation speed
    #[must_use]
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Set the characters used for the rain pattern
    #[must_use]
    pub fn chars(mut self, chars: Vec<char>) -> Self {
        self.chars = chars;
        self
    }

    /// Set the chance of a new drop being created (0.0 to 1.0)
    #[must_use]
    pub fn drop_chance(mut self, chance: f64) -> Self {
        self.drop_chance = chance.clamp(0.0, 1.0);
        self
    }

    /// Add a drop at a specific position (for testing)
    #[cfg(test)]
    pub fn add_drop(&mut self, x: f64) {
        self.drops.push((x, 0.0));
    }
}

impl Pattern for RainPattern {
    fn update(&mut self, delta: Duration) {
        self.time += delta.as_secs_f64() * self.speed;

        // Add new drops
        if rand::random::<f64>() < self.drop_chance {
            let x = rand::random::<f64>();
            self.drops.push((x, 0.0));
        }

        // Update existing drops
        self.drops.retain_mut(|(_x, y)| {
            *y += delta.as_secs_f64() * self.speed * 10.0;
            *y < 1.0 // Remove drops that fall off the bottom
        });
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn render(&self, area: Rect, buf: &mut Buffer) {
        for (x, y) in &self.drops {
            let screen_x = {
                let pos = (x * f64::from(area.width)).clamp(0.0, f64::from(area.width - 1));
                if pos.is_nan() {
                    area.left()
                } else {
                    area.left() + pos.floor() as u16
                }
            };
            let screen_y = {
                let pos = (y * f64::from(area.height)).clamp(0.0, f64::from(area.height - 1));
                if pos.is_nan() {
                    area.top()
                } else {
                    area.top() + pos.floor() as u16
                }
            };

            if screen_y < area.bottom() {
                let char_index = usize::from(screen_y == area.bottom() - 1);
                buf[(screen_x, screen_y)].set_char(self.chars[char_index]);

                // Add trail
                if screen_y > area.top() {
                    buf[(screen_x, screen_y - 1)].set_char(self.chars[2]);
                }
            }
        }
    }
}

impl Default for RainPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_timer() {
        let mut timer = AnimationTimer::new();
        assert!(timer.elapsed().as_secs() == 0);

        std::thread::sleep(Duration::from_millis(10));
        let delta = timer.tick();
        assert!(delta.as_millis() >= 10);

        timer.reset();
        assert!(timer.elapsed().as_secs() == 0);
    }

    #[test]
    fn test_wave_pattern() {
        let mut pattern = WavePattern::new();
        let area = Rect::new(0, 0, 10, 10);
        let mut buffer = Buffer::empty(area);

        pattern.update(Duration::from_secs_f64(0.1));
        pattern.render(area, &mut buffer);

        // Check that the buffer is not empty
        let has_content =
            (0..area.height).any(|y| (0..area.width).any(|x| buffer[(x, y)].symbol() != " "));
        assert!(has_content, "Buffer should contain wave pattern");
    }

    #[test]
    fn test_rain_pattern() {
        let mut pattern = RainPattern::new().speed(5.0); // Increase speed for testing

        // Add a drop at a position we know will be visible
        let x = 0.5; // This should map to the middle of the area
        let y = 0.2; // Start the drop partway down so it's immediately visible
        pattern.drops.push((x, y));

        let area = Rect::new(0, 0, 10, 10);
        let mut buffer = Buffer::empty(area);

        // No need to update since we placed the drop where it will be visible
        pattern.render(area, &mut buffer);

        // Debug print the buffer contents and check for content in one pass
        let mut has_content = false;
        for y in 0..area.height {
            for x in 0..area.width {
                let symbol = buffer[(x, y)].symbol();
                if symbol != " " {
                    has_content = true;
                    println!("Found symbol '{symbol}' at ({x}, {y})");
                }
            }
        }

        assert!(has_content, "Buffer should contain rain drops");
    }
}
