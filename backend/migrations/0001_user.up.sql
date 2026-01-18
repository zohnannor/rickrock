-- Create users table
CREATE TABLE users (
    -- intertal unique id of the user
    id UUID PRIMARY KEY,
    -- user's login credentials (email)
    login CITEXT NOT NULL CHECK (length(login) <= 255),
    -- user's password hash
    password_hash TEXT NOT NULL,
    -- user's handle (short unique id)
    handle TEXT NOT NULL CHECK (length(handle) BETWEEN 3 AND 30),
    -- user's optional display name
    display_name TEXT CHECK (
        length(display_name) > 0
        AND length(display_name) <= 50
    ),
    -- user's optional bio ("about me")
    bio TEXT CHECK (length(bio) <= 500),
    -- TODO: add these fiels when files are implemented
    -- user's optional profile picture id
    -- older avatars will be stored in a separate relation table
    -- current_avatar_id UUID REFERENCES user_avatars ON UPDATE RESTRICT ON DELETE RESTRICT,
    -- user's last activity timestamp
    last_seen_at TIMESTAMP WITH TIME ZONE,
    -- whether the user has privated their profile, only select people can see it
    is_private BOOLEAN DEFAULT FALSE,
    -- verification status (null for unverified)
    verified_at TIMESTAMP WITH TIME ZONE,
    -- user's creation timestamp
    created_at TIMESTAMP WITH TIME ZONE,
    -- user's deletion timestamp (30 days to recover the account)
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX users_login_lower_idx ON users (login);

CREATE UNIQUE INDEX users_handle_lower_idx ON users (lower(handle));

CREATE INDEX users_display_name_idx ON users (display_name);
