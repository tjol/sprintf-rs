// The libc crate on Windows doesn't have snprintf
#![cfg(not(windows))]

use std::convert::{TryFrom, TryInto};
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_char;

use libc::snprintf;

use sprintf::*;

fn check_fmt<T: Printf>(fmt: &str, arg: T) {
    let our_result = sprintf!(fmt, arg).unwrap();
    let mut buf = vec![0_u8; our_result.len() + 1];
    let cfmt = CString::new(fmt).unwrap();
    let clen: usize = unsafe {
        snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            cfmt.as_ptr(),
            arg,
        )
    }
    .try_into()
    .unwrap();
    buf.truncate(clen); // drop the final '\0', etc.
    let c_result = String::from_utf8(buf).unwrap();
    assert_eq!(our_result, c_result);
}

fn check_fmt_s(fmt: &str, arg: &str) {
    let our_result = sprintf!(fmt, arg).unwrap();
    let mut buf = vec![0_u8; our_result.len() + 1];
    let cfmt = CString::new(fmt).unwrap();
    let carg = CString::new(arg).unwrap();
    let clen: usize = unsafe {
        snprintf(
            buf.as_mut_ptr() as *mut c_char,
            buf.len(),
            cfmt.as_ptr(),
            carg.as_ptr(),
        )
    }
    .try_into()
    .unwrap();
    buf.truncate(clen); // drop the final '\0', etc.
    let c_result = String::from_utf8(buf).unwrap();
    assert_eq!(our_result, c_result);
}

#[test]
fn test_int() {
    check_fmt("%d", 12);
    check_fmt("~%d~", 148);
    check_fmt("00%dxx", -91232);
    check_fmt("%x", -9232);
    check_fmt("%X", 432);
    check_fmt("%09X", 432);
    check_fmt("%9X", 432);
    check_fmt("%+9X", 492);
    check_fmt("% #9x", 4589);
    check_fmt("%2o", 4);
    check_fmt("% 12d", -4);
    check_fmt("% 12d", 48);
    check_fmt("%ld", -4_i64);
    check_fmt("%lX", -4_i64);
    check_fmt("%ld", 48_i64);
    check_fmt("%-8hd", -12_i16);
    check_fmt("%llx", 0x0123456789abcdef_u64);
}

#[test]
fn test_float() {
    check_fmt("%f", -46.38);
    check_fmt("%012.3f", 1.2);
    check_fmt("%0012.3f", 1.2);
    check_fmt("%012.3e", 1.7);
    check_fmt("%e", 1e300);
    check_fmt("%012.3g%%!", 2.6);
    check_fmt("%012.5G", -2.69);
    check_fmt("%+7.4f", 42.785);
    check_fmt("{}% 7.4E", 493.12);
    check_fmt("% 7.4E", -120.3);
    check_fmt("%-10F", f64::INFINITY);
    check_fmt("%+010F", f64::INFINITY);
    check_fmt("%.0f", 9.99);
    check_fmt("%.1f", 999.99);
    check_fmt("%.1f", 9.99);
    check_fmt("%.1e", 9.99);
    check_fmt("%.2f", 9.99);
    check_fmt("%.2e", 9.99);
    check_fmt("%.3f", 9.99);
    check_fmt("%.3e", 9.99);
    check_fmt("%.1g", 9.99);
    check_fmt("%.1G", 9.99);
    check_fmt("%.1f", 2.99);
    check_fmt("%.1e", 2.99);
    check_fmt("%.1g", 2.99);
    check_fmt("%.1f", 2.599);
    check_fmt("%.1e", 2.599);
    check_fmt("%.1g", 2.599);

    // MacOS libc behaves differently from glibc for nan. glibc is the reference implementation.
    if cfg!(target_env = "gnu") {
        check_fmt("% f", f64::NAN);
        check_fmt("%+f", f64::NAN);
    } else {
        assert_eq!(sprintf!("% f", f64::NAN).unwrap(), " nan");
        assert_eq!(sprintf!("%+f", f64::NAN).unwrap(), "+nan");
    }
}

#[test]
fn test_str() {
    check_fmt_s("test %% with string: %s yay\n", "FOO");
    check_fmt_s(
        "%s",
        "testing with a slightly longer string to make sure it doesn't truncate",
    );
    check_fmt("test char %c", '~');
    let c_string = CString::new("test").unwrap();
    check_fmt("%s", c_string.as_c_str());
    check_fmt("%s", c_string);
    check_fmt_s("%4s", "A");
    check_fmt_s("%4s", "𒀀"); // multi-byte character test (4 bytes)
    check_fmt_s("%-4sX", "A");
    check_fmt_s("%-4sX", "𒀀"); // multi-byte character test (4 bytes)
    check_fmt_s("%1.3s", "ABCDEFG");
    check_fmt_s("%1.4s", "𒀀𒀀"); // multi-byte character test (4 bytes per char)
    check_fmt_s("%8.4s", "ABCDEFG");

    // glibc does not handle UTF-8 strings correctly when truncating, but we cannot produce malformed UTF-8
    // strings in Rust. Instead, we round down to the nearest character boundary.
    assert_eq!(sprintf!("%1.1s", "𒀀𒀀𒀀").unwrap(), " ");
    assert_eq!(sprintf!("%1.2s", "𒀀𒀀𒀀").unwrap(), " ");
    assert_eq!(sprintf!("%1.3s", "𒀀𒀀𒀀").unwrap(), " ");
    assert_eq!(sprintf!("%1.4s", "𒀀𒀀𒀀").unwrap(), "𒀀");
    assert_eq!(sprintf!("%1.5s", "𒀀𒀀𒀀").unwrap(), "𒀀");
    assert_eq!(sprintf!("%1.6s", "𒀀𒀀𒀀").unwrap(), "𒀀");
    assert_eq!(sprintf!("%1.7s", "𒀀𒀀𒀀").unwrap(), "𒀀");
    assert_eq!(sprintf!("%1.8s", "𒀀𒀀𒀀").unwrap(), "𒀀𒀀");
}

#[test]
fn test_char() {
    check_fmt("%c", 'x');
    check_fmt("%c", b'x');
    check_fmt("%c", b'x' as c_char);
    check_fmt("%c", u16::try_from('x').unwrap());
    check_fmt("%c", u32::try_from('x').unwrap());
    check_fmt("%4c", 'A');
    check_fmt("%-4cX", 'A');
}

#[test]
fn test_sanity() {
    // u8 must not misinterpret bytes from multi-byte UTF-8 characters
    let bytes = "∆".as_bytes();
    assert!(bytes.len() > 1);
    assert_eq!(sprintf!("%c", bytes[0]), Err(PrintfError::WrongType));
}

#[test]
fn test_ptr() {
    let buf: [u8; 4] = [0; 4];
    let ptr_const: *const u8 = buf.as_ptr();
    let ptr_mut: *mut u8 = ptr_const.cast_mut();

    // pointer: expects usize and pointer to have the same size
    assert_eq!(size_of::<usize>(), size_of::<*const u8>(),);
    check_fmt("%p", ptr_const);
    check_fmt("%p", ptr_mut);

    // numeric: works the same as libc if you use the correct length specifier
    if size_of::<usize>() == size_of::<u64>() {
        check_fmt("%llx", ptr_const);
        check_fmt("%llx", ptr_mut);
        check_fmt("%#llx", ptr_const);
        check_fmt("%#llx", ptr_mut);
    } else if size_of::<usize>() == size_of::<u32>() {
        check_fmt("%x", ptr_const);
        check_fmt("%x", ptr_mut);
        check_fmt("%#x", ptr_const);
        check_fmt("%#x", ptr_mut);
    }
}
