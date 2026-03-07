# \DirectoryApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**directory_forms_post**](DirectoryApi.md#directory_forms_post) | **POST** /directory/forms | Get all forms by form filters and organization filters
[**directory_organizations_post**](DirectoryApi.md#directory_organizations_post) | **POST** /directory/organizations | Get all organization by organization filters



## directory_forms_post

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableFormModel directory_forms_post(page_size, continuation_token, hello_asso_api_v5_common_models_directory_list_forms_request)
Get all forms by form filters and organization filters

Allows you to retrieve a list of all forms visible (only) matching all the filters in the directory until it is synchronized (using the continuationToken).  If filters are left empty, no filter is applied.  Results are ordered by Api visibility update date ascending.  Once the list is synchronized, only forms with an Api visibility update date greater than the last form sent are returned (still using the continuationToken).  This concerns the new forms to be inserted (wishing to appear in the directory) as well as the old ones to be deleted (no longer wishing to appear in the directory).  The total number of results (or pages) isn't retrievable, so the pagination information returned will always say -1.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> FormOpenDirectory<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**hello_asso_api_v5_common_models_directory_list_forms_request** | Option<[**HelloAssoApiV5CommonModelsDirectoryListFormsRequest**](HelloAssoApiV5CommonModelsDirectoryListFormsRequest.md)> | Body which contains the filters to apply |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableFormModel**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_SynchronizableFormModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## directory_organizations_post

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableOrganizationModel directory_organizations_post(page_size, continuation_token, hello_asso_api_v5_common_models_directory_list_organizations_request)
Get all organization by organization filters

Allows you to retrieve a list of all organizations visible (only) matching all the filters in the directory until it is synchronized (using the continuationToken).  If filters are left empty, no filter is applied.  Results are ordered by Api visibility update date ascending.  Once the list is synchronized, only organizations with an Api visibility update date greater than the last organization sent are returned (still using the continuationToken).  This concerns the new organizations to be inserted (wishing to appear in the directory) as well as the old ones to be deleted (no longer wishing to appear in the directory).  The total number of results (or pages) isn't retrievable, so the pagination information returned will always say -1.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> OrganizationOpenDirectory<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |
**hello_asso_api_v5_common_models_directory_list_organizations_request** | Option<[**HelloAssoApiV5CommonModelsDirectoryListOrganizationsRequest**](HelloAssoApiV5CommonModelsDirectoryListOrganizationsRequest.md)> | Body which contains the filters to apply |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelSynchronizableOrganizationModel**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_SynchronizableOrganizationModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

