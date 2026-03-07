# \OrganizationCashOutManagementApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_organization_slug_cash_out_cash_out_id_export_get**](OrganizationCashOutManagementApi.md#organizations_organization_slug_cash_out_cash_out_id_export_get) | **GET** /organizations/{organizationSlug}/cash-out/{cashOutId}/export | Download the cash-out details as a CSV, Excel or Json.



## organizations_organization_slug_cash_out_cash_out_id_export_get

> Vec<models::HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportRowModel> organizations_organization_slug_cash_out_cash_out_id_export_get(organization_slug, cash_out_id)
Download the cash-out details as a CSV, Excel or Json.

<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessTransactions<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization slug | [required] |
**cash_out_id** | **i32** | The cash-out id to details | [required] |

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsPaymentCashoutExportCashoutExportRowModel>**](HelloAsso.Api.V5.Common.Models.Payment.CashoutExport.CashoutExportRowModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.openxmlformats-officedocument.spreadsheetml.sheet, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

