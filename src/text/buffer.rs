use std::iter;

pub trait Buffer {
    fn len_lines(&self) -> usize;
    fn lines(&self) -> Box<dyn iter::Iterator<Item=&str> + '_>;

    fn is_editable(&self) -> bool;

    fn len_chars(&self) -> usize;

    /*
    This function will succeed with idx one beyond the limit, so with char_idx == len_chars().
    It's a piece of Ropey semantics I will not remove now.
     */
    fn char_to_line(&self, char_idx: usize) -> Option<usize>;
    fn line_to_char(&self, line_idx: usize) -> Option<usize>;

    fn insert_char(&mut self, char_idx: usize, ch: char) -> bool;
    fn remove(&mut self, char_idx_begin: usize, char_idx_end: usize) -> bool;
}