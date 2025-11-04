use std::ffi::c_void;
use std::ptr;
use crate::{encrypt_and_embed_png, extract_and_decrypt_png};

#[no_mangle]
pub extern "C" fn stgplus_encrypt_embed_png(
    cover_ptr: *const u8, cover_len: usize,
    msg_ptr: *const u8, msg_len: usize,
    pass_ptr: *const u8, pass_len: usize,
    out_ptr: *mut *mut u8, out_len: *mut usize
) -> i32 {
    unsafe {
        if cover_ptr.is_null() || msg_ptr.is_null() || pass_ptr.is_null() || out_ptr.is_null() || out_len.is_null() {
            return -1;
        }
        let cover = std::slice::from_raw_parts(cover_ptr, cover_len);
        let msg = std::slice::from_raw_parts(msg_ptr, msg_len);
        let pass = std::slice::from_raw_parts(pass_ptr, pass_len);

        match encrypt_and_embed_png(cover, msg, pass) {
            Ok(buf) => {
                let len = buf.len();
                let mem = libc::malloc(len) as *mut u8;
                if mem.is_null() { return -2; }
                ptr::copy_nonoverlapping(buf.as_ptr(), mem, len);
                *out_ptr = mem;
                *out_len = len;
                0
            }
            Err(_) => -3
        }
    }
}

#[no_mangle]
pub extern "C" fn stgplus_extract_decrypt_png(
    stego_ptr: *const u8, stego_len: usize,
    pass_ptr: *const u8, pass_len: usize,
    out_ptr: *mut *mut u8, out_len: *mut usize
) -> i32 {
    unsafe {
        if stego_ptr.is_null() || pass_ptr.is_null() || out_ptr.is_null() || out_len.is_null() {
            return -1;
        }
        let stego = std::slice::from_raw_parts(stego_ptr, stego_len);
        let pass = std::slice::from_raw_parts(pass_ptr, pass_len);

        match extract_and_decrypt_png(stego, pass) {
            Ok(buf) => {
                let len = buf.len();
                let mem = libc::malloc(len) as *mut u8;
                if mem.is_null() { return -2; }
                ptr::copy_nonoverlapping(buf.as_ptr(), mem, len);
                *out_ptr = mem;
                *out_len = len;
                0
            }
            Err(_) => -3
        }
    }
}

#[no_mangle]
pub extern "C" fn stgplus_free(ptr_: *mut u8, _len: usize) {
    unsafe {
        if !ptr_.is_null() {
            libc::free(ptr_ as *mut c_void);
        }
    }
}
