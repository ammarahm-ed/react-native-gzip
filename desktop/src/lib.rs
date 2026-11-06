rn_gpui_module::include_specs!();

use std::io::{Read, Write};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use rn_gpui_module::ModuleError;

pub struct Gzip;

fn failure(error: impl std::fmt::Display) -> ModuleError {
    ModuleError::with_code("ERROR_FAILED", error.to_string())
}

impl NativeGzipSpec for Gzip {
    fn new(_events: NativeGzipEvents) -> Self {
        Self
    }

    async fn inflate(&self, data: String) -> Result<String, ModuleError> {
        let bytes = STANDARD.decode(data.trim()).map_err(failure)?;
        let mut text = Vec::new();
        GzDecoder::new(bytes.as_slice())
            .read_to_end(&mut text)
            .map_err(failure)?;
        Ok(String::from_utf8_lossy(&text).into_owned())
    }

    async fn deflate(&self, data: String) -> Result<String, ModuleError> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data.as_bytes()).map_err(failure)?;
        Ok(STANDARD.encode(encoder.finish().map_err(failure)?))
    }
}

pub fn rn_gpui_register() {
    rn_gpui_module::register(native_gzip_module::<Gzip>());
}
