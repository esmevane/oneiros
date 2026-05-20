#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum LensParseError {
    #[error("unexpected end of input at byte {at}; expected {expected}")]
    UnexpectedEof { at: usize, expected: &'static str },

    #[error("unexpected character {found:?} at byte {at}; expected {expected}")]
    UnexpectedChar {
        found: char,
        at: usize,
        expected: &'static str,
    },

    #[error("unterminated string literal opened at byte {at}")]
    UnterminatedString { at: usize },

    #[error("invalid ref literal at byte {at}: {reason}")]
    InvalidRef { at: usize, reason: &'static str },

    #[error("trailing input at byte {at}: {found:?}")]
    TrailingInput { at: usize, found: String },

    #[error("missing argument at byte {at}: predicate args cannot be empty between commas")]
    MissingArgument { at: usize },

    #[error("invalid integer literal at byte {at}: {raw:?}")]
    InvalidInteger { at: usize, raw: String },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum LensValidationError {
    #[error("unknown predicate {name}")]
    UnknownPredicate { name: crate::PredicateName },

    #[error("predicate {name} expected {expected} arg(s), got {got}")]
    ArityMismatch {
        name: crate::PredicateName,
        expected: usize,
        got: usize,
    },

    #[error(
        "predicate {predicate} arg #{position}: expected {expected}, got {got}"
    )]
    ArgTypeMismatch {
        predicate: crate::PredicateName,
        position: usize,
        expected: &'static str,
        got: &'static str,
    },

    #[error(
        "set operator `{operator}` requires matching result types: left is {left}, right is {right}"
    )]
    ResultTypeMismatch {
        operator: &'static str,
        left: &'static str,
        right: &'static str,
    },
}

impl LensParseError {
    #[allow(dead_code)]
    pub(crate) fn span(&self) -> usize {
        match self {
            Self::UnexpectedEof { at, .. }
            | Self::UnexpectedChar { at, .. }
            | Self::UnterminatedString { at }
            | Self::InvalidRef { at, .. }
            | Self::TrailingInput { at, .. }
            | Self::MissingArgument { at }
            | Self::InvalidInteger { at, .. } => *at,
        }
    }
}
