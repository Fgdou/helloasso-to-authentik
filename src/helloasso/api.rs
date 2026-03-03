use serde::{Deserialize, Serialize};

const TOKEN_API_URL: &str = "https://api.helloasso.com/oauth2";
const API_URL: &str = "https://api.helloasso.com/v5";

pub struct ClientAPI {
    base_url: reqwest::Url,
    client: reqwest::Client,
}

impl ClientAPI {
    pub fn new(base_url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            base_url: reqwest::Url::parse(base_url)?,
            client: reqwest::Client::new(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct TokenRequestParam {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenRequestResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

impl TokenRequestParam {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type: "client_credentials".into(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenParam {
    pub grant_type: String,
    pub refresh_token: String,
}

impl RefreshTokenParam {
    pub fn new(refresh_token: String) -> Self {
        Self {
            refresh_token,
            grant_type: "refresh_token".into(),
        }
    }
}

impl ClientAPI {
    pub async fn request_token(
        &self,
        client_id: String,
        client_secret: String,
    ) -> Result<TokenRequestResponse, reqwest::Error> {
        let url = self
            .base_url
            .join("/oauth2/token")
            .expect("Failed to build base url");

        let param = TokenRequestParam::new(client_id, client_secret);

        dbg!(&url, &param);
        let resp = self.client.post(url).form(&param).send().await?;

        dbg!(&resp);

        if !resp.status().is_success() {
            panic!();
        }

        resp.json::<TokenRequestResponse>().await
    }

    pub async fn update_token(
        &self,
        update_token: String,
    ) -> Result<TokenRequestResponse, reqwest::Error> {
        let url = self
            .base_url
            .join("/oauth2/token")
            .expect("Failed to build base url");

        let param = RefreshTokenParam::new(update_token);
        let resp = self.client.post(url).json(&param).send().await?;

        if !resp.status().is_success() {
            panic!();
        }

        resp.json::<TokenRequestResponse>().await
    }
}
