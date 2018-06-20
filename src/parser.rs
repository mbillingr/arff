// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use std::{f64, i16, i32, i64, u16, u32, u64, u8};

use super::error::{Error, Result};

pub const I16_MIN: i64 = i16::MIN as i64;
pub const I16_MAX: i64 = i16::MAX as i64;
pub const I32_MIN: i64 = i32::MIN as i64;
pub const I32_MAX: i64 = i32::MAX as i64;

pub const U16_MAX: u64 = u16::MAX as u64;
pub const U32_MAX: u64 = u32::MAX as u64;

pub const I16_MINABS: u64 = I16_MAX as u64 + 1;
pub const I32_MINABS: u64 = I32_MAX as u64 + 1;

pub const I64_MAX: u64 = i64::MAX as u64;
pub const I64_MINABS: u64 = I64_MAX + 1;

pub enum DynamicValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F64(f64),
    String(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DType {
    Numeric,
    String,
    //Date(String),
    Nominal(Vec<String>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub dtype: DType,
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub attrs: Vec<Attribute>,
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
    buffer: Vec<u8>, // reusable scratch space
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut p = Parser {
            input: input.bytes(),
            current_char: 0,
            pos: TextPos { line: 1, column: 0 },
            buffer: Vec::new(),
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
                _ => return,
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
            return Err(Error::UnexpectedChar(
                self.pos,
                ch as char,
                self.current_char as char,
            ));
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
    pub fn parse_unquoted_string(&mut self) -> Result<String> {
        let mut s = Vec::new();
        loop {
            match self.current_char {
                0 | b' ' | b'\t' | b'\n' | b',' => break,
                ch => s.push(ch),
            }
            self.advance();

            if self.is_eof() {
                break;
            }
        }
        Ok(String::from_utf8(s)?)
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

        let pos = self.pos;

        let mut s = Vec::new();
        loop {
            match self.current_char {
                b'%' | b'\n' => break,
                ch => s.push(ch),
            }
            self.advance();
        }
        let mut s = String::from_utf8(s)?;

        if s.starts_with('{') && s.ends_with('}') {
            let categories = s[1..s.len() - 1]
                .split(',')
                .map(|s| s.trim().to_owned())
                .collect();
            return Ok(Attribute {
                name,
                dtype: DType::Nominal(categories),
            });
        }

        s.make_ascii_uppercase();

        match &s[..4] {
            "NUME" | "REAL" | "INTE" => Ok(Attribute {
                name,
                dtype: DType::Numeric,
            }),
            "STRI" => Ok(Attribute {
                name,
                dtype: DType::String,
            }),
            "DATE" => Err(Error::UnsupportedColumnType(pos, s)),
            _ => Err(Error::InvalidColumnType(pos, s)),
        }
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
                    b'%' => self.skip_until(b'\n'),
                    0 => return Err(Error::Eof),
                    _ => self.advance(),
                }
            }

            let pos = self.pos;
            let mut token = self.parse_unquoted_string()?;
            token.make_ascii_uppercase();

            match token.as_ref() {
                "@DATA" => {
                    self.ignore_comment();
                    self.consume_newline()?;
                    return Ok(Header { name, attrs });
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
                _ => return Err(Error::Expected(pos, "`@RELATION`, `@ATTRIBUTE`, or `@DATA")),
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
            b'\n' | 0 => true,
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

    pub fn parse_any_delimiter(&mut self) -> Result<()> {
        self.ignore_comment();
        match self.current_char {
            0 => Ok(()),
            b'\n' => self.consume_newline(),
            b',' | b'\t' => {
                self.advance();
                self.skip_spaces();
                Ok(())
            }
            _ => {
                return Err(Error::Expected(
                    self.pos,
                    "`,`, `\t`, newline, or end of input",
                ))
            }
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
            _ => Err(Error::Expected(pos, "boolean value")),
        }
    }

    /// Parse a unsigned integer value
    pub fn parse_u64(&mut self) -> Result<u64> {
        let pos = self.pos();

        let mut value = match self.current_char {
            ch @ b'0'...b'9' => (ch as u8 - b'0') as u64,
            b'+' => 0,
            _ => return Err(Error::ExpectedUnsignedValue(pos)),
        };

        loop {
            self.advance();
            match self.current_char {
                ch @ b'0'...b'9' => {
                    value = value
                        .checked_mul(10)
                        .ok_or(Error::NumericOverflow(pos))?
                        .checked_add((ch - b'0') as u64)
                        .ok_or(Error::NumericOverflow(pos))?;
                }
                _ => return Ok(value),
            }
        }
    }

    /// Parse a signed integer value
    pub fn parse_i64(&mut self) -> Result<i64> {
        let pos = self.pos();

        let negative = match self.current_char {
            b'-' => {
                self.advance();
                true
            }
            b'+' => {
                self.advance();
                false
            }
            _ => false,
        };

        match (negative, self.parse_u64()) {
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
                ch @ b'+' | ch @ b'-' | ch @ b'.' | ch @ b'e' | ch @ b'E' | ch @ b'0'...b'9' => {
                    s.push(ch)
                }
                _ => break,
            }
            self.advance();
        }

        match String::from_utf8(s).unwrap().parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::ExpectedFloatValue(pos)),
        }
    }

    /// Try to parse value in most compact representation.
    /// u8 > i8 > u16 > ... > f64 > String
    pub fn parse_dynamic(&mut self) -> Result<Option<DynamicValue>> {
        let pos = self.pos();

        if self.parse_is_missing() {
            return Ok(None);
        }

        // if it is quoted, it is certainly a string
        match self.current_char {
            b'\'' | b'\"' => {
                let s = self.parse_quoted_string()?;
                return Ok(Some(DynamicValue::String(s)));
            }
            _ => {}
        }

        // try to parse as integer

        self.buffer.clear();

        let negative = match self.current_char {
            b'-' => {
                self.buffer.push(self.current_char);
                self.advance();
                true
            }
            b'+' => {
                self.buffer.push(self.current_char);
                self.advance();
                false
            }
            _ => false,
        };

        let mut value = 0u64;
        loop {
            match self.current_char {
                ch @ b'0'...b'9' => {
                    value = value
                        .checked_mul(10)
                        .ok_or(Error::NumericOverflow(pos))?
                        .checked_add((ch - b'0') as u64)
                        .ok_or(Error::NumericOverflow(pos))?;
                }
                0 | b' ' | b'\t' | b'\n' | b',' => match (negative, value) {
                    (false, 0...255) => return Ok(Some(DynamicValue::U8(value as u8))),
                    (true, 0...128) => return Ok(Some(DynamicValue::I8((-(value as i64)) as i8))),
                    (false, 0...U16_MAX) => return Ok(Some(DynamicValue::U16(value as u16))),
                    (true, 0...I16_MINABS) => {
                        return Ok(Some(DynamicValue::I16((-(value as i64)) as i16)))
                    }
                    (false, 0...U32_MAX) => return Ok(Some(DynamicValue::U32(value as u32))),
                    (true, 0...I32_MINABS) => {
                        return Ok(Some(DynamicValue::I32((-(value as i64)) as i32)))
                    }
                    (false, _) => return Ok(Some(DynamicValue::U64(value))),
                    (true, I64_MINABS) => return Ok(Some(DynamicValue::I64(i64::MIN))),
                    (true, 0...I64_MINABS) => return Ok(Some(DynamicValue::I64(-(value as i64)))),
                    _ => break,
                },
                _ => break,
            }
            self.buffer.push(self.current_char);
            self.advance();
        }

        // not an integer => collect remaining characters
        loop {
            match self.current_char {
                0 | b' ' | b'\t' | b'\n' | b',' => break,
                _ => {
                    self.buffer.push(self.current_char);
                    self.advance();
                }
            }
        }

        let s = String::from_utf8(self.buffer.drain(..).collect()).unwrap();

        // either float or string
        match s.parse::<f64>() {
            Ok(value) => Ok(Some(DynamicValue::F64(value))),
            Err(_) => Ok(Some(DynamicValue::String(s))),
        }
    }
}

macro_rules! impl_parse_primitive_unsigned {
    ($name:ident, $typ:ident, $min:expr, $max:expr) => {
        impl<'a> Parser<'a> {
            pub fn $name(&mut self) -> Result<$typ> {
                let pos = self.pos();
                let value = self.parse_u64()?;
                match value {
                    $min...$max => Ok(value as $typ),
                    _ => Err(Error::NumericRange(pos, $min as i64, $max as i64)),
                }
            }
        }
    };
}

impl_parse_primitive_unsigned!(parse_u8, u8, 0, 255);
impl_parse_primitive_unsigned!(parse_u16, u16, 0, U16_MAX);
impl_parse_primitive_unsigned!(parse_u32, u32, 0, U32_MAX);

macro_rules! impl_parse_primitive_signed {
    ($name:ident, $typ:ident, $min:expr, $max:expr) => {
        impl<'a> Parser<'a> {
            pub fn $name(&mut self) -> Result<$typ> {
                let pos = self.pos();
                let value = self.parse_i64()?;
                match value {
                    $min...$max => Ok(value as $typ),
                    _ => Err(Error::NumericRange(pos, $min, $max)),
                }
            }
        }
    };
}

impl_parse_primitive_signed!(parse_i8, i8, -128, 127);
impl_parse_primitive_signed!(parse_i16, i16, I16_MIN, I16_MAX);
impl_parse_primitive_signed!(parse_i32, i32, I32_MIN, I32_MAX);

/// Error parsing unquoted strings that contain '0'.
/// https://github.com/mbillingr/arff/issues/1
#[test]
fn github_issue_1() {
    let mut parser = Parser::new("abc0def");
    assert_eq!(parser.parse_unquoted_string(), Ok("abc0def".into()));
    assert!(parser.is_eof());
}
