diesel::table! {
    users (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        image_url -> Nullable<Text>,
        followers_count -> Int4,
        following_count -> Int4,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        user_id -> Uuid,
        content -> Text,
        images -> Nullable<Array<Text>>,
        likes_count -> Int4,
        shares_count -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        post_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        images -> Nullable<Array<Text>>,
    }
}

diesel::table! {
    interactions (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        post_id -> Uuid,
        user_id -> Uuid,
        interaction_type -> Varchar,
    }
}

diesel::table! {
    follows (id) {
        id -> Uuid,
        created_at -> Timestamptz,
        follower_id -> Uuid,
        following_id -> Uuid,
    }
}

diesel::joinable!(posts -> users (user_id));
diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(interactions -> posts (post_id));
diesel::joinable!(interactions -> users (user_id));
diesel::joinable!(follows -> users (follower_id));
// diesel::joinable!(follows -> users (following_id));

diesel::allow_tables_to_appear_in_same_query!(users, posts, comments, interactions, follows,);
