/*
 * HelloAsso API
 *
 * You can find a complete HelloAsso API guide by following this [link](https://dev.helloasso.com/).    A python wrapper is also available by following this [link](https://github.com/HelloAsso/HaApiV5).    In order to access any endpoint you will need to authenticate using **OAuth 2.0** authentication server : `https://api.helloasso.com/oauth2`    When browsing the Swagger documentation :  - The easiest way to use it is to use the Swagger Authorize feature.    - Or you can override the Input `Bearer JWT` in the Authorization Header field. You can easily generate a JWT using Postman's Request Token feature    There are two levels of authorization :    - Client privileges : this defines the endpoints the client has access to.    - User Roles : being authorized by an organization grants rights on this organization.    <details><summary>Connecting to the API with OAuth 2.0</summary>    **In order to use the API, you must authenticate with the OAuth 2.0 server using a private API Client.**    *What is OAuth 2.0 ? : https://tools.ietf.org/html/rfc6749*    * If you are an **organization** (i.e. association), you can obtain a client (Id and Secret) in your administration area. You will be granted the following privileges : AccessPublicData, AccessTransactions and Checkout.  * If you are a **partner**, please contact HelloAsso at partenariats@helloasso.org in order to obtain a client.    In any case, please keep the client (id and secret) to yourself.    On HelloAsso, we support the following grants :  * Client Credentials    `grant_type=client_credentials`    If used by an organization, the rights are limited to the current organization to which the client has been issued.    If used by a partner, it does not grant you any rights (User Roles) on resource owned by an organization, you can only access routes without any User Role required.    See [more information below](#DescClientCredentials).  * Refresh    `grant_type=refresh_token`    See [more information below](#DescRefreshToken).  * Authorization Code    `grant_type=authorization_code`    Only for partners : it permits you to access private resources of organizations, by using the [authorization flow](#DescAuthorizeFlow).  * Disconnect (custom endpoint)    By calling https://api.helloasso.com/oauth2/disconnect, with a valid Bearer token in the Authorization Header, you will disconnect the user, and revoke its tokens. See [more information below](#DescDisconnect).    When authenticating, you will receive an `access_token` which is short-lived (30 minutes), and a `refresh_token`, that will permit you to obtain a new `access_token` for one month. When you have an `access_token` and a `refresh_token`, **you MUST obtain a new `access_token` using the `refresh_token` issued to you** (with the `grant_type=refresh_token`), **and MUST NOT obtain a new `access_token` by using the client**. When refreshing, you will obtain a new `access_token` valid for 30 minutes, and a new `refresh_token` valid for another month. If you continue to use each `refresh_token` that you receive, you could stay authenticated forever, without requiring to enter again your client secret or prompting the user for its login and password.    <details><summary id=\"DescClientCredentials\">Client Credentials</summary>    This is your first route to obtain an `access_token` to communicate with the API.    Route : **POST** `https://api.helloasso.com/oauth2/token`    <details><summary>Headers</summary>    Key          | Value  -------------|------------------------------------  Content-Type | *application/x-www-form-urlencoded*  </details>  <details><summary>Body</summary>    Key           | Information          | Required/Optional  --------------|----------------------|------------------  client_id     | Your Client Id       | **Required**  client_secret | Your Client Secret   | **Required**  grant_type    | *client_credentials* | **Required**  </details>  <details><summary>Result (JSON)</summary>    Key                | Information  -------------------|-------------------------------------------------------------------------  access_token       | The JWT token to use in future requests  refresh_token      | Token used to refresh the token and get a new JWT token after expiration  token_type         | Token Type : always \"*bearer*\"  expires_in         | The lifetime in seconds of the access token  </details>  <details><summary>CURL Example</summary>    ```  curl -X POST \\    https://api.helloasso.com/oauth2/token \\    -H 'content-type: application/x-www-form-urlencoded' \\    -d 'grant_type=client_credentials&client_id=9fdc22226bf24ff99b875f4a7c503715&client_secret=AvUYelYH1aSZZ3QNBiZOybmBlZTpUcNSonsufB5txuw='  ```  </details>  </details>  <details><summary id=\"DescRefreshToken\">Refresh Token</summary>    Your route to refresh indefinitely your `access_token` and obtain a new one.    Route : **POST** `https://api.helloasso.com/oauth2/token`    <details><summary>Headers</summary>    Key          | Value  -------------|------------------------------------  Content-Type | *application/x-www-form-urlencoded*  </details>  <details><summary>Body</summary>    Key           | Information        | Required/Optional  --------------|--------------------|------------------  grant_type    | *refresh_token*    | **Required**  refresh_token | Your Refresh Token | **Required**  </details>  <details><summary>Result (JSON)</summary>    Key                | Information  -------------------|-------------------------------------------------------------------------  access_token       | The JWT token to use in future requests  refresh_token      | Token used to refresh the token and get a new JWT token after expiration  token_type         | Token Type : always \"*bearer*\"  expires_in         | The lifetime in seconds of the access token  </details>  <details><summary>CURL Example</summary>    ```  curl -X POST \\    https://api.helloasso.com/oauth2/token \\    -H 'content-type: application/x-www-form-urlencoded' \\    -d 'grant_type=refresh_token&client_id=9fdc22226bf24ff99b875f4a7c503715&refresh_token=REFRESH_TOKEN'  ```  </details>  </details>  <details><summary id=\"DescAuthorizeFlow\">Authorize flow</summary>    The authorize flow will permit partner's applications to access protected resources (resources owned by an organization). Typically, you must display a button to your user to ask him to login on HelloAsso, and authorize you to access his resources.    Button example :    ![](/img/ImgHaAuthorizeButton.png)    <details><summary>Button Code (HTML & CSS)</summary>  ```  <button class=\"HaAuthorizeButton\">    <img src=\"/img/logo-ha.svg\" alt=\"\" class=\"HaAuthorizeButtonLogo\">    <span class=\"HaAuthorizeButtonTitle\">Connecter à HelloAsso</span>  </button>    <style>  .HaAuthorizeButton {    align-items: center;    -webkit-box-pack: center;    -ms-flex-pack: center;    background-color: #FFFFFF;    border: 0.0625rem solid #49D38A;    border-radius: 0.125rem;    display: -webkit-box;    display: -ms-flexbox;    display: flex;    padding: 0;  }  .HaAuthorizeButton:disabled {    background-color: #E9E9F0;    border-color: transparent;    cursor: not-allowed;  }  .HaAuthorizeButton:not(:disabled):focus {    box-shadow: 0 0 0 0.25rem rgba(73, 211, 138, 0.25);    -webkit-box-shadow: 0 0 0 0.25rem rgba(73, 211, 138, 0.25);  }  .HaAuthorizeButtonLogo {    padding: 0 0.8rem;    width: 2.25rem;  }  .HaAuthorizeButtonTitle {    background-color: #49D38A;    color: #FFFFFF;    font-size: 1rem;    font-weight: 700;    padding: 0.78125rem 1.5rem;  }  .HaAuthorizeButton:disabled .HaAuthorizeButtonTitle {    background-color: #E9E9F0;    color: #9A9DA8;  }  .HaAuthorizeButton:not(:disabled):hover .HaAuthorizeButtonTitle,  .HaAuthorizeButton:not(:disabled):focus .HaAuthorizeButtonTitle {    background-color: #30c677;  }  </style>  ```  </details>    When the user clicks on the button or link, you must open a popup window and direct him to this url : https://auth.helloasso.com/authorize with all the appropriate query parameters (see below).    <details><summary>Authorization Request</summary>    Route to display : **GET** `https://auth.helloasso.com/authorize`    <details><summary>Parameters</summary>    Key                    | Information                                                      | Required/Optional  -----------------------|------------------------------------------------------------------|------------------  client_id              | Your Client Id                                                   | **Required**  redirect_uri           | The redirect uri that will be used when the authorize is complete (success or error). For security considerations, the domain of the redirect uri must be the same configured on your client in our database. The redirect uri must use the secure protocol `https`. You can modify this domain with the following endpoint https://api.helloasso.com/v5/partners/me/api-clients (see section \"Partners Managment\" bellow) | **Required**  code_challenge         | The PKCE code challenge. See section [PKCE](#PKCE) below.        | **Required**  code_challenge_method  | The PKCE code challenge method, must be \"*S256*\"                 | **Required**  state                  | A value that will be sent back to you, to maintain state between the request and callback. The parameter should be used for preventing cross-site request forgery : **the state sent back must match the one you sent**. Must be a string, but you can use this to encode any data you want. The state should be less than 500 characters. | Optional    We recommend you to open the window with at least an height of 650px and a width of 500px.    </details>  <details><summary>Request Example</summary>    ```  https://auth.helloasso.com/authorize    ?client_id=9fdc22226bf24ff99b875f4a7c503715    &redirect_uri=YOUR_REDIRECT_URI    &code_challenge=YOUR_CODE_CHALLENGE    &code_challenge_method=S256    &state=abc  ```  </details>  </details>    This will display the login window and then the authorize window to the user :     <img alt=\"\" src=\"/img/ImgHaLoginWindow.png\" style=\"height:400px\"><img alt=\"\" src=\"/img/ImgHaAuthorizeWindow.png\" style=\"height:400px;margin-left:20px\">    The user has an option to sign up and register its organization, if he does not have one already.  When the user completes the process, the window will redirect to the given `redirect_uri`, with the `authorization_code` in parameter (or with an error code if an error occurred).    <details><summary>Redirect response</summary>    If success :  Key   | Information  ------|----------------------------------------------------------------------------------------------  code  | The authorization code generated by the authorization server. Has a lifetime of five minutes.  state | If you supplied a state in the request, will be sent back to you    If error :  Key               | Information  ------------------|-----------------------------------------------------------------  error             | A single error code.  error_description | Optional : a text providing additional information  state             | If you supplied a state in the request, will be sent back to you  See more info here on error codes : https://tools.ietf.org/html/rfc6749#section-4.1.2.1  </details>    You can then exchange the code for an `access_token` and a `refresh_token`.    <details><summary>Access Token Request</summary>    Route : **POST** `https://api.helloasso.com/oauth2/token`    <details><summary>Headers</summary>    Key          | Value  -------------|------------------------------------  Content-Type | *application/x-www-form-urlencoded*  </details>  <details><summary>Body</summary>    Key           | Information                                                       | Required/Optional  --------------|-------------------------------------------------------------------|------------------  client_id     | Your Client Id                                                    | **Required**  client_secret | Your Client Secret                                                | **Required**  grant_type    | *authorization_code*                                              | **Required**  code          | The authorization code received (when redirecting to your domain) | **Required**  redirect_uri  | The same redirect uri used when displaying the authorize window   | **Required**  code_verifier | The PKCE code verifier                                            | **Required**  </details>  <details><summary>Result (JSON)</summary>    Key                | Information  -------------------|-------------------------------------------------------------------------  access_token       | The JWT token to use in future requests  refresh_token      | Token used to refresh the token and get a new JWT token after expiration  token_type         | Token Type : always \"*bearer*\"  expires_in         | The lifetime in seconds of the access token  organization_slug  | The slug of the association which authorized the access  </details>  <details><summary>CURL Example</summary>    ```  curl -X POST \\    https://api.helloasso.com/oauth2/token \\    -H 'content-type: application/x-www-form-urlencoded' \\    -d 'grant_type=authorization_code&client_id=9fdc22226bf24ff99b875f4a7c503715&client_secret=AvUYelYH1aSZZ3QNBiZOybmBlZTpUcNSonsufB5txuw=' \\    -d 'code=AUTHORIZATION_CODE_RECEIVED&redirect_uri=YOUR_REDIRECT_URI&code_verifier=YOUR_CODE_VERIFIER'  ```  </details>  </details>  <details><summary id=\"PKCE\">PKCE</summary>    PKCE (Proof Key for Code Exchange) is a security measure for the authorization grant.    The specification can be found here : https://tools.ietf.org/html/rfc7636    We require you to use the challenge_method S256.    Basically, you must generate a random Code Verifier of 43 to 128 characters, from the following characters : `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~`. You then create the Code Challenge : you hash the code verifier with the SHA-256 hash function, encode it to base64, and then url encode it.    You will send the code challenge in the initial request to display the authorize window. HelloAsso will keep it stored with your request. When you will request a token in exchange for your authorization code, you will send the code verifier. HelloAsso will then validate it (length and characters used) and encode it the same way you did it (SHA-256, base 64 & uri encode) and compare the result with the code challenge stored during the authorize request. If it does not match, you will receive the error \"invalid_grant\".    If you want to test your code challenge generator, you can do so here with this online tool : https://tonyxu-io.github.io/pkce-generator/  </details>  </details>  <details><summary id=\"DescDisconnect\">Disconnect</summary>    Disconnect the user, and revoke its tokens. Returns 200 if disconnect worked.    Route : **GET** `https://api.helloasso.com/oauth2/disconnect`    <details><summary>Headers</summary>    Key           | Value  --------------|----------------------------------------------------------  Authorization | *Bearer JWT* (replace JWT with your `access_token` value)  </details>  <details><summary>CURL Example</summary>    ```  curl -X GET \\    https://api.helloasso.com/oauth2/disconnect \\    -H 'Authorization: Bearer JWT'  ```  </details>  </details>  </details>  <details><summary id=\"DescNotifications\">Handling the notifications system</summary>    You will receive a notification when one of the following events occur:  - A campaign is created  - An order is made (including free orders where there are no payments)  - A payment is made (whether it is a single payment or a payment by installment)  - A payment is refunded  - A payment is contested by the payer with his bank  - A payment by installment is refused  - An organization is renamed and his slug change    <details><summary>Define your notification URL</summary>    * If you are an **organization** (i.e. association), you can define and manage your notification URL in your administration area.  * If you are a **partner**, You can modify this notification URL with the following endpoint https://api.helloasso.com/v5/partners/me/api-notifications (see section \"Partners Managment\" bellow).     In any case, your callback URL **must use the secure protocol `https` and must support the `POST` verb.**    Then, in your code, you can handle and listen for information coming from the defined URL.  </details>    <details><summary>Get the notification content</summary>    The notification can have 4 different types : **Order**, **Payment**, **Form** (for campaign creation) or **Organization** (when the organization slug changed).    When a new content is available, we will call the notification URL callback defined before with the corresponding data in the body.    <details><summary>Order Result (JSON)</summary>  Key               | Information  ------------------|-------------------------------------------------------------------------  eventType         | Order  data              | The order data. See [more information on the order model below](#model-HelloAsso.Api.V5.Common.Orders.OrderDetail)  </details>  <details><summary id=\"PaymentResultJson\">Payment Result (JSON)</summary>  Key               | Information  ------------------|-------------------------------------------------------------------------  eventType         | Payment  data              | The payment data. See [more information on the payment model below](#model-HelloAsso.Api.V5.Common.Orders.PaymentDetail)  </details>  <details><summary>Form Result (JSON)</summary>  Key               | Information  ------------------|-------------------------------------------------------------------------  eventType         | Form  data              | The form data. See [more information on the form model below](#model-HelloAsso.Api.V5.Common.Models.Forms.FormPublicModel)  </details>  <details><summary>Organization Result (JSON)</summary>  Key               | Information  ------------------|-------------------------------------------------------------------------  eventType         | Organization  data              | { \"data\": { \"old_slug_organization\": \"ctb\", \"new_slug_organization\": \"club tennis bordeaux\"  }, \"eventType\": \"Organization\"}  </details>  </details>  </details>  </details>  <details><summary id=\"DescCheckout\">Checkout Form</summary>    The Checkout is a type of form, specifically designed to allow contributors to make payments prefilled by an authorized partner or an organization.  The partner/organization creates a checkout intent (with all the contributor information that he may have, and specified payment terms ; see the following API route *POST checkout-intents* of the [Checkout intents management](#Checkout%20intents%20management) section), and receives in response a url (which is valid for 15 minutes). The contributor follows the url, and then validates the form and pays. The contributor is then redirected to the page of your choosing, with a result parameter (error, success etc).  The initiator receives a notification of each Order and Payment authorized (if you have configured them, see [Handling the notifications system](#DescNotifications). You can also verify your checkout intent result and receive the created order. See the route *GET checkout-intents/{checkoutIntentId}* of the [Checkout intents management](#operations-tag-Checkout%20intents%20management) section.    You can find a more detailed description on how to integrate by following this [link](https://dev.helloasso.com/docs/int%C3%A9grer-le-paiement-sur-votre-site).    If you want try checkout payments please use our test environnement https://www.helloasso-sandbox.com/ to create an organization.  You can then retrieve your API client in the back office and make calls on https://api.helloasso-sandbox.com/v5.  Virtual credit cards are avaible by following this [link](https://docs.sips.Worldline-solutions.com/fr/cartes-de-test.html).    </details>
 *
 * The version of the OpenAPI document: public
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest {
    #[serde(rename = "tierList", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tier_list: Option<Option<Vec<models::HelloAssoApiV5CommonModelsFormsTierLightModel>>>,
    /// The banner of the form
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    /// The description of form
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// The datetime of the activity end
    #[serde(rename = "endDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// The logo of the form
    #[serde(rename = "logo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub logo: Option<Option<String>>,
    /// Private Title : displayed only in the organization back office
    #[serde(rename = "privateTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub private_title: Option<Option<String>>,
    /// The datetime of the activity start
    #[serde(rename = "startDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// The title of the form. It will be used to generate the url which that can't be changed.
    #[serde(rename = "title")]
    pub title: String,
    /// Activity type identifier, matching one of the provided type values <a href=\"index#!/Values/Values_Get\"> provided here</a>
    #[serde(rename = "activityTypeId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub activity_type_id: Option<Option<i32>>,
    #[serde(rename = "place", skip_serializing_if = "Option::is_none")]
    pub place: Option<Box<models::HelloAssoApiV5CommonModelsCommonPlaceModel>>,
    /// The datetime (Inclusive) at which the sales end.  If null the orders will be available until the end of the campaign.
    #[serde(rename = "saleEndDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sale_end_date: Option<Option<String>>,
    /// The datetime (Inclusive) at which the users can start placing orders.  If null the orders will be available as soon as the campaign is published.
    #[serde(rename = "saleStartDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sale_start_date: Option<Option<String>>,
    #[serde(rename = "validityType", skip_serializing_if = "Option::is_none")]
    pub validity_type: Option<models::HelloAssoApiV5CommonModelsEnumsMembershipValidityType>,
    /// Whether the user will be allowed to make a single open donation with an order. The amount of the donation is open, but 3 presets can be set in OpenDonationPresetAmount
    #[serde(rename = "acceptOpenDonation", skip_serializing_if = "Option::is_none")]
    pub accept_open_donation: Option<bool>,
    /// Whether the user will be allowed to make a monthly open donation for donation forms
    #[serde(rename = "acceptOpenMonthlyDonation", skip_serializing_if = "Option::is_none")]
    pub accept_open_monthly_donation: Option<bool>,
    /// allowComment
    #[serde(rename = "allowComment", skip_serializing_if = "Option::is_none")]
    pub allow_comment: Option<bool>,
    /// amountVisible
    #[serde(rename = "amountVisible", skip_serializing_if = "Option::is_none")]
    pub amount_visible: Option<bool>,
    /// The color of the form
    #[serde(rename = "color", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub color: Option<Option<String>>,
    /// The text displayed in the widget button
    #[serde(rename = "widgetButtonText", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub widget_button_text: Option<Option<String>>,
    #[serde(rename = "contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Box<models::HelloAssoApiV5CommonModelsCommonContactModel>>,
    /// Display contributor name for fundraiser and donation forms.
    #[serde(rename = "displayContributorName", skip_serializing_if = "Option::is_none")]
    pub display_contributor_name: Option<bool>,
    /// Indicates that the members count must be displayed on the form.
    #[serde(rename = "displayParticipantsCount", skip_serializing_if = "Option::is_none")]
    pub display_participants_count: Option<bool>,
    /// Indicates that the remaining entries must be displayed on the form.
    #[serde(rename = "displayRemainingEntries", skip_serializing_if = "Option::is_none")]
    pub display_remaining_entries: Option<bool>,
    /// Indicates the financial goal (amount of money raised) for the whole form. Null means no goal.
    #[serde(rename = "financialGoal", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub financial_goal: Option<Option<i64>>,
    /// Entrust the issuance of membership cards to HelloAsso (automatically sent by email to participants)
    #[serde(rename = "generateMembershipCards", skip_serializing_if = "Option::is_none")]
    pub generate_membership_cards: Option<bool>,
    /// Entrust the issuance of tickets to HelloAsso (automatically sent by email to participants)
    #[serde(rename = "generateTickets", skip_serializing_if = "Option::is_none")]
    pub generate_tickets: Option<bool>,
    /// Allows you to add the long description above the store catalog.
    #[serde(rename = "invertDescriptions", skip_serializing_if = "Option::is_none")]
    pub invert_descriptions: Option<bool>,
    /// Label conditions and terms file
    #[serde(rename = "labelConditionsAndTermsFile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label_conditions_and_terms_file: Option<Option<String>>,
    /// The long description of the form (rich Html)
    #[serde(rename = "longDescription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub long_description: Option<Option<String>>,
    /// The preset amounts to be shown to the user. Maximum 3 amounts.
    #[serde(rename = "openDonationPresetAmounts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub open_donation_preset_amounts: Option<Option<Vec<i32>>>,
    /// Personalized message for participants
    #[serde(rename = "personalizedMessage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub personalized_message: Option<Option<String>>,
    /// The project beneficiaries of the form (rich Html)
    #[serde(rename = "projectBeneficiaries", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_beneficiaries: Option<Option<String>>,
    /// Details of the project expenses (rich Html)
    #[serde(rename = "projectExpensesDetails", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_expenses_details: Option<Option<String>>,
    /// Description of the project owners (rich Html)
    #[serde(rename = "projectOwners", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_owners: Option<Option<String>>,
    /// Suggest monthly donation for donation forms.
    #[serde(rename = "suggestMonthlyDonation", skip_serializing_if = "Option::is_none")]
    pub suggest_monthly_donation: Option<bool>,
    /// The monthly donation option should be displayed or selected by default for donation forms.
    #[serde(rename = "displayMonthlyDonationsFirst", skip_serializing_if = "Option::is_none")]
    pub display_monthly_donations_first: Option<bool>,
    /// 3 letter country code
    #[serde(rename = "projectTargetCountry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project_target_country: Option<Option<String>>,
    /// Whether users are allowed to contribute to this form through an organism (only for donation and crowdfunding).
    #[serde(rename = "allowOrganismPayer", skip_serializing_if = "Option::is_none")]
    pub allow_organism_payer: Option<bool>,
    /// Whether user are allowed to personally contribute to this form (only for donation and crowdfunding).
    #[serde(rename = "allowIndividualPayer", skip_serializing_if = "Option::is_none")]
    pub allow_individual_payer: Option<bool>,
    /// Whether a reminder email should be sent for abandoned carts.
    #[serde(rename = "remindAbandonedCart", skip_serializing_if = "Option::is_none")]
    pub remind_abandoned_cart: Option<bool>,
    /// Indicates the maximum available entries for the whole form. Null means unlimited entries.
    #[serde(rename = "maxEntries", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_entries: Option<Option<i32>>,
}

impl HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest {
    pub fn new(title: String) -> HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest {
        HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest {
            tier_list: None,
            banner: None,
            description: None,
            end_date: None,
            logo: None,
            private_title: None,
            start_date: None,
            title,
            activity_type_id: None,
            place: None,
            sale_end_date: None,
            sale_start_date: None,
            validity_type: None,
            accept_open_donation: None,
            accept_open_monthly_donation: None,
            allow_comment: None,
            amount_visible: None,
            color: None,
            widget_button_text: None,
            contact: None,
            display_contributor_name: None,
            display_participants_count: None,
            display_remaining_entries: None,
            financial_goal: None,
            generate_membership_cards: None,
            generate_tickets: None,
            invert_descriptions: None,
            label_conditions_and_terms_file: None,
            long_description: None,
            open_donation_preset_amounts: None,
            personalized_message: None,
            project_beneficiaries: None,
            project_expenses_details: None,
            project_owners: None,
            suggest_monthly_donation: None,
            display_monthly_donations_first: None,
            project_target_country: None,
            allow_organism_payer: None,
            allow_individual_payer: None,
            remind_abandoned_cart: None,
            max_entries: None,
        }
    }
}

