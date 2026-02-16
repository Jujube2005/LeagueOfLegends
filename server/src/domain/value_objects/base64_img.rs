use anyhow::Result;
use base64::{Engine, engine::general_purpose};
#[derive(Debug, Clone)]
pub struct Base64Img(String);

impl Base64Img {
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn new(data: String) -> Result<Self> {
        let clean_data = data.replace(|c: char| c.is_whitespace(), "");
        if clean_data.is_empty() {
            return Err(anyhow::anyhow!("data can not be empty !!"));
        }
        let bytes = match general_purpose::STANDARD.decode(&clean_data) {
            Ok(bs) => bs,
            Err(_) => {
                // Try decoding without padding if standard fails
                match general_purpose::STANDARD_NO_PAD.decode(&clean_data) {
                    Ok(bs) => bs,
                    Err(e) => return Err(anyhow::anyhow!("invalid base64 data: {}. data length: {}", e, clean_data.len())),
                }
            }
        };

        let file_type = match infer::get(&bytes) {
            Some(t) => {
                let mime = t.mime_type();
                if mime == "image/png" || mime == "image/jpeg" || mime == "image/jpg" || mime == "image/webp" {
                    mime
                } else {
                    return Err(anyhow::anyhow!("unsupported file type: {}. only png, jpeg, and webp are allowed.", mime));
                }
            }
            None => return Err(anyhow::anyhow!("could not identify file type. first few bytes: {:02X?}", &bytes[..std::cmp::min(bytes.len(), 10)])),
        };

        let base64text = format!("data:{};base64,{}", file_type, clean_data);
        Ok(Self(base64text))
    }
}
