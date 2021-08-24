//! Libc s(n)printf clone written in Rust, so you can use printf-style
//! formatting without a libc (e.g. in WebAssembly).
//!
//! **Note:** *You're probably better off using standard Rust string formatting
//! instead of thie crate unless you specificaly need printf compatibility.*
//!
//! It follows the standard C semantics, except:
//!
//!  * Locale-aware UNIX extensions (`'` and GNU’s `I`) are not supported.
//!  * `%a`/`%A` (hexadecimal floating point) are currently not implemented.
//!  * Length modifiers (`h`, `l`, etc.) are checked, but ignored. The passed
//!    type is used instead.
//!
//! Usage example:
//!
//!     use sprintf::sprintf;
//!     let s = sprintf!("%d + %d = %d\n", 3, 9, 3+9).unwrap();
//!     assert_eq!(s, "3 + 9 = 12\n");
//!
//! The types of the arguments are checked at runtime.
//! 

mod format;
mod parser;

pub use format::Printf;
pub use parser::{ConversionType, ConversionSpecifier, NumericParam};
use parser::{parse_format_string, FormatElement};

/// Error type
#[derive(Debug, Clone, Copy)]
pub enum PrintfError {
    /// Error parsing the format string
    ParseError,
    /// Incorrect type passed as an argument
    WrongType,
    /// Too many arguments passed
    TooManyArgs,
    /// Too few arguments passed
    NotEnoughArgs,
    /// Other error (should never happen)
    Unknown,
}

pub type Result<T> = std::result::Result<T, PrintfError>;

/// Format a string. (Roughly equivalent to `vsnprintf` in C)
/// 
/// Takes a printf-style format string `format` and a slice of dynamically
/// typed arguments, `args`.
/// 
///     use sprintf::{vsprintf, Printf};
///     let n = 16;
///     let args: Vec<&dyn Printf> = vec![&n];
///     let s = vsprintf("%#06x", &args).unwrap();
///     assert_eq!(s, "0x0010");
/// 
/// See also: [sprintf]
pub fn vsprintf(format: &str, args: &[&dyn Printf]) -> Result<String> {
    vsprintfp(&parse_format_string(format)?, args)
}

fn vsprintfp(format: &[FormatElement], args: &[&dyn Printf]) -> Result<String> {
    let mut res = String::new();

    let mut args = args;
    let mut pop_arg = || {
        if args.is_empty() {
            Err(PrintfError::NotEnoughArgs)
        } else {
            let a = args[0];
            args = &args[1..];
            Ok(a)
        }
    };

    for elem in format {
        match elem {
            FormatElement::Verbatim(s) => {
                res.push_str(&s);
            }
            FormatElement::Format(spec) => {
                if spec.conversion_type == ConversionType::PercentSign {
                    res.push('%');
                } else {
                    let mut completed_spec = *spec;
                    if spec.width == NumericParam::FromArgument {
                        completed_spec.width = NumericParam::Literal(
                            pop_arg()?.as_int().ok_or(PrintfError::WrongType)?,
                        )
                    }
                    if spec.precision == NumericParam::FromArgument {
                        completed_spec.precision = NumericParam::Literal(
                            pop_arg()?.as_int().ok_or(PrintfError::WrongType)?,
                        )
                    }
                    res.push_str(&pop_arg()?.format(&completed_spec)?);
                }
            }
        }
    }

    if args.is_empty() {
        Ok(res)
    } else {
        Err(PrintfError::TooManyArgs)
    }
}

/// Format a string. (Roughly equivalent to `snprintf` in C)
/// 
/// Takes a printf-style format string `format` and a variable number of
/// additional arguments.
/// 
///     use sprintf::sprintf;
///     let s = sprintf!("%s = %*d", "forty-two", 4, 42).unwrap();
///     assert_eq!(s, "forty-two =   42");
/// 
/// Wrapper around [vsprintf].
#[macro_export]
macro_rules! sprintf {
    ($fmt:expr, $($arg:expr),*) => {
        sprintf::vsprintf($fmt, &[$( &($arg) as &dyn sprintf::Printf),* ][..])
    };
}
