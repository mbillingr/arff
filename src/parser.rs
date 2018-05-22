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


pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            input,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.input.is_empty()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().next()
    }

    fn next_char(&mut self) -> Result<char> {
        match self.peek_char() {
            Some(ch) => {
                self.input = &self.input[ch.len_utf8()..];
                Ok(ch)
            }
            None => Err(Error::Eof)
        }
    }

    fn skip_whitespace(&mut self, skip_newline: bool) -> Result<()> {
        loop {
            match (self.peek_char(), skip_newline) {
                (Some('\n'), true) => {}
                (Some('\n'), false) => return Ok(()),
                (Some(ch), _) if ch.is_whitespace() => {}
                (Some(_), _)  => return Ok(()),
                (None, _) => return Ok(()),
            }

            self.next_char()?;
        }
    }

    pub fn parse_token(&mut self, s: &'static str) -> Result<()> {
        for c in s.chars() {
            if self.next_char()?.to_ascii_uppercase() != c {
                return Err(Error::Expected(s))
            }
        }
        self.skip_whitespace(false)?;
        Ok(())
    }

    pub fn parse_optional(&mut self, c: char) -> Result<()> {
        match self.peek_char() {
            Some(ch) if ch == c => {
                self.next_char()?;
                self.skip_whitespace(false)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn parse_string(&mut self) -> Result<String> {
        match self.peek_char() {
            None => Err(Error::Eof),
            Some('\'') | Some('\"') => self.parse_quoted_string(),
            _ => self.parse_unquoted_string(),
        }
    }

    fn parse_quoted_string(&mut self) -> Result<String> {
        let delimiter = self.next_char()?;
        let mut s = String::new();
        loop {
            let ch = self.next_char()?;

            if ch == delimiter {
                self.skip_whitespace(false)?;
                return Ok(s)
            } else {
                s.push(ch)
            }
        }
    }

    fn parse_unquoted_string(&mut self) -> Result<String> {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                None => return Ok(s),
                Some(ch) => {
                    if ch.is_whitespace() || ch == ',' || ch == '{' || ch == '}' {
                        self.skip_whitespace(false)?;
                        return Ok(s)
                    } else {
                        s.push(self.next_char()?)
                    }
                }
            }
        }
    }

    fn parse_type(&mut self) -> Result<DType> {
        match self.peek_char().map(|c|c.to_ascii_uppercase()) {
            Some('N') => {
                self.parse_token("NUMERIC")?;
                Ok(DType::Numeric)
            }
            Some('I') => {
                self.parse_token("INTEGER")?;
                Ok(DType::Numeric)
            }
            Some('S') => {
                self.parse_token("STRING")?;
                Ok(DType::String)
            }
            Some('R') => {
                self.parse_token("E")?;
                match self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    Some('A') => {
                        // REAL
                        self.parse_token("AL")?;
                        Ok(DType::Numeric)
                    }
                    Some('L') => {
                        // RELATIONAL
                        self.parse_token("LATIONAL")?;
                        unimplemented!()
                    }
                    _ => Err(Error::Syntax)
                }
            }
            Some('D') => {
                self.parse_token("DATE")?;
                unimplemented!();
            }
            Some('{') => {
                let mut v = Vec::new();
                self.parse_token("{")?;
                while self.peek_char() != Some('}') {
                    v.push(self.parse_string()?);
                    self.parse_optional(',')?;
                }
                self.parse_token("}")?;
                Ok(DType::Nominal(v))
            }
            _ => Err(Error::Syntax)
        }
    }

    pub fn parse_header(&mut self) -> Result<Header> {
        self.skip_whitespace(true)?;

        self.parse_token("@RELATION")?;

        let name = self.parse_string()?;

        let mut attrs = Vec::new();
        loop {
            self.skip_whitespace(true)?;
            self.parse_token("@")?;

            match self.peek_char().map(|c|c.to_ascii_uppercase()) {
                Some('A') => {
                    self.parse_token("ATTRIBUTE")?;
                    let name = self.parse_string()?;
                    let dtype = self.parse_type()?;
                    attrs.push(Attribute{name, dtype});
                }
                Some('D') => {
                    self.parse_token("DATA")?;
                    break
                }
                _ => return Err(Error::Syntax)
            }
        }
        self.skip_whitespace(true)?;
        Ok(Header{name, attrs})
    }

    pub fn parse_newline(&mut self) -> Result<()> {
        self.parse_token("\n")
    }

    pub fn parse_unsigned(&mut self) -> Result<u64> {
        let mut i = match self.next_char()? {
            ch @ '0' ... '9' => (ch as u8 - b'0') as u64,
            '+' => 0,
            _ => return Err(Error::ExpectedUnsigned),
        };

        loop {
            match self.input.chars().next() {
                Some(ch @ '0' ... '9') => {
                    self.input = &self.input[1..];
                    i = i * 10 + (ch as u8 - b'0') as u64;
                }
                _ => {
                    self.skip_whitespace(false)?;
                    return Ok(i)
                }
            }
        }
    }

    pub fn parse_signed(&mut self) -> Result<i64> {
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

        let uval = self.parse_unsigned()?;

        match (sign, uval) {
            (false, 0...I64_MAX) => Ok(uval as i64),
            (true, 0...I64_MINABS) => Ok(-(uval as i64)),
            _ => Err(Error::NumericRange),
        }
    }

    pub fn parse_float(&mut self) -> Result<f64> {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                Some('+') | Some('-') | Some('.') | Some('e') | Some('E') |
                Some('0'...'9') => s.push(self.next_char()?),
                _ => break,
            }
        }
        match s.parse() {
            Ok(v) => Ok(v),
            Err(_) => Err(Error::FloatSyntax),
        }
    }

    pub fn parse_bool(&mut self) -> Result<bool> {
        let v = match self.next_char()?.to_ascii_uppercase() {
            '0' => false,
            '1' => true,
            'F' => {
                if let Some('A') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.parse_token("ALSE")?;
                }
                false
            },
            'T' => {
                if let Some('R') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.parse_token("RUE")?;
                }
                true
            },
            'N' => {
                if let Some('O') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.parse_token("O")?;
                }
                false
            },
            'Y' => {
                if let Some('E') = self.peek_char().map(|c|c.to_ascii_uppercase()) {
                    self.parse_token("ES")?;
                }
                true
            },
            _ => return Err(Error::Expected("bool"))
        };
        self.skip_whitespace(false)?;
        Ok(v)
    }
}
