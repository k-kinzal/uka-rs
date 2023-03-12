extern crate dlopen;
extern crate dlopen_derive;
extern crate libc;

use dlopen::symbor::Library;
use libc::{exit, memcpy};
use std::ffi::{c_void, OsStr, OsString};
use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::{panic, ptr, result};
use widestring::u16str;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::Foundation::{
    GetLastError, FALSE, HINSTANCE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, TRUE, WPARAM,
};
use windows::Win32::Globalization::MultiByteToWideChar;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{
    CreateFileA, FILE_ALL_ACCESS, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{
    CreateFileMappingA, GetProcessHeap, GlobalAlloc, GlobalSize, HeapAlloc, MapViewOfFile,
    FILE_MAP_ALL_ACCESS, GPTR, PAGE_EXECUTE_READWRITE, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{CreateMutexA, CreateMutexW, OpenMutexW, MUTEX_ALL_ACCESS};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcW, LoadCursorA, LoadCursorW, PostQuitMessage, RegisterClassA,
    RegisterWindowMessageA, ShowWindow, CW_USEDEFAULT, HMENU, IDC_ARROW, IDC_WAIT, SHOW_WINDOW_CMD,
    WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSA, WNDPROC, WS_OVERLAPPEDWINDOW,
};

fn main() {
    let library_path = "C:\\materia\\ghost\\first\\ghost\\master\\first.dll";
    let lib = Library::open(library_path).unwrap();
    let unload = unsafe { lib.symbol::<unsafe extern "C" fn() -> bool>("unload") }.unwrap();
    let load = unsafe { lib.symbol::<unsafe extern "C" fn(h: isize, len: usize) -> bool>("load") }
        .unwrap();
    let request = unsafe {
        lib.symbol::<unsafe extern "C" fn(h: isize, len: *mut usize) -> isize>("request")
    }
    .unwrap();

    let path = b"C:\\materia\\ghost\\first\\ghost\\master\\";
    let hglobal = unsafe { GlobalAlloc(GPTR, path.len()) };
    if hglobal == 0 {
        let err = unsafe { GetLastError() };
        panic!("failed to allocate memory: {err:?}\n");
    }
    let size = unsafe { GlobalSize(hglobal) };

    let ptr = hglobal as *mut u8;
    let ptr = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
    unsafe {
        memcpy(
            ptr.as_mut_ptr() as *mut c_void,
            path.as_ptr() as *const c_void,
            size,
        );
    }

    println!("execute load(0x{hglobal:x}, {size})");
    let ret = unsafe { load(hglobal, size) };
    println!("executed load(): {ret}");
}

// fn main() {
//     let result = panic::catch_unwind(|| unsafe {
//         foo();
//     });
// }

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    };

    LRESULT(0)
}

unsafe fn foo() {
    let mut file = File::create("bar.txt").unwrap();

    let class_name = "main window class";
    let class_name = class_name.as_ptr();
    let class_name = PCSTR::from_raw(class_name);

    let mut wc = WNDCLASSA::default();
    wc.lpfnWndProc = Some(window_proc);
    wc.hInstance = HINSTANCE(0);
    wc.lpszClassName = class_name;
    let atom = RegisterClassA(&wc);
    if atom == 0 {
        let err = GetLastError();
        if err.0 != 0 {
            file.write_all(format!("RegisterClassA error: {:?}, \n", err).as_bytes())
                .unwrap();
        }
        panic!("RegisterClassA error");
    }
    file.write_all(format!("RegisterClassA success: {:?}, \n", atom).as_bytes())
        .unwrap();

    file.write_all(format!("CreateWindowExA start\n").as_bytes())
        .unwrap();
    let hWnd = CreateWindowExA(
        WINDOW_EX_STYLE::default(),
        class_name,
        PCSTR::from_raw("main window".as_ptr()),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        HWND(0),
        HMENU(0),
        HINSTANCE(0),
        None,
    );
    file.write_all(format!("CreateWindowExA called: {:?}\n", hWnd).as_bytes())
        .unwrap();
    // if hWnd.0 == 0 {
    //     let err = GetLastError();
    //     file.write_all(format!("CreateWindowExA error: {:?}, \n", err).as_bytes())
    //         .unwrap();
    //     panic!("CreateWindowExA error");
    // }
    file.write_all(format!("CreateWindowExA success: {:?}, \n", hWnd).as_bytes())
        .unwrap();

    ShowWindow(hWnd, SHOW_WINDOW_CMD(5));

    let name = PCSTR::from_raw("Sakura".as_ptr());
    let res = RegisterWindowMessageA(name);
    if res == 0 {
        let err = GetLastError();
        if err.0 != 0 {
            file.write_all(format!("RegisterWindowMessageA error: {:?}, \n", err).as_bytes())
                .unwrap();
        }
        panic!("RegisterWindowMessageA error");
    }
    file.write_all(format!("RegisterWindowMessageA success: {:?}, \n", res).as_bytes())
        .unwrap();

    // let name = PCSTR::from_raw("kernel32.dll".as_ptr());
    // let res = GetModuleHandleA(name);
    // if res.is_err() {
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("GetModuleHandleA error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //     }
    //     panic!("GetModuleHandleA error");
    // }
    // let hInstance = res.unwrap();
    //
    // let name = PCWSTR(32000i32 as _);
    // let hInstance = HINSTANCE(0i32 as _);
    // let res = unsafe { LoadCursorW(hInstance, name) };
    // if res.is_err() {
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("LoadCursorW error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //     }
    //     panic!("LoadCursorW error");
    // }
    // file.write_all(format!("LoadCursorW success: {:?}, \n", res).as_bytes())
    //     .unwrap();

    // let name = u16str!("sakura3");
    // let name = OsStr::new(name);
    // let v = widestring::encode_utf8(name).
    // let ptr = name.as_ptr();

    let str = PCWSTR::from_raw(u16str!("sakura").as_ptr());
    let result = unsafe { CreateMutexW(None, FALSE, str) };
    if result.is_err() {
        file.write_all(format!("create mutex error: {:?}, \n", result.unwrap_err()).as_bytes())
            .unwrap();
        let err = GetLastError();
        if err.0 != 0 {
            file.write_all(format!("create mutex last error: {:?}, \n", err).as_bytes())
                .unwrap();
            panic!("create mutex last error");
        }
        panic!("create mutex error");
    }
    file.write_all(format!("create mutex: {:?}\n", result.unwrap()).as_bytes())
        .unwrap();

    let str = PCSTR::from_raw("sakura".as_ptr());
    let result = unsafe { CreateMutexA(None, FALSE, str) };
    if result.is_err() {
        file.write_all(format!("create mutex A error: {:?}, \n", result.unwrap_err()).as_bytes())
            .unwrap();
        let err = GetLastError();
        if err.0 != 0 {
            file.write_all(format!("create mutex A last error: {:?}, \n", err).as_bytes())
                .unwrap();
            panic!("create mutex A last error");
        }
        panic!("create mutex A error");
    }
    file.write_all(format!("create mutex A: {:?}\n", result.unwrap()).as_bytes())
        .unwrap();

    // let str = PCWSTR::from_raw(u16str!("sakura").as_ptr());
    // let sec = MUTEX_ALL_ACCESS;
    // let result = unsafe { OpenMutexW(sec, FALSE, str) };
    // if result.is_err() {
    //     file.write_all(format!("open mutex error: {:?}, \n", result.unwrap_err()).as_bytes())
    //         .unwrap();
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("open mutex last error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //         panic!("open mutex last error");
    //     }
    //     panic!("open mutex error");
    // }
    // file.write_all(format!("open mutex: {:?}\n", result.unwrap()).as_bytes())
    //     .unwrap();

    // let name = PCSTR::from_raw("sakrua.dll".as_ptr());
    // let result = CreateFileA(
    //     name,
    //     FILE_ALL_ACCESS,
    //     FILE_SHARE_READ | FILE_SHARE_WRITE,
    //     None,
    //     OPEN_EXISTING,
    //     FILE_ATTRIBUTE_NORMAL,
    //     None,
    // );
    // file.write_all(format!("create file executed\n").as_bytes())
    //     .unwrap();
    // if result.is_err() {
    //     file.write_all(format!("create file executed2\n").as_bytes())
    //         .unwrap();
    //     file.write_all(format!("create file error: {:?}, \n", result.unwrap_err()).as_bytes())
    //         .unwrap();
    //     panic!("create file error");
    // }
    // let err = GetLastError();
    // if err.0 != 0 {
    //     file.write_all(format!("create file executed3\n").as_bytes())
    //         .unwrap();
    //     file.write_all(format!("create file mapping last error: {:?}, \n", err).as_bytes())
    //         .unwrap();
    //     panic!("create file mapping last error");
    // }
    // file.write_all(format!("create file executed4\n").as_bytes())
    //     .unwrap();

    let result = CreateFileMappingA(
        INVALID_HANDLE_VALUE,
        None,
        PAGE_READWRITE,
        0,
        1024 * 64,
        PCSTR::from_raw("Sakura".as_ptr()),
    );
    if result.is_err() {
        file.write_all(
            format!("create file mapping error: {:?}, \n", result.unwrap_err()).as_bytes(),
        )
        .unwrap();
        panic!("create file mapping error");
    }
    let err = GetLastError();
    if err.0 != 0 {
        file.write_all(format!("create file mapping last error: {:?}, \n", err).as_bytes())
            .unwrap();
        panic!("create file mapping last error");
    }
    let handle = result.unwrap();

    let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 1024 * 64);
    if ptr.is_null() {
        file.write_all(format!("map view of file error: {:?}\n", ptr).as_bytes())
            .unwrap();

        let err = GetLastError();
        if err.0 != 0 {
            file.write_all(format!("create file mapping last error: {:?}, \n", err).as_bytes())
                .unwrap();
            panic!("create file mapping last error");
        }
        panic!("map view of file error");
    }
    let bytes = format!(
        "53137ee8825085dba1707e3bea9e474b.path\x01C:\\materia\\\r\n\
    53137ee8825085dba1707e3bea9e474b.hwnd\x01{:?}\r\n\
    53137ee8825085dba1707e3bea9e474b.name\x01sakura\r\n\
    53137ee8825085dba1707e3bea9e474b.keroname\x01unyu\r\n\
    53137ee8825085dba1707e3bea9e474b.sakura.surface\x010\r\n\
    53137ee8825085dba1707e3bea9e474b.kero.surface\x010\r\n\0",
        hWnd
    );
    let bytes = bytes.as_bytes();
    let len = (1024 * 64) as i32;
    let v = [len.to_be_bytes().to_vec(), bytes.to_vec()].concat();
    file.write_all(format!("before memcpy: {:?}\n", v).as_bytes())
        .unwrap();
    memcpy(ptr, v.as_ptr() as *const c_void, v.len());
    let len = v.len();
    let v = ptr as *const u8;
    let v = unsafe { std::slice::from_raw_parts(v, len) };
    file.write_all(format!("after memcpy: {:?}, {}\n", ptr, String::from_utf8_lossy(v)).as_bytes())
        .unwrap();

    // let mut buf = PathBuf::from("C:\\materia\\ghost\\first\\ghost\\master\\first.dll");
    // let mut buf = buf.canonicalize().unwrap();
    // let library_path = buf.to_str().unwrap();
    let library_path = "C:\\materia\\ghost\\first\\ghost\\master\\first.dll";
    file.write_all(format!("dll from {}\n", library_path).as_bytes())
        .unwrap();
    stdout().flush().unwrap();

    let lib = Library::open(library_path).unwrap();
    file.write_all(format!("dll opened\n").as_bytes()).unwrap();
    stdout().flush().unwrap();

    // buf.pop();
    // let path = format!("{}\\", buf.to_str().unwrap());
    let path = "C:\\materia\\ghost\\first\\ghost\\master\\";
    let path = encoding_rs::SHIFT_JIS.encode(path).0;
    let path = path.to_vec();
    // let path = CString::new(path).unwrap();
    let len = path.len();
    file.write_all(format!("path: {}, {}\n", String::from_utf8_lossy(&path), len).as_bytes())
        .unwrap();

    let hglobal =
        windows::Win32::System::Memory::GlobalAlloc(windows::Win32::System::Memory::GPTR, len);
    if hglobal == 0 {
        let err = GetLastError();
        panic!("failed to allocate memory: {:?}\n", err);
    }
    let size = windows::Win32::System::Memory::GlobalSize(hglobal);
    file.write_all(format!("allocate: 0x{:x}, {}\n", hglobal, size).as_bytes())
        .unwrap();
    stdout().flush().unwrap();

    let p = hglobal as *mut u8;
    let p = unsafe { std::slice::from_raw_parts_mut(p, len) };
    file.write_all(
        format!(
            "pointer: {:?}, {}, {:?}\n",
            p.as_ptr(),
            std::mem::size_of_val(p),
            p
        )
        .as_bytes(),
    )
    .unwrap();
    stdout().flush().unwrap();

    file.write_all(
        format!(
            "before memcpy: {:?}, {}\n",
            &p.as_ptr(),
            String::from_utf8_lossy(p)
        )
        .as_bytes(),
    )
    .unwrap();
    memcpy(
        p.as_mut_ptr() as *mut c_void,
        path.as_ptr() as *const c_void,
        len,
    );
    file.write_all(
        format!(
            "after memcpy: {:?}, {}, {:?}\n",
            &p.as_ptr(),
            String::from_utf8_lossy(p),
            p
        )
        .as_bytes(),
    )
    .unwrap();

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
    file.write_all(format!("symbol unload(): {:?}\n", _unload).as_bytes())
        .unwrap();
    file.write_all(format!("symbol load(): {:?}\n", load).as_bytes())
        .unwrap();
    file.write_all(format!("symbol request(): {:?}\n", _request).as_bytes())
        .unwrap();
    stdout().flush().unwrap();

    // println!("call unload()");
    // file.write_all(format!("call unload()\n").as_bytes())
    //     .unwrap();
    // let v = unsafe { _unload() };
    // file.write_all(format!("unload executed: {}", v).as_bytes())
    //     .unwrap();

    let pp = hglobal as *mut u8;
    let pp = unsafe { std::slice::from_raw_parts_mut(pp, len) };
    file.write_all(
        format!(
            "call load(0x{:x}({}), {}), {:?}\n",
            hglobal, hglobal, len, pp
        )
        .as_bytes(),
    )
    .unwrap();
    let v = unsafe { load(hglobal, len) };
    file.write_all(format!("load executed: {}\n", v).as_bytes())
        .unwrap();

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
    #[should_panic]
    fn it_works() {
        let result = panic::catch_unwind(|| unsafe {
            foo();
        });
        println!("result: {:?}", result);
        panic!("foo");
    }
}
