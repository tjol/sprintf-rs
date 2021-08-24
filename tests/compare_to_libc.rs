use std::ffi::CString;
use std::os::raw::c_char;
use std::convert::TryInto;

use libc::snprintf;

use sprintf::*;

fn check_fmt<T: Printf>(fmt: &str, arg: T) {
    let our_result = sprintf!(fmt, arg).unwrap();
    let mut buf = vec![0_u8; our_result.len() + 1];
    let cfmt = CString::new(fmt).unwrap();
    let clen: usize = unsafe {
        snprintf(buf.as_mut_ptr() as *mut c_char, buf.len(), cfmt.as_ptr(), arg)
    }.try_into().unwrap();
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
        snprintf(buf.as_mut_ptr() as *mut c_char, buf.len(), cfmt.as_ptr(), carg.as_ptr())
    }.try_into().unwrap();
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
}

#[test]
fn test_float() {
    check_fmt("%f", -46.38);
    check_fmt("%012.3f", 1.2);
    check_fmt("%012.3e", 1.7);
    check_fmt("%012.3g%%!", 2.6);
    check_fmt("%012.5G", -2.69);
    check_fmt("%+7.4f", 42.785);
    check_fmt("{}% 7.4E", 493.12);
    check_fmt("% 7.4E", -120.3);
    check_fmt("%-10F", f64::INFINITY);
    check_fmt("%+010F", f64::INFINITY);
    check_fmt("% f", f64::NAN);
    check_fmt("%+f", f64::NAN);
    check_fmt("%7f", -f64::NAN);
}

#[test]
fn test_str() {
    check_fmt_s("test %% with string: %s yay\n", "FOO");
    check_fmt("test char %c", '~');
}
