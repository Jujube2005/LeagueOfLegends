use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use anyhow::Result;

use crate::{
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        argon2,
        jwt::{authentication_model::{LoginModel, RecoverPasswordModel}, jwt_model::Passport},
    },
};
pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}
impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Sync + Send,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let username = login_model.username.clone();

        //find this user in database
        let user = self.brawler_repository.find_by_username(username).await?;
        let hashed_password = user.password;

        if !argon2::verify(login_model.password, hashed_password)? {
            return Err(anyhow::anyhow!("Invalid Password !!"));
        }

        let passport = Passport::new(user.id, user.display_name, user.avatar_url)?;
        Ok(passport)
    }

    pub async fn recover_password(&self, model: RecoverPasswordModel) -> Result<String> {
        let username = model.username;
        
        // Find user by username
        let user = self.brawler_repository.find_by_username(username.clone()).await;
        
        match user {
            Ok(u) => {
                // In a real application, we would generate a reset token and send an email here.
                // For now, we'll just simulate it.
                println!("Recover password requested for user: {}", u.username);
                println!("Sending recovery email...");
                
                // Simulate sending email by writing to a log file
                let log_message = format!(
                    "[{}] To User: {}\nSubject: Password Recovery\nBody: Click here to reset your password: http://localhost:4200/reset-password?token=mock_token_for_{}\n----------------------------------------\n",
                    chrono::Utc::now(),
                    u.username,
                    u.username
                );

                if let Ok(mut file) = tokio::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("recovery_emails.log")
                    .await 
                {
                    let _ = file.write_all(log_message.as_bytes()).await;
                }

                Ok("Recovery email sent (Simulation)".to_string())
            },
            Err(_) => {
                // User not found
                // In production, we should probably return Ok("Recovery email sent") to prevent user enumeration
                Err(anyhow::anyhow!("User with this username not found"))
            }
        }
    }
}