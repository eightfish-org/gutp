CREATE TABLE gutpuser (
    id VARCHAR PRIMARY KEY,
    account VARCHAR UNIQUE NOT NULL,  -- readable name, like `michael`
    oauth_source VARCHAR NOT NULL,  -- from where, like github, google, facebook
    nickname VARCHAR NOT NULL,                      
    avatar VARCHAR NOT NULL,
    role SMALLINT NOT NULL CHECK (role IN (0, 1, 2, 3, 4, 5)),  -- role for simple permission system
	status SMALLINT NOT NULL CHECK (status IN (0, 1, 2)),  -- 0: inactive, 1: active, 2: frozen
    created_time BIGINT NOT NULL,
	data_source VARCHAR NOT NULL
);

CREATE TABLE gutpsubspace (
    id VARCHAR PRIMARY KEY,
    slug VARCHAR NOT NULL,                         -- slug: readable name of this subspace in an url
    title VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    banner VARCHAR NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,   -- is this subspace a public square, all data in this subspace is plaintext
    status SMALLINT NOT NULL,                   -- normal, forzen, blacklist, deleted ...
    weight SMALLINT NOT NULL,                   -- weight of this subspace, used for ranking and recommendation
    owner_id VARCHAR REFERENCES gutpuser(id) ON DELETE RESTRICT,   -- Tracks the owner
    created_time BIGINT NOT NULL,
	data_source VARCHAR NOT NULL
);

CREATE TABLE gutppost (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    author_id VARCHAR NOT NULL REFERENCES gutpuser(id) ON DELETE RESTRICT,  -- Tracks the author,
    subspace_id VARCHAR NOT NULL REFERENCES gutpsubspace(id) ON DELETE RESTRICT,  -- Tracks the subspace
    parent_post_id VARCHAR REFERENCES gutppost(id) ON DELETE RESTRICT,  -- used to construct a post tree
    ext_link VARCHAR NOT NULL,                      -- for link aggregator-like application
    is_public BOOLEAN NOT NULL DEFAULT TRUE,                 -- if public, the content of this post will be stored in plaintext, otherwise encrypted
    status SMALLINT NOT NULL CHECK (status IN (0, 1, 2, 3, 4, 5, 6, 7)),
    weight SMALLINT NOT NULL,                   -- used for ranking and recommendation
    created_time BIGINT NOT NULL,
    updated_time BIGINT NOT NULL,                -- the last time timestamp of updating
	data_source VARCHAR NOT NULL
);

CREATE TABLE gutpcomment (
    id VARCHAR PRIMARY KEY,
    content VARCHAR NOT NULL,
    author_id VARCHAR NOT NULL REFERENCES gutpuser(id) ON DELETE RESTRICT,   -- Tracks the author,
    post_id VARCHAR NOT NULL REFERENCES gutppost(id) ON DELETE RESTRICT,
    parent_comment_id VARCHAR REFERENCES gutpcomment(id) ON DELETE RESTRICT,            -- used to construct a comment tree
    is_public BOOLEAN NOT NULL DEFAULT TRUE,
    status SMALLINT NOT NULL CHECK (status IN (0, 1, 2, 3, 4, 5, 6, 7)),
    weight INTEGER NOT NULL,
    created_time BIGINT NOT NULL,
	data_source VARCHAR NOT NULL
);

-- a tag always belongs to certain subspace
CREATE TABLE gutptag (
    id VARCHAR PRIMARY KEY,
    caption VARCHAR NOT NULL,
    subspace_id VARCHAR NOT NULL REFERENCES gutpsubspace(id) ON DELETE RESTRICT,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,  -- is this tag a public (plaintext) tag
    weight SMALLINT NOT NULL,
    created_time BIGINT NOT NULL,
	data_source VARCHAR NOT NULL
);

-- M:N relation table between subspace and tag
CREATE TABLE gutpsubspacetag (
    id VARCHAR PRIMARY KEY,
    subspace_id VARCHAR NOT NULL REFERENCES gutpsubspace(id) ON DELETE RESTRICT,
    tag_id VARCHAR NOT NULL REFERENCES gutptag(id) ON DELETE RESTRICT,
    created_time BIGINT NOT NULL
);

-- M:N relation table between post and tag
CREATE TABLE gutpposttag (
    id VARCHAR PRIMARY KEY,
    post_id VARCHAR NOT NULL REFERENCES gutppost(id) ON DELETE RESTRICT,
    tag_id VARCHAR NOT NULL REFERENCES gutptag(id) ON DELETE RESTRICT,
    created_time BIGINT NOT NULL
);

-- for history version control
CREATE TABLE gutppostdiff (
    id VARCHAR PRIMARY KEY,
    post_id VARCHAR NOT NULL REFERENCES gutppost(id) ON DELETE RESTRICT,
    diff TEXT NOT NULL,                               -- diff bewteen old version and new version
    version_num INTEGER NOT NULL,                     -- will increase 1 by every modification
    created_time BIGINT NOT NULL
);

-- subspace admins
CREATE TABLE gutpmoderator (
    id VARCHAR PRIMARY KEY,
    user_id VARCHAR NOT NULL REFERENCES gutpuser(id) ON DELETE RESTRICT,
    subspace_id VARCHAR NOT NULL REFERENCES gutpsubspace(id) ON DELETE RESTRICT,
    is_subspace_moderator BOOLEAN NOT NULL DEFAULT TRUE,  -- is it a subspace-scoped moderator
    perm_level SMALLINT NOT NULL,  -- a simple mechanism for permission control
    tag_id VARCHAR REFERENCES gutptag(id) ON DELETE RESTRICT,
    created_time BIGINT NOT NULL
);

-- an unified extension object for all levels
CREATE TABLE gutpextobj (
    id VARCHAR PRIMARY KEY,
    caption VARCHAR NOT NULL,
    content VARCHAR NOT NULL,  -- may be a json 
    user_id VARCHAR REFERENCES gutpuser(id) ON DELETE RESTRICT,
    subspace_id VARCHAR REFERENCES gutpsubspace(id) ON DELETE RESTRICT,
    tag_id VARCHAR REFERENCES gutptag(id) ON DELETE RESTRICT,
    post_id VARCHAR REFERENCES gutppost(id) ON DELETE RESTRICT,
    comment_id VARCHAR REFERENCES gutpcomment(id) ON DELETE RESTRICT,
    is_public BOOLEAN NOT NULL DEFAULT TRUE,  -- is it's data public/plaintext
    weight SMALLINT NOT NULL,  -- for ranking
    created_time BIGINT NOT NULL
);
