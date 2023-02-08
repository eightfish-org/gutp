-- The basic gutp user model
CREATE TABLE gutpuser (
  id varchar primary key,		-- for machine read
  account varchar unique not null,	-- for human read
  nickname varchar not null,
  avatar varchar not null,		-- user's avatar, usually an url link to outside
  role smallint not null,
  status smallint not null,
  signup_time bigint not null,
  publickey varchar not null,		-- for decentralized account
  ext varchar not null
);

-- The abstract of subforum, blogspace, personal space, etc.
CREATE TABLE subspace (
  id varchar primary key,		
  title varchar not null,
  description varchar not null,
  banner varchar not null,
  owner varchar not null,		-- who own this subspace
  created_time bigint not null,
  status smallint not null,
  weight smallint not null		-- used for ranks
);

CREATE TABLE post (
  id varchar primary key,
  title varchar not null,
  content varchar not null,
  author_id varchar not null,
  subspace_id varchar not null,		-- a post belongs to a subspace
  extlink varchar not null,
  status smallint not null,
  weight smallint not null		-- used for ranks
  created_time bigint not null,
  updated_time bigint not null
);

CREATE TABLE comment (
  id varchar primary key,
  content varchar not null,
  post_id varchar not null,
  author_id varchar not null,
  status smallint not null,
  created_time bigint not null
);

CREATE TABLE tag (
  id varchar primary key,
  caption varchar not null,
  subspace_id varchar not null,		-- a tag belongs to a subspace
  creator_id varchar not null,
  kind smallint not null,		-- system level tag, or customized tag
  created_time bigint not null
);

CREATE TABLE posttag (
  post_id varchar primary key,
  tag_id varchar not null
);

