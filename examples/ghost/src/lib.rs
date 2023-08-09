mod context;
mod handler;

use crate::context::ShioriContext;
use crate::handler::Handler;
use once_cell::sync::OnceCell;
use uka_shiori::dll::Adapter;
use uka_shiori::runtime::ContextData;

static SHIORI: OnceCell<Adapter<ShioriContext, Handler>> = OnceCell::new();

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn load(h: isize, len: usize) -> bool {
    let shiori = SHIORI.get_or_init(|| Handler::default().into());
    unsafe { shiori.load(h, len) }
}

#[no_mangle]
pub extern "C" fn unload() -> bool {
    match SHIORI.get() {
        Some(shiori) => shiori.unload(),
        None => false,
    }
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn request(h: isize, len: *mut usize) -> isize {
    let shiori = SHIORI.get().expect("unreachable");
    shiori.request(h, len)
}
