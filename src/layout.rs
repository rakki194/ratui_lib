#![warn(clippy::all, clippy::pedantic)]

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Creates a responsive grid layout based on available space and number of items
pub struct ResponsiveGrid {
    /// Minimum width for each column
    pub min_column_width: u16,
    /// Maximum number of columns
    pub max_columns: u16,
}

impl ResponsiveGrid {
    /// Create a new responsive grid with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            min_column_width: 30,
            max_columns: 4,
        }
    }

    /// Create a new responsive grid with custom settings
    #[must_use]
    pub fn with_settings(min_column_width: u16, max_columns: u16) -> Self {
        Self {
            min_column_width,
            max_columns,
        }
    }

    /// Calculate optimal number of columns based on available width
    fn calculate_columns(&self, width: u16) -> u16 {
        // Always ensure at least one column, even if narrower than min_column_width
        if width == 0 {
            1
        } else {
            (width / self.min_column_width).max(1).min(self.max_columns)
        }
    }

    /// Split area into a grid of cells based on number of items
    /// Returns a vector of Rects representing each cell
    #[must_use]
    /// We suppress these Clippy warnings because:
    /// - `cast_possible_truncation`: We're converting f64 to usize for row count, but we've already
    ///   handled edge cases (negative values, NaN, and values > `u32::MAX`) explicitly above.
    /// - `cast_sign_loss`: The `row_count` is guaranteed to be non-negative due to our checks,
    ///   so the sign loss in the conversion to usize is intentional and safe.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn split(&self, area: Rect, item_count: usize) -> Vec<Rect> {
        if item_count == 0 {
            return vec![];
        }

        let optimal_columns = self.calculate_columns(area.width);
        // Note: We no longer need the optimal_columns == 0 check since calculate_columns
        // always returns at least 1

        // Use f64 for better precision and handle the conversion explicitly
        let rows = {
            let cols = f64::from(optimal_columns);
            let count = f64::from(u32::try_from(item_count).unwrap_or(u32::MAX));
            let row_count = (count / cols).ceil();
            // Since we're dealing with layout, negative values don't make sense
            // and we want to clamp to reasonable values
            if row_count.is_sign_negative() || row_count.is_nan() {
                1_usize
            } else if row_count > f64::from(u32::MAX) {
                // Cap the maximum number of rows to prevent excessive memory usage
                1024
            } else {
                (row_count as usize).min(1024) // Cap at 1024 rows maximum
            }
        };

        // Create row constraints with safe conversion
        let rows_u32 = u32::try_from(rows).unwrap_or(u32::MAX);
        let row_constraints = vec![Constraint::Ratio(1, rows_u32); rows];
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(row_constraints)
            .split(area);

        // Create column constraints
        let col_constraints =
            vec![Constraint::Ratio(1, u32::from(optimal_columns)); optimal_columns as usize];

        let mut cells = Vec::with_capacity(rows * optimal_columns as usize);
        for (row_idx, row) in vertical_chunks.iter().enumerate() {
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(col_constraints.clone())
                .split(*row);

            for col_idx in 0..optimal_columns as usize {
                let item_idx = row_idx * optimal_columns as usize + col_idx;
                if item_idx < item_count {
                    cells.push(horizontal_chunks[col_idx]);
                }
            }
        }

        cells
    }
}

impl Default for ResponsiveGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a centered rectangle with specified dimensions
#[must_use]
pub fn centered_rect_with_size(width: u16, height: u16, container: Rect) -> Rect {
    let vertical_padding = container.height.saturating_sub(height) / 2;
    let horizontal_padding = container.width.saturating_sub(width) / 2;

    Rect::new(
        container.x + horizontal_padding,
        container.y + vertical_padding,
        width.min(container.width),
        height.min(container.height),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_responsive_grid() {
        let grid = ResponsiveGrid::new();
        let area = Rect::new(0, 0, 100, 100);

        // Test with different item counts
        assert_eq!(grid.split(area, 0).len(), 0);
        assert_eq!(grid.split(area, 1).len(), 1);
        assert_eq!(grid.split(area, 4).len(), 4);

        // Test column calculation
        assert_eq!(grid.calculate_columns(0), 1, "Should have at least 1 column for zero width");
        assert_eq!(grid.calculate_columns(29), 1, "Should have at least 1 column for narrow width");
        assert_eq!(grid.calculate_columns(30), 1);
        assert_eq!(grid.calculate_columns(60), 2);
        assert_eq!(grid.calculate_columns(200), 4); // Max columns
    }

    #[test]
    fn test_grid_row_calculations() {
        let grid = ResponsiveGrid::new();
        // Create a wide area that allows max columns (4)
        let area = Rect::new(0, 0, 200, 100);

        // Test various item counts and verify row calculations
        let test_cases = [
            // (item_count, expected_rows, expected_columns)
            (0, 0, 0),    // Empty grid
            (1, 1, 1),    // Single item
            (4, 1, 4),    // Full single row
            (5, 2, 4),    // One full row + one item
            (8, 2, 4),    // Two full rows
            (9, 3, 4),    // Two full rows + one item
            (16, 4, 4),   // Four full rows
        ];

        for (items, expected_rows, expected_cols) in test_cases {
            let cells = grid.split(area, items);
            assert_eq!(cells.len(), items, "Expected {} cells for {} items", items, items);
            
            if items > 0 {
                // Verify row count by checking the y coordinates
                let unique_rows = cells.iter()
                    .map(|r| r.y)
                    .collect::<std::collections::HashSet<_>>()
                    .len();
                assert_eq!(unique_rows, expected_rows, 
                    "Expected {} rows for {} items", expected_rows, items);

                // Verify column count by checking items in first row
                let first_row_items = cells.iter()
                    .filter(|r| r.y == cells[0].y)
                    .count();
                assert_eq!(first_row_items, expected_cols.min(items), 
                    "Expected {} columns in first row for {} items", 
                    expected_cols.min(items), items);
            }
        }
    }

    #[test]
    fn test_grid_edge_cases() {
        let grid = ResponsiveGrid::new();
        let area = Rect::new(0, 0, 200, 100);

        // Test with a large but reasonable number of items
        let large_count = 10_000;  // Large enough to test overflow handling but won't hang
        let cells = grid.split(area, large_count);
        assert!(!cells.is_empty(), "Should handle large item counts");
        assert!(cells.len() <= large_count, "Should not exceed requested count");
        
        // Test with area width smaller than minimum column width
        let narrow_area = Rect::new(0, 0, 20, 100);
        let cells = grid.split(narrow_area, 4);
        assert_eq!(cells.len(), 4, "Should still create cells in narrow area");
        
        // Test with zero height area
        let zero_height_area = Rect::new(0, 0, 200, 0);
        let cells = grid.split(zero_height_area, 4);
        assert_eq!(cells.len(), 4, "Should handle zero height areas");
        assert!(cells.iter().all(|r| r.height == 0), 
            "All cells should have zero height");

        // Test with zero width area
        let zero_width_area = Rect::new(0, 0, 0, 100);
        let cells = grid.split(zero_width_area, 4);
        assert_eq!(cells.len(), 4, "Should handle zero width areas");
    }

    #[test]
    fn test_centered_rect() {
        let container = Rect::new(0, 0, 100, 100);
        let centered = centered_rect_with_size(20, 20, container);

        assert_eq!(centered.x, 40);
        assert_eq!(centered.y, 40);
        assert_eq!(centered.width, 20);
        assert_eq!(centered.height, 20);
    }
}
