use chrono::{DateTime, TimeDelta, Utc};

use crate::helloasso::api::{ClientAPI, Form, Payment};

#[derive(Debug)]
pub struct HelloAsso {
    client_api: ClientAPI,
    client_id: String,
    access_token: String,
    refresh_token: String,
    expiration: DateTime<Utc>,
    organisation_slug: String,
    forms: Vec<Form>,
    payments: Vec<Payment>,
}

impl HelloAsso {
    pub async fn new(client_id: String, client_secret: String, organisation_slug: String) -> Self {
        let client_api = ClientAPI::new("https://api.helloasso.com").unwrap();

        let token_response = client_api
            .request_token(client_id.clone(), client_secret)
            .await
            .unwrap();

        let expiration = Utc::now();

        let mut res = Self {
            client_api,
            client_id,
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expiration,
            organisation_slug,
            forms: Default::default(),
            payments: Default::default(),
        };
        //        res.fetch_forms().await;
        res.fetch_payments().await;
        res
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

    async fn fetch_forms(&mut self) {
        let orgs = self
            .client_api
            .get_organization_forms(self.organisation_slug.clone(), &self.access_token)
            .await
            .unwrap();

        self.forms = orgs.data.unwrap_or(Vec::new());
    }

    async fn fetch_payments(&mut self) {
        let payments = self
            .client_api
            .get_payments(self.organisation_slug.clone(), &self.access_token)
            .await
            .unwrap();

        self.payments = payments.data.unwrap_or(Vec::new());
    }
}
