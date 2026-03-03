use chrono::{DateTime, TimeDelta, Utc};

use crate::helloasso::api::ClientAPI;

pub struct HelloAsso {
    client_api: ClientAPI,
    client_id: String,
    access_token: String,
    refresh_token: String,
    expiration: DateTime<Utc>,
    organisation_slug: String,
}

impl HelloAsso {
    pub async fn new(client_id: String, client_secret: String) -> Self {
        let client_api = ClientAPI::new("https://api.helloasso.com").unwrap();

        let token_response = client_api
            .request_token(client_id.clone(), client_secret)
            .await
            .unwrap();

        let expiration = Utc::now();

        Self {
            client_api,
            client_id,
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expiration,
        }
    }

    async fn refresh_token(&mut self) {
        let result = self
            .client_api
            .update_token(self.refresh_token.clone())
            .await
            .unwrap();
        self.refresh_token = result.refresh_token;
        self.expiration = Utc::now() + TimeDelta::seconds(result.expires_in as i64);
    }

    async fn is_token_expired(&mut self) -> bool {
        self.expiration > Utc::now()
    }

    async fn refresh_token_if_needed(&mut self) {
        if self.is_token_expired().await {
            self.refresh_token().await
        }
    }
}
