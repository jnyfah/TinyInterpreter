use crate::util::*;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum LexerTokenType {
    ParenOpen,
    ParenClose,
    FloatToken,
    IntToken,
    VarToken,
    PlusToken,
    MinusToken,
    DivideToken,
    MultiplyToken,
    AssignToken,
    PrintToken,
    Space,
    Tab,
    Newline,
    Eof,
}

impl LexerTokenType {
    pub fn to_string(&self) -> &str {
        match self {
            LexerTokenType::ParenOpen => "ParenOpen",
            LexerTokenType::ParenClose => "ParenClose",
            LexerTokenType::FloatToken => "FloatToken",
            LexerTokenType::IntToken => "IntToken",
            LexerTokenType::VarToken => "VarToken",
            LexerTokenType::PlusToken => "PlusToken",
            LexerTokenType::MinusToken => "MinusToken",
            LexerTokenType::DivideToken => "DivideToken",
            LexerTokenType::MultiplyToken => "MultiplyToken",
            LexerTokenType::AssignToken => "AssignToken",
            LexerTokenType::PrintToken => "PrintToken",
            LexerTokenType::Space => "Space",
            LexerTokenType::Tab => "Tab",
            LexerTokenType::Newline => "Newline",
            LexerTokenType::Eof => "Eof",
        }
    }
}

#[derive(Debug)]
pub struct LexerToken {
    pub value: String,
    pub location: location::SourceLocation,
    pub type_: LexerTokenType,
}

#[allow(dead_code)]
impl LexerToken {
    pub fn to_string(&self) -> String {
        format!(
            "[{}] -> \t{};\t {}",
            self.value,
            self.location.to_string(),
            self.type_.to_string()
        )
    }
}
