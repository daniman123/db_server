use crate::extractors::authentication_token::AuthenticationToken;
use crate::operations::get_ops::get_protected_ops::get_following;
use crate::operations::get_ops::get_username;
use crate::repositories::user_auth::{handle_login, handle_logout};
use crate::types::{AppState, JsonResponse};
use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::{Deserialize, Serialize};
// use sqlx::{Pool, Sqlite};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::get().to(protected))
        .route("/get-protected-followers", web::get().to(protected_followers))
        .route("/login", web::post().to(handle_login))
        .route("/logout", web::post().to(handle_logout))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct PersistResponse {
    message: String,
    id: i32,
    username: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshClaims {
    pub id: String,
    exp: usize,
}

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::minutes(15)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(EncodeResponse {
        message: "success".to_owned(),
        token,
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct DecodeBody {
    token: String,
}

#[derive(Serialize, Deserialize)]
struct DecodeResponse {
    message: String,
    id: usize,
}

async fn decode_token(body: web::Json<DecodeBody>, secret: web::Data<String>) -> HttpResponse {
    let decoded: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(token) => HttpResponse::Ok().json(DecodeResponse {
            message: "Authorized!".to_string(),
            id: token.claims.id,
        }),
        Err(e) => HttpResponse::BadRequest().json(Response {
            message: e.to_string(),
        }),
    }
}

async fn protected(auth_token: AuthenticationToken, state: web::Data<AppState>) -> HttpResponse {
    let username = get_username(auth_token.id as i32, state).await;

    HttpResponse::Ok().json(PersistResponse {
        message: "protected".to_owned(),
        id: auth_token.id.try_into().unwrap(),
        username,
    })
}

async fn protected_followers(
    auth_token: AuthenticationToken,
    state: web::Data<AppState>,
) -> HttpResponse {
    let result = get_following(auth_token.id as i64, state).await;

    let usernames: Vec<String> = result
        .iter()
        .map(|result| result.username.clone())
        .collect();

    HttpResponse::Ok().json(JsonResponse::new(usernames))
}
