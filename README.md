# Posts API

A Rust-based API built with Actix-web and Diesel ORM for PostgreSQL.

## Features

- User management (create, retrieve by ID/username)
- Post creation and retrieval
- Comments system
- Like and share functionality
- Follow/follower relationships
- User posts and social connections

## Tech Stack

- **Framework**: Actix-web
- **Database**: PostgreSQL with Diesel ORM
- **Authentication**: JWT tokens
- **Validation**: Validator crate
- **Password Hashing**: bcrypt

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Set up environment variables in `.env`:
```
DATABASE_URL=postgresql://username:password@localhost/posts_db
```

3. Run migrations:
```bash
diesel migration run
```

4. Start the server:
```bash
cargo run
```

Server runs on `http://127.0.0.1:8080`

## API Endpoints

### Users
- `POST /api/users` - Create user
- `GET /api/users/{id}` - Get user by ID
- `GET /api/users/username/{username}` - Get user by username
- `GET /api/users/{user_id}/posts` - Get user's posts
- `GET /api/users/{user_id}/followers` - Get user's followers
- `GET /api/users/{user_id}/following` - Get users being followed

### Posts
- `POST /api/posts` - Create post
- `GET /api/posts` - Get all posts
- `GET /api/posts/{post_id}/comments` - Get post comments

### Comments
- `POST /api/comments` - Create comment

### Interactions
- `POST /api/posts/{post_id}/like/{user_id}` - Like post
- `POST /api/posts/{post_id}/share/{user_id}` - Share post

### Social
- `POST /api/users/{follower_id}/follow/{following_id}` - Follow user

## Database Schema

- **users**: User profiles with follower/following counts
- **posts**: User posts with content and images
- **comments**: Post comments
- **interactions**: Likes and shares
- **follows**: User follow relationships