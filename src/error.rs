#[derive(Debug)]
pub struct TRError {
    kind: TRErrorKind,
    description: String,
    line: usize,
}

#[derive(Debug)]
pub enum TRErrorKind {
    UnknownToken,
    InvalidStatementPosition,
    InvalidCharConstant,
}

impl TRError {
    pub fn new(kind: TRErrorKind, description: &str, line: usize) -> TRError {
        TRError { 
            kind, 
            description: description.to_string(), 
            line
        }
    }
}

pub type TRResult<T> = Result<T, TRError>;
