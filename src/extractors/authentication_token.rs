use std::future::{ready, Ready};

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, web, Error as ActixWebError, FromRequest, HttpMessage,
    HttpRequest,
};
use jsonwebtoken::{decode, errors::Error as JwtError, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::scopes::users::Claims;

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let binding = match req.cookie("jwt") {
            Some(cookie) => cookie,
            None => return ready(Err(ActixWebError::from(ErrorUnauthorized("Unauthorized")))),
        };

        let auth_token: &str = binding.value();

        let secret: String = req.app_data::<web::Data<String>>().unwrap().to_string();

        // decode token w secret
        let decode: Result<TokenData<Claims>, JwtError> = decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );
        // return self - auth token
        match decode {
            Ok(token) => ready(Ok(AuthenticationToken {
                id: token.claims.id,
            })),
            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized"))),
        }
    }
}

// println!("{:?}", auth_token);

// get auth toke from auth headder
// let auth_header: Option<&http::HeaderValue> = req
//     .headers()
//     .get(http::header::AUTHORIZATION);
// let auth_token: String = auth_header.unwrap().to_str().unwrap_or("").to_string();
// if auth_token.is_empty() {
//     return ready(Err(ErrorUnauthorized("Invalid Auth token")));
// }
