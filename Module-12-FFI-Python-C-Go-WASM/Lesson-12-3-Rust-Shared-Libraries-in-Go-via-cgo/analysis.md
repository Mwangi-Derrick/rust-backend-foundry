# Lesson 12.3: Rust Shared Libraries in Go via cgo

## 🧠 Concept Summary

This lesson demonstrates how to build a Rust library that can be called from Go using **`cgo`**. This allows Go applications to leverage Rust's performance, safety, and rich ecosystem for specific tasks.

- **Why Rust with Go?** Go excels at concurrency and network services, but Rust can provide superior performance and memory safety for CPU-bound tasks or when integrating with low-level system components. `cgo` acts as the bridge.

- **`cgo`:** A Go tool that enables Go programs to call C code and vice versa. Since Rust can expose a C-compatible API, `cgo` can be used to call Rust functions from Go.

- **Exposing Rust to Go (via C ABI):** The process is identical to exposing Rust to C:
    1.  **`#[no_mangle]`:** Prevents Rust's compiler from changing the function name, making it discoverable by `cgo`.
    2.  **`pub extern "C"`:** Declares the function as public and uses the C calling convention, ensuring compatibility with `cgo`.

- **Memory Management Across FFI:** This is a critical aspect. When passing data (especially strings or complex structures) between Rust and Go, you must carefully manage memory allocation and deallocation to prevent leaks or crashes. Typically, the side that allocates the memory is responsible for freeing it.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Simple Addition Function

```rust
#[no_mangle]
pub extern "C" fn rust_add_for_go(a: i32, b: i32) -> i32 {
    a + b
}
```

This is a straightforward function that takes two `i32`s and returns their sum. It's marked with `#[no_mangle]` and `pub extern "C"` to make it callable from Go via `cgo`.

### String Manipulation Function

```rust
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
```

This is more complex due to string handling:

-   **`rust_greet_for_go`:** Takes a C-style pointer (`*const u8`) and length (`usize`) for the input string. It reconstructs a Rust `&str` (using `unsafe` because we trust the Go side to provide valid data), creates a greeting `String`, converts it to a C-compatible null-terminated string (`CString`), and then returns a raw pointer (`*mut u8`) to the allocated C string. **Crucially, Rust allocates this memory.**
-   **`rust_free_string`:** This function is provided for Go to call when it's done with the string returned by `rust_greet_for_go`. It takes the raw pointer, reconstructs the `CString` (which takes ownership of the memory), and then allows it to be dropped, freeing the memory. This adheres to the rule: the allocator is responsible for the deallocator.

## ⚙️ Build and Usage (Conceptual)

To use this, you would:

1.  **Compile Rust as a C-compatible static/dynamic library:**
    ```bash
    cargo build --release --target x86_64-unknown-linux-gnu # or your target
    # This creates libyourcrate.a (static) or libyourcrate.so (dynamic)
    ```
2.  **Create a Go program (`main.go`):**
    ```go
    package main

    /*
    #cgo LDFLAGS: -L. -lrust_lib_name
    #include "rust_header.h"
    */
    import "C"
    import (
        "fmt"
        "unsafe"
    )

    // rust_header.h would contain:
    // int32_t rust_add_for_go(int32_t a, int32_t b);
    // char* rust_greet_for_go(const char* name_ptr, size_t name_len);
    // void rust_free_string(char* ptr);

    func main() {
        result := C.rust_add_for_go(C.int(5), C.int(7))
        fmt.Printf("Rust add 5 + 7 is: %d\n", result)

        name := "Gopher"
        cName := C.CString(name)
        defer C.free(unsafe.Pointer(cName))

        cGreeting := C.rust_greet_for_go(cName, C.size_t(len(name)))
        goGreeting := C.GoString(cGreeting)
        defer C.rust_free_string(cGreeting) // Important: free memory allocated by Rust

        fmt.Printf("Rust greeting: %s\n", goGreeting)
    }
    ```
3.  **Compile and run Go:**
    ```bash
    go run main.go
    ```

## ⚔️ Cross-Language Insights

- **Go's `cgo`:** `cgo` is Go's primary mechanism for FFI. It allows you to embed C code directly in your Go files and call C functions. It handles the marshalling of basic types, but complex types and memory management require manual attention.

- **Memory Ownership:** The most critical aspect of FFI between Rust and Go (or any two languages) is clear ownership of allocated memory. Rust's `CString::into_raw()` transfers ownership to the C side, and `CString::from_raw()` takes it back.

## 🚀 Practical Reflection

- **Performance Boost:** This pattern is ideal for offloading computationally intensive parts of a Go application to Rust, where you can benefit from Rust's performance and control over system resources.

- **Safety Boundaries:** While the Rust code itself is safe, the FFI boundary is inherently `unsafe`. You must ensure that the Go side correctly handles pointers and memory, as Rust cannot protect against misuse from the foreign language.

- **Error Handling:** Passing complex error types across FFI boundaries can be challenging. Often, you'll return simple error codes or strings, and the Go side will translate them into Go-idiomatic errors.

## 🧩 Self-Review Prompts

- How would you pass a Go slice of integers to a Rust function that sums them?
- Implement a Rust function that takes a Go string, reverses it, and returns the reversed string to Go.
- What are the potential pitfalls of not calling `rust_free_string` from the Go side?
