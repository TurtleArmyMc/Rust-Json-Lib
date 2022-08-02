use std::{iter::Peekable, str::Chars};

use super::unmarshalable::UnmarshalError;

/// Stores the remaining characters for unmarshaling.
pub struct UnmarshalIter<'a> {
    chars: Peekable<Chars<'a>>,
    row: u32,
    col: u32,
}

impl<'a> UnmarshalIter<'a> {
    pub fn new(chars: Chars) -> UnmarshalIter {
        UnmarshalIter {
            chars: chars.peekable(),
            row: 1,
            col: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        if Some('\n') != c {
            self.col += 1;
        } else {
            self.row += 1;
            self.col = 1;
        }
        c
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn next_non_whitespace(&mut self) -> Option<char> {
        self.peek_non_whitespace();
        self.next()
    }

    pub fn peek_non_whitespace(&mut self) -> Option<&char> {
        while let Some(_) = self.peek().filter(|c| c.is_whitespace()) {
            self.next();
        }
        self.peek()
    }

    /// Returns an error if any unmarshaled characters remain in the iterator
    pub fn check_finished(&mut self) -> Result<(), UnmarshalError> {
        match self.next_non_whitespace() {
            None => Ok(()),
            Some(c) => Err(self.unexpected_char(c)),
        }
    }

    /// Attempts to read the next character as a digit
    pub fn try_next_digit(&mut self, radix: u32) -> Result<u32, UnmarshalError> {
        self.next().map_or(Err(UnmarshalError::EndOfChars), |c| {
            c.to_digit(radix).ok_or_else(|| self.unexpected_char(c))
        })
    }

    /// Returns an error for some unexpected value that was read
    pub fn unexpected(&self, got: Option<char>) -> UnmarshalError {
        match got {
            Some(c) => self.unexpected_char(c),
            None => UnmarshalError::EndOfChars,
        }
    }

    /// Returns an error for some unexpected character that was read
    pub fn unexpected_char(&self, c: char) -> UnmarshalError {
        return UnmarshalError::UnexpectedChar {
            c,
            row: self.row,
            col: self.col,
        };
    }
}
