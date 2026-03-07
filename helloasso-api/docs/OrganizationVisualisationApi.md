# \OrganizationVisualisationApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_organization_slug_get**](OrganizationVisualisationApi.md#organizations_organization_slug_get) | **GET** /organizations/{organizationSlug} | Get Organization details



## organizations_organization_slug_get

> models::HelloAssoApiV5CommonModelsOrganizationsOrganizationPublicModel organizations_organization_slug_get(organization_slug)
Get Organization details

Get the public information of the specified organization.<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization Slug | [required] |

### Return type

[**models::HelloAssoApiV5CommonModelsOrganizationsOrganizationPublicModel**](HelloAsso.Api.V5.Common.Models.Organizations.OrganizationPublicModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

