use crate::{
    ENV_TOKEN,
    constants::{ACCEPT_HEADER, DEFAULT_API_HOST, ENV_API_HOST, USER_AGENT},
    error::GistError,
    models::*,
};
use reqwest::{Client, Response, StatusCode, multipart};
use std::env;

pub struct GistClient {
    client: Client,
    pub api_host: String,
    pub token: Option<String>,
}

impl GistClient {
    pub fn new(token: Option<String>, api_host: Option<String>) -> Self {
        let env_api_host = env::var(ENV_API_HOST).ok();

        let api_host = api_host
            .or(env_api_host)
            .unwrap_or_else(|| DEFAULT_API_HOST.to_string());

        Self {
            client: Client::new(),
            api_host,
            token,
        }
    }

    pub fn from_env() -> Self {
        let api_host = env::var(ENV_API_HOST).ok();
        let token = env::var(ENV_TOKEN).ok();
        Self::new(token, api_host)
    }

    async fn send_request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<reqwest::Body>,
        form: Option<multipart::Form>,
    ) -> Result<Response, GistError> {
        let url = format!("{}{}", self.api_host, path);
        let mut request = self
            .client
            .request(method, &url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", ACCEPT_HEADER);

        if let Some(token) = &self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(body) = body {
            request = request
                .body(body)
                .header("Content-Type", "application/json");
        } else if let Some(form) = form {
            request = request.multipart(form);
        }

        let response = request.send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(GistError::from_response(status, error_message));
        }

        Ok(response)
    }

    pub async fn create_gist(&self, gist: &CreateGist) -> Result<FullGist, GistError> {
        let body = serde_json::to_vec(gist)?;
        let response = self
            .send_request(
                reqwest::Method::POST,
                "/gists",
                Some(reqwest::Body::from(body)),
                None,
            )
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn get_gist(&self, gist_id: &str) -> Result<FullGist, GistError> {
        let path = format!("/gists/{}", gist_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn update_gist(
        &self,
        gist_id: &str,
        update: &UpdateGist,
    ) -> Result<FullGist, GistError> {
        let body = serde_json::to_vec(update)?;
        let path = format!("/gists/{}", gist_id);
        let response = self
            .send_request(
                reqwest::Method::PATCH,
                &path,
                Some(reqwest::Body::from(body)),
                None,
            )
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn delete_gist(&self, gist_id: &str) -> Result<(), GistError> {
        let path = format!("/gists/{}", gist_id);
        self.send_request(reqwest::Method::DELETE, &path, None, None)
            .await?;
        Ok(())
    }

    pub async fn list_gist_comments(&self, gist_id: &str) -> Result<Vec<GistComment>, GistError> {
        let path = format!("/gists/{}/comments", gist_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn get_gist_comment(
        &self,
        gist_id: &str,
        comment_id: u64,
    ) -> Result<GistComment, GistError> {
        let path = format!("/gists/{}/comments/{}", gist_id, comment_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn create_gist_comment(
        &self,
        gist_id: &str,
        comment: &CreateComment,
    ) -> Result<GistComment, GistError> {
        let body = serde_json::to_vec(comment)?;
        let path = format!("/gists/{}/comments", gist_id);
        let response = self
            .send_request(
                reqwest::Method::POST,
                &path,
                Some(reqwest::Body::from(body)),
                None,
            )
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn update_gist_comment(
        &self,
        gist_id: &str,
        comment_id: u64,
        comment: &UpdateComment,
    ) -> Result<GistComment, GistError> {
        let body = serde_json::to_vec(comment)?;
        let path = format!("/gists/{}/comments/{}", gist_id, comment_id);
        let response = self
            .send_request(
                reqwest::Method::PATCH,
                &path,
                Some(reqwest::Body::from(body)),
                None,
            )
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn delete_gist_comment(
        &self,
        gist_id: &str,
        comment_id: u64,
    ) -> Result<(), GistError> {
        let path = format!("/gists/{}/comments/{}", gist_id, comment_id);
        self.send_request(reqwest::Method::DELETE, &path, None, None)
            .await?;
        Ok(())
    }

    pub async fn list_gist_commits(&self, gist_id: &str) -> Result<Vec<GistCommit>, GistError> {
        let path = format!("/gists/{}/commits", gist_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn get_gist_revision(&self, gist_id: &str, sha: &str) -> Result<FullGist, GistError> {
        let path = format!("/gists/{}/{}", gist_id, sha);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn fork_gist(&self, gist_id: &str) -> Result<FullGist, GistError> {
        let path = format!("/gists/{}/forks", gist_id);
        let response = self
            .send_request(reqwest::Method::POST, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn list_gist_forks(&self, gist_id: &str) -> Result<Vec<GistFork>, GistError> {
        let path = format!("/gists/{}/forks", gist_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn star_gist(&self, gist_id: &str) -> Result<(), GistError> {
        let path = format!("/gists/{}/star", gist_id);
        self.send_request(reqwest::Method::PUT, &path, None, None)
            .await?;
        Ok(())
    }

    pub async fn unstar_gist(&self, gist_id: &str) -> Result<(), GistError> {
        let path = format!("/gists/{}/star", gist_id);
        self.send_request(reqwest::Method::DELETE, &path, None, None)
            .await?;
        Ok(())
    }

    pub async fn check_gist_star(&self, gist_id: &str) -> Result<bool, GistError> {
        let path = format!("/gists/{}/star", gist_id);
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(true),
            StatusCode::NOT_FOUND => Ok(false),
            status => Err(GistError::ApiError {
                status,
                message: "Unexpected status when checking star".to_string(),
            }),
        }
    }

    pub async fn list_public_gists(&self, since: Option<&str>) -> Result<Vec<FullGist>, GistError> {
        let mut path = "/gists/public".to_string();
        if let Some(since) = since {
            path = format!("{}?since={}", path, since);
        }
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn list_starred_gists(
        &self,
        since: Option<&str>,
    ) -> Result<Vec<FullGist>, GistError> {
        let mut path = "/gists/starred".to_string();
        if let Some(since) = since {
            path = format!("{}?since={}", path, since);
        }
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }

    pub async fn list_user_gists(
        &self,
        username: &str,
        since: Option<&str>,
    ) -> Result<Vec<FullGist>, GistError> {
        let mut path = format!("/users/{}/gists", username);
        if let Some(since) = since {
            path = format!("{}?since={}", path, since);
        }
        let response = self
            .send_request(reqwest::Method::GET, &path, None, None)
            .await?;
        response.json().await.map_err(Into::into)
    }
}
