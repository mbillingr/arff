// Copyright 2018 Martin Billinger
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::fmt::{self, Display};
use std::string::FromUtf8Error;

use serde::{de, ser};

use parser::TextPos;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),

    // Serializer
    UnexpectedType,
    InconsistentType { row: usize, column: usize },

    // Deserializer
    Eof,
    Expected(TextPos, &'static str),
    ExpectedString(TextPos, String),
    UnexpectedChar(TextPos, char, char),
    ExpectedSequenceType,
    ExpectedUnsignedValue(TextPos),
    ExpectedIntegerValue(TextPos),
    ExpectedFloatValue(TextPos),
    NumericRange(TextPos, i64, i64),
    NumericOverflow(TextPos),
    Utf8Error(std::str::Utf8Error),

    InvalidColumnType(TextPos, String),
    WrongNominalValue(TextPos, String),
    UnsupportedColumnType(TextPos, String),

    ConversionError,
    UnexpectedMissingValue,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            Error::UnexpectedType => "unexpected data type",
            Error::InconsistentType { .. } => "inconsistent data type",
            Error::Eof => "unexpected end of input",
            Error::Expected(_, ref what) => what,
            Error::ExpectedString(_, ref what) => what,
            Error::UnexpectedChar(_, _, _) => "unexpected character",
            Error::ExpectedUnsignedValue(_) => "expected unsigned integer value",
            Error::ExpectedIntegerValue(_) => "expected integer value",
            Error::NumericRange(_, _, _) => "value outside numeric range",
            Error::NumericOverflow(_) => "value too large for u64",
            Error::ExpectedSequenceType => "attempt to parse data set as a non-sequence type",
            Error::ExpectedFloatValue(_) => "invalid floating point number",
            Error::Utf8Error(_) => "invalid UTF-8 string",
            Error::InvalidColumnType(_, _) => "column type not understood",
            Error::UnsupportedColumnType(_, _) => "column type not supported",
            Error::WrongNominalValue(_, _) => "wrong nominal value",
            Error::ConversionError => "conversion error",
            Error::UnexpectedMissingValue => "unexpected missing value",
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Error {
        Error::Utf8Error(e.utf8_error())
    }
}
