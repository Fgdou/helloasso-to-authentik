use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

const TOKEN_API_URL: &str = "https://api.helloasso.com/oauth2";
const API_URL: &str = "https://api.helloasso.com/v5";

#[derive(Debug)]
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

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub pageSize: i32,
    pub totalCount: i32,
    pub pageIndex: i32,
    pub totalPages: i32,
    pub continuationToken: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    id: Option<i32>,
    fileName: Option<String>,
    publicUrl: Option<String>,
    state: Option<ImageState>,
}
#[derive(Debug, Deserialize)]
pub enum ImageState {
    Saved,
    Pending,
    Rejected,
    Accepted,
}
#[derive(Debug, Deserialize)]
pub struct MetaObject {
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub authorEmail: Option<String>,
    pub expiresAt: Option<DateTime<Utc>>,
}
#[derive(Debug, Deserialize)]
pub enum FormState {
    Public,
    Private,
    Draft,
    Deleted,
    Disabled,
}
#[derive(Debug, Deserialize)]
pub enum FormType {
    CrowdFunding,
    Membership,
    Event,
    Donation,
    PaymentForm,
    Checkout,
    Shop,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub data: Option<Vec<T>>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
pub struct Form {
    pub banner: Option<Image>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub startDate: Option<DateTime<Utc>>,
    pub endDate: Option<DateTime<Utc>>,
    pub logo: Option<Image>,
    pub meta: Option<MetaObject>,
    pub state: FormState,
    pub tile: Option<String>,
    pub privateTile: Option<String>,
    pub widgetButtonUrl: Option<String>,
    pub widgetFullUrl: Option<String>,
    pub widgetVignetteHorizontalUrl: Option<String>,
    pub widgetVignetteVerticalUrl: Option<String>,
    pub widgetCounterUrl: Option<String>,
    pub formSlug: Option<String>,
    pub formType: FormType,
    pub url: Option<String>,
    pub organizationSlug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    order: Order,
    payer: Payer,
    items: Option<Vec<Item>>,
    cashOutDate: Option<DateTime<Utc>>,
    idCashOut: Option<i32>,
    cashOutState: Option<CashOutState>,
    paymentReceiptUrl: Option<String>,
    fiscalReceiptUrl: Option<String>,
    id: i32,
    amount: i32,
    amountTip: Option<i32>,
    date: DateTime<Utc>,
    paymentMeans: PaymentMeans,
    installmentNumber: Option<i32>,
    state: PaymentState,
    #[serde(rename = "type")]
    paymentType: Option<PaymentType>,
    meta: MetaObject,
    paymentOffLineMean: Option<PaymentMeans>,
    refundOperations: Option<Vec<RefundOperation>>,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    id: i32,
    date: DateTime<Utc>,
    formSlug: Option<String>,
    formType: FormType,
    organizationName: Option<String>,
    organizationSlug: Option<String>,
    organizationType: String,
    organizationIsUnderColucheLaw: bool,
    checkoutIntentId: Option<i32>,
    meta: MetaObject,
}

#[derive(Deserialize, Debug)]
pub struct Payer {
    email: Option<String>,
    address: Option<String>,
    city: Option<String>,
    zipCode: Option<String>,
    country: Option<String>,
    company: Option<String>,
    dateOfBirth: Option<DateTime<Utc>>,
    firstName: Option<String>,
    lastName: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    shareAmount: i32,
    shareItemAmount: i32,
    shareOptionsAmount: Option<i32>,
    id: i32,
    amount: i32,
    #[serde(rename = "type")]
    itemType: ItemType,
    initialAmount: Option<i32>,
    state: ItemState,
    name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RefundOperation {
    id: i32,
    amount: i32,
    amountTip: i32,
    status: OperationStatus,
    meta: MetaObject,
}

#[derive(Deserialize, Debug)]
pub enum OperationStatus {
    Unknown,
    Init,
    Processing,
    Processed,
    Error,
    InternalError,
    Refused,
    Aborted,
    Canceled,
}

#[derive(Deserialize, Debug)]
pub enum ItemState {
    Waiting,
    Processed,
    Registered,
    Deleted,
    Unknown,
    Canceled,
    Refused,
    Abandoned,
}

#[derive(Deserialize, Debug)]
pub enum ItemType {
    Donation,
    Payment,
    Registration,
    Membership,
    MonthlyDonation,
    MonthlyPayment,
    OfflineDonation,
    Contribution,
    Bonus,
    Product,
}

#[derive(Deserialize, Debug)]
pub enum CashOutState {
    MoneyIn,
    CantTransferReceiverFull,
    Transfered,
    Refunded,
    Refunding,
    WaitingForCashOutConfirmation,
    CashedOut,
    Unknown,
    Contested,
    TransferInProgress,
}

#[derive(Deserialize, Debug)]
pub enum PaymentMeans {
    None,
    Card,
    Sepa,
    Check,
    Cash,
    BankTransfer,
    Other,
}

#[derive(Deserialize, Debug)]
pub enum PaymentState {
    Pending,
    Authorized,
    Refused,
    Unknown,
    Registered,
    Error,
    Refunded,
    Refunding,
    Waiting,
    Canceled,
    Contested,
    WaitingBankValidation,
    WaitingBankWithdraw,
    Abandoned,
    WaitingAuthentication,
    AuthorizedPreprod,
    Corrected,
    Deleted,
    Inconsistent,
    NoDonation,
    Init,
}

#[derive(Deserialize, Debug)]
pub enum PaymentType {
    Offline,
    Credit,
    Debit,
}

#[derive(Debug)]
pub enum APIError {
    InvalidToken,
    Unauthorized,
    Reqwest(reqwest::Error),
    InternalError(u16),
    JsonError(serde_json::Error),
}

impl APIError {
    fn new(status_code: u16) -> Self {
        match status_code {
            401 => APIError::InvalidToken,
            403 => APIError::Unauthorized,
            n => APIError::InternalError(n),
        }
    }
}

impl ClientAPI {
    pub async fn request_token(
        &self,
        client_id: String,
        client_secret: String,
    ) -> Result<TokenRequestResponse, APIError> {
        let url = self
            .base_url
            .join("/oauth2/token")
            .expect("Failed to build base url");

        let param = TokenRequestParam::new(client_id, client_secret);

        let resp = self
            .client
            .post(url)
            .form(&param)
            .send()
            .await
            .map_err(APIError::Reqwest)?;

        map_response(resp).await
    }

    pub async fn update_token(
        &self,
        update_token: String,
    ) -> Result<TokenRequestResponse, APIError> {
        let url = self
            .base_url
            .join("/oauth2/token")
            .expect("Failed to build base url");

        let param = RefreshTokenParam::new(update_token);
        let resp = self
            .client
            .post(url)
            .json(&param)
            .send()
            .await
            .map_err(APIError::Reqwest)?;

        if !resp.status().is_success() {
            panic!();
        }

        map_response(resp).await
    }

    pub async fn get_organization_forms(
        &self,
        organization_slug: String,
        token: &str,
    ) -> Result<Response<Form>, APIError> {
        let url = self
            .base_url
            .join(&format!("/v5/organizations/{organization_slug}/forms"))
            .expect("Failed to build base url");
        let resp = self
            .client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(APIError::Reqwest)?;

        map_response(resp).await
    }

    pub async fn get_payments(
        &self,
        organization_slug: String,
        token: &str,
    ) -> Result<Response<Payment>, APIError> {
        let url = self
            .base_url
            .join(&format!("/v5/organizations/{organization_slug}/payments"))
            .expect("Failed to build base url");
        let resp = self
            .client
            .get(url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(APIError::Reqwest)?;

        map_response(resp).await
    }
}

async fn map_response<T>(http: reqwest::Response) -> Result<T, APIError>
where
    T: DeserializeOwned,
{
    if !http.status().is_success() {
        Err(APIError::new(http.status().as_u16()))
    } else {
        let txt = http.text().await.map_err(APIError::Reqwest)?;
        print!("{}", &txt);
        serde_json::from_str(&txt).map_err(APIError::JsonError)
    }
}
