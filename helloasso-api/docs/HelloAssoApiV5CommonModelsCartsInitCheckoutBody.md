# HelloAssoApiV5CommonModelsCartsInitCheckoutBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_amount** | **i32** | Total amount, all taxes included, in cents (required)  Must be equal to the sum of the initial amount and subsequent terms | 
**initial_amount** | **i32** | The amount for the first term, all taxes included, in cents (required) | 
**item_name** | **String** | Item name (required)  A text describing what the user paid for ('Renew license', '3 tickets', donation, etc).  Will be displayed in the near future in the user space and in the organization back office | 
**back_url** | **String** | Url followed by the contributor if he wants to return to its previous site | 
**error_url** | **String** | Url called in case of an error during the checkout process | 
**return_url** | **String** | Url called after the payment | 
**contains_donation** | **bool** | The sale (or a part of) is a donation | 
**terms** | Option<[**Vec<models::HelloAssoApiV5CommonModelsCartsCheckoutTerm>**](HelloAssoApiV5CommonModelsCartsCheckoutTerm.md)> | The list of future terms (if applicable) | [optional]
**payer** | Option<[**models::HelloAssoApiV5CommonModelsCartsCheckoutPayer**](HelloAssoApiV5CommonModelsCartsCheckoutPayer.md)> |  | [optional]
**metadata** | Option<**serde_json::Value**> | Metadata (optional)  Json object (max length : 20000) | [optional]
**payment_options** | Option<[**models::HelloAssoApiV5CommonModelsCartsCheckoutPaymentOptions**](HelloAssoApiV5CommonModelsCartsCheckoutPaymentOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


