# \PaymentsManagementApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_organization_slug_forms_form_type_form_slug_payments_get**](PaymentsManagementApi.md#organizations_organization_slug_forms_form_type_form_slug_payments_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/payments | Get information about payments made in a specific form
[**organizations_organization_slug_payments_get**](PaymentsManagementApi.md#organizations_organization_slug_payments_get) | **GET** /organizations/{organizationSlug}/payments | Get information about payments made to a specific organization
[**payments_payment_id_get**](PaymentsManagementApi.md#payments_payment_id_get) | **GET** /payments/{paymentId} | Get detailed information about a specific payment.
[**payments_payment_id_refund_post**](PaymentsManagementApi.md#payments_payment_id_refund_post) | **POST** /payments/{paymentId}/refund | Refund a payment.



## organizations_organization_slug_forms_form_type_form_slug_payments_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment organizations_organization_slug_forms_form_type_form_slug_payments_get(organization_slug, form_slug, form_type, from, to, user_search_key, page_index, page_size, continuation_token, states, sort_order, sort_field)
Get information about payments made in a specific form

<br/><br/><b>Your token must have one of these roles : </b><br/>FormAdmin<br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization slug | [required] |
**form_slug** | **String** | The form slug | [required] |
**form_type** | [**HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md) | The form type CrowdFunding, Membership, Event, Donation, PaymentForm, Checkout, Shop | [required] |
**from** | Option<**String**> | First Date Filter |  |
**to** | Option<**String**> | End Date Filter (exclusive) |  |
**user_search_key** | Option<**String**> | Filter results on user or payer first name, last name or email |  |
**page_index** | Option<**i32**> | The page of results to retrieve |  |[default to 1]
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsPaymentState>**](Models__HelloAssoApiV5CommonModelsEnumsPaymentState.md)> | Filter results by states of payments  Available values: * `Pending` - A payment scheduled at a later date, not yet processed. * `Authorized` - The payment has been authorized, validated, processed. * `Refused` - The payment has been refused by the bank. * `Unknown` * `Registered` - Represents a payment made offline.             Probably for an item of type * `Refunded` - The payment has been refunded. * `Refunding` - The payment is being refunded. * `Contested` - Payment has been contested by the contributor * `WaitingBankValidation` - The payment is pending validation from the bank (used by SEPA direct debit). |  |
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort payments by ascending or descending order. Default is descending |  |
**sort_field** | Option<[**HelloAssoApiV5CommonModelsEnumsSortField**](HelloAssoApiV5CommonModelsEnumsSortField.md)> | Sort payments by a specific field (Date or UpdateDate). Default is date |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_Payment.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_payments_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment organizations_organization_slug_payments_get(organization_slug, from, to, user_search_key, page_index, page_size, continuation_token, states, sort_order, sort_field)
Get information about payments made to a specific organization

Return list of payments according to parameters<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization Slug | [required] |
**from** | Option<**String**> | First Date Filter |  |
**to** | Option<**String**> | End Date Filter (exclusive) |  |
**user_search_key** | Option<**String**> | Filter results on user or payer first name, last name or email |  |
**page_index** | Option<**i32**> | The page of results to retrieve |  |[default to 1]
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsPaymentState>**](Models__HelloAssoApiV5CommonModelsEnumsPaymentState.md)> | The payment states  Available values: * `Pending` - A payment scheduled at a later date, not yet processed. * `Authorized` - The payment has been authorized, validated, processed. * `Refused` - The payment has been refused by the bank. * `Unknown` * `Registered` - Represents a payment made offline.             Probably for an item of type * `Refunded` - The payment has been refunded. * `Refunding` - The payment is being refunded. * `Contested` - Payment has been contested by the contributor * `WaitingBankValidation` - The payment is pending validation from the bank (used by SEPA direct debit). |  |
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort payments by ascending or descending order. Default is descending |  |
**sort_field** | Option<[**HelloAssoApiV5CommonModelsEnumsSortField**](HelloAssoApiV5CommonModelsEnumsSortField.md)> | Sort payments by a specific field (Date or UpdateDate). Default is date |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPayment**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_Payment.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_payment_id_get

> models::HelloAssoApiV5CommonModelsStatisticsPaymentDetail payments_payment_id_get(payment_id, with_failed_refund_operation)
Get detailed information about a specific payment.

<br/><br/><b>Your token must have one of these roles : </b><br/>FormAdmin<br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **i32** | The payment identifier. | [required] |
**with_failed_refund_operation** | Option<**bool**> | True to retrieve the refund operation in the states 'ABORTED', 'CANCELED', 'ERROR', 'REFUSED'. |  |[default to false]

### Return type

[**models::HelloAssoApiV5CommonModelsStatisticsPaymentDetail**](HelloAsso.Api.V5.Common.Models.Statistics.PaymentDetail.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payments_payment_id_refund_post

> models::HelloAssoApiV5CommonModelsPaymentRefundOperationModel payments_payment_id_refund_post(payment_id, comment, cancel_order, send_refund_mail, amount, x_mfa_access_authorization, x_mfa_sms_access_authorization, x_mfa_password_authorization)
Refund a payment.

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/>FormAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> RefundManagement<br/><br/><br/><br/> **This endpoint is protected with strong authentication.**<br/><br/> When called, it will return an error indicating how the user must authenticate in order to validate the operation.<br/><br/> The authentication tokens must each be added to the corresponding headers (or cookies for mfa access tokens).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**payment_id** | **i32** | The payment identifier. | [required] |
**comment** | Option<**String**> | The comment about this refund. |  |
**cancel_order** | Option<**bool**> | Whether the future payments and linked items of this order must be canceled (possible only if the payment is fully refunded) |  |[default to false]
**send_refund_mail** | Option<**bool**> | Whether a refund mail must be sent or not. |  |[default to true]
**amount** | Option<**i32**> | The amount in cents to refund. Enter this amount only for a partial refund for stripe. If not filled in then the entire payment is refunded |  |[default to 0]
**x_mfa_access_authorization** | Option<**String**> | Must be filled only if AuthorizationErrors.MFA.AccessTokenRequired error code was returned previously. |  |
**x_mfa_sms_access_authorization** | Option<**String**> | Must be filled only if AuthorizationErrors.MFA.AccessOtpSmsRequired error code was returned previously. |  |
**x_mfa_password_authorization** | Option<**String**> | Must be filled only if AuthorizationErrors.MFA.AccessPasswordTokenRequired error code was returned previously. |  |

### Return type

[**models::HelloAssoApiV5CommonModelsPaymentRefundOperationModel**](HelloAsso.Api.V5.Common.Models.Payment.RefundOperationModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

