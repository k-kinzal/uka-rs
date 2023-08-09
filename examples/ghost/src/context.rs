use std::path::PathBuf;
use uka_shiori::runtime::ContextData;
use uka_shiori::types::v3;

pub struct ShioriContext {
    #[allow(dead_code)]
    path: PathBuf,
}
impl ContextData for ShioriContext {
    type Error = v3::ShioriError;

    fn new(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self { path })
    }
}
