

use crate::{domain::{entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity}, value_objects::{base64_image::Base64Image, uploaded_image::UploadedImage}}, infrastructure::{cloudinary::UploadImageOptions, jwt::jwt_model::Passport}};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport>;
    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity>;
    async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage>;
}
