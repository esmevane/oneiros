//! Style definitions and styled text primitives.
//!
//! Uses `anstyle` for zero-cost style definitions. Styles are applied
//! via `Styled`, which wraps any `Display` value with ANSI codes.
//! `anstream` strips these automatically when color is disabled.

use anstyle::{AnsiColor, Color, Style};

/// Semantic styles for CLI output.
///
/// Each style maps to a role in the output, not a specific color.
/// This lets us evolve the palette without touching every call site.
pub(crate) struct Palette;

impl Palette {
    /// Headings and section titles.
    pub(crate) const HEADING: Style = Style::new().bold();

    /// Emphasis within body text.
    pub(crate) const EMPHASIS: Style = Style::new().bold();

    /// Success confirmations — "created", "recorded", etc.
    pub(crate) const SUCCESS: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));

    /// Warnings — degraded state, fallbacks taken.
    pub(crate) const WARNING: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Yellow)));

    /// Errors — something failed.
    pub(crate) const ERROR: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)))
        .bold();

    /// Muted text — secondary info, metadata, ref tokens.
    pub(crate) const MUTED: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));

    /// Labels in key-value pairs.
    pub(crate) const LABEL: Style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Cyan)));

    /// Hints and suggestions — "try running...", "did you mean...?"
    pub(crate) const HINT: Style = Style::new()
        .fg_color(Some(Color::Ansi(AnsiColor::Blue)))
        .italic();
}

/// A value wrapped with an ANSI style.
///
/// Renders the style's opening codes, the inner value, then the reset
/// sequence. `anstream` strips these when color is off.
pub(crate) struct Styled<T> {
    style: Style,
    value: T,
}

impl<T: std::fmt::Display> Styled<T> {
    pub(crate) fn new(style: Style, value: T) -> Self {
        Self { style, value }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Styled<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.style.render(),
            self.value,
            self.style.render_reset()
        )
    }
}

/// Apply a style to a displayable value — standalone function.
pub(crate) fn painted<T: std::fmt::Display>(value: T, style: Style) -> Styled<T> {
    Styled::new(style, value)
}

/// Extension trait for applying styles to any `Display` value.
///
/// Works on both owned and borrowed values — methods take `&self`
/// and return `Styled<&Self>`, so you can paint struct fields
/// without cloning.
pub(crate) trait Paint: std::fmt::Display {
    fn paint(&self, style: Style) -> Styled<&Self> {
        Styled::new(style, self)
    }

    fn heading(&self) -> Styled<&Self> {
        self.paint(Palette::HEADING)
    }

    fn success(&self) -> Styled<&Self> {
        self.paint(Palette::SUCCESS)
    }

    fn warning(&self) -> Styled<&Self> {
        self.paint(Palette::WARNING)
    }

    fn error(&self) -> Styled<&Self> {
        self.paint(Palette::ERROR)
    }

    fn muted(&self) -> Styled<&Self> {
        self.paint(Palette::MUTED)
    }

    fn label(&self) -> Styled<&Self> {
        self.paint(Palette::LABEL)
    }

    fn hint(&self) -> Styled<&Self> {
        self.paint(Palette::HINT)
    }
}

/// Blanket implementation — anything that implements `Display` can be painted.
impl<T: std::fmt::Display + ?Sized> Paint for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styled_wraps_value_with_ansi_codes() {
        let styled = "hello".paint(Palette::SUCCESS);
        let rendered = styled.to_string();

        assert!(rendered.contains("hello"));
        assert!(rendered.contains("\x1b["));
        assert!(rendered.contains("\x1b[0m"));
    }

    #[test]
    fn heading_applies_bold() {
        let rendered = "title".heading().to_string();
        assert!(rendered.contains("title"));
        assert!(rendered.contains("\x1b["));
    }

    #[test]
    fn paint_works_on_owned_strings() {
        let s = String::from("owned");
        let rendered = s.paint(Palette::MUTED).to_string();
        assert!(rendered.contains("owned"));
    }

    #[test]
    fn paint_works_on_numbers() {
        let rendered = 42.success().to_string();
        assert!(rendered.contains("42"));
    }
}
