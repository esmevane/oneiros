/// When to use colored output.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, clap::ValueEnum, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum ColorChoice {
    /// Detect terminal capabilities automatically.
    #[default]
    Auto,
    /// Always emit ANSI color codes.
    Always,
    /// Never emit color codes — plain text only.
    Never,
}

impl ColorChoice {
    /// Apply this choice globally to `anstream`.
    ///
    /// Call once, early — before any colored output is written.
    pub fn apply_global(&self) {
        let choice = match self {
            Self::Auto => anstream::ColorChoice::Auto,
            Self::Always => anstream::ColorChoice::Always,
            Self::Never => anstream::ColorChoice::Never,
        };

        anstream::ColorChoice::write_global(choice);
    }
}

impl std::fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
        }
    }
}
