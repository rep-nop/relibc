use super::{c_int, c_ulong, wchar_t, wctype_t, wint_t};

mod tables;

pub type wctype_t = c_ulong;
pub type wctrans_t = *const c_int;

pub const WEOF: wint_t = 0xFFFF;

#[no_mangle]
pub extern "C" fn iswalnum(c: wint_t) -> c_int {
    if iswalpha(c) || iswdigit(c) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswalpha(c: wint_t) -> c_int {
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
pub extern "C" fn iswcntrl(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswdigit(c: wint_t) -> c_int {
    // FIXME: Casting? There doesn't seem to be a `wuint_t` type.
    ((c as u16 - b'0' as u16) < 10) as c_int
}

#[no_mangle]
pub extern "C" fn iswgraph(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswlower(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswprint(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswpunct(c: wint_t) -> c_int {
    use tables::PUNCT;

    if c < 0x20000 {
        (PUNCT[(PUNCT[(c >> 8) as usize] * 32 + ((c & 255) >> 3)) as usize] >> (c & 7)) & 1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn iswspace(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswupper(c: wint_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn iswxdigit(c: wint_t) -> c_int {
    // FIXME: Casting? There doesn't seem to be a `wuint_t` type.
    let result_dec = c as u16 - b'0' as u16;
    let result_hex = ((c as u16) | 32) - b'a' as u16;

    (result_dec < 10 || result_hex < 6) as c_int
}

#[no_mangle]
pub extern "C" fn iswctype(c: wint_t, t: wctype_t) -> c_int {
    unimplemented!()
}

#[no_mangle]
pub extern "C" fn towctrans(c: wint_T, t: wctr) -> c_int {
    unimplemented!()
}
