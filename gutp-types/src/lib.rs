use eightfish::EightFishModel;
use eightfish_derive::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{DbValue, Decode, ParameterValue};

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
struct GutpUser {
    id: String,
    account: String,
    nickname: String,
    avatar: String,
    role: i16,
    status: i16,
    signup_time: i64,
    pub_settings: String,
    ext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
struct GutpSubspace {
    id: String,
    title: String,
    description: String,
    banner: String,
    owner: String,
    profession: String,
    appid: String,
    is_public: bool,
    status: i16,
    weight: i16,
    created_time: i64,
}

struct GutpPost {
    id: String,
    title: String,
    content: String,
    author_id: String,
    subspace_id: String,
    extlink: String,
    profession: String,
    appid: String,
    is_public: bool,
    status: i16,
    weight: i16,
    created_time: i64,
    updated_time: i64,
}

struct GutpComment {
    id: String,
    content: String,
    author_id: String,
    post_id: String,
    parent_comment_id: String,
    is_public: bool,
    status: i16,
    weight: i32,
    created_time: i64,
}

struct GutpTag {
    id: String,
    caption: String,
    subspace_id: String,
    creator_id: String,
    is_public: bool,
    weight: i16,
    created_time: i64,
}

struct GutpPostTag {
    id: String,
    post_id: String,
    tag_id: String,
    created_time: i64,
}

struct GutpModerator {
    id: String,
    user_id: String,
    is_subspace_moderator: bool,
    subspace_id: String,
    tag_id: String,
    permission_level: i16,
    created_time: i64,
}

pub struct GutpPostDiff {
    id: String,
    post_id: String,
    diff: String,
    version_num: i32,
    created_time: i64,
}
