use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GistFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GistOwner {
    pub login: String,
    pub id: u64,
    pub avatar_url: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub gists_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateGist {
    pub description: Option<String>,
    pub public: Option<bool>,
    pub files: HashMap<String, GistFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateGist {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<HashMap<String, Option<GistFile>>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FullGist {
    pub id: String,
    pub description: Option<String>,
    pub public: bool,
    pub owner: Option<GistOwner>,
    pub files: HashMap<String, GistFile>,
    pub html_url: String,
    pub git_pull_url: String,
    pub git_push_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub comments: u64,
    pub comments_url: String,
    pub forks: Option<Vec<GistFork>>,
    pub history: Option<Vec<GistCommit>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GistCommit {
    pub url: String,
    pub version: String,
    pub user: Option<GistOwner>,
    pub change_status: GistChangeStatus,
    pub committed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GistChangeStatus {
    pub total: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GistFork {
    pub id: String,
    pub url: String,
    pub user: GistOwner,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GistComment {
    pub id: u64,
    pub body: String,
    pub user: Option<GistOwner>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateComment {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateComment {
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StarGistResponse {
    pub starred: bool,
}
