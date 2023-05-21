use std::string::String;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    line: usize,
    col: usize,
}

#[allow(dead_code)]
impl SourceLocation {
    pub fn new(line: usize, col: usize) -> Self {
        SourceLocation { line, col }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn to_string(&self) -> String {
        format!("<line:{}, col:{}>", self.line, self.col)
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