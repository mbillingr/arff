// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{i16, i32, i64, u8, u16, u32, u64, f64};
use std::slice;
use std::str;

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
pub enum DType {
    Numeric,
    String,
    //Date(String),
    Nominal(Vec<String>),
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub dtype: DType,
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub attrs: Vec<Attribute>
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
    input: str::Bytes<'a>,
    current_char: Option<u8>,
    pos: TextPos,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut input = input.bytes();
        let current_char = input.next();
        Parser {
            input,
            current_char,
            pos: TextPos {line: 1, column: 1}
        }
    }

    pub fn pos(&self) -> TextPos {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        self.current_char.is_none()
    }

    pub fn peek_u8(&self) -> Option<u8> {
        self.current_char
    }

    fn next_u8(&mut self) -> Result<u8> {
        match self.peek_u8() {
            Some(ch) => {
                self.current_char = self.input.next();

                if ch == b'\n' {
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

    fn skip_until(&mut self, delimiter: u8) -> Result<()> {
        while self.peek_u8() != Some(delimiter) {
            self.next_u8()?;
        }
        Ok(())
    }

    pub fn skip_whitespace(&mut self, skip_newline: bool) -> Result<()> {
        loop {
            match (self.peek_u8(), skip_newline) {
                (Some(b'%'), _) => {
                    self.skip_until(b'\n')?;
                    continue
                },
                (Some(b'\n'), true) => {}
                (Some(b'\n'), false) => return Ok(()),
                (Some(b' '), _) |
                (Some(b'\t'), _) => {}
                (Some(_), _)  => return Ok(()),
                (None, _) => return Ok(()),
            }

            self.current_char = self.input.next();
            self.pos.column += 1;
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
        for c in s.bytes() {
            if self.next_u8()?.to_ascii_uppercase() != c {
                return Err(Error::Expected(self.pos, s))
            }
        }
        self.skip_whitespace(false)?;
        Ok(())
    }

    pub fn match_optional(&mut self, c: u8) -> Result<bool> {
        match self.peek_u8() {
            Some(ch) if ch == c => {
                self.next_u8()?;
                self.skip_whitespace(false)?;
                Ok(true)
            }
            _ => Ok(false)
        }
    }

    pub fn parse_string(&mut self) -> Result<String> {
        match self.peek_u8() {
            None => Err(Error::Eof),
            Some(b'\'') | Some(b'\"') => self.parse_quoted_string(),
            _ => self.parse_unquoted_string(),
        }
    }

    fn parse_quoted_string(&mut self) -> Result<String> {
        let delimiter = self.next_u8()?;
        let mut s = Vec::new();
        loop {
            let ch = self.next_u8()?;

            if ch == delimiter {
                self.skip_whitespace(false)?;
                return Ok(String::from_utf8(s).unwrap())
            } else {
                s.push(ch);
            }
        }
    }

    fn parse_unquoted_string(&mut self) -> Result<String> {
        let mut s = Vec::new();
        loop {
            match self.peek_u8() {
                None => return Ok(String::from_utf8(s).unwrap()),
                Some(ch) => {
                    if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b',' || ch == b'{' || ch == b'}' {
                        self.skip_whitespace(false)?;
                        return Ok(String::from_utf8(s).unwrap())
                    } else {
                        self.next_u8()?;
                        s.push(ch);
                    }
                }
            }
        }
    }

    fn parse_type(&mut self) -> Result<DType> {
        match self.peek_u8().map(|c|c.to_ascii_uppercase()) {
            Some(b'N') => {
                self.match_token("NUMERIC")?;
                Ok(DType::Numeric)
            }
            Some(b'I') => {
                self.match_token("INTEGER")?;
                Ok(DType::Numeric)
            }
            Some(b'S') => {
                self.match_token("STRING")?;
                Ok(DType::String)
            }
            Some(b'R') => {
                self.match_token("RE")?;
                match self.peek_u8().map(|c|c.to_ascii_uppercase()) {
                    Some(b'A') => {
                        // REAL
                        self.match_token("AL")?;
                        Ok(DType::Numeric)
                    }
                    Some(b'L') => {
                        // RELATIONAL
                        self.match_token("LATIONAL")?;
                        unimplemented!()
                    }
                    _ => Err(Error::Expected(self.pos, "`@NUMERIC`, `@INTEGER`, `@STRING`, `@REAL`, `@RELATIONAL`, `@DATE`, or `{<identifier list>}`"))
                }
            }
            Some(b'D') => {
                self.match_token("DATE")?;
                unimplemented!();
            }
            Some(b'{') => {
                let mut v = Vec::new();
                self.match_token("{")?;
                while self.peek_u8() != Some(b'}') {
                    v.push(self.parse_string()?);
                    self.match_optional(b',')?;
                }
                self.match_token("}")?;
                Ok(DType::Nominal(v))
            }
            _ => Err(Error::Expected(self.pos, "`@NUMERIC`, `@INTEGER`, `@STRING`, `@REAL`, `@RELATIONAL`, `@DATE`, or `{<identifier list>}`"))
        }
    }

    pub fn parse_header(&mut self) -> Result<Header> {
        self.skip_whitespace(true)?;

        self.match_token("@RELATION")?;

        let name = self.parse_string()?;

        let mut attrs = Vec::new();
        loop {
            self.skip_whitespace(true)?;
            self.match_token("@")?;

            match self.peek_u8().map(|c|c.to_ascii_uppercase()) {
                Some(b'A') => {
                    self.match_token("ATTRIBUTE")?;
                    let name = self.parse_string()?;
                    let dtype = self.parse_type()?;
                    attrs.push(Attribute{name, dtype});
                }
                Some(b'D') => {
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

        let mut i = match self.next_u8()? {
            ch @ b'0' ... b'9' => (ch as u8 - b'0') as u64,
            b'+' => 0,
            _ => return Err(Error::ExpectedUnsignedValue(pos)),
        };

        loop {
            match self.current_char {
                Some(ch @ b'0' ... b'9') => {
                    self.current_char = self.input.next();
                    self.pos.column += 1;
                    i = i
                        .checked_mul(10)
                        .ok_or(Error::NumericOverflow(pos))?
                        .checked_add((ch - b'0') as u64)
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
        let sign = match self.peek_u8() {
            Some(b'-') => {
                self.next_u8()?;
                true
            },
            Some(b'+') => {
                self.next_u8()?;
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

        let mut s = Vec::new();
        loop {
            match self.peek_u8() {
                Some(ch@b'+') | Some(ch@b'-') | Some(ch@b'.') | Some(ch@b'e') | Some(ch@b'E') |
                Some(ch@b'0'...b'9') => {
                    self.next_u8()?;
                    s.push(ch);
                },
                _ => break,
            }
        }
        match String::from_utf8(s).unwrap().parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ExpectedFloatValue(pos)),
        }
    }

    pub fn parse_bool(&mut self) -> Result<bool> {
        let pos = self.pos();
        let result = match self.next_u8()?.to_ascii_uppercase() {
            b'0' => Ok(false),
            b'1' => Ok(true),
            b'F' => {
                if let Some(b'A') = self.peek_u8().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("ALSE").map(|_|false)
                } else {
                    Ok(false)
                }
            },
            b'T' => {
                if let Some(b'R') = self.peek_u8().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("RUE").map(|_|true)
                } else {
                    Ok(true)
                }
            },
            b'N' => {
                if let Some(b'O') = self.peek_u8().map(|c|c.to_ascii_uppercase()) {
                    self.match_token("O").map(|_|false)
                } else {
                    Ok(false)
                }
            },
            b'Y' => {
                if let Some(b'E') = self.peek_u8().map(|c|c.to_ascii_uppercase()) {
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
