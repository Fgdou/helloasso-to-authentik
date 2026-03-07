# \ValuesDefinitionsApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**values_company_legal_status_get**](ValuesDefinitionsApi.md#values_company_legal_status_get) | **GET** /values/company-legal-status | Get company legal status list
[**values_form_form_type_types_get**](ValuesDefinitionsApi.md#values_form_form_type_types_get) | **GET** /values/form/{formType}/types | Get all activity types for a form type
[**values_organization_categories_get**](ValuesDefinitionsApi.md#values_organization_categories_get) | **GET** /values/organization/categories | Get list of JO categories
[**values_tags_get**](ValuesDefinitionsApi.md#values_tags_get) | **GET** /values/tags | Get list of public tags



## values_company_legal_status_get

> Vec<models::HelloAssoApiV5CommonModelsAccountsCompanyLegalStatusModel> values_company_legal_status_get()
Get company legal status list

<br/><br/>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsAccountsCompanyLegalStatusModel>**](HelloAsso.Api.V5.Common.Models.Accounts.CompanyLegalStatusModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## values_form_form_type_types_get

> Vec<models::HelloAssoApiV5CommonModelsFormsFormActivityModel> values_form_form_type_types_get(form_type)
Get all activity types for a form type

Use this in order to build your dropdown of form subtypes<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> FormAdministration<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_type** | [**HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md) |  | [required] |

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsFormsFormActivityModel>**](HelloAsso.Api.V5.Common.Models.Forms.FormActivityModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## values_organization_categories_get

> Vec<models::HelloAssoApiV5CommonModelsAccountsOrganismCategoryModel> values_organization_categories_get()
Get list of JO categories

Use this in order to build your category list of organization<br/><br/>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsAccountsOrganismCategoryModel>**](HelloAsso.Api.V5.Common.Models.Accounts.OrganismCategoryModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## values_tags_get

> Vec<models::HelloAssoApiV5CommonModelsTagsPublicTagModel> values_tags_get()
Get list of public tags

Use this in order to get list of used tags<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsTagsPublicTagModel>**](HelloAsso.Api.V5.Common.Models.Tags.PublicTagModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

