---------------------------------
---------  Content Part ----------

-- The basic gutp user model
CREATE TABLE gutpuser (
  id varchar primary key,		-- for machine read
  account varchar not null,	-- for human read
  nickname varchar not null,
  avatar varchar not null,		-- user's avatar, usually an url link to outside
  role smallint not null,
  status smallint not null,
  signup_time bigint not null,
  publickey varchar not null,		-- for decentralized account
  ext varchar not null
);

CREATE TABLE gutpuser_idhash (
  id varchar primary key,		
  hash varchar not null
);

-- The abstract of subforum, blogspace, personal space, etc.
CREATE TABLE subspace (
  id varchar primary key,		
  title varchar not null,
  description varchar not null,
  banner varchar not null,
  owner varchar not null,		-- who own this subspace
  profession varchar not null,
  status smallint not null,
  weight smallint not null		-- used for ranks
  created_time bigint not null,
);

CREATE TABLE subspace_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE post (
  id varchar primary key,
  title varchar not null,
  content varchar not null,
  author_id varchar not null,
  subspace_id varchar not null,		-- a post belongs to a subspace
  extlink varchar not null,
  profession varchar not null,
  status smallint not null,
  weight smallint not null		-- used for ranks
  created_time bigint not null,
  updated_time bigint not null
);

CREATE TABLE post_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE comment (
  id varchar primary key,
  content varchar not null,
  post_id varchar not null,
  author_id varchar not null,
  status smallint not null,
  created_time bigint not null
);

CREATE TABLE comment_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE tag (
  id varchar primary key,
  caption varchar not null,
  subspace_id varchar not null,		-- a tag belongs to a subspace
  creator_id varchar not null,
  kind smallint not null,		-- system level tag, or customized tag
  created_time bigint not null
);

CREATE TABLE tag_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE posttag (
  id varchar primary key,		-- the post id
  tag_id varchar not null		-- the tag id
);

CREATE TABLE posttag_idhash (
  id varchar primary key,		
  hash varchar not null
);

---------------------------------
---------  Social Part ----------

CREATE TABLE likeit (
  id varchar primary key,		-- target id
  ttype varchar not null,		-- target type
  user_id: varchar not null		-- who do it
);

CREATE TABLE likeit_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE reward (
  id varchar primary key,		-- target id
  ttype varchar not null,		-- target type
  user_id: varchar not null,		-- who do it
  amount: int not null			-- the reward amount
);

CREATE TABLE reward_idhash (
  id varchar primary key,		
  hash varchar not null
);

CREATE TABLE follow (
  id varchar primary key,		-- target user id
  user_id: varchar not null,		-- who follows the target
  time: bigint not null			-- the followed time
);

CREATE TABLE follow_idhash (
  id varchar primary key,		
  hash varchar not null
);

