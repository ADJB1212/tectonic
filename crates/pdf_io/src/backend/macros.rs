macro_rules! pdf_format {
    ($format:expr $(, $arg:expr)* $(,)?) => {{
        let mut __tectonic_pdf_message_buffer = [0u8; 1024];
        unsafe {
            libc::snprintf(__tectonic_pdf_message_buffer.as_mut_ptr().cast(), __tectonic_pdf_message_buffer.len(), $format $(, $arg)*);
            std::ffi::CStr::from_ptr(__tectonic_pdf_message_buffer.as_ptr().cast()).to_owned()
        }
    }};
}
macro_rules! pdf_abort {
    ($($arg:expr),* $(,)?) => { crate::backend::runtime::raise(pdf_format!($($arg),*)) };
}
macro_rules! pdf_message {
    ($($arg:expr),* $(,)?) => { pdf_log!(false, $($arg),*) };
}
macro_rules! pdf_warning {
    ($($arg:expr),* $(,)?) => { pdf_log!(true, $($arg),*) };
}
macro_rules! pdf_log {
    ($warning:expr, $format:expr $(, $arg:expr)* $(,)?) => {{
        let mut __tectonic_pdf_message_buffer = [0u8; 1024];
        unsafe {
            libc::snprintf(__tectonic_pdf_message_buffer.as_mut_ptr().cast(), __tectonic_pdf_message_buffer.len(), $format $(, $arg)*);
            crate::backend::runtime::emit(std::ffi::CStr::from_ptr(__tectonic_pdf_message_buffer.as_ptr().cast()), $warning);
        }
    }};
}
