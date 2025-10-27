use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,        // user id
    pub username: String, // username
    pub exp: i64,         // expiration time
    pub iat: i64,         // issued at
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, message = "Username must be at least 3 characters"))]
    pub username: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,

    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: AuthUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    pub image_url: Option<String>,
}

#[derive(Debug)]
pub struct AuthError {
    pub message: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError: {}", self.message)
    }
}

impl std::error::Error for AuthError {}

impl From<jsonwebtoken::errors::Error> for AuthError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AuthError {
            message: format!("JWT error: {}", err),
        }
    }
}

impl From<bcrypt::BcryptError> for AuthError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AuthError {
            message: format!("Bcrypt error: {}", err),
        }
    }
}

pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl JwtConfig {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expiration_hours = env::var("JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);

        JwtConfig {
            secret,
            expiration_hours,
        }
    }

    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_ref())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_ref())
    }
}

pub fn create_token(
    user_id: Uuid,
    username: &str,
    config: &JwtConfig,
) -> Result<String, AuthError> {
    let now = Utc::now();
    let expires = now + Duration::hours(config.expiration_hours);

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expires.timestamp(),
        iat: now.timestamp(),
    };

    let token = encode(&Header::default(), &claims, &config.encoding_key())?;
    Ok(token)
}

pub fn validate_token(token: &str, config: &JwtConfig) -> Result<Claims, AuthError> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &config.decoding_key(), &validation)?;
    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    Ok(bcrypt::hash(password, bcrypt::DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    Ok(bcrypt::verify(password, hash)?)
}
