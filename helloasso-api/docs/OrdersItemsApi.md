# \OrdersItemsApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**items_item_id_get**](OrdersItemsApi.md#items_item_id_get) | **GET** /items/{itemId} | Get the detail of an item contained in an order
[**orders_order_id_cancel_post**](OrdersItemsApi.md#orders_order_id_cancel_post) | **POST** /orders/{orderId}/cancel | Cancels future payments for an order, no refunds will be given.
[**orders_order_id_get**](OrdersItemsApi.md#orders_order_id_get) | **GET** /orders/{orderId} | Get detailed information about a specific order
[**organizations_organization_slug_forms_form_type_form_slug_items_get**](OrdersItemsApi.md#organizations_organization_slug_forms_form_type_form_slug_items_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/items | Get a list of items \"sold\" in a form
[**organizations_organization_slug_forms_form_type_form_slug_orders_get**](OrdersItemsApi.md#organizations_organization_slug_forms_form_type_form_slug_orders_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/orders | Get form orders
[**organizations_organization_slug_items_get**](OrdersItemsApi.md#organizations_organization_slug_items_get) | **GET** /organizations/{organizationSlug}/items | Get a list of items sold by an organization
[**organizations_organization_slug_orders_get**](OrdersItemsApi.md#organizations_organization_slug_orders_get) | **GET** /organizations/{organizationSlug}/orders | Get a list of orders within a specific organization



## items_item_id_get

> models::HelloAssoApiV5CommonModelsStatisticsItemDetail items_item_id_get(item_id, with_details)
Get the detail of an item contained in an order

<br/><br/><b>Your token must have one of these roles : </b><br/>FormAdmin<br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **i32** | The item ID | [required] |
**with_details** | Option<**bool**> | Set to true to return CustomFields and Options |  |[default to false]

### Return type

[**models::HelloAssoApiV5CommonModelsStatisticsItemDetail**](HelloAsso.Api.V5.Common.Models.Statistics.ItemDetail.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_order_id_cancel_post

> orders_order_id_cancel_post(order_id)
Cancels future payments for an order, no refunds will be given.

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/>FormAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> RefundManagement<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i32** | The order identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orders_order_id_get

> models::HelloAssoApiV5CommonModelsStatisticsOrderDetail orders_order_id_get(order_id, with_form_data)
Get detailed information about a specific order

<br/><br/><b>Your token must have one of these roles : </b><br/>FormAdmin<br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **i32** | The order ID | [required] |
**with_form_data** | Option<**bool**> | Set to true to include form data in the response |  |[default to false]

### Return type

[**models::HelloAssoApiV5CommonModelsStatisticsOrderDetail**](HelloAsso.Api.V5.Common.Models.Statistics.OrderDetail.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_form_type_form_slug_items_get

> organizations_organization_slug_forms_form_type_form_slug_items_get(organization_slug, form_slug, form_type, from, to, user_search_key, page_index, page_size, continuation_token, tier_types, item_states, tier_name, with_details, sort_order, sort_field)
Get a list of items \"sold\" in a form

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
**tier_types** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsTierType>**](Models__HelloAssoApiV5CommonModelsEnumsTierType.md)> | The type of tiers |  |
**item_states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsItemState>**](Models__HelloAssoApiV5CommonModelsEnumsItemState.md)> | The item states  Available values: * `Processed` - The item is paid and is valid * `Registered` - The item has been registered manually by the organization and is valid * `Unknown` * `Canceled` - The item has been canceled, and is no longer valid |  |
**tier_name** | Option<**String**> | The name of a tier |  |
**with_details** | Option<**bool**> | Set to true to return CustomFields and Options |  |[default to false]
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort forms items by ascending or descending order. Default is descending |  |
**sort_field** | Option<[**HelloAssoApiV5CommonModelsEnumsSortField**](HelloAssoApiV5CommonModelsEnumsSortField.md)> | Sort forms items by a specific field (Date or UpdateDate). Default is date |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_form_type_form_slug_orders_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder organizations_organization_slug_forms_form_type_form_slug_orders_get(organization_slug, form_slug, form_type, from, to, user_search_key, page_index, page_size, continuation_token, with_details, sort_order)
Get form orders

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
**with_details** | Option<**bool**> | Set to true to return CustomFields |  |[default to false]
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort forms orders by ascending or descending order. Default is descending |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_Order.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_items_get

> organizations_organization_slug_items_get(organization_slug, from, to, user_search_key, page_index, page_size, continuation_token, tier_types, item_states, tier_name, with_details, sort_order, sort_field)
Get a list of items sold by an organization

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization slug | [required] |
**from** | Option<**String**> | First Date Filter |  |
**to** | Option<**String**> | End Date Filter (exclusive) |  |
**user_search_key** | Option<**String**> | Filter results on user or payer first name, last name or email |  |
**page_index** | Option<**i32**> | The page of results to retrieve |  |[default to 1]
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**tier_types** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsTierType>**](Models__HelloAssoApiV5CommonModelsEnumsTierType.md)> | The type of tiers Donation, Payment, Registration, Membership, MonthlyDonation, MonthlyPayment, OfflineDonation, Contribution, Bonus |  |
**item_states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsItemState>**](Models__HelloAssoApiV5CommonModelsEnumsItemState.md)> | The item states  Available values: * `Processed` - The item is paid and is valid * `Registered` - The item has been registered manually by the organization and is valid * `Unknown` * `Canceled` - The item has been canceled, and is no longer valid |  |
**tier_name** | Option<**String**> | The name of a tier |  |
**with_details** | Option<**bool**> | Set to true to return CustomFields and Options |  |[default to false]
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort organizations items by ascending or descending order. Default is descending |  |
**sort_field** | Option<[**HelloAssoApiV5CommonModelsEnumsSortField**](HelloAssoApiV5CommonModelsEnumsSortField.md)> | Sort organizations items by a specific field (Date or UpdateDate). Default is date |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_orders_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder organizations_organization_slug_orders_get(organization_slug, from, to, user_search_key, page_index, page_size, continuation_token, form_types, with_details, sort_order)
Get a list of orders within a specific organization

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization slug | [required] |
**from** | Option<**String**> | First Date Filter |  |
**to** | Option<**String**> | End Date Filter (exclusive) |  |
**user_search_key** | Option<**String**> | Filter results on user or payer first name, last name or email |  |
**page_index** | Option<**i32**> | The page of results to retrieve |  |[default to 1]
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**form_types** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormType>**](Models__HelloAssoApiV5CommonModelsEnumsFormType.md)> | The type of the form CrowdFunding, Membership, Event, Donation, PaymentForm, Checkout, Shop |  |
**with_details** | Option<**bool**> | Set to true to return CustomFields |  |[default to false]
**sort_order** | Option<[**HelloAssoApiV5CommonModelsEnumsSortOrder**](HelloAssoApiV5CommonModelsEnumsSortOrder.md)> | Sort organizations orders by ascending or descending order. Default is descending |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelOrder**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_Order.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

