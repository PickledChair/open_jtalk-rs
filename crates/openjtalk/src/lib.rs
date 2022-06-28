use std::ffi::{CStr, CString};

#[repr(i32)]
pub enum Text2MecabError {
    Range = openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_RANGE_ERROR as i32,
    InvalidArgument = openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_INVALID_ARGUMENT as i32,
}

pub fn text2mecab(text: impl AsRef<str>) -> Result<String, Text2MecabError> {
    const MAX_TEXT2MECAB_SIZE: usize = 8192;
    let mut output = String::with_capacity(MAX_TEXT2MECAB_SIZE);
    let text = CString::new(text.as_ref()).unwrap();
    let output_vec = unsafe { output.as_mut_vec() };

    let result = unsafe {
        openjtalk_sys::text2mecab(
            output_vec.as_mut_ptr() as *mut i8,
            MAX_TEXT2MECAB_SIZE,
            text.as_ptr(),
        )
    };
    if result == openjtalk_sys::text2mecab_result_t::TEXT2MECAB_RESULT_SUCCESS {
        unsafe {
            output_vec.set_len(
                CStr::from_bytes_with_nul_unchecked(output_vec)
                    .to_bytes()
                    .len(),
            )
        }

        Ok(output)
    } else {
        Err(unsafe { std::mem::transmute(result) })
    }
}
