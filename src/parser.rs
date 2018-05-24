// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{i16, i32, i64, u8, u16, u32, u64, f64};
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


/*#[derive(Debug)]
pub enum DType {
    Numeric,
    String,
    //Date(String),
    Nominal(Vec<String>),
}*/

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub dtype: String,  // for now do not parse the data type
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
    current_char: u8,
    pos: TextPos,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut p = Parser {
            input:  input.bytes(),
            current_char: 0,
            pos: TextPos {line: 1, column: 0}
        };
        p.advance();
        p
    }

    /// has the parser reached the end of input?
    pub fn is_eof(&self) -> bool {
        self.current_char == 0
    }

    /// get current parser position in the input
    pub fn pos(&self) -> TextPos {
        self.pos
    }

    /// advance parser to next character
    fn advance(&mut self) {
        self.current_char = self.input.next().unwrap_or(0);
        self.pos.column += 1;
    }

    /// set parser to next non-space character
    fn skip_spaces(&mut self) {
        while self.current_char == b' ' {
            self.advance();
        }
    }

    /// set parser to next non-space character, skipping empty lines and comments
    pub fn skip_empty(&mut self) {
        loop {
            match self.current_char {
                b' ' => self.advance(),
                b'\n' => self.assume_newline(),
                b'%' => self.skip_until(b'\n'),
                _ => return
            }
        }
    }

    /// set parser to next occurence of given character
    fn skip_until(&mut self, ch: u8) {
        while self.current_char != ch {
            self.advance();
        }
    }

    /// consume one expected character
    fn consume(&mut self, ch: u8) -> Result<()> {
        if self.current_char != ch {
            return Err(Error::UnexpectedChar(self.pos, ch as char,
                                             self.current_char as char))
        }
        self.advance();
        Ok(())
    }

    /// optionally consume one expected character
    fn consume_optional(&mut self, ch: u8) -> bool {
        if self.current_char == ch {
            self.advance();
            true
        } else {
            false
        }
    }

    /// consume a newline character
    ///
    /// This should be the only function used for parsing newlines, because it adjusts the
    /// internal position counter.
    fn consume_newline(&mut self) -> Result<()> {
        self.consume(b'\n')?;
        self.pos.line += 1;
        self.pos.column = 1;
        Ok(())
    }

    /// assumes a newline character
    ///
    /// Same as consume_newline, but does not check the character and cannot fail
    fn assume_newline(&mut self) {
        self.advance();
        self.pos.line += 1;
        self.pos.column = 1;
    }

    /// parse a quoted or unquoted string
    pub fn parse_string(&mut self) -> Result<String> {
        match self.current_char {
            b'\'' | b'\"' => self.parse_quoted_string(),
            _ => self.parse_unquoted_string(),
        }
    }

    /// parse a string with `'` or `"`  delimiting characters
    fn parse_quoted_string(&mut self) -> Result<String> {
        let delimiter = self.current_char;
        self.advance();

        let mut s = Vec::new();
        loop {
            match self.current_char {
                0 => return Err(Error::Eof),
                ch if ch == delimiter => break,
                ch => s.push(ch),
            }
            self.advance();
        }

        self.advance();
        Ok(String::from_utf8(s)?)
    }

    /// parse an unquoted string
    fn parse_unquoted_string(&mut self) -> Result<String> {
        let mut s = Vec::new();
        loop {
            match self.current_char {
                b'0' | b' ' | b'\t' | b'\n' | b',' => return Ok(String::from_utf8(s)?),
                ch => s.push(ch),
            }
            self.advance();
        }
    }

    /// skip spaces, if a `%`  character is encountered, the remaining line is skipped
    pub fn ignore_comment(&mut self) {
        self.skip_spaces();
        if self.current_char == b'%' {
            self.skip_until(b'\n');
        }
    }

    /// parse name and dtype of an @ATTRIBUTE declaration
    pub fn parse_attribute(&mut self) -> Result<Attribute> {
        let name = self.parse_string()?;
        self.skip_spaces();
        let dtype = self.parse_unquoted_string()?;
        Ok(Attribute {name, dtype})
    }

    /// parse ARFF header
    pub fn parse_header(&mut self) -> Result<Header> {
        let mut name = String::from("unnamed_data");
        let mut attrs = Vec::new();

        loop {
            loop {
                // we are pretty liberal in accepting anything before and between @-declarations
                match self.current_char {
                    b'@' => break,
                    b'\n' => self.consume_newline()?,
                    0 => return Err(Error::Eof),
                    _ => self.advance(),
                }
            }

            let pos = self.pos;
            let token = self.parse_unquoted_string()?;

            match token.as_ref() {
                "@DATA" =>{
                    self.ignore_comment();
                    self.consume_newline()?;
                    return Ok(Header{name, attrs})
                }
                "@RELATION" => {
                    self.skip_spaces();
                    name = self.parse_string()?;
                    self.ignore_comment();
                }
                "@ATTRIBUTE" => {
                    self.skip_spaces();
                    attrs.push(self.parse_attribute()?);
                    self.ignore_comment();
                }
                _ => return Err(Error::Expected(pos, "`@RELATION`, `@ATTRIBUTE`, or `@DATA@"))
            }
        }
    }

    /// Make sure the parser has reached the end of input (spaces, newlines, and comments allowed)
    pub fn parse_eof(&mut self) -> Result<()> {
        self.skip_empty();
        loop {
            match self.current_char {
                b' ' => self.advance(),
                b'\n' => self.consume_newline()?,
                0 => return Ok(()),
                _ => return Err(Error::Expected(self.pos, "end of input")),
            }
        }
    }

    /// Parse a column delimiter: comma or tab followed by optional spaces
    pub fn parse_column_delimiter(&mut self) -> Result<()> {
        match self.current_char {
            b',' | b'\t' => self.advance(),
            _ => return Err(Error::Expected(self.pos, "`,` or `\t`")),
        }
        self.skip_spaces();
        Ok(())
    }

    /// Are we at the end of a row?
    pub fn check_row_delimiter(&mut self) -> bool {
        match self.current_char {
            b'\n' |
            0 => true,
            _ => false,
        }
    }

    /// Parse a row delimiter: optional comment followed by newline or eof
    pub fn parse_row_delimiter(&mut self) -> Result<()> {
        self.ignore_comment();
        if self.current_char == 0 {
            Ok(())
        } else {
            self.consume_newline()
        }
    }

    /// Check for a missing value. This cannot fail.
    pub fn parse_is_missing(&mut self) -> bool {
        self.consume_optional(b'?')
    }

    /// Parse a boolean value
    pub fn parse_bool(&mut self) -> Result<bool> {
        let pos = self.pos;
        let strval = self.parse_unquoted_string()?.to_ascii_uppercase();
        match strval.as_ref() {
            "0" | "F" | "FALSE" | "N" | "NO" => Ok(false),
            "1" | "T" | "TRUE" | "Y" | "YES" => Ok(true),
            _ => Err(Error::Expected(pos, "boolean value"))
        }
    }

    /// Parse a unsigned integer value
    pub fn parse_unsigned(&mut self) -> Result<u64> {
        let pos = self.pos();

        let mut value = match self.current_char {
            ch @ b'0' ... b'9' => (ch as u8 - b'0') as u64,
            b'+' => 0,
            _ => return Err(Error::ExpectedUnsignedValue(pos)),
        };

        loop {
            self.advance();
            match self.current_char {
                ch @ b'0' ... b'9' => {
                    value = value.checked_mul(10)
                        .ok_or(Error::NumericOverflow(pos))?
                        .checked_add((ch - b'0') as u64)
                        .ok_or(Error::NumericOverflow(pos))?;
                }
                _ => return Ok(value),
            }
        }
    }

    /// Parse a signed integer value
    pub fn parse_signed(&mut self) -> Result<i64> {
        let pos = self.pos();

        let negative = match self.current_char {
            b'-' => { self.advance(); true }
            b'+' => { self.advance(); false }
            _ => false,
        };

        match (negative, self.parse_unsigned()) {
            (_, Err(Error::ExpectedUnsignedValue(_))) => Err(Error::ExpectedIntegerValue(pos)),
            (_, Err(e)) => Err(e),
            (true, Ok(I64_MINABS)) => Ok(i64::MIN),
            (true, Ok(uval @ 0...I64_MAX)) => Ok(-(uval as i64)),
            (false, Ok(uval @ 0...I64_MAX)) => Ok(uval as i64),
            _ => Err(Error::NumericRange(pos, i64::MIN, i64::MAX)),
        }
    }

    /// Parse a floating point value
    pub fn parse_float(&mut self) -> Result<f64> {
        let pos = self.pos();

        let mut s = Vec::new();
        loop {
            match self.current_char {
                ch @ b'+' | ch @ b'-' | ch @ b'.' |
                ch @ b'e' | ch @ b'E' |
                ch @ b'0'...b'9' => s.push(ch),
                _ => break,
            }
            self.advance();
        }

        match String::from_utf8(s).unwrap().parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ExpectedFloatValue(pos)),
        }
    }
}
