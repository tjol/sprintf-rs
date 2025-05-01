use lformat::format;

#[test]
fn test_quoted_string_format() {
    let s = format("%q", &[&"hello"]).unwrap();
    assert_eq!(s, "\"hello\"");

    let s = format("%q", &[&"hello\nworld"]).unwrap();
    assert_eq!(s, "\"hello\\nworld\"");

    let s = format("%q", &[&"hello \"world\" \\test"]).unwrap();
    assert_eq!(s, "\"hello \\\"world\\\" \\\\test\"");

    let s = format("%q", &[&""]).unwrap();
    assert_eq!(s, "\"\"");

    let s = format("%q", &[&"hello\0world\x01\x02"]).unwrap();
    assert_eq!(s, "\"hello\\0world\\1\\2\"");
}

#[test]
fn test_hex_float_format() {
    let s = format("%a", &[&1.5f64]).unwrap();
    assert!(
        s.contains("0x1.8p+0") || s.contains("0x1.8000"),
        "Expected hex float format, got: {}",
        s
    );

    let s = format("%A", &[&1.5f64]).unwrap();
    assert!(
        s.contains("0X1.8P+0") || s.contains("0X1.8000"),
        "Expected uppercase hex float format, got: {}",
        s
    );

    let s = format("%a", &[&0.0f64]).unwrap();
    assert!(
        s.contains("0x0") || s.contains("0x0.0"),
        "Expected hex float format for zero, got: {}",
        s
    );

    let s = format("%a", &[&-16.0f64]).unwrap();
    assert!(
        s.contains("-0x1p+4") || s.contains("-0x1.0000"),
        "Expected negative hex float format, got: {}",
        s
    );

    let s = format("%a", &[&1e-10f64]).unwrap();
    assert!(
        s.contains("p-") || s.contains("0x"),
        "Expected scientific hex float notation, got: {}",
        s
    );

    let s = format("%20.10a", &[&1.5f64]).unwrap();
    assert_eq!(s.len(), 20);
}

#[test]
fn test_float_formats() {
    let s = format("%f", &[&1.5f64]).unwrap();
    assert_eq!(s, "1.500000");

    let s = format("%e", &[&1.5f64]).unwrap();
    assert!(s.contains("1.500000e+00") || s.contains("1.500000e+0"));

    let s = format("%g", &[&1.5f64]).unwrap();
    assert_eq!(s, "1.5");

    let s = format("%F", &[&1.5f64]).unwrap();
    assert_eq!(s, "1.500000");

    let s = format("%E", &[&1.5f64]).unwrap();
    assert!(s.contains("1.500000E+00") || s.contains("1.500000E+0"));

    let s = format("%G", &[&1.5f64]).unwrap();
    assert_eq!(s, "1.5");
}

#[test]
fn test_integer_formats() {
    let s = format("%d", &[&42]).unwrap();
    assert_eq!(s, "42");

    let s = format("%o", &[&42]).unwrap();
    assert_eq!(s, "52");

    let s = format("%x", &[&42]).unwrap();
    assert_eq!(s, "2a");

    let s = format("%X", &[&42]).unwrap();
    assert_eq!(s, "2A");

    let s = format("%#x", &[&42]).unwrap();
    assert_eq!(s, "0x2a");

    let s = format("%#X", &[&42]).unwrap();
    assert_eq!(s, "0X2A");

    let s = format("%+d", &[&42]).unwrap();
    assert_eq!(s, "+42");

    let s = format("% d", &[&42]).unwrap();
    assert_eq!(s, " 42");

    let s = format("%06d", &[&42]).unwrap();
    assert_eq!(s, "000042");
}

#[test]
fn test_string_and_float_formatting() {
    let s = format("%.4f %.4e %.4g %q %a", &[&1.5, &1.5, &1.5, &"test", &1.5]).unwrap();
    assert!(s.contains("1.5000 1.5000e") && s.contains("1.5 \"test\"") && s.contains("0x"));

    let s = format("%s %q", &[&"regular", &"quoted\"string"]).unwrap();
    assert_eq!(s, "regular \"quoted\\\"string\"");
}

#[test]
fn test_width_and_alignment() {
    let s = format("%-10s", &[&"left"]).unwrap();
    assert_eq!(s, "left      ");

    let s = format("%10s", &[&"right"]).unwrap();
    assert_eq!(s, "     right");

    let s = format("%010d", &[&42]).unwrap();
    assert_eq!(s, "0000000042");
}

#[test]
fn test_error_handling() {
    let result = format("%d %d", &[&1]);
    assert!(result.is_err());

    let result = format("%d", &[&1, &2]);
    assert!(result.is_err());

    let result = format("%d", &[&"string"]);
    assert!(result.is_err());
}
