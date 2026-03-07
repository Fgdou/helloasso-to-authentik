# Rust API client for openapi

You can find a complete HelloAsso API guide by following this [link](https://dev.helloasso.com/).

A python wrapper is also available by following this [link](https://github.com/HelloAsso/HaApiV5).

In order to access any endpoint you will need to authenticate using **OAuth 2.0** authentication server : `https://api.helloasso.com/oauth2`

When browsing the Swagger documentation :
- The easiest way to use it is to use the Swagger Authorize feature.  
- Or you can override the Input `Bearer JWT` in the Authorization Header field. You can easily generate a JWT using Postman's Request Token feature

There are two levels of authorization :
  - Client privileges : this defines the endpoints the client has access to.
  - User Roles : being authorized by an organization grants rights on this organization.

<details><summary>Connecting to the API with OAuth 2.0</summary>

**In order to use the API, you must authenticate with the OAuth 2.0 server using a private API Client.**

*What is OAuth 2.0 ? : https://tools.ietf.org/html/rfc6749*

* If you are an **organization** (i.e. association), you can obtain a client (Id and Secret) in your administration area. You will be granted the following privileges : AccessPublicData, AccessTransactions and Checkout.
* If you are a **partner**, please contact HelloAsso at partenariats@helloasso.org in order to obtain a client.

In any case, please keep the client (id and secret) to yourself.

On HelloAsso, we support the following grants :
* Client Credentials
  `grant_type=client_credentials`
  If used by an organization, the rights are limited to the current organization to which the client has been issued.
  If used by a partner, it does not grant you any rights (User Roles) on resource owned by an organization, you can only access routes without any User Role required.
  See [more information below](#DescClientCredentials).
* Refresh
  `grant_type=refresh_token`
  See [more information below](#DescRefreshToken).
* Authorization Code
  `grant_type=authorization_code`
  Only for partners : it permits you to access private resources of organizations, by using the [authorization flow](#DescAuthorizeFlow).
* Disconnect (custom endpoint)
  By calling https://api.helloasso.com/oauth2/disconnect, with a valid Bearer token in the Authorization Header, you will disconnect the user, and revoke its tokens. See [more information below](#DescDisconnect).

When authenticating, you will receive an `access_token` which is short-lived (30 minutes), and a `refresh_token`, that will permit you to obtain a new `access_token` for one month. When you have an `access_token` and a `refresh_token`, **you MUST obtain a new `access_token` using the `refresh_token` issued to you** (with the `grant_type=refresh_token`), **and MUST NOT obtain a new `access_token` by using the client**. When refreshing, you will obtain a new `access_token` valid for 30 minutes, and a new `refresh_token` valid for another month. If you continue to use each `refresh_token` that you receive, you could stay authenticated forever, without requiring to enter again your client secret or prompting the user for its login and password.

<details><summary id=\"DescClientCredentials\">Client Credentials</summary>

This is your first route to obtain an `access_token` to communicate with the API.

Route : **POST** `https://api.helloasso.com/oauth2/token`

<details><summary>Headers</summary>

Key          | Value
-------------|------------------------------------
Content-Type | *application/x-www-form-urlencoded*
</details>
<details><summary>Body</summary>

Key           | Information          | Required/Optional
--------------|----------------------|------------------
client_id     | Your Client Id       | **Required**
client_secret | Your Client Secret   | **Required**
grant_type    | *client_credentials* | **Required**
</details>
<details><summary>Result (JSON)</summary>

Key                | Information
-------------------|-------------------------------------------------------------------------
access_token       | The JWT token to use in future requests
refresh_token      | Token used to refresh the token and get a new JWT token after expiration
token_type         | Token Type : always \"*bearer*\"
expires_in         | The lifetime in seconds of the access token
</details>
<details><summary>CURL Example</summary>

```
curl -X POST \\
  https://api.helloasso.com/oauth2/token \\
  -H 'content-type: application/x-www-form-urlencoded' \\
  -d 'grant_type=client_credentials&client_id=9fdc22226bf24ff99b875f4a7c503715&client_secret=AvUYelYH1aSZZ3QNBiZOybmBlZTpUcNSonsufB5txuw='
```
</details>
</details>
<details><summary id=\"DescRefreshToken\">Refresh Token</summary>

Your route to refresh indefinitely your `access_token` and obtain a new one.

Route : **POST** `https://api.helloasso.com/oauth2/token`

<details><summary>Headers</summary>

Key          | Value
-------------|------------------------------------
Content-Type | *application/x-www-form-urlencoded*
</details>
<details><summary>Body</summary>

Key           | Information        | Required/Optional
--------------|--------------------|------------------
grant_type    | *refresh_token*    | **Required**
refresh_token | Your Refresh Token | **Required**
</details>
<details><summary>Result (JSON)</summary>

Key                | Information
-------------------|-------------------------------------------------------------------------
access_token       | The JWT token to use in future requests
refresh_token      | Token used to refresh the token and get a new JWT token after expiration
token_type         | Token Type : always \"*bearer*\"
expires_in         | The lifetime in seconds of the access token
</details>
<details><summary>CURL Example</summary>

```
curl -X POST \\
  https://api.helloasso.com/oauth2/token \\
  -H 'content-type: application/x-www-form-urlencoded' \\
  -d 'grant_type=refresh_token&client_id=9fdc22226bf24ff99b875f4a7c503715&refresh_token=REFRESH_TOKEN'
```
</details>
</details>
<details><summary id=\"DescAuthorizeFlow\">Authorize flow</summary>

The authorize flow will permit partner's applications to access protected resources (resources owned by an organization). Typically, you must display a button to your user to ask him to login on HelloAsso, and authorize you to access his resources.

Button example :

![](/img/ImgHaAuthorizeButton.png)

<details><summary>Button Code (HTML & CSS)</summary>
```
<button class=\"HaAuthorizeButton\">
  <img src=\"/img/logo-ha.svg\" alt=\"\" class=\"HaAuthorizeButtonLogo\">
  <span class=\"HaAuthorizeButtonTitle\">Connecter à HelloAsso</span>
</button>

<style>
.HaAuthorizeButton {
  align-items: center;
  -webkit-box-pack: center;
  -ms-flex-pack: center;
  background-color: #FFFFFF;
  border: 0.0625rem solid #49D38A;
  border-radius: 0.125rem;
  display: -webkit-box;
  display: -ms-flexbox;
  display: flex;
  padding: 0;
}
.HaAuthorizeButton:disabled {
  background-color: #E9E9F0;
  border-color: transparent;
  cursor: not-allowed;
}
.HaAuthorizeButton:not(:disabled):focus {
  box-shadow: 0 0 0 0.25rem rgba(73, 211, 138, 0.25);
  -webkit-box-shadow: 0 0 0 0.25rem rgba(73, 211, 138, 0.25);
}
.HaAuthorizeButtonLogo {
  padding: 0 0.8rem;
  width: 2.25rem;
}
.HaAuthorizeButtonTitle {
  background-color: #49D38A;
  color: #FFFFFF;
  font-size: 1rem;
  font-weight: 700;
  padding: 0.78125rem 1.5rem;
}
.HaAuthorizeButton:disabled .HaAuthorizeButtonTitle {
  background-color: #E9E9F0;
  color: #9A9DA8;
}
.HaAuthorizeButton:not(:disabled):hover .HaAuthorizeButtonTitle,
.HaAuthorizeButton:not(:disabled):focus .HaAuthorizeButtonTitle {
  background-color: #30c677;
}
</style>
```
</details>

When the user clicks on the button or link, you must open a popup window and direct him to this url : https://auth.helloasso.com/authorize with all the appropriate query parameters (see below).

<details><summary>Authorization Request</summary>

Route to display : **GET** `https://auth.helloasso.com/authorize`

<details><summary>Parameters</summary>

Key                    | Information                                                      | Required/Optional
-----------------------|------------------------------------------------------------------|------------------
client_id              | Your Client Id                                                   | **Required**
redirect_uri           | The redirect uri that will be used when the authorize is complete (success or error). For security considerations, the domain of the redirect uri must be the same configured on your client in our database. The redirect uri must use the secure protocol `https`. You can modify this domain with the following endpoint https://api.helloasso.com/v5/partners/me/api-clients (see section \"Partners Managment\" bellow) | **Required**
code_challenge         | The PKCE code challenge. See section [PKCE](#PKCE) below.        | **Required**
code_challenge_method  | The PKCE code challenge method, must be \"*S256*\"                 | **Required**
state                  | A value that will be sent back to you, to maintain state between the request and callback. The parameter should be used for preventing cross-site request forgery : **the state sent back must match the one you sent**. Must be a string, but you can use this to encode any data you want. The state should be less than 500 characters. | Optional

We recommend you to open the window with at least an height of 650px and a width of 500px.

</details>
<details><summary>Request Example</summary>

```
https://auth.helloasso.com/authorize
  ?client_id=9fdc22226bf24ff99b875f4a7c503715
  &redirect_uri=YOUR_REDIRECT_URI
  &code_challenge=YOUR_CODE_CHALLENGE
  &code_challenge_method=S256
  &state=abc
```
</details>
</details>

This will display the login window and then the authorize window to the user : 

<img alt=\"\" src=\"/img/ImgHaLoginWindow.png\" style=\"height:400px\"><img alt=\"\" src=\"/img/ImgHaAuthorizeWindow.png\" style=\"height:400px;margin-left:20px\">

The user has an option to sign up and register its organization, if he does not have one already.
When the user completes the process, the window will redirect to the given `redirect_uri`, with the `authorization_code` in parameter (or with an error code if an error occurred).

<details><summary>Redirect response</summary>

If success :
Key   | Information
------|----------------------------------------------------------------------------------------------
code  | The authorization code generated by the authorization server. Has a lifetime of five minutes.
state | If you supplied a state in the request, will be sent back to you

If error :
Key               | Information
------------------|-----------------------------------------------------------------
error             | A single error code.
error_description | Optional : a text providing additional information
state             | If you supplied a state in the request, will be sent back to you
See more info here on error codes : https://tools.ietf.org/html/rfc6749#section-4.1.2.1
</details>

You can then exchange the code for an `access_token` and a `refresh_token`.

<details><summary>Access Token Request</summary>

Route : **POST** `https://api.helloasso.com/oauth2/token`

<details><summary>Headers</summary>

Key          | Value
-------------|------------------------------------
Content-Type | *application/x-www-form-urlencoded*
</details>
<details><summary>Body</summary>

Key           | Information                                                       | Required/Optional
--------------|-------------------------------------------------------------------|------------------
client_id     | Your Client Id                                                    | **Required**
client_secret | Your Client Secret                                                | **Required**
grant_type    | *authorization_code*                                              | **Required**
code          | The authorization code received (when redirecting to your domain) | **Required**
redirect_uri  | The same redirect uri used when displaying the authorize window   | **Required**
code_verifier | The PKCE code verifier                                            | **Required**
</details>
<details><summary>Result (JSON)</summary>

Key                | Information
-------------------|-------------------------------------------------------------------------
access_token       | The JWT token to use in future requests
refresh_token      | Token used to refresh the token and get a new JWT token after expiration
token_type         | Token Type : always \"*bearer*\"
expires_in         | The lifetime in seconds of the access token
organization_slug  | The slug of the association which authorized the access
</details>
<details><summary>CURL Example</summary>

```
curl -X POST \\
  https://api.helloasso.com/oauth2/token \\
  -H 'content-type: application/x-www-form-urlencoded' \\
  -d 'grant_type=authorization_code&client_id=9fdc22226bf24ff99b875f4a7c503715&client_secret=AvUYelYH1aSZZ3QNBiZOybmBlZTpUcNSonsufB5txuw=' \\
  -d 'code=AUTHORIZATION_CODE_RECEIVED&redirect_uri=YOUR_REDIRECT_URI&code_verifier=YOUR_CODE_VERIFIER'
```
</details>
</details>
<details><summary id=\"PKCE\">PKCE</summary>

PKCE (Proof Key for Code Exchange) is a security measure for the authorization grant.

The specification can be found here : https://tools.ietf.org/html/rfc7636

We require you to use the challenge_method S256.

Basically, you must generate a random Code Verifier of 43 to 128 characters, from the following characters : `ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~`. You then create the Code Challenge : you hash the code verifier with the SHA-256 hash function, encode it to base64, and then url encode it.

You will send the code challenge in the initial request to display the authorize window. HelloAsso will keep it stored with your request. When you will request a token in exchange for your authorization code, you will send the code verifier. HelloAsso will then validate it (length and characters used) and encode it the same way you did it (SHA-256, base 64 & uri encode) and compare the result with the code challenge stored during the authorize request. If it does not match, you will receive the error \"invalid_grant\".

If you want to test your code challenge generator, you can do so here with this online tool : https://tonyxu-io.github.io/pkce-generator/
</details>
</details>
<details><summary id=\"DescDisconnect\">Disconnect</summary>

Disconnect the user, and revoke its tokens. Returns 200 if disconnect worked.

Route : **GET** `https://api.helloasso.com/oauth2/disconnect`

<details><summary>Headers</summary>

Key           | Value
--------------|----------------------------------------------------------
Authorization | *Bearer JWT* (replace JWT with your `access_token` value)
</details>
<details><summary>CURL Example</summary>

```
curl -X GET \\
  https://api.helloasso.com/oauth2/disconnect \\
  -H 'Authorization: Bearer JWT'
```
</details>
</details>
</details>
<details><summary id=\"DescNotifications\">Handling the notifications system</summary>

You will receive a notification when one of the following events occur:
- A campaign is created
- An order is made (including free orders where there are no payments)
- A payment is made (whether it is a single payment or a payment by installment)
- A payment is refunded
- A payment is contested by the payer with his bank
- A payment by installment is refused
- An organization is renamed and his slug change

<details><summary>Define your notification URL</summary>

* If you are an **organization** (i.e. association), you can define and manage your notification URL in your administration area.
* If you are a **partner**, You can modify this notification URL with the following endpoint https://api.helloasso.com/v5/partners/me/api-notifications (see section \"Partners Managment\" bellow). 

In any case, your callback URL **must use the secure protocol `https` and must support the `POST` verb.**

Then, in your code, you can handle and listen for information coming from the defined URL.
</details>

<details><summary>Get the notification content</summary>

The notification can have 4 different types : **Order**, **Payment**, **Form** (for campaign creation) or **Organization** (when the organization slug changed).

When a new content is available, we will call the notification URL callback defined before with the corresponding data in the body.

<details><summary>Order Result (JSON)</summary>
Key               | Information
------------------|-------------------------------------------------------------------------
eventType         | Order
data              | The order data. See [more information on the order model below](#model-HelloAsso.Api.V5.Common.Orders.OrderDetail)
</details>
<details><summary id=\"PaymentResultJson\">Payment Result (JSON)</summary>
Key               | Information
------------------|-------------------------------------------------------------------------
eventType         | Payment
data              | The payment data. See [more information on the payment model below](#model-HelloAsso.Api.V5.Common.Orders.PaymentDetail)
</details>
<details><summary>Form Result (JSON)</summary>
Key               | Information
------------------|-------------------------------------------------------------------------
eventType         | Form
data              | The form data. See [more information on the form model below](#model-HelloAsso.Api.V5.Common.Models.Forms.FormPublicModel)
</details>
<details><summary>Organization Result (JSON)</summary>
Key               | Information
------------------|-------------------------------------------------------------------------
eventType         | Organization
data              | { \"data\": { \"old_slug_organization\": \"ctb\", \"new_slug_organization\": \"club tennis bordeaux\"  }, \"eventType\": \"Organization\"}
</details>
</details>
</details>
</details>
<details><summary id=\"DescCheckout\">Checkout Form</summary>

The Checkout is a type of form, specifically designed to allow contributors to make payments prefilled by an authorized partner or an organization.
The partner/organization creates a checkout intent (with all the contributor information that he may have, and specified payment terms ; see the following API route *POST checkout-intents* of the [Checkout intents management](#Checkout%20intents%20management) section), and receives in response a url (which is valid for 15 minutes). The contributor follows the url, and then validates the form and pays. The contributor is then redirected to the page of your choosing, with a result parameter (error, success etc).
The initiator receives a notification of each Order and Payment authorized (if you have configured them, see [Handling the notifications system](#DescNotifications). You can also verify your checkout intent result and receive the created order. See the route *GET checkout-intents/{checkoutIntentId}* of the [Checkout intents management](#operations-tag-Checkout%20intents%20management) section.

You can find a more detailed description on how to integrate by following this [link](https://dev.helloasso.com/docs/int%C3%A9grer-le-paiement-sur-votre-site).

If you want try checkout payments please use our test environnement https://www.helloasso-sandbox.com/ to create an organization.
You can then retrieve your API client in the back office and make calls on https://api.helloasso-sandbox.com/v5.
Virtual credit cards are avaible by following this [link](https://docs.sips.Worldline-solutions.com/fr/cartes-de-test.html).

</details>


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: public
- Package version: public
- Generator version: 7.19.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.helloasso.com/v5*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*CheckoutIntentsManagementApi* | [**organizations_organization_slug_checkout_intents_checkout_intent_id_get**](docs/CheckoutIntentsManagementApi.md#organizations_organization_slug_checkout_intents_checkout_intent_id_get) | **GET** /organizations/{organizationSlug}/checkout-intents/{checkoutIntentId} | Retrieve a checkout intent, with the order if the payment has been authorized.
*CheckoutIntentsManagementApi* | [**organizations_organization_slug_checkout_intents_post**](docs/CheckoutIntentsManagementApi.md#organizations_organization_slug_checkout_intents_post) | **POST** /organizations/{organizationSlug}/checkout-intents | Init a checkout.
*DirectoryApi* | [**directory_forms_post**](docs/DirectoryApi.md#directory_forms_post) | **POST** /directory/forms | Get all forms by form filters and organization filters
*DirectoryApi* | [**directory_organizations_post**](docs/DirectoryApi.md#directory_organizations_post) | **POST** /directory/organizations | Get all organization by organization filters
*FormsApi* | [**organizations_organization_slug_form_types_get**](docs/FormsApi.md#organizations_organization_slug_form_types_get) | **GET** /organizations/{organizationSlug}/formTypes | Get a list of formTypes for an organization
*FormsApi* | [**organizations_organization_slug_forms_form_type_action_quick_create_post**](docs/FormsApi.md#organizations_organization_slug_forms_form_type_action_quick_create_post) | **POST** /organizations/{organizationSlug}/forms/{formType}/action/quick-create | Create a simplified event for an Organism
*FormsApi* | [**organizations_organization_slug_forms_form_type_form_slug_public_get**](docs/FormsApi.md#organizations_organization_slug_forms_form_type_form_slug_public_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/public | Get detailed public data about a specific form
*FormsApi* | [**organizations_organization_slug_forms_form_type_form_slug_state_put**](docs/FormsApi.md#organizations_organization_slug_forms_form_type_form_slug_state_put) | **PUT** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/state | Update a form state
*FormsApi* | [**organizations_organization_slug_forms_get**](docs/FormsApi.md#organizations_organization_slug_forms_get) | **GET** /organizations/{organizationSlug}/forms | Get the forms of a specific organization
*OrdersItemsApi* | [**items_item_id_get**](docs/OrdersItemsApi.md#items_item_id_get) | **GET** /items/{itemId} | Get the detail of an item contained in an order
*OrdersItemsApi* | [**orders_order_id_cancel_post**](docs/OrdersItemsApi.md#orders_order_id_cancel_post) | **POST** /orders/{orderId}/cancel | Cancels future payments for an order, no refunds will be given.
*OrdersItemsApi* | [**orders_order_id_get**](docs/OrdersItemsApi.md#orders_order_id_get) | **GET** /orders/{orderId} | Get detailed information about a specific order
*OrdersItemsApi* | [**organizations_organization_slug_forms_form_type_form_slug_items_get**](docs/OrdersItemsApi.md#organizations_organization_slug_forms_form_type_form_slug_items_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/items | Get a list of items \"sold\" in a form
*OrdersItemsApi* | [**organizations_organization_slug_forms_form_type_form_slug_orders_get**](docs/OrdersItemsApi.md#organizations_organization_slug_forms_form_type_form_slug_orders_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/orders | Get form orders
*OrdersItemsApi* | [**organizations_organization_slug_items_get**](docs/OrdersItemsApi.md#organizations_organization_slug_items_get) | **GET** /organizations/{organizationSlug}/items | Get a list of items sold by an organization
*OrdersItemsApi* | [**organizations_organization_slug_orders_get**](docs/OrdersItemsApi.md#organizations_organization_slug_orders_get) | **GET** /organizations/{organizationSlug}/orders | Get a list of orders within a specific organization
*OrganizationCashOutManagementApi* | [**organizations_organization_slug_cash_out_cash_out_id_export_get**](docs/OrganizationCashOutManagementApi.md#organizations_organization_slug_cash_out_cash_out_id_export_get) | **GET** /organizations/{organizationSlug}/cash-out/{cashOutId}/export | Download the cash-out details as a CSV, Excel or Json.
*OrganizationVisualisationApi* | [**organizations_organization_slug_get**](docs/OrganizationVisualisationApi.md#organizations_organization_slug_get) | **GET** /organizations/{organizationSlug} | Get Organization details
*PartnerManagementApi* | [**partners_me_api_clients_put**](docs/PartnerManagementApi.md#partners_me_api_clients_put) | **PUT** /partners/me/api-clients | A partner can update his domain
*PartnerManagementApi* | [**partners_me_api_notifications_delete**](docs/PartnerManagementApi.md#partners_me_api_notifications_delete) | **DELETE** /partners/me/api-notifications | A partner can delete his main notification url
*PartnerManagementApi* | [**partners_me_api_notifications_organizations_organization_slug_delete**](docs/PartnerManagementApi.md#partners_me_api_notifications_organizations_organization_slug_delete) | **DELETE** /partners/me/api-notifications/organizations/{organizationSlug} | A partner can delete a notification url linked to an organization
*PartnerManagementApi* | [**partners_me_api_notifications_organizations_organization_slug_put**](docs/PartnerManagementApi.md#partners_me_api_notifications_organizations_organization_slug_put) | **PUT** /partners/me/api-notifications/organizations/{organizationSlug} | A partner can update a notification url linked to an organization
*PartnerManagementApi* | [**partners_me_api_notifications_put**](docs/PartnerManagementApi.md#partners_me_api_notifications_put) | **PUT** /partners/me/api-notifications | A partner can update his main notification url
*PartnerManagementApi* | [**partners_me_get**](docs/PartnerManagementApi.md#partners_me_get) | **GET** /partners/me | A partner can retrieve his information
*PartnerManagementApi* | [**partners_me_organizations_get**](docs/PartnerManagementApi.md#partners_me_organizations_get) | **GET** /partners/me/organizations | Get all organization by partner
*PaymentsManagementApi* | [**organizations_organization_slug_forms_form_type_form_slug_payments_get**](docs/PaymentsManagementApi.md#organizations_organization_slug_forms_form_type_form_slug_payments_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/payments | Get information about payments made in a specific form
*PaymentsManagementApi* | [**organizations_organization_slug_payments_get**](docs/PaymentsManagementApi.md#organizations_organization_slug_payments_get) | **GET** /organizations/{organizationSlug}/payments | Get information about payments made to a specific organization
*PaymentsManagementApi* | [**payments_payment_id_get**](docs/PaymentsManagementApi.md#payments_payment_id_get) | **GET** /payments/{paymentId} | Get detailed information about a specific payment.
*PaymentsManagementApi* | [**payments_payment_id_refund_post**](docs/PaymentsManagementApi.md#payments_payment_id_refund_post) | **POST** /payments/{paymentId}/refund | Refund a payment.
*TagsApi* | [**tags_tag_name_get**](docs/TagsApi.md#tags_tag_name_get) | **GET** /tags/{tagName} | Get Internal Tag Detail
*UsersApi* | [**users_me_organizations_get**](docs/UsersApi.md#users_me_organizations_get) | **GET** /users/me/organizations | Get my organizations
*ValuesDefinitionsApi* | [**values_company_legal_status_get**](docs/ValuesDefinitionsApi.md#values_company_legal_status_get) | **GET** /values/company-legal-status | Get company legal status list
*ValuesDefinitionsApi* | [**values_form_form_type_types_get**](docs/ValuesDefinitionsApi.md#values_form_form_type_types_get) | **GET** /values/form/{formType}/types | Get all activity types for a form type
*ValuesDefinitionsApi* | [**values_organization_categories_get**](docs/ValuesDefinitionsApi.md#values_organization_categories_get) | **GET** /values/organization/categories | Get list of JO categories
*ValuesDefinitionsApi* | [**values_tags_get**](docs/ValuesDefinitionsApi.md#values_tags_get) | **GET** /values/tags | Get list of public tags


## Documentation For Models

 - [HelloAssoApiV5CommonModelsAccountsClientsApiClientModel](docs/HelloAssoApiV5CommonModelsAccountsClientsApiClientModel.md)
 - [HelloAssoApiV5CommonModelsAccountsClientsPublicPutApiClientRequest](docs/HelloAssoApiV5CommonModelsAccountsClientsPublicPutApiClientRequest.md)
 - [HelloAssoApiV5CommonModelsAccountsCompanyLegalStatusModel](docs/HelloAssoApiV5CommonModelsAccountsCompanyLegalStatusModel.md)
 - [HelloAssoApiV5CommonModelsAccountsOrganismCategoryModel](docs/HelloAssoApiV5CommonModelsAccountsOrganismCategoryModel.md)
 - [HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType](docs/HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType.md)
 - [HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel](docs/HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel.md)
 - [HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody](docs/HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody.md)
 - [HelloAssoApiV5CommonModelsCartsCheckoutIntentResponse](docs/HelloAssoApiV5CommonModelsCartsCheckoutIntentResponse.md)
 - [HelloAssoApiV5CommonModelsCartsCheckoutPayer](docs/HelloAssoApiV5CommonModelsCartsCheckoutPayer.md)
 - [HelloAssoApiV5CommonModelsCartsCheckoutPaymentOptions](docs/HelloAssoApiV5CommonModelsCartsCheckoutPaymentOptions.md)
 - [HelloAssoApiV5CommonModelsCartsCheckoutTerm](docs/HelloAssoApiV5CommonModelsCartsCheckoutTerm.md)
 - [HelloAssoApiV5CommonModelsCartsInitCheckoutBody](docs/HelloAssoApiV5CommonModelsCartsInitCheckoutBody.md)
 - [HelloAssoApiV5CommonModelsCartsInitCheckoutResponse](docs/HelloAssoApiV5CommonModelsCartsInitCheckoutResponse.md)
 - [HelloAssoApiV5CommonModelsCommonContactModel](docs/HelloAssoApiV5CommonModelsCommonContactModel.md)
 - [HelloAssoApiV5CommonModelsCommonDocumentModel](docs/HelloAssoApiV5CommonModelsCommonDocumentModel.md)
 - [HelloAssoApiV5CommonModelsCommonImageModel](docs/HelloAssoApiV5CommonModelsCommonImageModel.md)
 - [HelloAssoApiV5CommonModelsCommonMetaModel](docs/HelloAssoApiV5CommonModelsCommonMetaModel.md)
 - [HelloAssoApiV5CommonModelsCommonPaginationModel](docs/HelloAssoApiV5CommonModelsCommonPaginationModel.md)
 - [HelloAssoApiV5CommonModelsCommonPlaceModel](docs/HelloAssoApiV5CommonModelsCommonPlaceModel.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelFormLightModel](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelFormLightModel.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelItem](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelItem.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPartnerOrganizationModel](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPartnerOrganizationModel.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableFormModel](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableFormModel.md)
 - [HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableOrganizationModel](docs/HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableOrganizationModel.md)
 - [HelloAssoApiV5CommonModelsCommonVideoModel](docs/HelloAssoApiV5CommonModelsCommonVideoModel.md)
 - [HelloAssoApiV5CommonModelsComplianceV2DocumentsDocumentState](docs/HelloAssoApiV5CommonModelsComplianceV2DocumentsDocumentState.md)
 - [HelloAssoApiV5CommonModelsDirectoryDirectoryOrganizationPublicModel](docs/HelloAssoApiV5CommonModelsDirectoryDirectoryOrganizationPublicModel.md)
 - [HelloAssoApiV5CommonModelsDirectoryListFormsRequest](docs/HelloAssoApiV5CommonModelsDirectoryListFormsRequest.md)
 - [HelloAssoApiV5CommonModelsDirectoryListOrganizationsRequest](docs/HelloAssoApiV5CommonModelsDirectoryListOrganizationsRequest.md)
 - [HelloAssoApiV5CommonModelsDirectoryPartnerOrganizationModel](docs/HelloAssoApiV5CommonModelsDirectoryPartnerOrganizationModel.md)
 - [HelloAssoApiV5CommonModelsDirectorySynchronizableFormModel](docs/HelloAssoApiV5CommonModelsDirectorySynchronizableFormModel.md)
 - [HelloAssoApiV5CommonModelsDirectorySynchronizableOrganizationModel](docs/HelloAssoApiV5CommonModelsDirectorySynchronizableOrganizationModel.md)
 - [HelloAssoApiV5CommonModelsEnumsFieldType](docs/HelloAssoApiV5CommonModelsEnumsFieldType.md)
 - [HelloAssoApiV5CommonModelsEnumsFormState](docs/HelloAssoApiV5CommonModelsEnumsFormState.md)
 - [HelloAssoApiV5CommonModelsEnumsFormType](docs/HelloAssoApiV5CommonModelsEnumsFormType.md)
 - [HelloAssoApiV5CommonModelsEnumsItemState](docs/HelloAssoApiV5CommonModelsEnumsItemState.md)
 - [HelloAssoApiV5CommonModelsEnumsMembershipValidityType](docs/HelloAssoApiV5CommonModelsEnumsMembershipValidityType.md)
 - [HelloAssoApiV5CommonModelsEnumsOperationState](docs/HelloAssoApiV5CommonModelsEnumsOperationState.md)
 - [HelloAssoApiV5CommonModelsEnumsOrganizationType](docs/HelloAssoApiV5CommonModelsEnumsOrganizationType.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentCashOutState](docs/HelloAssoApiV5CommonModelsEnumsPaymentCashOutState.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentFrequencyType](docs/HelloAssoApiV5CommonModelsEnumsPaymentFrequencyType.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentMeans](docs/HelloAssoApiV5CommonModelsEnumsPaymentMeans.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentProviderType](docs/HelloAssoApiV5CommonModelsEnumsPaymentProviderType.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentState](docs/HelloAssoApiV5CommonModelsEnumsPaymentState.md)
 - [HelloAssoApiV5CommonModelsEnumsPaymentType](docs/HelloAssoApiV5CommonModelsEnumsPaymentType.md)
 - [HelloAssoApiV5CommonModelsEnumsPriceCategory](docs/HelloAssoApiV5CommonModelsEnumsPriceCategory.md)
 - [HelloAssoApiV5CommonModelsEnumsRecordActionType](docs/HelloAssoApiV5CommonModelsEnumsRecordActionType.md)
 - [HelloAssoApiV5CommonModelsEnumsSortField](docs/HelloAssoApiV5CommonModelsEnumsSortField.md)
 - [HelloAssoApiV5CommonModelsEnumsSortOrder](docs/HelloAssoApiV5CommonModelsEnumsSortOrder.md)
 - [HelloAssoApiV5CommonModelsEnumsTagType](docs/HelloAssoApiV5CommonModelsEnumsTagType.md)
 - [HelloAssoApiV5CommonModelsEnumsTierType](docs/HelloAssoApiV5CommonModelsEnumsTierType.md)
 - [HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel](docs/HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel.md)
 - [HelloAssoApiV5CommonModelsFormsExtraOptionPublicModel](docs/HelloAssoApiV5CommonModelsFormsExtraOptionPublicModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormActivityModel](docs/HelloAssoApiV5CommonModelsFormsFormActivityModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormBasicModel](docs/HelloAssoApiV5CommonModelsFormsFormBasicModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormLightModel](docs/HelloAssoApiV5CommonModelsFormsFormLightModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormPublicModel](docs/HelloAssoApiV5CommonModelsFormsFormPublicModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormQuickCreateModel](docs/HelloAssoApiV5CommonModelsFormsFormQuickCreateModel.md)
 - [HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest](docs/HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest.md)
 - [HelloAssoApiV5CommonModelsFormsFormStateRequest](docs/HelloAssoApiV5CommonModelsFormsFormStateRequest.md)
 - [HelloAssoApiV5CommonModelsFormsTermModel](docs/HelloAssoApiV5CommonModelsFormsTermModel.md)
 - [HelloAssoApiV5CommonModelsFormsTierLightModel](docs/HelloAssoApiV5CommonModelsFormsTierLightModel.md)
 - [HelloAssoApiV5CommonModelsFormsTierPublicModel](docs/HelloAssoApiV5CommonModelsFormsTierPublicModel.md)
 - [HelloAssoApiV5CommonModelsOrganizationsOrganizationBasicModel](docs/HelloAssoApiV5CommonModelsOrganizationsOrganizationBasicModel.md)
 - [HelloAssoApiV5CommonModelsOrganizationsOrganizationLightModel](docs/HelloAssoApiV5CommonModelsOrganizationsOrganizationLightModel.md)
 - [HelloAssoApiV5CommonModelsOrganizationsOrganizationPublicModel](docs/HelloAssoApiV5CommonModelsOrganizationsOrganizationPublicModel.md)
 - [HelloAssoApiV5CommonModelsPartnersPartnerPublicModel](docs/HelloAssoApiV5CommonModelsPartnersPartnerPublicModel.md)
 - [HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportPaymentOperation](docs/HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportPaymentOperation.md)
 - [HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportPaymentStatus](docs/HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportPaymentStatus.md)
 - [HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportRowModel](docs/HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportRowModel.md)
 - [HelloAssoApiV5CommonModelsPaymentRefundOperationModel](docs/HelloAssoApiV5CommonModelsPaymentRefundOperationModel.md)
 - [HelloAssoApiV5CommonModelsStatisticsItem](docs/HelloAssoApiV5CommonModelsStatisticsItem.md)
 - [HelloAssoApiV5CommonModelsStatisticsItemCustomField](docs/HelloAssoApiV5CommonModelsStatisticsItemCustomField.md)
 - [HelloAssoApiV5CommonModelsStatisticsItemDetail](docs/HelloAssoApiV5CommonModelsStatisticsItemDetail.md)
 - [HelloAssoApiV5CommonModelsStatisticsItemDiscount](docs/HelloAssoApiV5CommonModelsStatisticsItemDiscount.md)
 - [HelloAssoApiV5CommonModelsStatisticsItemOption](docs/HelloAssoApiV5CommonModelsStatisticsItemOption.md)
 - [HelloAssoApiV5CommonModelsStatisticsItemPayment](docs/HelloAssoApiV5CommonModelsStatisticsItemPayment.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrder](docs/HelloAssoApiV5CommonModelsStatisticsOrder.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrderAmountModel](docs/HelloAssoApiV5CommonModelsStatisticsOrderAmountModel.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrderDetail](docs/HelloAssoApiV5CommonModelsStatisticsOrderDetail.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrderItem](docs/HelloAssoApiV5CommonModelsStatisticsOrderItem.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrderLight](docs/HelloAssoApiV5CommonModelsStatisticsOrderLight.md)
 - [HelloAssoApiV5CommonModelsStatisticsOrderPayment](docs/HelloAssoApiV5CommonModelsStatisticsOrderPayment.md)
 - [HelloAssoApiV5CommonModelsStatisticsPayer](docs/HelloAssoApiV5CommonModelsStatisticsPayer.md)
 - [HelloAssoApiV5CommonModelsStatisticsPayment](docs/HelloAssoApiV5CommonModelsStatisticsPayment.md)
 - [HelloAssoApiV5CommonModelsStatisticsPaymentDetail](docs/HelloAssoApiV5CommonModelsStatisticsPaymentDetail.md)
 - [HelloAssoApiV5CommonModelsStatisticsPaymentItem](docs/HelloAssoApiV5CommonModelsStatisticsPaymentItem.md)
 - [HelloAssoApiV5CommonModelsStatisticsRefundOperationLightModel](docs/HelloAssoApiV5CommonModelsStatisticsRefundOperationLightModel.md)
 - [HelloAssoApiV5CommonModelsStatisticsShareItem](docs/HelloAssoApiV5CommonModelsStatisticsShareItem.md)
 - [HelloAssoApiV5CommonModelsStatisticsSharePayment](docs/HelloAssoApiV5CommonModelsStatisticsSharePayment.md)
 - [HelloAssoApiV5CommonModelsStatisticsUser](docs/HelloAssoApiV5CommonModelsStatisticsUser.md)
 - [HelloAssoApiV5CommonModelsTagsInternalTagModel](docs/HelloAssoApiV5CommonModelsTagsInternalTagModel.md)
 - [HelloAssoApiV5CommonModelsTagsPublicTagModel](docs/HelloAssoApiV5CommonModelsTagsPublicTagModel.md)
 - [HelloAssoApiV5ModelsPartnerStatisticsModel](docs/HelloAssoApiV5ModelsPartnerStatisticsModel.md)
 - [HelloAssoModelsEnumsGlobalRole](docs/HelloAssoModelsEnumsGlobalRole.md)
 - [HelloAssoModelsSharedGeoLocation](docs/HelloAssoModelsSharedGeoLocation.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



