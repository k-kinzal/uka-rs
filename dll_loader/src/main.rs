extern crate dlopen;
extern crate dlopen_derive;
extern crate libc;

use dlopen::symbor::Library;
use libc::{exit, memcpy};
use once_cell::sync::Lazy;
use plthook::ObjectFile;
use std::ffi::{c_char, c_void, CStr, CString, OsStr, OsString};
use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::ptr::null;
use std::sync::Mutex;
use std::{mem, panic, ptr, result};
use widestring::u16str;
use windows::core::{IntoParam, PCSTR, PCWSTR, PSTR, PWSTR};
use windows::imp::{
    FormatMessageW, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
};
use windows::Win32::Foundation::{
    GetLastError, BOOL, FALSE, HINSTANCE, HWND, INVALID_HANDLE_VALUE, LPARAM, LRESULT, MAX_PATH,
    TRUE, WPARAM,
};
use windows::Win32::Globalization::MultiByteToWideChar;
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{
    CreateFileA, FILE_ALL_ACCESS, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS32;
use windows::Win32::System::LibraryLoader::{
    FindResourceA, GetModuleFileNameA, GetModuleHandleA, GetProcAddress, LoadLibraryA,
    LoadLibraryExA, LoadResource, LockResource, SizeofResource, DONT_RESOLVE_DLL_REFERENCES,
    LOAD_LIBRARY_AS_DATAFILE, LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE, LOAD_LIBRARY_AS_IMAGE_RESOURCE,
};
use windows::Win32::System::Memory;
use windows::Win32::System::Memory::{
    CreateFileMappingA, GetProcessHeap, GlobalAlloc, GlobalSize, HeapAlloc, MapViewOfFile,
    VirtualProtect, FILE_MAP_ALL_ACCESS, GPTR, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
    PAGE_READWRITE,
};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_IMPORT_DESCRIPTOR};
use windows::Win32::System::Threading::{CreateMutexA, CreateMutexW, OpenMutexW, MUTEX_ALL_ACCESS};
use windows::Win32::System::WindowsProgramming::IMAGE_THUNK_DATA32;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExA, DefWindowProcW, LoadCursorA, LoadCursorW, PostQuitMessage, RegisterClassA,
    RegisterWindowMessageA, ShowWindow, CW_USEDEFAULT, HMENU, IDC_ARROW, IDC_WAIT, RT_DIALOG,
    SHOW_WINDOW_CMD, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSA, WNDPROC, WS_OVERLAPPEDWINDOW,
};

// fn main() {
//     let library_path = "C:\\materia\\ghost\\first\\ghost\\master\\first.dll";
//     let lib = Library::open(library_path).unwrap();
//     let unload = unsafe { lib.symbol::<unsafe extern "C" fn() -> bool>("unload") }.unwrap();
//     let load = unsafe { lib.symbol::<unsafe extern "C" fn(h: isize, len: usize) -> bool>("load") }
//         .unwrap();
//     let request = unsafe {
//         lib.symbol::<unsafe extern "C" fn(h: isize, len: *mut usize) -> isize>("request")
//     }
//     .unwrap();
//
//     let path = b"C:\\materia\\ghost\\first\\ghost\\master\\";
//     let hglobal = unsafe { GlobalAlloc(GPTR, path.len()) };
//     if hglobal == 0 {
//         let err = unsafe { GetLastError() };
//         panic!("failed to allocate memory: {err:?}\n");
//     }
//     let size = unsafe { GlobalSize(hglobal) };
//
//     let ptr = hglobal as *mut u8;
//     let ptr = unsafe { std::slice::from_raw_parts_mut(ptr, size) };
//     unsafe {
//         memcpy(
//             ptr.as_mut_ptr() as *mut c_void,
//             path.as_ptr() as *const c_void,
//             size,
//         );
//     }
//
//     println!("execute load(0x{hglobal:x}, {size})");
//     let ret = unsafe { load(hglobal, size) };
//     println!("executed load(): {ret}");
// }

fn main() {
    let result = panic::catch_unwind(|| unsafe {
        foo();
    });
}

type GetModuleFileNameA =
    extern "stdcall" fn(h_module: *const c_void, lp_filename: PSTR, n_size: u32) -> u32;
static ORIGINAL_GET_MODULE_FILE_NAME_A: Lazy<Mutex<GetModuleFileNameA>> = Lazy::new(|| {
    Mutex::new({
        let hmodule = unsafe { GetModuleHandleA(PCSTR("kernel32.dll\0".as_ptr())).unwrap() };
        let lpprocname = PCSTR("GetModuleFileNameA\0".as_ptr());
        let address = unsafe { GetProcAddress(hmodule, lpprocname) }.unwrap();

        unsafe { *(address as *const GetModuleFileNameA) }
    })
});

extern "stdcall" fn get_module_file_name_a(
    h_module: *const c_void,
    lp_filename: PSTR,
    n_size: u32,
) -> u32 {
    if h_module.is_null() {
        let fake_exe_name = "C:\\materia\\materia.exe";
        unsafe {
            ptr::copy_nonoverlapping(
                fake_exe_name.as_ptr() as *const c_void,
                lp_filename.0 as *mut c_void,
                fake_exe_name.len(),
            );
        }
        (fake_exe_name.len()) as u32
    } else {
        match ORIGINAL_GET_MODULE_FILE_NAME_A.lock() {
            Ok(f) => f(h_module, lp_filename, n_size),
            Err(_) => 0,
        }
    }
}

unsafe fn hook_iat(module_name: &str) {
    let mut file = File::create("hoge.txt").unwrap();

    let module_name = format!("{}\0", module_name);
    let result = GetModuleHandleA(PCSTR(module_name.as_ptr()));
    if result.is_err() {
        error(&mut file, "GetModuleHandleA()")
    }
    let h_module = result.unwrap();

    let p_dos_header = h_module.0 as *const IMAGE_DOS_HEADER;
    let p_nt_headers = (h_module.0 as *const u8).add((*p_dos_header).e_lfanew as usize)
        as *const IMAGE_NT_HEADERS32;

    let p_import_directory = (*p_nt_headers).OptionalHeader.DataDirectory.get(1).unwrap();
    let p_import_descriptor = (h_module.0 as *const u8)
        .add(p_import_directory.VirtualAddress as usize)
        as *const IMAGE_IMPORT_DESCRIPTOR;

    let mut p_import_descriptor_mut = p_import_descriptor;
    let mut hooked = false;

    let address = GetProcAddress(
        GetModuleHandleA(PCSTR("kernel32.dll\0".as_ptr())).unwrap(),
        PCSTR("GetModuleFileNameA\0".as_ptr()),
    );
    let original_function: *mut c_void = mem::transmute(address);

    while (*p_import_descriptor_mut).Name != 0 && !hooked {
        let p_thunk = (h_module.0 as *const u8).add((*p_import_descriptor_mut).FirstThunk as usize)
            as *mut IMAGE_THUNK_DATA32;

        let mut p_thunk_mut = p_thunk;
        while (*p_thunk_mut).u1.Ordinal != 0 {
            let p_function = (*p_thunk_mut).u1.Function as *mut _;

            if p_function == original_function {
                let get_module_file_name_a: GetModuleFileNameA = get_module_file_name_a;
                let new_function: u32 = get_module_file_name_a as *const () as u32;

                let mut old_protect = PAGE_PROTECTION_FLAGS(0);
                let protect_result = VirtualProtect(
                    p_thunk_mut as *mut _ as _,
                    std::mem::size_of::<u32>() as _,
                    Memory::PAGE_EXECUTE_READWRITE,
                    &mut old_protect as *mut _ as _,
                );

                if protect_result == BOOL(0) {
                    panic!("VirtualProtect failed to change memory protection.");
                }

                (*p_thunk_mut).u1.Function = new_function;

                // Restore original protection
                let mut dummy_protect = PAGE_PROTECTION_FLAGS(0);
                VirtualProtect(
                    p_thunk_mut as *mut _ as _,
                    std::mem::size_of::<u32>() as _,
                    old_protect,
                    &mut dummy_protect as *mut _ as _,
                )
                .ok();

                hooked = true;
                break;
            }

            p_thunk_mut = p_thunk_mut.add(1);
        }

        p_import_descriptor_mut = p_import_descriptor_mut.add(1);
    }
}

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

unsafe fn error(file: &mut File, name: &str) {
    let err = unsafe { GetLastError() };
    let mut buf: Vec<u16> = vec![0; 1024];

    let _ = unsafe {
        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null(),
            err.0,
            0x0400,
            PWSTR(buf.as_mut_ptr()),
            buf.len() as u32,
            null(),
        )
    };

    let error_message = String::from_utf16_lossy(&buf)
        .trim_end_matches('\x00')
        .to_owned();
    file.write_all(format!("{} error: {:?}, {:?}\n", name, err, error_message).as_bytes())
        .unwrap();

    panic!("error");
}

unsafe fn foo() {
    let mut file = File::create("bar.txt").unwrap();
    // hook_iat();

    let object = ObjectFile::open_main_program().unwrap();
    object
        .replace("GetModuleFileNameA", get_module_file_name_a as *const _)
        .unwrap();

    let mut buffer = [0u8; 1024];
    let result = GetModuleFileNameA(HINSTANCE(0), &mut buffer);
    if result == 0 {
        error(&mut file, "GetModuleFileNameA");
    }
    let exe_path = CStr::from_ptr(buffer.as_ptr() as *const c_char)
        .to_str()
        .unwrap();
    file.write_all(format!("Exec name: {result} {exe_path}\n").as_bytes())
        .unwrap();

    let mut exe_path = "C:\\materia2\\materia.exe\0";
    let result = LoadLibraryExA(
        PCSTR::from_raw(exe_path.as_ptr()),
        None,
        LOAD_LIBRARY_AS_DATAFILE,
    );
    if result.is_err() {
        error(&mut file, "LoadLibraryExA");
    }
    let h_instance = result.unwrap();
    file.write_all(format!("LoadLibraryExA success: {h_instance:?}\n").as_bytes())
        .unwrap();

    //
    file.write_all(format!("load mai.dll: {h_instance:?}\n").as_bytes())
        .unwrap();

    let v1 = format!("#{}\0", 102i16);
    let v2 = format!("#{}\0", 12i16);
    let result = FindResourceA(h_instance, PCSTR(v1.as_ptr() as _), PCSTR(v2.as_ptr() as _));
    if result.is_err() {
        error(&mut file, "FindResourceA");
    }
    file.write_all(format!("FindResourceA success: {h_instance:?}\n").as_bytes())
        .unwrap();
    let hr_src = result.unwrap();

    let result = LoadResource(h_instance, hr_src);
    if result.is_err() {
        error(&mut file, "LoadResource");
    }
    file.write_all(format!("LoadResource success: {h_instance:?}\n").as_bytes())
        .unwrap();
    let h_global = result.unwrap();

    let result = LockResource(h_global);
    if result.is_null() {
        error(&mut file, "LockResource");
    }
    file.write_all(format!("LockResource success: {result:?}\n").as_bytes())
        .unwrap();
    let h_global = result;

    let size = SizeofResource(h_instance, hr_src);
    if size == 0 {
        error(&mut file, "SizeofResource");
    }
    file.write_all(format!("Sizeof success: {size:?}\n").as_bytes())
        .unwrap();

    let data = unsafe { std::slice::from_raw_parts(h_global as *const u8, size as usize) };
    file.write_all(format!("mai.dll: {data:?}\n").as_bytes())
        .unwrap();
    let mut f = File::create("C:\\materia\\mai.dll").unwrap();
    f.write_all(data).unwrap();
    file.write_all(format!("export mai.dll: {h_instance:?}\n").as_bytes())
        .unwrap();

    //

    file.write_all(format!("load sayuri.dll: {h_instance:?}\n").as_bytes())
        .unwrap();

    let v1 = format!("#{}\0", 102i16);
    let v2 = format!("#{}\0", 12i16);
    let result = FindResourceA(h_instance, PCSTR(v1.as_ptr() as _), PCSTR(v2.as_ptr() as _));
    if result.is_err() {
        error(&mut file, "FindResourceA");
    }
    file.write_all(format!("FindResourceA success: {h_instance:?}\n").as_bytes())
        .unwrap();
    let hr_src = result.unwrap();

    let result = LoadResource(h_instance, hr_src);
    if result.is_err() {
        error(&mut file, "LoadResource");
    }
    file.write_all(format!("LoadResource success: {h_instance:?}\n").as_bytes())
        .unwrap();
    let h_global = result.unwrap();

    let result = LockResource(h_global);
    if result.is_null() {
        error(&mut file, "LockResource");
    }
    file.write_all(format!("LockResource success: {result:?}\n").as_bytes())
        .unwrap();
    let h_global = result;

    let size = SizeofResource(h_instance, hr_src);
    if size == 0 {
        error(&mut file, "SizeofResource");
    }
    file.write_all(format!("Sizeof success: {size:?}\n").as_bytes())
        .unwrap();

    let data = unsafe { std::slice::from_raw_parts(h_global as *const u8, size as usize) };
    file.write_all(format!("mai.dll: {data:?}\n").as_bytes())
        .unwrap();
    let mut f = File::create("C:\\materia\\sayuri.dll").unwrap();
    f.write_all(data).unwrap();
    file.write_all(format!("export sayuri.dll: {h_instance:?}\n").as_bytes())
        .unwrap();

    //

    // let class_name = "main window class";
    // let class_name = class_name.as_ptr();
    // let class_name = PCSTR::from_raw(class_name);
    //
    // let mut wc = WNDCLASSA::default();
    // wc.lpfnWndProc = Some(window_proc);
    // wc.hInstance = HINSTANCE(0);
    // wc.lpszClassName = class_name;
    // let atom = RegisterClassA(&wc);
    // if atom == 0 {
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("RegisterClassA error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //     }
    //     panic!("RegisterClassA error");
    // }
    // file.write_all(format!("RegisterClassA success: {:?}, \n", atom).as_bytes())
    //     .unwrap();
    //
    // file.write_all(format!("CreateWindowExA start\n").as_bytes())
    //     .unwrap();
    // let hWnd = CreateWindowExA(
    //     WINDOW_EX_STYLE::default(),
    //     class_name,
    //     PCSTR::from_raw("main window".as_ptr()),
    //     WS_OVERLAPPEDWINDOW,
    //     CW_USEDEFAULT,
    //     CW_USEDEFAULT,
    //     CW_USEDEFAULT,
    //     CW_USEDEFAULT,
    //     HWND(0),
    //     HMENU(0),
    //     HINSTANCE(0),
    //     None,
    // );
    // file.write_all(format!("CreateWindowExA called: {:?}\n", hWnd).as_bytes())
    //     .unwrap();

    // if hWnd.0 == 0 {
    //     let err = GetLastError();
    //     file.write_all(format!("CreateWindowExA error: {:?}, \n", err).as_bytes())
    //         .unwrap();
    //     panic!("CreateWindowExA error");
    // }
    // file.write_all(format!("CreateWindowExA success: {:?}, \n", hWnd).as_bytes())
    //     .unwrap();

    // ShowWindow(hWnd, SHOW_WINDOW_CMD(5));

    // let name = PCSTR::from_raw("Sakura".as_ptr());
    // let res = RegisterWindowMessageA(name);
    // if res == 0 {
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("RegisterWindowMessageA error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //     }
    //     panic!("RegisterWindowMessageA error");
    // }
    // file.write_all(format!("RegisterWindowMessageA success: {:?}, \n", res).as_bytes())
    //     .unwrap();

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

    // let str = PCWSTR::from_raw(u16str!("sakura").as_ptr());
    // let result = unsafe { CreateMutexW(None, FALSE, str) };
    // if result.is_err() {
    //     file.write_all(format!("create mutex error: {:?}, \n", result.unwrap_err()).as_bytes())
    //         .unwrap();
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("create mutex last error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //         panic!("create mutex last error");
    //     }
    //     panic!("create mutex error");
    // }
    // file.write_all(format!("create mutex: {:?}\n", result.unwrap()).as_bytes())
    //     .unwrap();

    // let str = PCSTR::from_raw("sakura".as_ptr());
    // let result = unsafe { CreateMutexA(None, FALSE, str) };
    // if result.is_err() {
    //     file.write_all(format!("create mutex A error: {:?}, \n", result.unwrap_err()).as_bytes())
    //         .unwrap();
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("create mutex A last error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //         panic!("create mutex A last error");
    //     }
    //     panic!("create mutex A error");
    // }
    // file.write_all(format!("create mutex A: {:?}\n", result.unwrap()).as_bytes())
    //     .unwrap();

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

    // let result = CreateFileMappingA(
    //     INVALID_HANDLE_VALUE,
    //     None,
    //     PAGE_READWRITE,
    //     0,
    //     1024 * 64,
    //     PCSTR::from_raw("Sakura".as_ptr()),
    // );
    // if result.is_err() {
    //     file.write_all(
    //         format!("create file mapping error: {:?}, \n", result.unwrap_err()).as_bytes(),
    //     )
    //     .unwrap();
    //     panic!("create file mapping error");
    // }
    // let err = GetLastError();
    // if err.0 != 0 {
    //     file.write_all(format!("create file mapping last error: {:?}, \n", err).as_bytes())
    //         .unwrap();
    //     panic!("create file mapping last error");
    // }
    // let handle = result.unwrap();
    //
    // let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, 1024 * 64);
    // if ptr.is_null() {
    //     file.write_all(format!("map view of file error: {:?}\n", ptr).as_bytes())
    //         .unwrap();
    //
    //     let err = GetLastError();
    //     if err.0 != 0 {
    //         file.write_all(format!("create file mapping last error: {:?}, \n", err).as_bytes())
    //             .unwrap();
    //         panic!("create file mapping last error");
    //     }
    //     panic!("map view of file error");
    // }
    // let bytes = format!(
    //     "53137ee8825085dba1707e3bea9e474b.path\x01C:\\materia\\\r\n\
    // 53137ee8825085dba1707e3bea9e474b.hwnd\x01{:?}\r\n\
    // 53137ee8825085dba1707e3bea9e474b.name\x01sakura\r\n\
    // 53137ee8825085dba1707e3bea9e474b.keroname\x01unyu\r\n\
    // 53137ee8825085dba1707e3bea9e474b.sakura.surface\x010\r\n\
    // 53137ee8825085dba1707e3bea9e474b.kero.surface\x010\r\n\0",
    //     hWnd
    // );
    // let bytes = bytes.as_bytes();
    // let len = (1024 * 64) as i32;
    // let v = [len.to_be_bytes().to_vec(), bytes.to_vec()].concat();
    // file.write_all(format!("before memcpy: {:?}\n", v).as_bytes())
    //     .unwrap();
    // memcpy(ptr, v.as_ptr() as *const c_void, v.len());
    // let len = v.len();
    // let v = ptr as *const u8;
    // let v = unsafe { std::slice::from_raw_parts(v, len) };
    // file.write_all(format!("after memcpy: {:?}, {}\n", ptr, String::from_utf8_lossy(v)).as_bytes())
    //     .unwrap();

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

    hook_iat(library_path);
    file.write_all(format!("hook iat\n").as_bytes()).unwrap();
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
    if hglobal.is_err() {
        let err = GetLastError();
        panic!("failed to allocate memory: {:?}\n", err);
    }
    let hglobal = hglobal.unwrap();
    let size = windows::Win32::System::Memory::GlobalSize(hglobal);
    // file.write_all(format!("allocate: 0x{:x}, {:x}\n", hglobal, size).as_bytes())
    //     .unwrap();
    // stdout().flush().unwrap();

    let p = hglobal.0 as *mut u8;
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

    let pp = hglobal.0 as *mut u8;
    let pp = unsafe { std::slice::from_raw_parts_mut(pp, len) };
    file.write_all(
        format!(
            "call load(0x{:x}({}), {}), {:?}\n",
            hglobal.0, hglobal.0, len, pp
        )
        .as_bytes(),
    )
    .unwrap();
    let v = unsafe { load(hglobal.0, len) };
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
