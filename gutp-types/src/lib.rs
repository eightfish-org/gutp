use eightfish_derive::EightFishModel;
use eightfish_sdk::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{DbValue, Decode, ParameterValue};

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpUser {
    pub id: String,           // VARCHAR PRIMARY KEY
    pub account: String,      // VARCHAR UNIQUE NOT NULL
    pub oauth_source: String, // VARCHAR NOT NULL
    pub nickname: String,     // VARCHAR NOT NULL
    pub avatar: String,       // VARCHAR NOT NULL
    pub role: i16,            // SMALLINT NOT NULL CHECK (role IN (0, 1, 2, 3, 4, 5))
    pub status: i16,          // SMALLINT NOT NULL CHECK (status IN (0, 1, 2))
    pub created_time: i64,    // BIGINT NOT NULL
    pub data_source: String,  // VARCHAR NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpSubspace {
    pub id: String,               // VARCHAR PRIMARY KEY
    pub slug: String,             // VARCHAR NOT NULL
    pub title: String,            // VARCHAR NOT NULL
    pub description: String,      // VARCHAR NOT NULL
    pub banner: String,           // VARCHAR NOT NULL
    pub is_public: bool,          // BOOLEAN NOT NULL DEFAULT TRUE
    pub status: i16,              // SMALLINT NOT NULL
    pub weight: i16,              // SMALLINT NOT NULL
    pub owner_id: Option<String>, // VARCHAR REFERENCES gutpuser(id), nullable
    pub created_time: i64,        // BIGINT NOT NULL
    pub data_source: String,      // VARCHAR NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPost {
    pub id: String,                     // VARCHAR PRIMARY KEY
    pub title: String,                  // VARCHAR NOT NULL
    pub content: String,                // TEXT NOT NULL
    pub author_id: String,              // VARCHAR NOT NULL REFERENCES gutpuser(id)
    pub subspace_id: String,            // VARCHAR NOT NULL REFERENCES gutpsubspace(id)
    pub parent_post_id: Option<String>, // VARCHAR REFERENCES gutppost(id), nullable
    pub ext_link: String,               // VARCHAR NOT NULL
    pub is_public: bool,                // BOOLEAN NOT NULL DEFAULT TRUE
    pub status: i16, // SMALLINT NOT NULL CHECK (status IN (0, 1, 2, 3, 4, 5, 6, 7))
    pub weight: i16, // SMALLINT NOT NULL
    pub created_time: i64, // BIGINT NOT NULL
    pub updated_time: i64, // BIGINT NOT NULL
    pub data_source: String, // VARCHAR NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpComment {
    pub id: String,                        // VARCHAR PRIMARY KEY
    pub content: String,                   // VARCHAR NOT NULL
    pub author_id: String,                 // VARCHAR NOT NULL REFERENCES gutpuser(id)
    pub post_id: String,                   // VARCHAR NOT NULL REFERENCES gutppost(id)
    pub parent_comment_id: Option<String>, // VARCHAR REFERENCES gutpcomment(id), nullable
    pub is_public: bool,                   // BOOLEAN NOT NULL DEFAULT TRUE
    pub status: i16, // SMALLINT NOT NULL CHECK (status IN (0, 1, 2, 3, 4, 5, 6, 7))
    pub weight: i32, // INTEGER NOT NULL
    pub created_time: i64, // BIGINT NOT NULL
    pub data_source: String, // VARCHAR NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpTag {
    pub id: String,          // VARCHAR PRIMARY KEY
    pub caption: String,     // VARCHAR NOT NULL
    pub subspace_id: String, // VARCHAR NOT NULL REFERENCES gutpsubspace(id)
    pub is_public: bool,     // BOOLEAN NOT NULL DEFAULT TRUE
    pub weight: i16,         // SMALLINT NOT NULL
    pub created_time: i64,   // BIGINT NOT NULL
    pub data_source: String, // VARCHAR NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpSubspaceTag {
    pub id: String,          // VARCHAR PRIMARY KEY
    pub subspace_id: String, // VARCHAR NOT NULL REFERENCES gutpsubspace(id)
    pub tag_id: String,      // VARCHAR NOT NULL REFERENCES gutptag(id)
    pub created_time: i64,   // BIGINT NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPostTag {
    pub id: String,        // VARCHAR PRIMARY KEY
    pub post_id: String,   // VARCHAR NOT NULL REFERENCES gutppost(id)
    pub tag_id: String,    // VARCHAR NOT NULL REFERENCES gutptag(id)
    pub created_time: i64, // BIGINT NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPostDiff {
    pub id: String,        // VARCHAR PRIMARY KEY
    pub post_id: String,   // VARCHAR NOT NULL REFERENCES gutppost(id)
    pub diff: String,      // TEXT NOT NULL
    pub version_num: i32,  // INTEGER NOT NULL
    pub created_time: i64, // BIGINT NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpModerator {
    pub id: String,                  // VARCHAR PRIMARY KEY
    pub user_id: String,             // VARCHAR NOT NULL REFERENCES gutpuser(id)
    pub subspace_id: String,         // VARCHAR NOT NULL REFERENCES gutpsubspace(id)
    pub is_subspace_moderator: bool, // BOOLEAN NOT NULL DEFAULT TRUE
    pub perm_level: i16,             // SMALLINT NOT NULL
    pub tag_id: Option<String>,      // VARCHAR REFERENCES gutptag(id), nullable
    pub created_time: i64,           // BIGINT NOT NULL
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpExtObj {
    pub id: String,                  // VARCHAR PRIMARY KEY
    pub caption: String,             // VARCHAR NOT NULL
    pub content: String,             // VARCHAR NOT NULL
    pub user_id: Option<String>,     // VARCHAR REFERENCES gutpuser(id), nullable
    pub subspace_id: Option<String>, // VARCHAR REFERENCES gutpsubspace(id), nullable
    pub tag_id: Option<String>,      // VARCHAR REFERENCES gutptag(id), nullable
    pub post_id: Option<String>,     // VARCHAR REFERENCES gutppost(id), nullable
    pub comment_id: Option<String>,  // VARCHAR REFERENCES gutpcomment(id), nullable
    pub is_public: bool,             // BOOLEAN NOT NULL DEFAULT TRUE
    pub weight: i16,                 // SMALLINT NOT NULL
    pub created_time: i64,           // BIGINT NOT NULL
}
