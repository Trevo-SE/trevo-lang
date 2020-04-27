use crate::error::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

pub struct TRToken {
    value: String,
    line: usize,
    kind: TRTokenKind,
    statement_pos: TRStatementPosition,
}

#[derive(Debug, Clone)]
pub enum TRTokenKind {
    BasicType,
    Stack,
    Keyword,
    Integer,
    FloatPoint,
    CharConst,
    StringConst,
    Identifier,
    Symbol,
}

#[derive(Debug, Clone)]
pub enum TRStatementPosition {
    Return,
    Function,
    Argument1,
    Argument2,
}

impl TRToken {
    pub fn value(&self) -> &str {
        &self.value
    }
    
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn kind(&self) -> &TRTokenKind {
        &self.kind
    }

    pub fn statement_pos(&self) -> &TRStatementPosition {
        &self.statement_pos
    }

    pub fn basic_type(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::BasicType,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn stack(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::Stack,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn keyword(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::Keyword,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn integer(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::Integer,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn float_point(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::FloatPoint,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn char_const(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::CharConst,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn string_const(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::StringConst,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn identifier(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::Identifier,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }

    pub fn symbol(value: &str, line: usize, statement_pos: TRStatementPosition) -> TRToken {
        TRToken {
            kind: TRTokenKind::Symbol,
            value: value.to_string(),
            line,
            statement_pos,
        }
    }
}

impl std::fmt::Debug for TRToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. <{:?}|{}> {:?}", self. line, self.kind, self.value,
            self.statement_pos)
    }
}

fn next_statement_pos(pos: &mut TRStatementPosition) {
    match pos {
        TRStatementPosition::Return => *pos = TRStatementPosition::Function,
        TRStatementPosition::Function => *pos = TRStatementPosition::Argument1,
        TRStatementPosition::Argument1 => *pos = TRStatementPosition::Argument2,
        TRStatementPosition::Argument2 => *pos = TRStatementPosition::Return,
    }
}

fn substring(string: &str, i: usize, j: usize) -> String {
    string.chars().skip(i).take(j - i).collect::<String>()
}

fn remove_space_inside(line: &str) -> String {
    let mut line = line.to_string();
    let str_regex = Regex::new(r#"(".*")|('.*')"#).expect("Cannot create str regex");
    let mut i = vec![];
    let mut j = vec![];

    for captures in str_regex.captures_iter(&line) {
        for cap in captures.iter() {
            if let Some(cap) = cap {
                i.push(cap.start());
                j.push(cap.end());
            }
        }
    }

    for idx in 0..i.len() {
        let mut sub_line = substring(&line, i[idx], j[idx]);
        sub_line = sub_line.replace(" ", "\x07");
        line.replace_range(i[idx]..j[idx], &sub_line);
    }

    line
}

fn remove_commentary(line: &str) -> String {
    let mut line = line.to_string();
    let comm_regex = Regex::new(r"//.*$").expect("Cannot create comm regex");

    if !comm_regex.is_match(&line) {
        return line;
    }

    line = comm_regex.replace_all(&line, "").to_string().trim().to_string();

    line
}

fn basic_type_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(int|float|char|struct|array|func)$").unwrap();
    }
    RE.is_match(text)
}

fn stack_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(push|pop)$").unwrap();
    }
    RE.is_match(text)
}

fn keyword_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(copy|add|type|size|sum|minus|mult|div|mod|land|lor|eq|diff|grt|lst|not|and|or|xor|end|if|elif|else|while|arg|ret|return|import|use)$").unwrap();
    }
    RE.is_match(text)
}

fn integer_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([+-]?[1-9]\d*|0)$").unwrap();
    }
    RE.is_match(text)
}

fn float_point_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[+-]?[1-9]\d*[\.]\d+$").unwrap();
    }
    RE.is_match(text)
}

fn char_const_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^'.*'$").unwrap();
    }
    RE.is_match(text)
}

fn string_const_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^".*"$"#).unwrap();
    }
    RE.is_match(text)
}

fn identifier_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*$").unwrap();
    }
    RE.is_match(text)
}

fn symbol_regex(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[$.*@]$").unwrap();
    }
    RE.is_match(text)
}

fn tokenize(filename: &str) -> TRResult<Vec<TRToken>> {
    let mut toks = vec![];
    let file = File::open(filename).expect("[lexer] Cannot open file");
    let reader = BufReader::new(file);
    let mut s_pos = TRStatementPosition::Return;
    let split_regex = Regex::new(r"\s+").expect("Cannot create split regex");
    let comment_regex = Regex::new(r"^//");

    for (idx, line) in (1..).zip(reader.lines()) {
        let line = line.expect("Cannot get line");
        let line = line.trim();
        let line = remove_commentary(line);
        let line = remove_space_inside(&line);
        let segments = split_regex.split(&line).into_iter().collect::<Vec<&str>>();

        for s in segments {
            if s == "" {
                continue;
            }
            if keyword_regex(s) {
                toks.push(TRToken::keyword(s, idx, s_pos.clone()))
            } else if stack_regex(s) {
                toks.push(TRToken::stack(s, idx, s_pos.clone()))
            } else if basic_type_regex(s) {
                toks.push(TRToken::basic_type(s, idx, s_pos.clone()))
            } else if identifier_regex(s) {
                toks.push(TRToken::identifier(s, idx, s_pos.clone()))
            } else if integer_regex(s) {
                toks.push(TRToken::integer(s, idx, s_pos.clone()))
            } else if float_point_regex(s) {
                toks.push(TRToken::float_point(s, idx, s_pos.clone()))
            } else if char_const_regex(s) {
                let value = &s[1..(s.len()-1)];
                toks.push(TRToken::char_const(&value.replace("\x07", " "), idx, s_pos.clone()))
            } else if string_const_regex(s) {
                let value = &s[1..(s.len()-1)].to_string();
                toks.push(TRToken::string_const(&value.replace("\x07", " "), idx, s_pos.clone()))
            } else if symbol_regex(s) {
                toks.push(TRToken::symbol(s, idx, s_pos.clone()))
            } else {
                let value = s.to_string().replace("\x07", " ");
                return Err(TRError::new(TRErrorKind::UnknownToken, &format!("Token not recognized: {}", value), idx));
            }
            next_statement_pos(&mut s_pos);
        }

        s_pos = TRStatementPosition::Return;
    }

    Ok(toks)
}

fn check_syntax(tokens: Vec<TRToken>) -> TRResult<Vec<TRToken>> {

    for tok in &tokens {
        match tok.kind {
            TRTokenKind::BasicType => {
                match tok.statement_pos {
                    TRStatementPosition::Return => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The type \"{}\" cannot be the return of a statement", tok.value), tok.line)),
                    _ => {},
                }
            },
            TRTokenKind::Stack => {
                match tok.statement_pos {
                    TRStatementPosition::Function => {},
                    _ => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The keyword \"{}\" must be the functions of a statement", tok.value), tok.line)),
                }
            },
            TRTokenKind::Keyword => {
                match tok.statement_pos {
                    TRStatementPosition::Function => {},
                    _ => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The keyword \"{}\" must be the functions of a statement", tok.value), tok.line)),
                }
            },
            TRTokenKind::Integer => {
                match tok.statement_pos {
                    TRStatementPosition::Return => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The integer constant \"{}\" cannot be the return of a statement", tok.value), tok.line)),
                    TRStatementPosition::Function => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The integer constant \"{}\" cannot be the function of a statement", tok.value), tok.line)),
                    _ => {},
                }
            },
            TRTokenKind::FloatPoint => {
                match tok.statement_pos {
                    TRStatementPosition::Return => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The float point constant \"{}\" cannot be the return of a statement", tok.value), tok.line)),
                    TRStatementPosition::Function => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The float point constant \"{}\" cannot be the function of a statement", tok.value), tok.line)),
                    _ => {},
                }
            },
            TRTokenKind::CharConst => {
                match tok.statement_pos {
                    TRStatementPosition::Return => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The char constant '{}' cannot be the return of a statement", tok.value), tok.line)),
                    TRStatementPosition::Function => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The char constant '{}' cannot be the function of a statement", tok.value), tok.line)),
                    _ => {},
                }

                if tok.value.len() == 2 && &tok.value[0..1] != "\\" || tok.value.len() > 2 {
                    return Err(TRError::new(TRErrorKind::InvalidCharConstant, "The size of char constants must be 1", tok.line));
                }
            },
            TRTokenKind::StringConst => {
                match tok.statement_pos {
                    TRStatementPosition::Return => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The string constant \"{}\" cannot be the return of a statement", tok.value), tok.line)),
                    TRStatementPosition::Function => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The string constant \"{}\" cannot be the function of a statement", tok.value), tok.line)),
                    _ => {},
                }
            },
            TRTokenKind::Identifier => {},
            TRTokenKind::Symbol => {
                match tok.value.as_str() {
                    "." => match tok.statement_pos {
                        TRStatementPosition::Return => {},
                        _ => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, "The symbol \".\" must be the return of a statement", tok.line)),
                    },
                    "*" | "$" | "@" => match tok.statement_pos {
                        TRStatementPosition::Function => return Err(TRError::new(TRErrorKind::InvalidStatementPosition, &format!("The symbol \"{}\" cannot be the function of a statement", tok.value), tok.line)),
                        _ => {},
                    },
                    _ => {},
                }
            },
        }
    }

    Ok(tokens)
}

pub fn lexer(filename: &str) -> TRResult<Vec<TRToken>> {
    let tokens = tokenize(filename)?;
    check_syntax(tokens)
}
