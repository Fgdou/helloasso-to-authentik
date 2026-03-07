# \UsersApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_me_organizations_get**](UsersApi.md#users_me_organizations_get) | **GET** /users/me/organizations | Get my organizations



## users_me_organizations_get

> Vec<models::HelloAssoApiV5CommonModelsOrganizationsOrganizationLightModel> users_me_organizations_get()
Get my organizations

Returns the list of organizations where the connected user has rights ( Form or Organization itself )<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsOrganizationsOrganizationLightModel>**](HelloAsso.Api.V5.Common.Models.Organizations.OrganizationLightModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

