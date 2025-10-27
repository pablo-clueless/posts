use crate::database::Database;
use crate::models::*;
use actix_web::{HttpResponse, Responder, web};
use serde::Deserialize;
use uuid::Uuid;

pub async fn create_user(db: web::Data<Database>, new_user: web::Json<NewUser>) -> impl Responder {
    match db.create_user(new_user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                HttpResponse::Conflict().body("Username or email already exists")
            } else {
                HttpResponse::InternalServerError().body(format!("Error creating user: {}", e))
            }
        }
    }
}

pub async fn get_user(db: web::Data<Database>, user_id: web::Path<Uuid>) -> impl Responder {
    match db.get_user(*user_id).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching user: {}", e)),
    }
}

pub async fn get_user_by_username(
    db: web::Data<Database>,
    username: web::Path<String>,
) -> impl Responder {
    match db.get_user_by_username(&username).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching user: {}", e)),
    }
}

pub async fn create_post(db: web::Data<Database>, new_post: web::Json<NewPost>) -> impl Responder {
    match db.create_post(new_post.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating post: {}", e)),
    }
}

pub async fn get_posts(
    db: web::Data<Database>,
    query: web::Query<PaginatedQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    match db.get_posts(limit, offset).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error fetching posts: {}", e)),
    }
}

pub async fn get_user_posts(
    db: web::Data<Database>,
    user_id: web::Path<Uuid>,
    query: web::Query<PaginatedQuery>,
) -> impl Responder {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);

    match db.get_user_posts(*user_id, limit, offset).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching user posts: {}", e))
        }
    }
}

pub async fn create_comment(
    db: web::Data<Database>,
    new_comment: web::Json<NewComment>,
) -> impl Responder {
    match db.create_comment(new_comment.into_inner()).await {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error creating comment: {}", e))
        }
    }
}

pub async fn get_post_comments(
    db: web::Data<Database>,
    post_id: web::Path<Uuid>,
) -> impl Responder {
    match db.get_post_comments(*post_id).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching comments: {}", e))
        }
    }
}

pub async fn like_post(db: web::Data<Database>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (post_id, user_id) = path.into_inner();

    match db.like_post(post_id, user_id).await {
        Ok(interaction) => HttpResponse::Created().json(interaction),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                HttpResponse::Conflict().body("Post already liked by user")
            } else {
                HttpResponse::InternalServerError().body(format!("Error liking post: {}", e))
            }
        }
    }
}

pub async fn share_post(db: web::Data<Database>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (post_id, user_id) = path.into_inner();

    match db.share_post(post_id, user_id).await {
        Ok(interaction) => HttpResponse::Created().json(interaction),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                HttpResponse::Conflict().body("Post already shared by user")
            } else {
                HttpResponse::InternalServerError().body(format!("Error sharing post: {}", e))
            }
        }
    }
}

pub async fn follow_user(db: web::Data<Database>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
    let (follower_id, following_id) = path.into_inner();

    if follower_id == following_id {
        return HttpResponse::BadRequest().body("Cannot follow yourself");
    }

    match db.follow_user(follower_id, following_id).await {
        Ok(follow) => HttpResponse::Created().json(follow),
        Err(e) => {
            if e.to_string().contains("unique constraint") {
                HttpResponse::Conflict().body("Already following user")
            } else {
                HttpResponse::InternalServerError().body(format!("Error following user: {}", e))
            }
        }
    }
}

pub async fn get_user_followers(
    db: web::Data<Database>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    match db.get_user_followers(*user_id).await {
        Ok(followers) => HttpResponse::Ok().json(followers),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching followers: {}", e))
        }
    }
}

pub async fn get_user_following(
    db: web::Data<Database>,
    user_id: web::Path<Uuid>,
) -> impl Responder {
    match db.get_user_following(*user_id).await {
        Ok(following) => HttpResponse::Ok().json(following),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching following: {}", e))
        }
    }
}

#[derive(Deserialize)]
pub struct PaginatedQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
