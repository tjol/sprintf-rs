use thiserror::Error;

mod format;
pub mod parser;

pub use format::Format;
#[doc(hidden)]
pub use parser::{ConversionSpecifier, ConversionType, NumericParam};
use parser::{FormatElement, parse_format_string};

/// Error type
#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum FormatError {
    /// Error parsing the format string
    #[error("Error parsing the format string")]
    ParseError,
    /// Incorrect type passed as an argument
    #[error("Incorrect type passed as an argument")]
    WrongType,
    /// Too many arguments passed
    #[error("Too many arguments passed")]
    TooManyArgs,
    /// Too few arguments passed
    #[error("Too few arguments passed")]
    NotEnoughArgs,
    /// Other error (should never happen)
    #[error("Other error (should never happen)")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, FormatError>;

pub fn format(format: &str, args: &[&dyn Format]) -> Result<String> {
    lformat(&parse_format_string(format)?, args)
}

fn lformat(format: &[FormatElement], args: &[&dyn Format]) -> Result<String> {
    let mut res = String::new();

    let mut args = args;
    let mut pop_arg = || {
        if args.is_empty() {
            Err(FormatError::NotEnoughArgs)
        } else {
            let a = args[0];
            args = &args[1..];
            Ok(a)
        }
    };

    for elem in format {
        match elem {
            FormatElement::Verbatim(s) => {
                res.push_str(s);
            }
            FormatElement::Format(spec) => {
                if spec.conversion_type == ConversionType::PercentSign {
                    res.push('%');
                } else {
                    let completed_spec = *spec;
                    res.push_str(&pop_arg()?.format(&completed_spec)?);
                }
            }
        }
    }

    if args.is_empty() {
        Ok(res)
    } else {
        Err(FormatError::TooManyArgs)
    }
}
