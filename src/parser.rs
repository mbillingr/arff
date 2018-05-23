// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{i16, i32, i64, u8, u16, u32, u64, f64};

use super::error::{Error, Result};


pub const I16_MIN: i64 = i16::MIN as i64;
pub const I16_MAX: i64 = i16::MAX as i64;
pub const I32_MIN: i64 = i32::MIN as i64;
pub const I32_MAX: i64 = i32::MAX as i64;

pub const U16_MAX: u64 = u16::MAX as u64;
pub const U32_MAX: u64 = u32::MAX as u64;

pub const I64_MAX: u64 = i64::MAX as u64;
pub const I64_MINABS: u64 = I64_MAX + 1;


#[derive(Debug)]
pub enum DType<'a> {
    Numeric,
    String,
    //Date(String),
    Nominal(Vec<&'a str>),
}

#[derive(Debug)]
pub struct Attribute<'a> {
    pub name: &'a str,
    pub dtype: DType<'a>,
}

#[derive(Debug)]
pub struct Header<'a> {
    pub name: &'a str,
    pub attrs: Vec<Attribute<'a>>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TextPos {
    line: usize,
    column: usize,
}

impl TextPos {
    pub fn new(line: usize, column: usize) -> Self {
        TextPos { line, column }
    }
}


pub struct Parser<'a> {
    input: &'a str,
    pos: TextPos,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input,
            pos: TextPos {line: 1, column: 1}
        }
    }

    pub fn pos(&self) -> TextPos {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        self.input.is_empty()
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.input.chars().next()
    }

    fn next_char(&mut self) -> Result<char> {
        match self.peek_char() {
            Some(ch) => {
                self.input = &self.input[ch.len_utf8()..];

                if ch == '\n' {
                    self.pos.line += 1;
                    self.pos.column = 1;
                }
                else {
                    self.pos.column += 1;
                }

                Ok(ch)
            }
            None => Err(Error::Eof)
        }
    }

    fn skip_until(&mut self, delimiter: char) -> Result<()> {
        while self.peek_char() != Some(delimiter) {
            self.next_char()?;
        }
        Ok(())
    }

    pub fn skip_whitespace(&mut self, skip_newline: bool) -> Result<()> {
        loop {
            match (self.peek_char(), skip_newline) {
                (Some('%'), _) => {
                    self.skip_until('\n')?;
                    continue
                },
                (Some('\n'), true) => {}
                (Some('\n'), false) => return Ok(()),
                (Some(ch), _) if ch.is_whitespace() => {}
                (Some(_), _)  => return Ok(()),
                (None, _) => return Ok(()),
            }

            self.next_char()?;
        }
    }

    pub fn match_eof(&mut self) -> Result<()> {
        if self.is_eof() {
            Ok(())
        } else {
            Err(Error::Expected(self.pos, "end of input"))
        }
    }

    pub fn match_token(&mut self, s: &'static str) -> Result<()> {
        for c in s.chars() {
            if self.next_char()?.to_ascii_uppercase() != c {
                return Err(Error::Expected(self.pos, s))
            }
        }
        self.skip_whitespace(false)?;
        Ok(())
    }

    pub fn match_optional(&mut self, c: char) -> Result<bool> {
        match self.peek_char() {
            Some(ch) if ch == c => {
                self.next_char()?;
                self.skip_whitespace(false)?;
                Ok(true)
            }
            _ => Ok(false)
        }
    }

    pub fn parse_string(&mut self) -> Result<&'a str> {
        match self.peek_char() {
            None => Err(Error::Eof),
            Some('\'') | Some('\"') => self.parse_quoted_string(),
            _ => self.parse_unquoted_string(),
        }
    }

    fn parse_quoted_string(&mut self) -> Result<&'a str> {
        let delimiter = self.next_char()?;
        let start = self.input;
        let mut n_bytes = 0;
        loop {
            let ch = self.next_char()?;

            if ch == delimiter {
                self.skip_whitespace(false)?;
                return Ok(&start[..n_bytes])
            } else {
                n_bytes += ch.len_utf8();
            }
        }
    }

    fn parse_unquoted_string(&mut self) -> Result<&'a str> {
        let start = self.input;
        let mut n_bytes = 0;
        loop {
            match self.peek_char() {
                None => return Ok(&start[..n_bytes]),
                Some(ch) => {
                    if ch.is_whitespace() || ch == ',' || ch == '{' || ch == '}' {
                        self.skip_whitespace(false)?;
                        return Ok(&start[..n_bytes])
                    } else {
                        self.next_char()?;
                        n_bytes += ch.len_utf8();
                    }
                }
            }
        }
    }

    fn parse_type(&mut self) -> Result<DType<'a>> {
        match self.peek_char().map(|c|c.to_ascii_uppercase()) {
            Some('N') => {
                self.match_token("NUMERIC")?;
                Ok(DType::Numeric)
            }
            Some('I') => {
                self.match_token("INTEGER")?;
                Ok(DType::Numeric)
            }
            Some('S') => {
                self.match_token("STRING")?;
                Ok(DType::String)
            }
            Some('R') => {
                self.match_token("RE")?;
                match self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    Some('A') => {
                        // REAL
                        self.match_token("AL")?;
                        Ok(DType::Numeric)
                    }
                    Some('L') => {
                        // RELATIONAL
                        self.match_token("LATIONAL")?;
                        unimplemented!()
                    }
                    _ => Err(Error::Expected(self.pos, "`@NUMERIC`, `@INTEGER`, `@STRING`, `@REAL`, `@RELATIONAL`, `@DATE`, or `{<identifier list>}`"))
                }
            }
            Some('D') => {
                self.match_token("DATE")?;
                unimplemented!();
            }
            Some('{') => {
                let mut v = Vec::new();
                self.match_token("{")?;
                while self.peek_char() != Some('}') {
                    v.push(self.parse_string()?);
                    self.match_optional(',')?;
                }
                self.match_token("}")?;
                Ok(DType::Nominal(v))
            }
            _ => Err(Error::Expected(self.pos, "`@NUMERIC`, `@INTEGER`, `@STRING`, `@REAL`, `@RELATIONAL`, `@DATE`, or `{<identifier list>}`"))
        }
    }

    pub fn parse_header(&mut self) -> Result<Header<'a>> {
        self.skip_whitespace(true)?;

        self.match_token("@RELATION")?;

        let name = self.parse_string()?;

        let mut attrs = Vec::new();
        loop {
            self.skip_whitespace(true)?;
            self.match_token("@")?;

            match self.peek_char().map(|c|c.to_ascii_uppercase()) {
                Some('A') => {
                    self.match_token("ATTRIBUTE")?;
                    let name = self.parse_string()?;
                    let dtype = self.parse_type()?;
                    attrs.push(Attribute{name, dtype});
                }
                Some('D') => {
                    self.match_token("DATA")?;
                    break
                }
                _ => return Err(Error::Expected(self.pos, "`@ATTRIBUTE <identifier> <type>` or `@DATA`"))
            }
        }
        self.skip_whitespace(true)?;
        Ok(Header{name, attrs})
    }

    pub fn parse_newline(&mut self) -> Result<()> {
        self.match_token("\n")
    }

    pub fn parse_unsigned(&mut self) -> Result<u64> {
        let pos = self.pos();

        let mut i = match self.next_char()? {
            ch @ '0' ... '9' => (ch as u8 - b'0') as u64,
            '+' => 0,
            _ => return Err(Error::ExpectedUnsignedValue(pos)),
        };

        loop {
            match self.input.chars().next() {
                Some(ch @ '0' ... '9') => {
                    self.input = &self.input[1..];
                    self.pos.column += 1;
                    i = i
                        .checked_mul(10)
                        .ok_or(Error::NumericOverflow(pos))?
                        .checked_add((ch as u8 - b'0') as u64)
                        .ok_or(Error::NumericOverflow(pos))?;
                }
                _ => {
                    self.skip_whitespace(false)?;
                    return Ok(i)
                }
            }
        }
    }

    pub fn parse_signed(&mut self) -> Result<i64> {
        let pos = self.pos();
        let sign = match self.peek_char() {
            Some('-') => {
                self.next_char()?;
                true
            },
            Some('+') => {
                self.next_char()?;
                false
            },
            _ => false,
        };

        match (sign, self.parse_unsigned()) {
            (_, Err(Error::ExpectedUnsignedValue(_))) => Err(Error::ExpectedIntegerValue(pos)),
            (_, Err(e)) => Err(e),
            (true, Ok(I64_MINABS)) => Ok(i64::MIN),
            (false, Ok(uval @ 0...I64_MAX)) => Ok(uval as i64),
            (true, Ok(uval @ 0...I64_MAX)) => Ok(-(uval as i64)),
            _ => Err(Error::NumericRange(pos, i64::MIN, i64::MAX)),
        }
    }

    pub fn parse_float(&mut self) -> Result<f64> {
        let pos = self.pos();

        let s = self.input;
        let mut n = 0;
        loop {
            match self.peek_char() {
                Some('+') | Some('-') | Some('.') | Some('e') | Some('E') |
                Some('0'...'9') => {
                    self.next_char()?;
                    n += 1;
                },
                _ => break,
            }
        }
        match s[..n].parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ExpectedFloatValue(pos)),
        }
    }

    pub fn parse_bool(&mut self) -> Result<bool> {
        let pos = self.pos();
        let result = match self.next_char()?.to_ascii_uppercase() {
            '0' => Ok(false),
            '1' => Ok(true),
            'F' => {
                if let Some('A') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("ALSE").map(|_|false)
                } else {
                    Ok(false)
                }
            },
            'T' => {
                if let Some('R') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("RUE").map(|_|true)
                } else {
                    Ok(true)
                }
            },
            'N' => {
                if let Some('O') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("O").map(|_|false)
                } else {
                    Ok(false)
                }
            },
            'Y' => {
                if let Some('E') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("ES").map(|_|true)
                } else {
                    Ok(true)
                }
            },
            _ => return Err(Error::Expected(pos, ""))
        };

        self.skip_whitespace(false)?;

        match result {
            Ok(v) => Ok(v),
            Err(Error::Expected(_, _)) => Err(Error::Expected(pos, "`1`, `0`, `T`, `F`, `Y`, `N`, `TRUE`, `FALSE`, `YES`, or `NO`")),
            Err(e) => Err(e),
        }
    }
}
