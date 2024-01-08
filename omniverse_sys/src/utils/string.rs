use std::{
    ffi::{c_char, CStr, CString, NulError},
    mem::forget,
    slice::from_raw_parts,
    str::Utf8Error,
};

pub fn to_cstring_array_raw(values: impl IntoIterator<Item = CString>) -> *mut *mut c_char {
    let mut values = values
        .into_iter()
        .map(std::ffi::CString::into_raw)
        .collect::<Vec<_>>();
    let values_ptr = values.as_mut_ptr();
    forget(values);
    values_ptr
}

/// Converts an iterator of Rust strings to a pointer to an array of pointers to null-terminated C
/// strings.
///
/// # Errors
///
/// Returns an error if strings cannot be converted to a C strings.
pub fn to_cstring_array(
    values: impl IntoIterator<Item = String>,
) -> Result<*mut *mut c_char, NulError> {
    Ok(to_cstring_array_raw(
        values
            .into_iter()
            .map(CString::new)
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

// pub unsafe fn from_cstring_array_raw(data: *mut *mut c_char, count: usize) -> Vec<CString> {
//     from_raw_parts(data, count)
//         .iter()
//         .map(|s| CStr::from_ptr(*s).to_owned())
//         .collect()
// }

/// Converts a pointer to an array of pointers to null-terminated C strings to a vector of Rust
/// strings.
///
/// # Safety
///
/// `data` must be a valid pointer to an array of `count` pointers to null-terminated C strings.
///
/// # Errors
///
/// Returns an error if any of the C strings are not valid UTF-8.
pub unsafe fn from_cstring_array(
    data: *mut *mut c_char,
    count: usize,
) -> Result<Vec<String>, Utf8Error> {
    from_raw_parts(data, count)
        .iter()
        .map(|s| {
            CStr::from_ptr(*s)
                .to_str()
                .map(std::borrow::ToOwned::to_owned)
        })
        .collect()
}
