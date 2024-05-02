use libc::c_void;
use objc2::encode::{Encoding, RefEncode};

#[repr(C)]
pub struct __CGColor(c_void);

pub type CGColorRef = *const __CGColor;

unsafe impl RefEncode for __CGColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("__CGColor", &[]));
}

#[repr(C)]
pub struct __CFString(c_void);

pub type CFStringRef = *const __CFString;

unsafe impl RefEncode for __CFString {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Encoding::Struct("__CFString", &[]));
}
