use thiserror::Error;

#[derive(Debug, Error)]
pub enum SatukitanError {
    #[error("parse error: {0}")]
    Parse(String),
    #[error("evaluation error: {0}")]
    Eval(String),
    #[error("type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },
    #[error("undefined symbol: {0}")]
    UndefinedSymbol(String),
    #[error("arity mismatch in {name}: expected {expected}, found {found}")]
    ArityMismatch {
        name: String,
        expected: String,
        found: usize,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl SatukitanError {
    pub fn type_mismatch(expected: impl Into<String>, found: impl Into<String>) -> Self {
        SatukitanError::TypeMismatch {
            expected: expected.into(),
            found: found.into(),
        }
    }

    pub fn arity_exact(name: impl Into<String>, expected: usize, found: usize) -> Self {
        SatukitanError::ArityMismatch {
            name: name.into(),
            expected: expected.to_string(),
            found,
        }
    }

    pub fn arity_at_least(name: impl Into<String>, expected: usize, found: usize) -> Self {
        SatukitanError::ArityMismatch {
            name: name.into(),
            expected: format!(">= {expected}"),
            found,
        }
    }
}

pub fn map_nom_error(input: &str, err: nom::Err<nom::error::Error<&str>>) -> SatukitanError {
    match err {
        nom::Err::Incomplete(_) => SatukitanError::Parse("incomplete input".into()),
        nom::Err::Error(e) | nom::Err::Failure(e) => SatukitanError::Parse(format!(
            "unexpected token near '{}'",
            snippet(input, e.input)
        )),
    }
}

fn snippet(full: &str, tail: &str) -> String {
    if tail.is_empty() {
        return "<end>".to_string();
    }
    let offset = full.len().saturating_sub(tail.len());
    let preview = tail.chars().take(20).collect::<String>();
    if offset == 0 {
        preview
    } else {
        format!("…{preview}")
    }
}
