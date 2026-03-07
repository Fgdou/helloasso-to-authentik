# \TagsApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tags_tag_name_get**](TagsApi.md#tags_tag_name_get) | **GET** /tags/{tagName} | Get Internal Tag Detail



## tags_tag_name_get

> models::HelloAssoApiV5CommonModelsTagsInternalTagModel tags_tag_name_get(tag_name, with_count, with_amount)
Get Internal Tag Detail

<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> FormOpenDirectory<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag_name** | **String** |  | [required] |
**with_count** | Option<**bool**> | If true : Count of times Tag is used |  |[default to false]
**with_amount** | Option<**bool**> | If true : Amount collected by all forms linked to this Tag |  |[default to false]

### Return type

[**models::HelloAssoApiV5CommonModelsTagsInternalTagModel**](HelloAsso.Api.V5.Common.Models.Tags.InternalTagModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

