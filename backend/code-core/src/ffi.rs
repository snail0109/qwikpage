use std::ffi::CString;
use std::os::raw::c_char;

use crate::types::ffi::FfiResult;

use super::CodeGenerator;

#[repr(C)]
pub struct GeneratorHandle(*mut dyn CodeGenerator);

/// 安全转换函数
pub unsafe fn result_to_cstring<T: serde::Serialize>(result: Result<T, super::GeneratorError>) -> *mut c_char {
    let ffi_result: FfiResult<T> = result.into();
    match serde_json::to_string(&ffi_result) {
        Ok(s) => CString::new(s).unwrap().into_raw(),
        Err(e) => CString::new(format!(r#"{{"success":false,"error":"{}"}}"#, e)).unwrap().into_raw()
    }
}

/// 创建生成器句柄
pub unsafe fn create_generator(ptr: *mut dyn CodeGenerator) -> GeneratorHandle {
    GeneratorHandle(ptr)
}

/// 释放生成器资源
pub unsafe fn free_generator(handle: GeneratorHandle) {
    if !handle.0.is_null() {
        let _ = Box::from_raw(handle.0);
    }
}

/// 统一内存释放函数
#[no_mangle]
pub extern "C" fn common_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { CString::from_raw(s) };
    }
}
