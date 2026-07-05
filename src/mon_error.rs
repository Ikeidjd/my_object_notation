use std::fmt::Display;

use crate::text::Text;

pub struct MonError<ErrorType: Display> {
    ttype: ErrorType,
    line_str: String,
    line: usize,
    start_pos: usize,
    end_pos: usize,
}

impl<ErrorType: Display> MonError<ErrorType> {
    pub fn new(ttype: ErrorType, text: &Text) -> Self {
        let start_of_line = text.start_index - text.start_pos;

        let mut end_of_line = text.index;

        while end_of_line < text.chars.len() && text.chars[end_of_line] != '\n' {
            end_of_line += 1;
        }

        Self {
            ttype,
            line_str: text.chars[start_of_line..end_of_line].iter().collect(),
            line: text.start_line,
            start_pos: text.start_pos,
            end_pos: text.pos,
        }
    }
}

impl<ErrorType: Display> Display for MonError<ErrorType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error at line {}, pos {}.\n{}", self.line, self.start_pos, self.line_str)?;

        for _ in 0..self.start_pos {
            write!(f, " ")?;
        }

        write!(f, "\x1b[1;31m")?;

        for _ in self.start_pos..self.end_pos {
            write!(f, "^")?;
        }
        
        write!(f, " {}\x1b[0m", self.ttype)
    }
}
