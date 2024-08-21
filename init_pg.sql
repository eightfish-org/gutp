CREATE TABLE gutpuser (
    id TEXT PRIMARY KEY,
    account TEXT NOT NULL,                      -- readable name, like michael
    oauth_source TEXT NOT NULL,                 -- from where, like github, google, facebook
    nickname TEXT NOT NULL,                      
    avatar TEXT NOT NULL,
    role SMALLINT NOT NULL,                     -- role for simple permission system
    status SMALLINT NOT NULL,                   -- current status of this account, like normal, frozen, deleted, ...
    pub_settings TEXT NOT NULL,                 -- some settings could be public (those with no privacy issues)
    ext TEXT NOT NULL,                          -- some extension content
    created_time BIGINT NOT NULL,
);
CREATE TABLE gutpuser_idhash (
	id TEXT PRIMARY KEY,
	hash TEXT NOT NULL
);

CREATE TABLE gutpsubspace (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL,                         -- slug readable name of this subspace within an url
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    banner TEXT NOT NULL,
    category TEXT NOT NULL,                     -- the category of this subspace 
    is_public BOOLEAN NOT NULL,                 -- is this subspace a public square, all data in this subspace is plaintext
    status SMALLINT NOT NULL,                   -- normal, forzen, blacklist, deleted ...
    weight SMALLINT NOT NULL,                   -- weight of this subspace, used for ranking and recommendation
    owner_id TEXT NOT NULL,                     -- is this subspace a personal blog space, owner_id is the person's id
    app_id TEXT NOT NULL,                       -- multiple app's can be connected to the same gutp instance, so use app_id to distinguish
    created_time BIGINT NOT NULL,
);
CREATE TABLE gutpsubspace_idhash (
	id TEXT PRIMARY KEY,
	hash TEXT NOT NULL
);

CREATE TABLE gutppost (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_nickname TEXT NOT NULL,              -- for convinence, no need to do join operation when show info of this post
    subspace_id TEXT NOT NULL,                  -- belongs to which subspace
    parent_post_id TEXT NOT NULL,               -- used to construct a post tree
    extlink TEXT NOT NULL,                      -- for link aggregator-like application
    is_public BOOLEAN NOT NULL,                 -- if public, the content of this post will be stored in plaintext, otherwise encrypted
    status SMALLINT NOT NULL,
    weight SMALLINT NOT NULL,                   -- used for ranking and recommendation
    category TEXT NOT NULL,                     -- the same meaning with the one of subspace, used to retreive quickly 
    app_id TEXT NOT NULL,                       -- which app's post
    created_time BIGINT NOT NULL,
    updated_time BIGINT NOT NULL,               -- the last time timestamp of updating
);
CREATE TABLE gutppost_idhash (
	id TEXT PRIMARY KEY,
	hash TEXT NOT NULL
);

CREATE TABLE gutpcomment (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    author_id TEXT NOT NULL,
    author_nickname TEXT NOT NULL,
    post_id TEXT NOT NULL,                      -- which post it belongs to
    parent_comment_id TEXT NOT NULL,            -- used to construct a comment tree
    is_public BOOLEAN NOT NULL,
    status SMALLINT NOT NULL,
    weight INTEGER NOT NULL,
    created_time BIGINT NOT NULL,
);
CREATE TABLE gutpcomment_idhash (
	id TEXT PRIMARY KEY,
	hash TEXT NOT NULL
);

CREATE TABLE gutptag (
    id TEXT PRIMARY KEY,
    caption TEXT NOT NULL,
    subspace_id TEXT NOT NULL,
    creator_id TEXT NOT NULL,
    is_subspace_tag BOOLEAN NOT NULL,
    is_public BOOLEAN NOT NULL,
    weight SMALLINT NOT NULL,
    created_time BIGINT NOT NULL,
);
CREATE TABLE gutptag_idhash (
	id varchar PRIMARY KEY,
	hash varchar NOT NULL
);

CREATE TABLE gutpposttag (
    id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_time BIGINT NOT NULL,
    create_time_on_chain bigint not null
);
CREATE TABLE gutpposttag_idhash (
	id varchar PRIMARY KEY,
	hash varchar NOT NULL
);

CREATE TABLE gutppostdiff (
    id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL,
    diff TEXT NOT NULL,
    version_num INTEGER NOT NULL,
    created_time BIGINT NOT NULL,
    create_time_on_chain bigint not null
);
CREATE TABLE gutppostdiff_idhash (
	id varchar PRIMARY KEY,
	hash varchar NOT NULL
);

CREATE TABLE gutpmoderator (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    is_subspace_moderator BOOLEAN NOT NULL,
    subspace_id TEXT,
    tag_id TEXT,
    permission_level SMALLINT NOT NULL,
    created_time BIGINT NOT NULL,
    create_time_on_chain bigint not null
);
CREATE TABLE gutpmoderator_idhash (
	id varchar PRIMARY KEY,
	hash varchar NOT NULL
);

CREATE TABLE gutpextobj (
    id TEXT PRIMARY KEY,
    caption TEXT NOT NULL,
    content TEXT NOT NULL,
    subspace_id TEXT,
    tag_id TEXT,
    creator_id TEXT NOT NULL,
    is_subspace_ext BOOLEAN NOT NULL,
    weight SMALLINT NOT NULL,
    created_time BIGINT NOT NULL,
    create_time_on_chain bigint not null
);
CREATE TABLE gutpextobj_idhash (
	id varchar PRIMARY KEY,
	hash varchar NOT NULL
);
