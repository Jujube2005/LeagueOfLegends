use anyhow::Result;
use base64::{Engine, engine::general_purpose};
#[derive(Debug, Clone)]
pub struct Base64Img(String);

impl Base64Img {
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn new(data: String) -> Result<Self> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("data can not be empty !!"));
        }
        let bytes = match general_purpose::STANDARD.decode(&data) {
            Ok(bs) => bs,
            Err(_) => return Err(anyhow::anyhow!("invalid img data !!")),
        };
        let file_type = match infer::get(&bytes) {
            Some(t) if t.mime_type() == "image/png" || t.mime_type() == "image/jpeg" => {
                t.mime_type()
            }
            _ => return Err(anyhow::anyhow!("un-support file type")),
        };

        let base64text = format!("data:{};base64,{}", file_type, data);
        Ok(Self(base64text))
    }
}
