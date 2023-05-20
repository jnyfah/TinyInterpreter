use std::string::String;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    line: u16,
    col: u16,
}

impl SourceLocation {
    pub fn new(line: u16, col: u16) -> Self {
        SourceLocation { line, col }
    }

    pub fn line(&self) -> u16 {
        self.line
    }

    pub fn col(&self) -> u16 {
        self.col
    }

    pub fn to_string(&self) -> String {
        format!("<line:{}, col:{}>", self.line, self.col)
    }
    
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}


#[derive(Debug)]
pub struct SourceRange {
    start: SourceLocation,
    stop: SourceLocation,
}

#[allow(dead_code)]
impl SourceRange {
    pub fn new(start: SourceLocation, stop: SourceLocation) -> SourceRange {
        SourceRange { start, stop }
    }

    pub fn get_start(&self) -> &SourceLocation {
        &self.start
    }

    pub fn get_end(&self) -> &SourceLocation {
        &self.stop
    }
}