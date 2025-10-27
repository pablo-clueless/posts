CREATE TABLE interactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    interaction_type VARCHAR NOT NULL CHECK (interaction_type IN ('like', 'share'))
);

CREATE UNIQUE INDEX idx_interactions_user_post_type ON interactions(user_id, post_id, interaction_type);
CREATE INDEX idx_interactions_post_id ON interactions(post_id);