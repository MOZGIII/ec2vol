pub fn i8_as_u8<'a>(buf: &'a [i8]) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) }
}

pub fn string_from_buf(buf: &[i8]) -> String {
    String::from_utf8_lossy(i8_as_u8(buf)).trim_end().to_owned()
}
