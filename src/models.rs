use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub username: String,
    pub image_url: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub username: String,
    pub image_url: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub likes_count: i32,
    pub shares_count: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub user_id: Uuid,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::interactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Interaction {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub interaction_type: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::interactions)]
pub struct NewInteraction {
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub interaction_type: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::follows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Follow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub follower_id: Uuid,
    pub following_id: Uuid,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::follows)]
pub struct NewFollow {
    pub follower_id: Uuid,
    pub following_id: Uuid,
}

// Response models with relationships
#[derive(Serialize, Deserialize, Debug)]
pub struct UserWithRelations {
    #[serde(flatten)]
    pub user: User,
    pub followers: Vec<User>,
    pub following: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithRelations {
    #[serde(flatten)]
    pub post: Post,
    pub user: User,
    pub comments: Vec<CommentWithUser>,
    pub likes: Vec<InteractionWithUser>,
    pub shares: Vec<InteractionWithUser>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentWithUser {
    #[serde(flatten)]
    pub comment: Comment,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionWithUser {
    #[serde(flatten)]
    pub interaction: Interaction,
    pub user: User,
}
