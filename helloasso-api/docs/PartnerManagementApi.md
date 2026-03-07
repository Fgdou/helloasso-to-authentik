# \PartnerManagementApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**partners_me_api_clients_put**](PartnerManagementApi.md#partners_me_api_clients_put) | **PUT** /partners/me/api-clients | A partner can update his domain
[**partners_me_api_notifications_delete**](PartnerManagementApi.md#partners_me_api_notifications_delete) | **DELETE** /partners/me/api-notifications | A partner can delete his main notification url
[**partners_me_api_notifications_organizations_organization_slug_delete**](PartnerManagementApi.md#partners_me_api_notifications_organizations_organization_slug_delete) | **DELETE** /partners/me/api-notifications/organizations/{organizationSlug} | A partner can delete a notification url linked to an organization
[**partners_me_api_notifications_organizations_organization_slug_put**](PartnerManagementApi.md#partners_me_api_notifications_organizations_organization_slug_put) | **PUT** /partners/me/api-notifications/organizations/{organizationSlug} | A partner can update a notification url linked to an organization
[**partners_me_api_notifications_put**](PartnerManagementApi.md#partners_me_api_notifications_put) | **PUT** /partners/me/api-notifications | A partner can update his main notification url
[**partners_me_get**](PartnerManagementApi.md#partners_me_get) | **GET** /partners/me | A partner can retrieve his information
[**partners_me_organizations_get**](PartnerManagementApi.md#partners_me_organizations_get) | **GET** /partners/me/organizations | Get all organization by partner



## partners_me_api_clients_put

> partners_me_api_clients_put(hello_asso_api_v5_common_models_accounts_clients_public_put_api_client_request)
A partner can update his domain

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hello_asso_api_v5_common_models_accounts_clients_public_put_api_client_request** | Option<[**HelloAssoApiV5CommonModelsAccountsClientsPublicPutApiClientRequest**](HelloAssoApiV5CommonModelsAccountsClientsPublicPutApiClientRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_api_notifications_delete

> partners_me_api_notifications_delete(notification_type)
A partner can delete his main notification url

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_type** | Option<[**HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType**](HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType.md)> | Do not specify a notification type to remove the main notification Url |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_api_notifications_organizations_organization_slug_delete

> partners_me_api_notifications_organizations_organization_slug_delete(organization_slug, notification_type)
A partner can delete a notification url linked to an organization

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**notification_type** | Option<[**HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType**](HelloAssoApiV5CommonModelsApiNotificationsApiNotificationType.md)> | Do not specify a notification type to remove the main notification Url |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_api_notifications_organizations_organization_slug_put

> models::HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel partners_me_api_notifications_organizations_organization_slug_put(organization_slug, hello_asso_api_v5_common_models_api_notifications_post_api_url_notification_body)
A partner can update a notification url linked to an organization

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**hello_asso_api_v5_common_models_api_notifications_post_api_url_notification_body** | Option<[**HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody**](HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody.md)> | The body of the request, do not specify a notification type to update the main notification Url |  |

### Return type

[**models::HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel**](HelloAsso.Api.V5.Common.Models.ApiNotifications.ApiUrlNotificationModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_api_notifications_put

> models::HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel partners_me_api_notifications_put(hello_asso_api_v5_common_models_api_notifications_post_api_url_notification_body)
A partner can update his main notification url

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hello_asso_api_v5_common_models_api_notifications_post_api_url_notification_body** | Option<[**HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody**](HelloAssoApiV5CommonModelsApiNotificationsPostApiUrlNotificationBody.md)> | The body of the request, do not specify a notification type to update the main notification Url |  |

### Return type

[**models::HelloAssoApiV5CommonModelsApiNotificationsApiUrlNotificationModel**](HelloAsso.Api.V5.Common.Models.ApiNotifications.ApiUrlNotificationModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_get

> models::HelloAssoApiV5CommonModelsPartnersPartnerPublicModel partners_me_get()
A partner can retrieve his information

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HelloAssoApiV5CommonModelsPartnersPartnerPublicModel**](HelloAsso.Api.V5.Common.Models.Partners.PartnerPublicModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## partners_me_organizations_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPartnerOrganizationModel partners_me_organizations_get(page_size, continuation_token)
Get all organization by partner

List all organization linked to partner.  Results are ordered by Api visibility update date ascending.  The total number of results (or pages) isn't retrievable, so the pagination information returned will always say -1.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelPartnerOrganizationModel**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_PartnerOrganizationModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

