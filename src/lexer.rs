use crate::util::*;
use std::string::String;
use crate::token::*;
use std::error::Error;
use crate::util::error::*;



pub struct Lexer{
    data:  String,
    x_pos: usize,
    y_pos: usize,
    pos: usize,
}

#[allow(dead_code)]
impl Lexer {
    fn new(data: String) -> Self {
        Lexer {
            data,
            pos: 0,
            y_pos: 1,
            x_pos: 1,
        }
    }

    fn current_location(&self) -> location::SourceLocation
    {
        return  location::SourceLocation::new(self.y_pos, self.x_pos);
    }

    fn next_char(&mut self) -> char {
        if self.pos >= self.data.len() {
            return '\0';
        }

        self.x_pos += 1;
        if self.data.chars().nth(self.pos) == Some('\n') {
            self.y_pos += 1;
            self.x_pos = 1;
        }

        let next_char = self.data.chars().nth(self.pos);
        self.pos += 1;

        next_char.unwrap_or('\0')
    }

    fn peek_next_char(&mut self) -> char {
        if self.pos >= self.data.len() {
            return '\0';
        }
        let next_char = self.data.chars().nth(self.pos);
        next_char.unwrap_or('\0')
    }

    fn is_integer(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_numeric(&self, c: char) -> bool {
        c.is_digit(10) || c == '.'
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic()
    }


    pub fn get_next_token(&mut self) -> Result<LexerToken, Box<TError>> {
        let location = self.current_location();
        let start_pos = self.pos;
        let nchar = self.next_char();
    
        if nchar == '\0' {
            return Ok(LexerToken {
                value: "".to_string(),
                location,
                type_: LexerTokenType::Eof,
            });
        }
        if nchar == '\n' {
            return Ok(LexerToken {
                value: "".to_string(),
                location,
                type_: LexerTokenType::Newline,
            });
        }
        if nchar == '\t' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::Tab,
            });
        }
        if nchar == '(' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::ParenOpen,
            });
        }
        if nchar == ')' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::ParenClose,
            });
        }
        if nchar == '+' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::PlusToken,
            });
        }
        if nchar == '/' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::DivideToken,
            });
        }
        if nchar == '*' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::MultiplyToken,
            });
        }
        if nchar == '-' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::MinusToken,
            });
        }
        if nchar == '=' {
            return Ok(LexerToken {
                value: self.data[start_pos ..start_pos  + 1].to_owned(),
                location,
                type_: LexerTokenType::AssignToken,
            });
        }
        if nchar == ' ' {
            return Ok(LexerToken{
                value: self.fetch_consecutive(start_pos , ' '), location, type_: LexerTokenType::Space
            })
        }
        if !(self.is_alpha(nchar) || self.is_numeric(nchar)) {
            return Err(Box::new(TError::new(&format!("Unknown character at line {}", location.to_string()))));
        }

        let substr = self.next_valid_sequences(start_pos.into());
        if substr.chars().all(|c| self.is_numeric(c)) {
            if substr.chars().all(|c| self.is_integer(c)) {
                return Ok(LexerToken {
                    value: substr.to_string(),
                    location,
                    type_: LexerTokenType::IntToken,
                });
            } else {
                return Ok(LexerToken {
                    value: substr.to_string(),
                    location,
                    type_: LexerTokenType::FloatToken,
                });
            }
        }
        if substr == "print" || substr == "Print" {
            return Ok(LexerToken {
                value: substr,
                location,
                type_: LexerTokenType::PrintToken,
            });
        }
        Ok(LexerToken {
            value: substr,
            location,
            type_: LexerTokenType::VarToken,
        })
    }
    
    
    fn next_valid_sequences(&mut self, from: usize) -> String {
        let mut counts: usize = 0;
        loop {
            let c = self.peek_next_char();
            counts += 1;
            if !(self.is_alpha(c) || self.is_numeric(c)) {
                break;
            }
            self.next_char();
        }
        self.data[from..from + counts].to_string()
    }
    
    fn fetch_consecutive(&mut self, start_pos: usize, ch: char) -> String {
        let mut count = 1;
        while self.peek_next_char() == ch {
            count += 1;
            self.next_char();
        }
        self.data[start_pos..start_pos + count].to_string()
    }
    

      
}

