extern crate dlopen;
extern crate dlopen_derive;
extern crate libc;

use dlopen::symbor::Library;
use libc::memcpy;
use std::ffi::c_void;
use std::io::{stdout, Write};
use windows::Win32::Foundation::GetLastError;

unsafe fn foo() {
    let library_path = "Z:\\Downloads\\ghost\\first\\ghost\\master\\first.dll";
    println!("dll from {}", library_path);
    stdout().flush().unwrap();

    let lib = Library::open(library_path).unwrap();
    println!("dll opened");
    stdout().flush().unwrap();

    let path = "Z:\\Downloads\\ghost\\first\\ghost\\master\\";
    // let path = CString::new(path).unwrap();
    let len = path.as_bytes().len();
    println!("path: {}, {}", path, len);

    let hglobal = windows::Win32::System::Memory::GlobalAlloc(
        windows::Win32::System::Memory::GMEM_FIXED,
        len,
    );
    if hglobal == 0 {
        let err = GetLastError();
        panic!("failed to allocate memory: {:?}", err);
    }
    let size = windows::Win32::System::Memory::GlobalSize(hglobal);
    println!("allocate: 0x{:x}, {}", hglobal, size);
    stdout().flush().unwrap();

    let p = hglobal as *mut u8;
    let p = unsafe { std::slice::from_raw_parts_mut(p, len) };
    println!(
        "pointer: {:?}, {}, {:?}",
        &p.as_ptr(),
        std::mem::size_of_val(p),
        p
    );
    stdout().flush().unwrap();

    println!(
        "before memcpy: {:?}, {}",
        &p.as_ptr(),
        String::from_utf8_lossy(p)
    );
    memcpy(
        p.as_mut_ptr() as *mut c_void,
        path.as_ptr() as *const c_void,
        len,
    );
    println!(
        "after memcpy: {:?}, {}",
        &p.as_ptr(),
        String::from_utf8_lossy(p)
    );
    // let path_ptr = path.as_bytes();
    // let path_ptr = path_ptr.as_ptr();
    // let v = PCSTR::from_raw(path_ptr);
    // println!("create pcstr: {:?}, {}", v, v.to_string().unwrap());
    // stdout().flush().unwrap();
    //
    // let _ = lstrcpynA(p, v);
    // println!("cpy: {:?}, {}", &p.as_ptr(), String::from_utf8_lossy(p));
    // stdout().flush().unwrap();

    let _unload = unsafe { lib.symbol::<unsafe extern "C" fn() -> bool>("unload") }.unwrap();
    let load = unsafe { lib.symbol::<unsafe extern "C" fn(h: isize, len: usize) -> bool>("load") }
        .unwrap();
    let _request = unsafe {
        lib.symbol::<unsafe extern "C" fn(h: isize, len: *mut usize) -> isize>("request")
    }
    .unwrap();
    println!("symbol unload(): {:?}", _unload);
    println!("symbol load(): {:?}", load);
    println!("symbol request(): {:?}", _request);
    stdout().flush().unwrap();

    // println!("call unload()");
    // let v = unsafe { _unload() };
    // println!("unload executed: {}", v);

    println!("call load(0x{:x}, {})", hglobal, p.len());
    let v = unsafe { load(hglobal, p.len()) };
    println!("load executed: {}", v);

    // let mut len = len;
    // let len = &mut len as *mut usize;
    // println!("call request(0x{:x}, {:?})", hglobal, len);
    // let v = unsafe { _request(hglobal, len) };
    // println!("request executed: {}, {:?}", v, len);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn it_works() {
        let result = panic::catch_unwind(|| unsafe {
            foo();
        });
        println!("result: {:?}", result);
        panic!("foo");
    }
}
