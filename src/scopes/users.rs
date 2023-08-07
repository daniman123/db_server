use actix_web::{ web, HttpResponse, Scope };
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
    Validation,
    Algorithm,
    DecodingKey,
    TokenData,
    decode,
    errors::Error as JwtError,
};
use serde::{ Serialize, Deserialize };
use chrono::{ Utc, Duration };
use crate::{
    extractors::authentication_token::AuthenticationToken,
    types::{ UserLoginData, AppState }, utils::tools::generate_token,
};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode-token/{id}", web::get().to(encode_token))
        .route("/decode-token", web::post().to(decode_token))
        .route("/protected", web::get().to(protected))
        // .route("/refresh/{id}", web::get().to(encode_refresh_token))
        .route("/login", web::post().to(handle_login))
}

#[derive(Debug)]
struct UserExists {
    user_id: Option<i64>,
    // username: Option<String>,
}
async fn handle_login(body: web::Json<UserLoginData>, state: web::Data<AppState>, secret: web::Data<String>) -> HttpResponse {
    let db_res = sqlx::query_as!(UserExists,"SELECT user_id FROM users WHERE username = ?", body.username)
    .fetch_optional(&state.db.clone()).await;

    match db_res.unwrap() {
        Some(db_res) => {
        let deez= sqlx::query!("SELECT * FROM user_credentials WHERE user_id = ?", db_res.user_id).fetch_one(&state.db.clone()).await;
        let check_pass = deez.unwrap().passphrase;
        let compared_check = argon2::verify_encoded(&check_pass, &body.password.as_bytes()).unwrap_or(false);

        if compared_check {
            let id:i32 = db_res.user_id.unwrap().try_into().unwrap();

            let access_duration = Duration::minutes(15);
            let access_token = generate_token(id, secret.into_inner().to_string(), access_duration).await;
            // HttpResponse::Ok().set_header("Set-cookie", access_token.clone()).json(serde_json::json!({"username": body.username, "user_id":id, "access_token":access_token }))
            let cookie_header = format!("jwt={}; HttpOnly", access_token);
            HttpResponse::Ok()
                .header("Set-Cookie", cookie_header)
                .json(serde_json::json!({"username": body.username, "user_id":id, "access_token":access_token }))
        } else{
            HttpResponse::BadRequest().body("WORNG PASOWRD")   
        }
    },
        None => HttpResponse::BadRequest().body("NOT WORK")   
    }

}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
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

// async fn encode_refresh_token(path: web::Path<String>, secret: web::Data<String>) -> HttpResponse {
//     // let id: String = path.to_string();
//     // let exp: usize = (Utc::now() + Duration::minutes(15)).timestamp() as usize;
//     // let claims: RefreshClaims = RefreshClaims { id, exp };
//     // let token: String = encode(
//     //     &Header::default(),
//     //     &claims,
//     //     &EncodingKey::from_secret(secret.as_str().as_ref())
//     // ).unwrap();

//     HttpResponse::Ok().body("body")
//     // HttpResponse::Ok().json(EncodeResponse { message: "success".to_owned(), token })
// }

async fn encode_token(path: web::Path<usize>, secret: web::Data<String>) -> HttpResponse {
    let id: usize = path.into_inner();
    let exp: usize = (Utc::now() + Duration::minutes(15)).timestamp() as usize;
    let claims: Claims = Claims { id, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap();

    HttpResponse::Ok().json(EncodeResponse { message: "success".to_owned(), token })
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
        &Validation::new(Algorithm::HS256)
    );

    match decoded {
        Ok(token) =>
            HttpResponse::Ok().json(DecodeResponse {
                message: "Authorized!".to_string(),
                id: token.claims.id,
            }),
        Err(e) => HttpResponse::BadRequest().json(Response { message: e.to_string() }),
    }
}

async fn protected(auth_token: AuthenticationToken) -> HttpResponse {
    println!("{}", auth_token.id);
    HttpResponse::Ok().json(Response { message: "protected".to_owned() })
}
