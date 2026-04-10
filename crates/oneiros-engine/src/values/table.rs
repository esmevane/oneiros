//! Simple table formatter for columnar CLI output.
//!
//! Columns auto-size to content width and are addressable by key
//! for future column selection and configuration.

use crate::*;

/// A column definition for a table.
pub struct Column {
    /// Identifier — for selection, config, API references.
    pub key: String,
    /// Display header.
    pub header: String,
    /// Right-align values (useful for numbers).
    pub right_align: bool,
    /// Maximum display width — longer values are truncated with "…".
    pub max_width: Option<usize>,
}

impl Column {
    /// A column with matching key and header.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            key: name.to_lowercase(),
            header: name,
            right_align: false,
            max_width: None,
        }
    }

    /// A column with an explicit key distinct from the header.
    pub fn key(key: impl Into<String>, header: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            header: header.into(),
            right_align: false,
            max_width: None,
        }
    }

    /// Right-aligned column (numbers, counts).
    pub fn right(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            key: name.to_lowercase(),
            header: name,
            right_align: true,
            max_width: None,
        }
    }

    /// Set a maximum display width for this column.
    pub fn max(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }
}

/// A simple auto-sizing table.
///
/// Columns are addressable by key for future selection support.
/// Rendering uses `Paint` styles from the palette — `anstream`
/// handles stripping ANSI codes when color is disabled.
pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            rows: Vec::new(),
        }
    }

    /// Add a row (builder style).
    pub fn row(mut self, cells: Vec<impl Into<String>>) -> Self {
        self.rows.push(cells.into_iter().map(Into::into).collect());
        self
    }

    /// Add a row (mutating).
    pub fn push_row(&mut self, cells: Vec<impl Into<String>>) {
        self.rows.push(cells.into_iter().map(Into::into).collect());
    }

    /// Whether the table has any data rows.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Truncate a string to fit within a maximum width, appending "…" if cut.
    fn truncate(s: &str, max: usize) -> String {
        if s.len() <= max {
            s.to_string()
        } else if max <= 1 {
            "…".to_string()
        } else {
            let mut end = max - 1;
            while !s.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            format!("{}…", &s[..end])
        }
    }

    /// Compute column widths from headers and data, respecting max_width.
    fn widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> = self.columns.iter().map(|c| c.header.len()).collect();

        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        for (i, col) in self.columns.iter().enumerate() {
            if let Some(max) = col.max_width {
                widths[i] = widths[i].min(max);
            }
        }

        widths
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.columns.is_empty() {
            return Ok(());
        }

        let widths = self.widths();
        let gap = "  ";

        // Header row — pad plain text first, then style.
        for (i, col) in self.columns.iter().enumerate() {
            if i > 0 {
                write!(f, "{gap}")?;
            }
            let w = widths[i];
            let padded = if col.right_align {
                format!("{:>w$}", col.header)
            } else {
                format!("{:<w$}", col.header)
            };
            write!(f, "{}", padded.heading())?;
        }
        writeln!(f)?;

        // Separator
        for (i, w) in widths.iter().enumerate() {
            if i > 0 {
                write!(f, "{gap}")?;
            }
            write!(f, "{}", "─".repeat(*w).muted())?;
        }
        writeln!(f)?;

        // Data rows
        for row in &self.rows {
            for (i, col) in self.columns.iter().enumerate() {
                if i > 0 {
                    write!(f, "{gap}")?;
                }
                let raw = row.get(i).map(String::as_str).unwrap_or("");
                let cell = if col.max_width.is_some() {
                    Self::truncate(raw, widths[i])
                } else {
                    raw.to_string()
                };
                let w = widths[i];
                if col.right_align {
                    write!(f, "{cell:>w$}")?;
                } else {
                    write!(f, "{cell:<w$}")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_table_renders_nothing() {
        let table = Table::new(vec![]);
        assert_eq!(table.to_string(), "");
    }

    #[test]
    fn table_auto_sizes_columns() {
        let table = Table::new(vec![Column::new("Name"), Column::right("Count")])
            .row(vec!["alice", "42"])
            .row(vec!["bob", "7"]);

        let output = table.to_string();
        assert!(output.contains("alice"));
        assert!(output.contains("bob"));
        assert!(output.contains("42"));
        assert!(output.contains("7"));
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn right_aligned_column_pads_left() {
        let table = Table::new(vec![Column::right("N")])
            .row(vec!["1"])
            .row(vec!["100"]);

        let output = table.to_string();
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn is_empty_reflects_row_count() {
        let empty = Table::new(vec![Column::new("A")]);
        assert!(empty.is_empty());

        let full = Table::new(vec![Column::new("A")]).row(vec!["x"]);
        assert!(!full.is_empty());
    }

    #[test]
    fn column_key_defaults_to_lowercase_header() {
        let col = Column::new("Name");
        assert_eq!(col.key, "name");
        assert_eq!(col.header, "Name");
    }

    #[test]
    fn column_key_can_differ_from_header() {
        let col = Column::key("agent_name", "Name");
        assert_eq!(col.key, "agent_name");
        assert_eq!(col.header, "Name");
    }

    #[test]
    fn truncation_respects_max_width() {
        let table = Table::new(vec![Column::new("Text").max(10)])
            .row(vec!["short"])
            .row(vec!["this is a long string that should be truncated"]);

        let output = table.to_string();
        assert!(output.contains("short"));
        assert!(output.contains("…"));
        // The long string should not appear in full
        assert!(!output.contains("truncated"));
    }
}
