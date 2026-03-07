use chrono::{DateTime, TimeDelta, Utc};
use helloasso_api::{
    apis::{
        Error, configuration::Configuration,
        orders_items_api::OrganizationsOrganizationSlugOrdersGetError,
    },
    models::HelloAssoApiV5CommonModelsStatisticsItemCustomField,
};

use crate::helloasso::api::ClientAPI;

#[derive(Debug)]
pub struct HelloAsso {
    client_api: ClientAPI,
    client_id: String,
    refresh_token: String,
    expiration: DateTime<Utc>,
    organisation_slug: String,
    configuration: Configuration,
}

#[derive(Debug)]
pub struct Member {
    pub firstname: String,
    pub lastname: String,
    pub company: Option<String>,
    pub payment: DateTime<Utc>,
    pub email: String,
    pub address: String,
    pub member_type: String,
}

#[derive(Debug)]
pub enum ClientError<T> {
    MissingField(String),
    Http(T),
    Date,
}

impl HelloAsso {
    pub async fn new(client_id: String, client_secret: String, organisation_slug: String) -> Self {
        let client_api = ClientAPI::new("https://api.helloasso.com").unwrap();

        let token_response = client_api
            .request_token(client_id.clone(), client_secret)
            .await
            .unwrap();

        let expiration = Utc::now();

        let mut configuration = Configuration::default();
        configuration.bearer_access_token = Some(token_response.access_token);

        Self {
            client_api,
            client_id,
            refresh_token: token_response.refresh_token,
            expiration,
            organisation_slug,
            configuration,
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

    pub async fn get_members(
        &self,
    ) -> Result<Vec<Member>, ClientError<Error<OrganizationsOrganizationSlugOrdersGetError>>> {
        let res =
            helloasso_api::apis::orders_items_api::organizations_organization_slug_orders_get(
                &self.configuration,
                &self.organisation_slug,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(true),
                None,
            )
            .await
            .map_err(|e| ClientError::Http(e))?;
        let orders = res.data.unwrap_or_default().unwrap_or_default();
        orders
            .into_iter()
            .flat_map(|order| {
                let date = order.date.clone();
                order
                    .items
                    .unwrap_or_default()
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| {
                        let user = *item.user.ok_or(ClientError::MissingField("user".into()))?;
                        let custom_fields = item.custom_fields.flatten().unwrap_or_default();
                        let date_str = date
                            .clone()
                            .ok_or(ClientError::MissingField("payment_date".into()))?;
                        let date = DateTime::parse_from_rfc3339(&date_str)
                            .map_err(|_| ClientError::Date)?;
                        let member = Member {
                            firstname: user
                                .first_name
                                .flatten()
                                .ok_or(ClientError::MissingField("firstname".into()))?,
                            lastname: user
                                .last_name
                                .flatten()
                                .ok_or(ClientError::MissingField("lastname".into()))?,
                            address: get_custom_field(&custom_fields, "Adresse")
                                .ok_or(ClientError::MissingField("Adresse".into()))?,
                            company: get_custom_field(&custom_fields, "Entreprise (si commerçant)"),
                            email: get_custom_field(&custom_fields, "Email")
                                .ok_or(ClientError::MissingField("Email".into()))?,
                            member_type: item
                                .name
                                .flatten()
                                .ok_or(ClientError::MissingField("member_type".into()))?,
                            payment: date.into(),
                        };
                        Ok(member)
                    })
                    .collect::<Vec<Result<Member, ClientError<_>>>>()
            })
            .collect()
    }
}

fn get_custom_field(
    custom_fields: &[HelloAssoApiV5CommonModelsStatisticsItemCustomField],
    field: &str,
) -> Option<String> {
    custom_fields
        .iter()
        .find(|f| f.name == Some(Some(field.into())))
        .and_then(|f| f.answer.clone().flatten())
}
