// Lesson 12.3: Rust Shared Libraries in Go via cgo

// This lesson demonstrates how to build a Rust library that can be called from
// Go using `cgo`.

// --- Why Rust with Go? ---

// Go is excellent for building concurrent network services. However, for certain
// performance-critical tasks or when integrating with existing C/C++ libraries,
// Rust can offer advantages. Using Rust as a shared library allows Go to leverage
// Rust's performance and safety guarantees for specific components.

// --- Exposing Rust to Go ---

// The process is similar to exposing Rust to C, as `cgo` acts as a bridge to C.
// We need to:
// 1. Use `#[no_mangle]` to prevent Rust from mangling the function name.
// 2. Use `extern "C"` to tell Rust to use the C calling convention.

#[no_mangle]
pub extern "C" fn rust_add_for_go(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_greet_for_go(name_ptr: *const u8, name_len: usize) -> *mut u8 {
    let name_slice = unsafe { std::slice::from_raw_parts(name_ptr, name_len) };
    let name = String::from_utf8_lossy(name_slice);

    let greeting = format!("Hello, {} from Rust!", name);
    let c_string = std::ffi::CString::new(greeting).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn rust_free_string(ptr: *mut u8) {
    unsafe {
        let _ = std::ffi::CString::from_raw(ptr);
    }
}

fn main() {
    println!("This lesson demonstrates FFI with Go using cgo.");
    println!("To fully test this, you would need to compile the Rust code as a");
    println!("shared library and link it with a Go program.");
}
