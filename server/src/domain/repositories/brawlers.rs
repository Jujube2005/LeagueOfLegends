use crate::{
    domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        value_objects::{
            base64_img::Base64Img, brawler_model::BrawlerModel, mission_model::MissionModel, mission_summary::MissionSummaryModel, uploaded_img::UploadedImg
        },
    },
    infrastructure::{cloudinary::UploadImageOptions, jwt::jwt_model::Passport},
};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait BrawlerRepository: Send + Sync {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport>;
    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity>;
    async fn find_by_id(&self, id: i32) -> Result<BrawlerEntity>;
    // *เพิ่ม
    async fn find_by_email(&self, email: String) -> Result<BrawlerEntity>;
    async fn upload_base64img(
        &self,
        user_id: i32,
        base64img: Base64Img,
        opt: UploadImageOptions,
    ) -> Result<UploadedImg>;

    async fn get_missions(&self, brawler_id: i32) -> Result<Vec<MissionModel>>;
    // *เพิ่ม
    async fn get_mission_summary(&self, brawler_id: i32) -> Result<MissionSummaryModel>;
    async fn crew_counting(&self, mission_id: i32) -> Result<u32>;
    // *เพิ่ม
    async fn get_leaderboard(&self) -> Result<Vec<BrawlerModel>>;
    async fn get_all_brawlers(&self) -> Result<Vec<BrawlerModel>>;
}
