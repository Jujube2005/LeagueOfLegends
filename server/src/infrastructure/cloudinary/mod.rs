use crate::{
    config::{config_loader::get_cloudinary_env, config_model::CloudinaryEnv},
    domain::value_objects::{base64_img::Base64Img, uploaded_img::UploadedImg},
};
use anyhow::{Context, Ok, Result};
use chrono::Utc;
use reqwest::multipart::{Form, Part};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tracing::{debug, error};

pub struct UploadImageOptions {
    pub folder: Option<String>,
    pub public_id: Option<String>,
    pub transformation: Option<String>,
}

fn form_builder(option: UploadImageOptions, cloud_env: &CloudinaryEnv) -> Result<Form> {
    let mut form = Form::new();
    let timestamp = Utc::now().timestamp().to_string(); // Cloudinary expects seconds, not milliseconds

    let mut params_to_sign: HashMap<String, String> = HashMap::new();
    params_to_sign.insert("timestamp".to_string(), timestamp.clone());
    
    if let Some(folder_name) = option.folder {
        params_to_sign.insert("folder".to_string(), folder_name);
    }
    if let Some(public_id) = option.public_id {
        params_to_sign.insert("public_id".to_string(), public_id);
    }
    if let Some(transformation) = option.transformation {
        params_to_sign.insert("transformation".to_string(), transformation);
    }

    // Sort keys alphabetically
    let mut sorted_keys: Vec<_> = params_to_sign.keys().collect();
    sorted_keys.sort();

    // Build the signature string: key1=value1&key2=value2...API_SECRET
    let mut signature_parts = Vec::new();
    for key in sorted_keys {
        if let Some(value) = params_to_sign.get(key) {
            signature_parts.push(format!("{}={}", key, value));
            // Add to form
            form = form.text(key.clone(), value.clone());
        }
    }

    let signature_string = format!("{}{}", signature_parts.join("&"), cloud_env.api_secret);
    
    let mut hasher = Sha1::new();
    hasher.update(signature_string.as_bytes());
    let signature = format!("{:x}", hasher.finalize());

    form = form.text("signature", signature);
    form = form.text("api_key", cloud_env.api_key.clone());
    
    // Cloudinary also needs resource_type in the form for some endpoints, 
    // but for /image/upload it's often redundant or inferred.
    // However, some Cloudinary accounts require it to be pinned.
    // We'll keep it but ensure it's NOT in the signature (already handled).
    form = form.text("resource_type", "image");

    Ok(form)
}

pub async fn upload(base64_image: Base64Img, option: UploadImageOptions) -> Result<UploadedImg> {
    let cloud_env = get_cloudinary_env()
        .context("Missing Cloudinary environment variables (CLOUDINARY_CLOUD_NAME, CLOUDINARY_API_KEY, CLOUDINARY_API_SECRET)")?;

    let cloud_name = cloud_env.cloud_name.clone();
    let form = form_builder(option, &cloud_env)?;
    
    // Cloudinary supports Data URIs directly in the 'file' parameter
    let multipart = form.text("file", base64_image.into_inner());
    
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.cloudinary.com/v1_1/{}/image/upload",
        cloud_name
    );

    debug!("Uploading to Cloudinary: {}", url);

    let response = client
        .post(&url)
        .multipart(multipart)
        .send()
        .await
        .map_err(|e| {
            error!("Cloudinary connection error: {:?}", e);
            anyhow::anyhow!("Failed to connect to Cloudinary: {}", e)
        })?;

    let status = response.status();
    let text = response.text().await.unwrap_or_else(|_| "Could not read response body".to_string());

    if !status.is_success() {
        error!("Cloudinary upload failed. Status: {}, Response: {}", status, text);
        return Err(anyhow::anyhow!(
            "Cloudinary upload failed (Status {}): {}",
            status,
            text
        ));
    }

    let json: UploadedImg = serde_json::from_str(&text)
        .map_err(|e| {
            error!("Failed to parse Cloudinary response: {}. Error: {}", text, e);
            anyhow::anyhow!("Failed to parse Cloudinary response: {}. Error: {}", text, e)
        })?;
    
    debug!("Successfully uploaded image to Cloudinary: {}", json.url);
    Ok(json)
}
