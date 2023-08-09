use once_cell::sync::OnceCell;
use std::path::PathBuf;
use uka_shiori::dll::Adapter;
use uka_shiori::runtime::{box_handler, BoxHandlerV3, Context, ContextData};
use uka_shiori::types::v3;

struct ShioriContext {
    #[allow(dead_code)]
    path: PathBuf,
}
impl ContextData for ShioriContext {
    type Error = v3::ShioriError;

    fn new(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self { path })
    }
}

static SHIORI: OnceCell<Adapter<ShioriContext, BoxHandlerV3<ShioriContext>>> = OnceCell::new();

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn load(h: isize, len: usize) -> bool {
    let shiori = SHIORI.get_or_init(|| {
        box_handler(|_ctx: Context<ShioriContext>, _req: v3::Request| async {
            Ok(v3::Response::builder()
                .version(v3::Version::SHIORI_30)
                .status_code(v3::StatusCode::OK)
                .header(v3::HeaderName::SENDER, "Sakura")
                .header(v3::HeaderName::VALUE, "value")
                .charset(v3::Charset::UTF8)
                .build()
                .expect("unreachable"))
        })
        .into()
    });
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
