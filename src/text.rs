use std::{fmt::{Debug, Display}, rc::Rc};

#[derive(Clone)]
pub struct Text {
    pub chars: Rc<[char]>,
    pub start_index: usize,
    pub start_line: usize,
    pub start_pos: usize,
    pub index: usize,
    pub line: usize,
    pub pos: usize,
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chars[self.start_index..self.index].iter().collect::<String>())
    }
}
