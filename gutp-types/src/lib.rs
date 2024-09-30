use eightfish_derive::EightFishModel;
use eightfish_sdk::EightFishModel;
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{DbValue, Decode, ParameterValue};

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpUser {
    pub id: String,
    pub account: String,
    pub oauth_source: String,
    pub nickname: String,
    pub avatar: String,
    pub role: i16,
    pub status: i16,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpSubspace {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub banner: String,
    pub is_public: bool,
    pub status: i16,
    pub weight: i16,
    pub owner_id: String,
    pub category: String,
    pub app_id: String,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub author_nickname: String,
    pub subspace_id: String,
    pub parent_post_id: String,
    pub ext_link: String,
    pub is_public: bool,
    pub status: i16,
    pub weight: i16,
    pub category: String,
    pub app_id: String,
    pub created_time: i64,
    pub updated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpComment {
    pub id: String,
    pub content: String,
    pub author_id: String,
    pub author_nickname: String,
    pub post_id: String,
    pub parent_comment_id: String,
    pub is_public: bool,
    pub status: i16,
    pub weight: i32,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpTag {
    pub id: String,
    pub caption: String,
    pub subspace_id: String,
    pub is_public: bool,
    pub weight: i16,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPostTag {
    pub id: String,
    pub post_id: String,
    pub tag_id: String,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpPostDiff {
    pub id: String,
    pub post_id: String,
    pub diff: String,
    pub version_num: i32,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpModerator {
    pub id: String,
    pub user_id: String,
    pub subspace_id: String,
    pub is_subspace_moderator: bool,
    pub tag_id: String,
    pub permission_level: i16,
    pub created_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, EightFishModel)]
pub struct GutpExtobj {
    pub id: String,
    pub caption: String,
    pub content: String,
    pub user_id: String,
    pub subspace_id: String,
    pub tag_id: String,
    pub post_id: String,
    pub comment_id: String,
    pub is_public: bool,
    pub weight: i16,
    pub created_time: i64,
}
