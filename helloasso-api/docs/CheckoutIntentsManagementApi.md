# \CheckoutIntentsManagementApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_organization_slug_checkout_intents_checkout_intent_id_get**](CheckoutIntentsManagementApi.md#organizations_organization_slug_checkout_intents_checkout_intent_id_get) | **GET** /organizations/{organizationSlug}/checkout-intents/{checkoutIntentId} | Retrieve a checkout intent, with the order if the payment has been authorized.
[**organizations_organization_slug_checkout_intents_post**](CheckoutIntentsManagementApi.md#organizations_organization_slug_checkout_intents_post) | **POST** /organizations/{organizationSlug}/checkout-intents | Init a checkout.



## organizations_organization_slug_checkout_intents_checkout_intent_id_get

> models::HelloAssoApiV5CommonModelsCartsCheckoutIntentResponse organizations_organization_slug_checkout_intents_checkout_intent_id_get(organization_slug, checkout_intent_id, with_failed_refund_operation)
Retrieve a checkout intent, with the order if the payment has been authorized.

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> Checkout<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**checkout_intent_id** | **i32** |  | [required] |
**with_failed_refund_operation** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::HelloAssoApiV5CommonModelsCartsCheckoutIntentResponse**](HelloAsso.Api.V5.Common.Models.Carts.CheckoutIntentResponse.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_checkout_intents_post

> models::HelloAssoApiV5CommonModelsCartsInitCheckoutResponse organizations_organization_slug_checkout_intents_post(organization_slug, hello_asso_api_v5_common_models_carts_init_checkout_body)
Init a checkout.

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> Checkout<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**hello_asso_api_v5_common_models_carts_init_checkout_body** | Option<[**HelloAssoApiV5CommonModelsCartsInitCheckoutBody**](HelloAssoApiV5CommonModelsCartsInitCheckoutBody.md)> |  |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCartsInitCheckoutResponse**](HelloAsso.Api.V5.Common.Models.Carts.InitCheckoutResponse.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

