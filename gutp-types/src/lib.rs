#[derive(Debug, Clone, EightFishModel)]
struct GutpUser {
    id: String,
    account: String,
    nickname: String,
    avatar: String,
    role: u16,
    status: u16,
    signup_time: u64,
    ext: String,
}

struct GutpSubspace {
    id: String,
    title: String,
    description: String,
    banner: String,
    owner: String,
    profession: String,
    appid: String,
    private: bool,
    status: u16,
    weight: u16,
    created_time: u64,
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
    private: bool,
    status: u16,
    weight: u16,
    created_time: u64,
    updated_time: u64,
}

struct GutpComment {
    id: String,
    content: String,
    author_id: String,
    post_id: String,
    parent_comment_id: String,
    private: bool,
    status: u16,
    created_time: u64,
}

struct GutpTag {
    id: String,
    caption: String,
    subspace_id: String,
    creator_id: String,
    weight: u16,
    private: bool,
    created_time: u64,
}

struct GutpModerator {
    id: String,
    user_id: String,
    subspace_id: String,
    subspace_moderator: bool,
    tag_id: String,
    permission_level: u16,
}

struct GutpPostTag {
    id: String,
    post_id: String,
    tag_id: String,
}
