extern crate dlopen;
extern crate libloading;
#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{Container, WrapperApi};
use libloading::{Library, Symbol};

// extern "C" __declspec(dllexport) HGLOBAL __cdecl getresponse(HGLOBAL h, long len);
// function getresponse(h: hglobal; var len: longint): hglobal; cdecl; export;
type GetResponse = unsafe fn(h: i32, len: i32) -> i32;

#[derive(WrapperApi)]
struct Api {
    getresponse: unsafe fn(h: i32, len: i32) -> i32,
}

unsafe fn foo() {
    let library_path = "/opt/wine/ssp/ghost/emily4/ghost/master/yaya.dll";
    println!("Loading getresponse() from {}", library_path);

    let mut cont: Container<Api> = unsafe { Container::load(library_path) }.unwrap();
    // let lib = Library::new(library_path).unwrap();
    //
    // unsafe {
    //     let func: Symbol<GetResponse> = lib.get(b"getresponse").unwrap();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            foo();
        }
    }
}
