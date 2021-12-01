use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i64,
    pub name: String,
    pub url: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "creator_id")]
    pub creator_id: i64,
    #[serde(rename = "community_id")]
    pub community_id: i64,
    pub removed: bool,
    pub locked: bool,
    pub published: String,
    pub updated: Option<chrono::NaiveDateTime>,
    pub deleted: bool,
    pub nsfw: bool,
    pub stickied: bool,
    #[serde(rename = "embed_title")]
    pub embed_title: Option<String>,
    #[serde(rename = "embed_description")]
    pub embed_description: Option<String>,
    #[serde(rename = "embed_html")]
    pub embed_html: Option<String>,
    #[serde(rename = "thumbnail_url")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "ap_id")]
    pub ap_id: String,
    pub local: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub id: i64,
    pub name: String,
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub banned: bool,
    pub published: String,
    pub updated: Option<chrono::NaiveDateTime>,
    #[serde(rename = "actor_id")]
    pub actor_id: String,
    pub bio: Option<String>,
    pub local: bool,
    pub banner: Option<String>,
    pub deleted: bool,
    #[serde(rename = "inbox_url")]
    pub inbox_url: String,
    #[serde(rename = "shared_inbox_url")]
    pub shared_inbox_url: String,
    #[serde(rename = "matrix_user_id")]
    pub matrix_user_id: Option<String>,
    pub admin: bool,
    #[serde(rename = "bot_account")]
    pub bot_account: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub id: i64,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub removed: bool,
    pub published: String,
    pub updated: Option<chrono::NaiveDateTime>,
    pub deleted: bool,
    pub nsfw: bool,
    #[serde(rename = "actor_id")]
    pub actor_id: String,
    pub local: bool,
    pub icon: Option<String>,
    pub banner: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counts {
    pub id: i64,
    #[serde(rename = "post_id")]
    pub post_id: i64,
    pub comments: i64,
    pub score: i64,
    pub upvotes: i64,
    pub downvotes: i64,
    pub stickied: bool,
    pub published: Option<chrono::NaiveDateTime>,
    #[serde(rename = "newest_comment_time_necro")]
    pub newest_comment_time_necro: Option<chrono::NaiveDateTime>,
    #[serde(rename = "newest_comment_time")]
    pub newest_comment_time: Option<chrono::NaiveDateTime>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostWrapper {
    pub post: Post,
    pub creator: Creator,
    pub community: Community,
    #[serde(rename = "creator_banned_from_community")]
    pub creator_banned_from_community: bool,
    pub counts: Counts,
    pub subscribed: bool,
    pub saved: bool,
    pub read: bool,
    #[serde(rename = "creator_blocked")]
    pub creator_blocked: bool,
    #[serde(rename = "my_vote")]
    pub my_vote: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub posts: Vec<PostWrapper>,
}
