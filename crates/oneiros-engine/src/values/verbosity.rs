/// How much detail to show in CLI output.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, clap::ValueEnum, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Verbosity {
    /// Minimal output — only errors and essential confirmations.
    Quiet,
    /// Standard output — confirmations, summaries, and warnings.
    #[default]
    Normal,
    /// Detailed output — includes extra context and debug-level info.
    Verbose,
}

impl Verbosity {
    /// Whether output should be suppressed to essentials.
    pub fn is_quiet(&self) -> bool {
        matches!(self, Self::Quiet)
    }

    /// Whether extra detail should be included.
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose)
    }
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quiet => write!(f, "quiet"),
            Self::Normal => write!(f, "normal"),
            Self::Verbose => write!(f, "verbose"),
        }
    }
}
