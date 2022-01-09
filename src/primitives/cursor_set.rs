// Copyright 2018-2020 Google LLC, 2021 Andrzej J Skalski
// This version of the file (2021+) is licensed with GNU LGPLv3 License.
// For older version of file (licensed under Apache 2 license), see sly-editor, at
// https://github.com/njskalski/sly-editor/blob/master/src/cursor_set.rs

// Cursor == (Selection, Anchor), thanks Kakoune!
// both positions and anchor are counted in CHARS not offsets.

// The cursor points to a index where a NEW character will be included, or old character will be
// REPLACED.

// Cursor pointing to a newline character is visualized as an option to append preceding it line.

// So Cursor can point 1 character BEYOND length of buffer!

// Newline is always an end of previous line, not a beginning of new.


use std::collections::HashSet;
use std::slice::Iter;

use ropey::Rope;

use crate::text::buffer::Buffer;

const NEWLINE_LENGTH: usize = 1; // TODO(njskalski): add support for multisymbol newlines?

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CursorStatus {
    None,
    WithinSelection,
    UnderCursor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Selection {
    pub b: usize,
    //begin inclusive
    pub e: usize, //end EXCLUSIVE (as *everywhere*)
}

impl Selection {
    pub fn within(self, char_idx: usize) -> bool {
        char_idx >= self.b && char_idx < self.e
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub s: Option<Selection>,
    // selection
    pub a: usize,
    //anchor
    pub preferred_column: Option<usize>,
}

impl Cursor {
    pub fn single() -> Self {
        Cursor {
            s: None,
            a: 0,
            preferred_column: None,
        }
    }

    pub fn clear_selection(&mut self) {
        self.s = None;
    }

    pub fn clear_pc(&mut self) {
        self.preferred_column = None;
    }

    // Clears both selection and preferred column.
    pub fn clear_both(&mut self) {
        self.s = None;
        self.preferred_column = None;
    }

    pub fn get_cursor_status_for_char(&self, char_idx: usize) -> CursorStatus {
        if char_idx == self.a {
            return CursorStatus::UnderCursor;
        }

        if self.s.is_some() {
            if self.s.unwrap().within(char_idx) {
                return CursorStatus::WithinSelection;
            }
        }

        CursorStatus::None
    }

    // Returns FALSE if noop.
    pub fn home(&mut self, rope: &dyn Buffer) -> bool {
        let line = rope.char_to_line(self.a);
        let char_idx = rope.line_to_char(line);


        let res = if char_idx == self.a && self.s.is_none() {
            false
        } else {
            true
        };

        // home DOES clear preferred column.
        self.clear_both();

        res
    }

    // Returns FALSE if noop.
    pub fn end(&mut self, rope: &dyn Buffer) -> bool {
        let next_line = rope.char_to_line(self.a) + 1;

        let new_idx = if rope.len_lines() > next_line {
            rope.line_to_char(next_line) - 1
        } else {
            rope.len_chars() // yes, one beyond num chars
        };

        let res = if new_idx == self.a && self.s.is_some() {
            false
        } else {
            true
        };

        // end DOES clear preferred column
        self.clear_both();

        res
    }
}

impl Into<Cursor> for (usize, usize, usize) {
    fn into(self) -> Cursor {
        Cursor {
            s: Some(Selection {
                b: self.0,
                e: self.1,
            }),
            a: self.2,
            preferred_column: None,
        }
    }
}

impl Into<Cursor> for usize {
    fn into(self) -> Cursor {
        Cursor {
            s: None,
            a: self,
            preferred_column: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorSet {
    set: Vec<Cursor>,
}

impl CursorSet {
    pub fn single() -> Self {
        CursorSet {
            set: vec![Cursor::single()],
        }
    }

    pub fn new(set: Vec<Cursor>) -> Self {
        CursorSet { set }
    }

    pub fn set(&self) -> &Vec<Cursor> {
        &self.set
    }

    // Returns only element OR None if the set is NOT a singleton.
    pub fn as_single(&self) -> Option<&Cursor> {
        if self.set.len() != 1 {
            None
        } else {
            self.set.first()
        }
    }
}

impl CursorSet {
    pub fn move_left(&mut self) {
        self.move_left_by(1);
    }

    pub fn move_left_by(&mut self, l: usize) {
        for mut c in &mut self.set {
            c.clear_both();
            if c.a > 0 {
                c.a -= std::cmp::min(c.a, l);
            };
        }
    }

    pub fn move_right(&mut self, rope: &dyn Buffer) {
        self.move_right_by(rope, 1);
    }

    pub fn move_right_by(&mut self, rope: &dyn Buffer, l: usize) {
        let len = rope.len_chars();

        for mut c in &mut self.set {
            c.clear_both();
            //we allow anchor after last char (so you can backspace last char)
            if c.a < len {
                c.a = std::cmp::min(c.a + l, len);
            };
        }
    }

    pub fn move_vertically_by(&mut self, rope: &dyn Buffer, l: isize) {
        if l == 0 {
            return;
        }

        let last_line_idx = rope.len_lines() - 1;

        for mut c in &mut self.set {
            //getting data
            let cur_line_idx = if c.a > rope.len_chars() {
                rope.len_lines()
            } else {
                rope.char_to_line(c.a)
            };
            let cur_line_begin_char_idx = rope.line_to_char(cur_line_idx);
            let current_char_idx = c.a - cur_line_begin_char_idx;

            if cur_line_idx as isize + l > last_line_idx as isize
            /* && l > 0, checked before */
            {
                c.preferred_column = Some(current_char_idx);
                c.a = rope.len_chars(); // pointing to index higher than last valid one.
                continue;
            }

            if cur_line_idx as isize + l < 0 {
                c.preferred_column = Some(current_char_idx);
                c.a = 0;
                continue;
            }

            // at this point we know that 0 <= cur_line_idx <= last_line_idx
            debug_assert!(0 <= cur_line_idx);
            debug_assert!(cur_line_idx <= last_line_idx);
            let new_line_idx = (cur_line_idx as isize + l) as usize;

            // This is actually right. Ropey counts '\n' as last character of current line.
            let last_char_idx_in_new_line = if new_line_idx == last_line_idx {
                //this corresponds to a notion of "potential new character" beyond the buffer. It's a valid cursor position.
                rope.len_chars()
            } else {
                rope.line_to_char(new_line_idx + 1) - NEWLINE_LENGTH
            };

            let new_line_begin = rope.line_to_char(new_line_idx);
            let new_line_num_chars = last_char_idx_in_new_line + 1 - new_line_begin;

            //setting data

            c.clear_selection();

            if let Some(preferred_column) = c.preferred_column {
                debug_assert!(preferred_column >= current_char_idx);
                if preferred_column <= new_line_num_chars {
                    c.clear_pc();
                    c.a = new_line_begin + preferred_column;
                } else {
                    c.a = new_line_begin + new_line_num_chars;
                }
            } else {
                let addon = if new_line_idx == last_line_idx { 1 } else { 0 };
                // inequality below is interesting.
                // The line with characters 012 is 3 characters long. So if current char idx is 3
                // it means that line below needs at least 4 character to host it without shift left.
                // "addon" is there to make sure that last line is counted as "one character longer"
                // than it actually is, so we can position cursor one character behind buffer
                // (appending).
                if new_line_num_chars + addon <= current_char_idx {
                    c.a = new_line_begin + new_line_num_chars - 1; //this -1 is needed.
                    c.preferred_column = Some(current_char_idx);
                } else {
                    c.a = new_line_begin + current_char_idx;
                }
            }
        }
    }

    /// TODO(njskalski): how to reduce selections? Overlapping selections?
    /// TODO(njskalski): it would make a sense not to reduce cursors that have identical .a but different .preferred_column.
    /// Yet we want not to put characters twice for overlapping cursors.
    pub fn reduce(&mut self) {
        let _curs: HashSet<usize> = HashSet::new();

        //        dbg!(&self.set);

        let mut old_curs: Vec<Cursor> = vec![];
        std::mem::swap(&mut old_curs, &mut self.set);

        for c in &old_curs {
            let mut found = false;
            for oc in &self.set {
                if c.a == oc.a {
                    found = true;
                    break;
                }
            }

            if !found {
                self.set.push(c.clone());
            }
        }

        //        dbg!(&self.set);

        //        self.set.sort();
        //        self.set.dedup();
    }

    pub fn get_cursor_status_for_char(&self, char_idx: usize) -> CursorStatus {
        let mut current_status = CursorStatus::None;

        for i in self.set.iter() {
            let new_status = i.get_cursor_status_for_char(char_idx);

            if new_status == CursorStatus::WithinSelection && current_status == CursorStatus::None {
                current_status = new_status;
            }

            if new_status == CursorStatus::UnderCursor {
                current_status = new_status;
                break;
            }
        }

        current_status
    }

    pub fn iter(&self) -> Iter<'_, Cursor> {
        self.set.iter()
    }

    // Returns FALSE if results in no-op
    // TODO test
    pub fn home(&mut self, rope: &dyn Buffer) -> bool {
        let mut res = false;

        for c in self.set.iter_mut() {
            res |= c.home(rope);
        };

        self.reduce();

        res
    }

    // Returns FALSE if results in noop.
    pub fn end(&mut self, rope: &dyn Buffer) -> bool {
        let mut res = false;

        for c in self.set.iter_mut() {
            res |= c.end(rope);
        };

        self.reduce();

        res
    }
}

/*
thread 'main' panicked at 'assertion failed: preferred_column >= current_char_idx', src/primitives/cursor_set.rs:219:17
stack backtrace:
   0: rust_begin_unwind
             at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/std/src/panicking.rs:493:5
   1: core::panicking::panic_fmt
             at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/panicking.rs:92:14
   2: core::panicking::panic
             at /rustc/53cb7b09b00cbea8754ffb78e7e3cb521cb8af4b/library/core/src/panicking.rs:50:5
   3: bernardo::primitives::cursor_set::CursorSet::move_vertically_by
             at ./src/primitives/cursor_set.rs:219:17
   4: <bernardo::widget::text_editor::TextEditorWidget as bernardo::widget::widget::Widget>::update
             at ./src/widget/text_editor.rs:90:21
   5: bernardo::main::recursive_treat_views
             at ./src/main.rs:84:41
   6: bernardo::main
             at ./src/main.rs:103:17
   7: core::ops::function::FnOnce::call_once
             at /home/andrzej/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

 */