//! wctype implementation for Redox, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/wctype.h.html

#![no_std]

extern crate platform;

mod tables;

use platform::types::*;

#[no_mangle]
pub extern "C" fn iswalnum(wc: wint_t) -> c_int {
    if iswalpha(c) || iswdigit(c) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswalpha(wc: wint_t) -> c_int {
    use tables::ALPHA;

    if c < 0x20000 {
        (ALPHA[(ALPHA[(c >> 8) as usize] * 32 + ((c & 255) >> 3)) as usize] >> (c & 7)) & 1
    } else if c < 0x2FFFE {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswcntrl(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswdigit(wc: wint_t) -> c_int {
    // FIXME: Casting? There doesn't seem to be a `wuint_t` type.
    ((c as u16 - b'0' as u16) < 10) as c_int
}

#[no_mangle]
pub extern "C" fn iswgraph(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswlower(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswprint(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswpunct(wc: wint_t) -> c_int {
    use tables::PUNCT;

    if c < 0x20000 {
        (PUNCT[(PUNCT[(c >> 8) as usize] * 32 + ((c & 255) >> 3)) as usize] >> (c & 7)) & 1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswspace(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswupper(wc: wint_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn iswxdigit(wc: wint_t) -> c_int {
    // FIXME: Casting? There doesn't seem to be a `wuint_t` type.
    let result_dec = c as u16 - b'0' as u16;
    let result_hex = ((c as u16) | 32) - b'a' as u16;

    (result_dec < 10 || result_hex < 6) as c_int
}

#[no_mangle]
pub extern "C" fn iswctype(wc: wint_t, charclass: wctype_t) -> c_int {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn towlower(wc: wint_t) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn towupper(wc: wint_t) -> wint_t {
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn wctype(property: *const c_char) -> c_int {
    unimplemented!();
}
