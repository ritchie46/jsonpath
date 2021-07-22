use std::result::Result;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum ReaderError {
    Eof,
}

pub struct PathReader<'a> {
    input: &'a str,
    pos: usize,
    chars: Peekable<Chars<'a>>,
}

impl<'a> PathReader<'a> {
    pub fn new(input: &'a str) -> Self {
        PathReader { input, pos: 0, chars: input.chars().peekable() }
    }

    pub fn peek_char(&mut self) -> Result<char, ReaderError> {
        let ch = self.chars.peek().ok_or(ReaderError::Eof)?;
        Ok(*ch)
    }

    pub fn take_while<F>(&mut self, fun: F) -> Result<(usize, &str), ReaderError>
    where
        F: Fn(&char) -> bool,
    {

        let mut char_len: usize = 0;
        while let Some(c) = self.chars.peek() {
            if !fun(c) {
                break;
            }
            char_len += self.chars.next().unwrap().len_utf8();
        }

        self.pos += char_len;
        self.input = &self.input[char_len..];
        Ok((self.pos, &self.input[..char_len]))
    }

    pub fn next_char(&mut self) -> Result<(usize, char), ReaderError> {
        let ch = self.chars.next().ok_or(ReaderError::Eof)?;
        let ret = Ok((self.pos, ch));
        self.pos += ch.len_utf8();
        self.input = &self.input[ch.len_utf8()..];
        ret
    }

    pub fn current_pos(&self) -> usize {
        self.pos
    }
}
