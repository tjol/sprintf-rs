use fhex::ToHex;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};

use crate::{
    FormatError, Result,
    parser::{ConversionSpecifier, ConversionType, NumericParam},
};

pub trait Format {
    /// Format `self` based on the conversion configured in `spec`.
    fn format(&self, spec: &ConversionSpecifier) -> Result<String>;
    /// Get `self` as an integer for use as a field width, if possible.
    fn as_int(&self) -> Option<i32>;
}

impl Format for u64 {
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
                return Err(FormatError::WrongType);
            }
        }
        let prefix = if spec.alt_form {
            alt_prefix.to_owned()
        } else {
            String::new()
        };

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

        let width: usize = match spec.width {
            NumericParam::Literal(w) => w,
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

impl Format for i64 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::DecInt => {
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
                };

                let formatted = (abs_val as u64).format(&mod_spec)?;
                let mut actual_number = &formatted[0..];
                let mut leading_spaces = &formatted[0..0];
                if let Some(first_non_space) = formatted.find(|c| c != ' ') {
                    actual_number = &formatted[first_non_space..];
                    leading_spaces = &formatted[0..first_non_space];
                }
                Ok(leading_spaces.to_owned() + &sign_prefix + actual_number)
            }
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u64).format(spec)
            }
            _ => Err(FormatError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Format for i32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            // signed integer format
            ConversionType::DecInt => (*self as i64).format(spec),
            // unsigned-only formats
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u32).format(spec)
            }
            _ => Err(FormatError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self)
    }
}

impl Format for u32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::Char => {
                if let Some(c) = char::from_u32(*self) {
                    c.format(spec)
                } else {
                    Err(FormatError::WrongType)
                }
            }
            _ => (*self as u64).format(spec),
        }
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Format for i16 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::DecInt => (*self as i64).format(spec),
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u16).format(spec)
            }
            _ => Err(FormatError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Format for u16 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::Char => {
                if let Some(Ok(c)) = char::decode_utf16([*self]).next() {
                    c.format(spec)
                } else {
                    Err(FormatError::WrongType)
                }
            }
            _ => (*self as u64).format(spec),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Format for i8 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::DecInt => (*self as i64).format(spec),
            ConversionType::HexIntLower | ConversionType::HexIntUpper | ConversionType::OctInt => {
                (*self as u8).format(spec)
            }
            // c_char
            ConversionType::Char => (*self as u8).format(spec),
            _ => Err(FormatError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Format for u8 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::Char => {
                if self.is_ascii() {
                    char::from(*self).format(spec)
                } else {
                    Err(FormatError::WrongType)
                }
            }
            _ => (*self as u64).format(spec),
        }
    }
    fn as_int(&self) -> Option<i32> {
        Some(*self as i32)
    }
}

impl Format for usize {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Format for isize {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as u64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        i32::try_from(*self).ok()
    }
}

impl Format for f64 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        let mut prefix = String::new();
        let mut number;

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
            let mut use_hex = false;
            let mut hex_uppercase = false;
            let abs = self.abs();
            let mut exponent;
            let NumericParam::Literal(mut precision) = spec.precision;
            if precision <= 0 {
                precision = 0;
            }

            match spec.conversion_type {
                ConversionType::DecFloatLower | ConversionType::DecFloatUpper => {}
                ConversionType::SciFloatLower => {
                    use_scientific = true;
                }
                ConversionType::SciFloatUpper => {
                    use_scientific = true;
                    exp_symb = 'E';
                }
                ConversionType::CompactFloatLower | ConversionType::CompactFloatUpper => {
                    if spec.conversion_type == ConversionType::CompactFloatUpper {
                        exp_symb = 'E';
                    }
                    strip_trailing_0s = true;
                    if precision == 0 {
                        precision = 1;
                    }
                    exponent = abs.log10().floor() as i32;
                    let rounding_factor = 10.0_f64.powf((precision - 1 - exponent) as f64);
                    let rounded_fixed = (abs * rounding_factor).round();
                    let abs_rounded = rounded_fixed / rounding_factor;
                    exponent = abs_rounded.log10().floor() as i32;
                    if exponent < -4 || exponent >= precision {
                        use_scientific = true;
                        precision -= 1;
                    } else {
                        precision -= 1 + exponent;
                    }
                }
                ConversionType::HexFloatLower => {
                    use_hex = true;
                }
                ConversionType::HexFloatUpper => {
                    use_hex = true;
                    hex_uppercase = true;
                }
                _ => {
                    return Err(FormatError::WrongType);
                }
            }

            if use_hex {
                number = abs.to_hex();

                if number.starts_with('-') {
                    number = number[1..].to_string();
                }

                if hex_uppercase {
                    number = number.to_uppercase();
                }
            } else if use_scientific {
                exponent = abs.log10().floor() as i32;
                let mut normal = abs / 10.0_f64.powf(exponent as f64);
                number = String::new();

                if precision > 0 {
                    let mut int_part = normal.trunc();
                    let mut exp_factor = 10.0_f64.powf(precision as f64);
                    let mut tail = ((normal - int_part) * exp_factor).round() as u64;
                    while tail >= exp_factor as u64 {
                        int_part += 1.0;
                        tail -= exp_factor as u64;
                        if int_part >= 10.0 {
                            exponent += 1;
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
                number = String::new();
                if precision > 0 {
                    let mut int_part = abs.trunc();
                    let exp_factor = 10.0_f64.powf(precision as f64);
                    let mut tail = ((abs - int_part) * exp_factor).round() as u64;
                    let mut rev_tail_str = String::new();
                    if tail >= exp_factor as u64 {
                        int_part += 1.0;
                        tail -= exp_factor as u64;
                    }
                    for _ in 0..precision {
                        rev_tail_str.push((b'0' + (tail % 10) as u8) as char);
                        tail /= 10;
                    }
                    number.push_str(&format!("{}", int_part));
                    number.push('.');
                    number.push_str(&rev_tail_str.chars().rev().collect::<String>());
                    if strip_trailing_0s {
                        number = number.trim_end_matches('0').to_owned();
                        if number.ends_with('.') {
                            number.pop();
                        }
                    }
                } else {
                    number.push_str(&format!("{}", abs.round()));
                }
            }
        } else {
            number = String::new();
            match spec.conversion_type {
                ConversionType::DecFloatLower
                | ConversionType::SciFloatLower
                | ConversionType::CompactFloatLower
                | ConversionType::HexFloatLower => {
                    if self.is_infinite() {
                        number.push_str("inf")
                    } else {
                        number.push_str("nan")
                    }
                }
                ConversionType::DecFloatUpper
                | ConversionType::SciFloatUpper
                | ConversionType::CompactFloatUpper
                | ConversionType::HexFloatUpper => {
                    if self.is_infinite() {
                        number.push_str("INF")
                    } else {
                        number.push_str("NAN")
                    }
                }
                _ => {
                    return Err(FormatError::WrongType);
                }
            }
        }
        // Take care of padding
        let width: usize = match spec.width {
            NumericParam::Literal(w) => w,
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

impl Format for f32 {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as f64).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Format for &str {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        match spec.conversion_type {
            ConversionType::String => {
                println!("String");
                let mut s = String::new();

                let width: usize = match spec.width {
                    NumericParam::Literal(w) => w,
                }
                .try_into()
                .unwrap_or_default();

                if spec.left_adj {
                    s.push_str(self);
                    while s.len() < width {
                        s.push(' ');
                    }
                } else {
                    while s.len() + self.len() < width {
                        s.push(' ');
                    }
                    s.push_str(self);
                }
                Ok(s)
            }
            ConversionType::QuotedString => {
                println!("QuotedString");
                let mut quoted = String::with_capacity(self.len() + 2);
                quoted.push('"');
                for byte in self.bytes() {
                    match byte {
                        b'"' => quoted.push_str("\\\""),
                        b'\\' => quoted.push_str("\\\\"),
                        b'\n' => quoted.push_str("\\n"),
                        b'\r' => quoted.push_str("\\r"),
                        b'\0' => quoted.push_str("\\0"),
                        b if (1..=31).contains(&b) || b == 127 => {
                            quoted.push_str(&format!("\\{}", b));
                        }
                        b => quoted.push(b as char),
                    }
                }
                quoted.push('"');
                Ok(quoted)
            }
            _ => Err(FormatError::WrongType),
        }
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Format for char {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        if spec.conversion_type == ConversionType::Char {
            let mut s = String::new();

            let width: usize = match spec.width {
                NumericParam::Literal(w) => w,
            }
            .try_into()
            .unwrap_or_default();

            if spec.left_adj {
                s.push(*self);
                while s.len() < width {
                    s.push(' ');
                }
            } else {
                while s.len() + self.len_utf8() < width {
                    s.push(' ');
                }
                s.push(*self);
            }
            Ok(s)
        } else {
            Err(FormatError::WrongType)
        }
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Format for String {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (self as &str).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Format for &CStr {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        if let Ok(s) = self.to_str() {
            s.format(spec)
        } else {
            Err(FormatError::WrongType)
        }
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl Format for CString {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        self.as_c_str().format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl<T> Format for *const T {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as usize).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}

impl<T> Format for *mut T {
    fn format(&self, spec: &ConversionSpecifier) -> Result<String> {
        (*self as usize).format(spec)
    }
    fn as_int(&self) -> Option<i32> {
        None
    }
}
