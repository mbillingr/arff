use std;
use std::fmt::{self, Display};

use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),

    NotOneField,

    InconsistentDataType,

    WrongDatasetType,

    Eof,
    Syntax,
    TrailingCharacters,

    Expected(&'static str),
    ExpectedUnsigned,
    NumericRange,
    FloatSyntax,
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
            Error::NotOneField => "expected a struct with exactly one field",
            Error::InconsistentDataType => "inconsistent data type",
            Error::Eof => "unexpected end of input",
            Error::Syntax => "syntax error",
            Error::TrailingCharacters => "unexpected characters at end of input",
            Error::Expected(ref what) => what,
            Error::ExpectedUnsigned => "expected unsigned integer",
            Error::NumericRange => "value outside numeric range",
            Error::WrongDatasetType => "attempt to parse data set as a non-sequence type",
            Error::FloatSyntax => "invalid floating point number",
        }
    }
}
