use std::convert::{TryFrom, TryInto};

use crate::{ConversionSpecifier, ConversionType, NumericParam, PrintfError, Result};

/// Trait for types that can be formatted using printf strings
///
/// Implemented for the basic types and shouldn't need implementing for
/// anything else.
pub trait Printf {
    /// Format `self` based on the conversion configured in `spec`.
    fn format(&self, spec: &ConversionSpecifier) -> Result<String>;
    /// Get `self` as an integer for use as a field width, if possible.
    fn as_int(&self) -> Option<i32>;
}

impl Printf for u64 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        let mut base = 10;
        let mut digits: Vec<char> = "0123456789".chars().collect();
        let mut alt_prefix = "";
        match spec.conversion_type {
            ConversionType::DecInt => {}
            ConversionType::HexIntLower => {
                base = 16;
                digits = "0123456789abcdef".chars().collect();
                alt_prefix = "0x";
            }
            ConversionType::HexIntUpper => {
                base = 16;
                digits = "0123456789ABCDEF".chars().collect();
                alt_prefix = "0X";
            }
            ConversionType::OctInt => {
                base = 8;
                digits = "01234567".chars().collect();
                alt_prefix = "0";
            }
            _ => {
                return Err(PrintfError::WrongType);
            }
        }
        let prefix = if spec.alt_form {
            alt_prefix.to_owned()
        } else {
            String::new()
        };

        // Build the actual number (in reverse)
        let mut rev_num = String::new();
        let mut n = *self;
        while n > 0 {
            let digit = n % base;
            n /= base;
            rev_num.push(digits[digit as usize]);
        }
        if rev_num.is_empty() {
            rev_num.push('0');
        }

        // Take care of padding
        let width: usize = match spec.width {
            NumericParam::Literal(w) => w,
            _ => {
                return Err(PrintfError::Unknown); // should not happen at this point!!
            }
        }
        .try_into()
        .unwrap_or_default();
        let formatted = if spec.left_adj {
            let mut num_str = prefix + &rev_num.chars().rev().collect::<String>();
            while num_str.len() < width {
                num_str.push(' ');
            }
            num_str
        } else if spec.zero_pad {
            while prefix.len() + rev_num.len() < width {
                rev_num.push('0');
            }
            prefix + &rev_num.chars().rev().collect::<String>()
        } else {
            let mut num_str = prefix + &rev_num.chars().rev().collect::<String>();
            while num_str.len() < width {
                num_str = " ".to_owned() + &num_str;
            }
            num_str
        };

        Ok(formatted)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Printf for i64 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            // signed integer format
            ConversionType::DecInt => {
                // do I need a sign prefix?
                let negative = *self < 0;
                let abs_val = self.abs();
                let sign_prefix = if negative {
                    "-"
                } else if spec.force_sign {
                    "+"
                } else if spec.space_sign {
                    " "
                } else {
                    ""
                }
                .to_owned();
                let mut mod_spec = *spec;
                mod_spec.width = match spec.width {
                    NumericParam::Literal(w) => NumericParam::Literal(w - sign_prefix.len() as i32),
                    _ => {
                        return Err(PrintfError::Unknown);
                    }
                };

                let formatted = (abs_val as u64).format(&mod_spec)?;
                // put the sign a after any leading spaces
                let mut actual_number = &formatted[0..];
                let mut leading_spaces = &formatted[0..0];
                if let Some(first_non_space) = formatted.find(|c| c != ' ') {
                    actual_number = &formatted[first_non_space..];
                    leading_spaces = &formatted[0..first_non_space];
                }
                Ok(leading_spaces.to_owned() + &sign_prefix + actual_number)
            }
            // unsigned-only formats
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u64).format(spec)
            }
            _ => Err(PrintfError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Printf for i32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            // signed integer format
            ConversionType::DecInt => (*self as i64).format(spec),
            // unsigned-only formats
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u32).format(spec)
            }
            _ => Err(PrintfError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self)
    }
}

impl Printf for u32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Printf for i16 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            // signed integer format
            ConversionType::DecInt => (*self as i64).format(spec),
            // unsigned-only formats
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u16).format(spec)
            }
            _ => Err(PrintfError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Printf for u16 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Printf for i8 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            // signed integer format
            ConversionType::DecInt => (*self as i64).format(spec),
            // unsigned-only formats
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u8).format(spec)
            }
            _ => Err(PrintfError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Printf for u8 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Printf for usize {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Printf for isize {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Printf for f64 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        let mut prefix = String::new();
        let mut number = String::new();

        // set up the sign
        if self.is_sign_negative() {
            prefix.push('-');
        } else if spec.space_sign {
            prefix.push(' ');
        } else if spec.force_sign {
            prefix.push('+');
        }

        if self.is_finite() {
            let mut use_scientific = false;
            let mut exp_symb = 'e';
            let mut strip_trailing_0s = false;
            let mut abs = self.abs();
            let mut exponent = abs.log10().floor() as i32;
            let mut precision = match spec.precision {
                NumericParam::Literal(p) => p,
                _ => {
                    return Err(PrintfError::Unknown);
                }
            };
            if precision <= 0 {
                precision = 0;
            }
            match spec.conversion_type {
                ConversionType::DecFloatLower | ConversionType::DecFloatUpper => {
                    // default
                }
                ConversionType::SciFloatLower => {
                    use_scientific = true;
                }
                ConversionType::SciFloatUpper => {
                    use_scientific = true;
                    exp_symb = 'E';
                }
                ConversionType::CompactFloatLower | ConversionType::CompactFloatUpper => {
                    if spec.conversion_type == ConversionType::CompactFloatUpper {
                        exp_symb = 'E'
                    }
                    strip_trailing_0s = true;
                    if precision == 0 {
                        precision = 1;
                    }
                    // exponent signifies significant digits - we must round now
                    // to (re)calculate the exponent
                    let rounding_factor = 10.0_f64.powf((precision - 1 - exponent) as f64);
                    let rounded_fixed = (abs * rounding_factor).round();
                    abs = rounded_fixed / rounding_factor;
                    exponent = abs.log10().floor() as i32;
                    if exponent < -4 || exponent >= precision {
                        use_scientific = true;
                        precision -= 1;
                    } else {
                        // precision specifies the number of significant digits
                        precision -= 1 + exponent;
                    }
                }
                _ => {
                    return Err(PrintfError::WrongType);
                }
            }

            if use_scientific {
                let mut normal = abs / 10.0_f64.powf(exponent as f64);

                if precision > 0 {
                    let mut int_part = normal.trunc();
                    let mut exp_factor = 10.0_f64.powf(precision as f64);
                    let mut tail = ((normal - int_part) * exp_factor).round() as u64;
                    while tail >= exp_factor as u64 {
                        // Overflow, must round
                        int_part += 1.0;
                        tail -= exp_factor as u64;
                        if int_part >= 10.0 {
                            exponent += 1;
                            // keep same precision - recalculate tail
                            exp_factor /= 10.0;
                            normal /= 10.0;
                            int_part = normal.trunc();
                            tail = ((normal - int_part) * exp_factor).round() as u64;
                        }
                    }

                    let mut rev_tail_str = String::new();
                    for _ in 0..precision {
                        rev_tail_str.push((b'0' + (tail % 10) as u8) as char);
                        tail /= 10;
                    }
                    number.push_str(&format!("{}", int_part));
                    number.push('.');
                    number.push_str(&rev_tail_str.chars().rev().collect::<String>());
                    if strip_trailing_0s {
                        number = number.trim_end_matches('0').to_owned();
                    }
                } else {
                    number.push_str(&format!("{}", normal.round()));
                }
                number.push(exp_symb);
                number.push_str(&format!("{:+03}", exponent));
            } else {
                if precision > 0 {
                    let mut int_part = abs.trunc() as i64;
                    let mut tail =
                        ((abs - abs.trunc()) * 10.0_f64.powf(precision as f64)).round() as u64;
                    let mut rev_tail_str = String::new();
                    for _ in 0..precision {
                        rev_tail_str.push((b'0' + (tail % 10) as u8) as char);
                        tail /= 10;
                    }
                    if tail > 0 {
                        // overflow - we must round up
                        int_part += 1;
                    }
                    number.push_str(&format!("{}", int_part));
                    number.push('.');
                    number.push_str(&rev_tail_str.chars().rev().collect::<String>());
                    if strip_trailing_0s {
                        number = number.trim_end_matches('0').to_owned();
                    }
                } else {
                    number.push_str(&format!("{}", abs.round()));
                }
            }
        } else {
            // not finite
            match spec.conversion_type {
                ConversionType::DecFloatLower
                | ConversionType::SciFloatLower
                | ConversionType::CompactFloatLower => {
                    if self.is_infinite() {
                        number.push_str("inf")
                    } else {
                        number.push_str("nan")
                    }
                }
                ConversionType::DecFloatUpper
                | ConversionType::SciFloatUpper
                | ConversionType::CompactFloatUpper => {
                    if self.is_infinite() {
                        number.push_str("INF")
                    } else {
                        number.push_str("NAN")
                    }
                }
                _ => {
                    return Err(PrintfError::WrongType);
                }
            }
        }
        // Take care of padding
        let width: usize = match spec.width {
            NumericParam::Literal(w) => w,
            _ => {
                return Err(PrintfError::Unknown); // should not happen at this point!!
            }
        }
        .try_into()
        .unwrap_or_default();
        let formatted = if spec.left_adj {
            let mut full_num = prefix + &number;
            while full_num.len() < width {
                full_num.push(' ');
            }
            full_num
        } else if spec.zero_pad && self.is_finite() {
            while prefix.len() + number.len() < width {
                prefix.push('0');
            }
            prefix + &number
        } else {
            let mut full_num = prefix + &number;
            while full_num.len() < width {
                full_num = " ".to_owned() + &full_num;
            }
            full_num
        };
        Ok(formatted)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Printf for f32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as f64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Printf for &str {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        if spec.conversion_type == ConversionType::String {
            Ok((*self).to_owned())
        } else {
            Err(PrintfError::WrongType)
        }
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Printf for char {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        if spec.conversion_type == ConversionType::Char {
            let mut s = String::new();
            s.push(*self);
            Ok(s)
        } else {
            Err(PrintfError::WrongType)
        }
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Printf for String {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (self as &str).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}
