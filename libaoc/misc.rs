//! Miscellaneous functions that didn't really fit
//! anywhere else in this library.

/// Given a char representing an ASCII / Unicode
/// digit, return its value.
// XXX I swear there's a standard rust way to do this, but I
// have no memory of where it is.
pub fn digit_to_int(d: char) -> u8 {
    if d < '0' || d > '9' {
        panic!("non-digit: {}", d);
    }
    d as u8 - '0' as u8
}
