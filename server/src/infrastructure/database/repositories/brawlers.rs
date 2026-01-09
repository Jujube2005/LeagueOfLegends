use anyhow::Result;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper, insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use std::sync::Arc;

use crate::{
    config::config_loader::get_jwt_env, domain::{
        entities::brawlers::{BrawlerEntity, RegisterBrawlerEntity},
        repositories::brawlers::BrawlerRepository, value_objects::{base64_image::Base64Image, uploaded_image::UploadedImage},
    }, infrastructure::{cloudinary::UploadImageOptions, database::{postgresql_connection::PgPoolSquad, schema::brawlers}, jwt::{generate_token, jwt_model::{Claims, Passport}}}
};

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<Passport> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let brawler_id = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        let jwt_env = get_jwt_env().expect("JWT environment not configured");
        let token_type = "Bearer".to_string();
        let expires_in = (Utc::now() + Duration::days(jwt_env.life_time_days)).timestamp() as usize;
        let display_name = format!("Brawler{}", brawler_id);

        let claims = Claims {
            sub: brawler_id,
            exp: expires_in,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = generate_token(jwt_env.secret, &claims)?;

        Ok(Passport {
            token_type,
            display_name, 
            access_token, 
            expires_in, 
            avatar_url: None, 
        })
    }

    async fn find_by_username(&self, username: &String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }

    async fn upload_avatar(
        &self,
        brawler_id: i32,
        base64_image: Base64Image,
        option: UploadImageOptions,
    ) -> Result<UploadedImage> {

        let uploaded_image =    
            crate::infrastructure::cloudinary::upload(base64_image, option).await?;

        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(brawlers::table)
            .filter(brawlers::id.eq(brawler_id))
            .set((
                brawlers::avatar_url.eq(uploaded_image.url.clone()),
                brawlers::avatar_public_id.eq(uploaded_image.public_id.clone()),
            ))
            .execute(&mut conn)?;

        Ok(uploaded_image)
    }
}


