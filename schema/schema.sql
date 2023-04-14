----------------------------------
---------  Content Part ----------

-- The basic gutp user model
CREATE TABLE gutpuser (
  id varchar primary key,		    -- for machine read
  account varchar not null,	        -- for human read
  nickname varchar not null,
  avatar varchar not null,		    -- user's avatar, usually an url link to outside
  role smallint not null,           -- permission model
  status smallint not null,
  signup_time bigint not null,
  publickey varchar not null,	    -- for decentralized account
  ext varchar not null              -- for certain extension
);

CREATE TABLE gutpuser_idhash (
  id varchar primary key,		
  hash varchar not null
);

-- The abstract of subforum, blogspace, personal space, etc.
CREATE TABLE gutpsubspace (
  id varchar primary key,		
  title varchar not null,
  description varchar not null,
  banner varchar not null,          -- a pic url to capture people's eyes
  owner varchar not null,		    -- who ownes this subspace
  profession varchar not null,      -- which profession this subspace belongs to
  appid varchar not null,           -- which app/platform this subspace belongs to 
  status smallint not null,
  weight smallint not null		    -- used for ranks
  created_time bigint not null,
);

CREATE TABLE gutpsubspace_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE gutppost (
  id varchar primary key,
  title varchar not null,
  content varchar not null,
  author_id varchar not null,
  subspace_id varchar not null,		-- which subspace a post belongs to
  extlink varchar not null,
  profession varchar not null,      -- which profession this subspace belongs to
  appid varchar not null,           -- which app/platform this subspace belongs to
  status smallint not null,
  weight smallint not null		    -- used for ranks
  created_time bigint not null,
  updated_time bigint not null
);

CREATE TABLE gutppost_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE gutpcomment (
  id varchar primary key,
  content varchar not null,
  author_id varchar not null,
  post_id varchar not null,         -- the post this comment belongs to
  rcomment_id varchar not null,     -- the replied comment id, if has
  status smallint not null,
  created_time bigint not null
);

CREATE TABLE gutpcomment_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE gutptag (
  id varchar primary key,
  caption varchar not null,
  subspace_id varchar not null,		  -- which subspace a tag belongs to
  creator_id varchar not null,
  kind smallint not null,		      -- system level tag, or customized tag
  created_time bigint not null
);

CREATE TABLE gutptag_idhash (
  id varchar primary key,		
  hash varchar not null
);

-- The M:N index mapping table
CREATE TABLE gutpposttag (
  id varchar primary key,		-- the post id
  tag_id varchar not null		-- the tag id
);

CREATE TABLE gutpposttag_idhash (
  id varchar primary key,		
  hash varchar not null
);

---------------------------------
---------  Social Part ----------

CREATE TABLE gutplikeit (
  id varchar primary key,		-- target id
  ttype varchar not null,		-- target type
  user_id: varchar not null		-- who does it
);

CREATE TABLE gutplikeit_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE gutpreward (
  id varchar primary key,		-- target id
  ttype varchar not null,		-- target type
  user_id: varchar not null,	-- who do it
  amount: int not null			-- the reward amount
);

CREATE TABLE gutpreward_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE gutpfollow (
  id varchar primary key,		-- target user id
  user_id: varchar not null,	-- who follows the target
  time: bigint not null			-- the followed time
);

CREATE TABLE gutpfollow_idhash (
  id varchar primary key,		
  hash varchar not null
);

