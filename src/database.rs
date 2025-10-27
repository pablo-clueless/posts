use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use uuid::Uuid;

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
// pub type DbResult<T> = Result<T, DbError>;

fn interact_error_to_db_error(e: deadpool_diesel::InteractError) -> DbError {
    format!("{}", e).into()
}

pub struct Database {
    pub pool: deadpool_diesel::postgres::Pool,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

impl Database {
    pub fn new(pool: deadpool_diesel::postgres::Pool) -> Self {
        Self { pool }
    }

    // User operations
    pub async fn create_user(&self, new_user: NewUser) -> Result<User, DbError> {
        let conn = self.pool.get().await?;
        let user = conn
            .interact(move |conn| {
                diesel::insert_into(users::table)
                    .values(&new_user)
                    .returning(User::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(user)
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<Option<User>, DbError> {
        let conn = self.pool.get().await?;
        let user = conn
            .interact(move |conn| {
                users::table
                    .filter(users::id.eq(user_id))
                    .select(User::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(user)
    }

    pub async fn get_user_by_username(&self, username_str: &str) -> Result<Option<User>, DbError> {
        let username = username_str.to_string();
        let conn = self.pool.get().await?;
        let user = conn
            .interact(move |conn| {
                users::table
                    .filter(users::username.eq(username))
                    .select(User::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(user)
    }

    // Post operations
    pub async fn create_post(&self, new_post: NewPost) -> Result<Post, DbError> {
        let conn = self.pool.get().await?;
        let post = conn
            .interact(move |conn| {
                diesel::insert_into(posts::table)
                    .values(&new_post)
                    .returning(Post::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(post)
    }

    pub async fn get_posts(&self, limit: i64, offset: i64) -> Result<Vec<Post>, DbError> {
        let conn = self.pool.get().await?;
        let posts = conn
            .interact(move |conn| {
                posts::table
                    .order(posts::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .select(Post::as_select())
                    .load(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(posts)
    }

    pub async fn get_user_posts(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, DbError> {
        let conn = self.pool.get().await?;
        let posts = conn
            .interact(move |conn| {
                posts::table
                    .filter(posts::user_id.eq(user_id))
                    .order(posts::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .select(Post::as_select())
                    .load(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(posts)
    }

    // Comment operations
    pub async fn create_comment(&self, new_comment: NewComment) -> Result<Comment, DbError> {
        let conn = self.pool.get().await?;
        let comment = conn
            .interact(move |conn| {
                diesel::insert_into(comments::table)
                    .values(&new_comment)
                    .returning(Comment::as_returning())
                    .get_result(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(comment)
    }

    pub async fn get_post_comments(&self, post_id: Uuid) -> Result<Vec<Comment>, DbError> {
        let conn = self.pool.get().await?;
        let comments = conn
            .interact(move |conn| {
                comments::table
                    .filter(comments::post_id.eq(post_id))
                    .order(comments::created_at.asc())
                    .select(Comment::as_select())
                    .load(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(comments)
    }

    // Interaction operations
    pub async fn like_post(&self, post_id: Uuid, user_id: Uuid) -> Result<Interaction, DbError> {
        let conn = self.pool.get().await?;
        let interaction = conn
            .interact(move |conn| {
                conn.transaction(|conn| {
                    // Create interaction
                    let interaction = diesel::insert_into(interactions::table)
                        .values(NewInteraction {
                            post_id,
                            user_id,
                            interaction_type: "like".to_string(),
                        })
                        .returning(Interaction::as_returning())
                        .get_result(conn)?;

                    // Update likes count
                    diesel::update(posts::table.filter(posts::id.eq(post_id)))
                        .set(posts::likes_count.eq(posts::likes_count + 1))
                        .execute(conn)?;

                    Ok(interaction)
                })
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(interaction)
    }

    pub async fn share_post(&self, post_id: Uuid, user_id: Uuid) -> Result<Interaction, DbError> {
        let conn = self.pool.get().await?;
        let interaction = conn
            .interact(move |conn| {
                conn.transaction(|conn| {
                    // Create interaction
                    let interaction = diesel::insert_into(interactions::table)
                        .values(NewInteraction {
                            post_id,
                            user_id,
                            interaction_type: "share".to_string(),
                        })
                        .returning(Interaction::as_returning())
                        .get_result(conn)?;

                    // Update shares count
                    diesel::update(posts::table.filter(posts::id.eq(post_id)))
                        .set(posts::shares_count.eq(posts::shares_count + 1))
                        .execute(conn)?;

                    Ok(interaction)
                })
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(interaction)
    }

    // Follow operations
    pub async fn follow_user(
        &self,
        follower_id: Uuid,
        following_id: Uuid,
    ) -> Result<Follow, DbError> {
        let conn = self.pool.get().await?;
        let follow = conn
            .interact(move |conn| {
                conn.transaction(|conn| {
                    // Create follow relationship
                    let follow = diesel::insert_into(follows::table)
                        .values(NewFollow {
                            follower_id,
                            following_id,
                        })
                        .returning(Follow::as_returning())
                        .get_result(conn)?;

                    // Update follower's following count
                    diesel::update(users::table.filter(users::id.eq(follower_id)))
                        .set(users::following_count.eq(users::following_count + 1))
                        .execute(conn)?;

                    // Update following user's followers count
                    diesel::update(users::table.filter(users::id.eq(following_id)))
                        .set(users::followers_count.eq(users::followers_count + 1))
                        .execute(conn)?;

                    Ok(follow)
                })
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e: diesel::result::Error| Box::new(e) as DbError)?;
        Ok(follow)
    }

    pub async fn get_user_followers(&self, user_id: Uuid) -> Result<Vec<User>, DbError> {
        let conn = self.pool.get().await?;
        let followers = conn
            .interact(move |conn| {
                follows::table
                    .inner_join(users::table.on(follows::follower_id.eq(users::id)))
                    .filter(follows::following_id.eq(user_id))
                    .select(User::as_select())
                    .load(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(followers)
    }

    pub async fn get_user_following(&self, user_id: Uuid) -> Result<Vec<User>, DbError> {
        let conn = self.pool.get().await?;
        let following = conn
            .interact(move |conn| {
                follows::table
                    .inner_join(users::table.on(follows::following_id.eq(users::id)))
                    .filter(follows::follower_id.eq(user_id))
                    .select(User::as_select())
                    .load(conn)
            })
            .await
            .map_err(interact_error_to_db_error)?
            .map_err(|e| Box::new(e) as DbError)?;
        Ok(following)
    }
}
