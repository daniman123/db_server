use crate::utils::tools::{hash_password, input_validator, is_email_valid, metaphone_encoding};
use actix_web::web::{self};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Pool, Sqlite};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserData {
    pub username: String,
    pub email: String,
    pub passphrase: String,
}

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub phonetic_username: Cow<'static, str>,
    pub email: String,
    pub passphrase: String,
}

impl NewUser {
    pub fn new(request_body_data: web::Json<NewUserData>) -> Result<Self, String> {
        if !input_validator(&request_body_data.username, 15) {
            return Err("Invalid username".to_string());
        }

        if !is_email_valid(&request_body_data.email) {
            return Err("Invalid Email".to_string());
        }

        let username = request_body_data.username.clone();
        let phonetic_username = Cow::Owned(metaphone_encoding(&username));
        let hashed_passphrase = hash_password(&request_body_data.passphrase);

        Ok(NewUser {
            username,
            phonetic_username,
            email: request_body_data.email.clone(),
            passphrase: hashed_passphrase,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchQuery {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonObject {
    pub limit: Value,
}

pub struct AppState {
    pub db: Pool<Sqlite>,
}

#[derive(FromRow)]
pub struct DbResult {
    pub user_id: i64,
}

#[derive(sqlx::FromRow)]
pub struct ChannelNames {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub result: Vec<String>,
}

impl JsonResponse {
    pub fn new(database_return: Vec<String>) -> Self {
        JsonResponse {
            result: database_return,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMetaData {
    pub user_id: i32,
    pub subject_user_id: i32,
    pub activity_type: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub id: i32,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenRes {
    pub access: String,
}
