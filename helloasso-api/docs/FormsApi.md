# \FormsApi

All URIs are relative to *https://api.helloasso.com/v5*

Method | HTTP request | Description
------------- | ------------- | -------------
[**organizations_organization_slug_form_types_get**](FormsApi.md#organizations_organization_slug_form_types_get) | **GET** /organizations/{organizationSlug}/formTypes | Get a list of formTypes for an organization
[**organizations_organization_slug_forms_form_type_action_quick_create_post**](FormsApi.md#organizations_organization_slug_forms_form_type_action_quick_create_post) | **POST** /organizations/{organizationSlug}/forms/{formType}/action/quick-create | Create a simplified event for an Organism
[**organizations_organization_slug_forms_form_type_form_slug_public_get**](FormsApi.md#organizations_organization_slug_forms_form_type_form_slug_public_get) | **GET** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/public | Get detailed public data about a specific form
[**organizations_organization_slug_forms_form_type_form_slug_state_put**](FormsApi.md#organizations_organization_slug_forms_form_type_form_slug_state_put) | **PUT** /organizations/{organizationSlug}/forms/{formType}/{formSlug}/state | Update a form state
[**organizations_organization_slug_forms_get**](FormsApi.md#organizations_organization_slug_forms_get) | **GET** /organizations/{organizationSlug}/forms | Get the forms of a specific organization



## organizations_organization_slug_form_types_get

> Vec<models::HelloAssoApiV5CommonModelsEnumsFormType> organizations_organization_slug_form_types_get(organization_slug, states)
Get a list of formTypes for an organization

List all the formTypes where the organization has at least one form. This also can be filtered by states.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization Slug | [required] |
**states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormState>**](Models__HelloAssoApiV5CommonModelsEnumsFormState.md)> | List of Form States to filter with. If none specified, it won't filter results.  Available values: * `Public` - The form is publicly visible and findable on search engines * `Private` - The form is visible only with the URL, you cannot find it on search engines * `Draft` - The form is not yet published but visible if you have admin rights * `Disabled` - The form is disabled and can be reenabled by changing state to public or private |  |

### Return type

[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormType>**](HelloAsso.Api.V5.Common.Models.Enums.FormType.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_form_type_action_quick_create_post

> models::HelloAssoApiV5CommonModelsFormsFormQuickCreateModel organizations_organization_slug_forms_form_type_action_quick_create_post(organization_slug, form_type, hello_asso_api_v5_common_models_forms_form_quick_create_request)
Create a simplified event for an Organism

This is a limited service to create an event with only limited information and few simple pricing.  Event created this way can be further edited with other services<br/><br/><b>Your token must have one of these roles : </b><br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> FormAdministration<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization Slug | [required] |
**form_type** | [**HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md) | The form type to create - only Event type is supported | [required] |
**hello_asso_api_v5_common_models_forms_form_quick_create_request** | Option<[**HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest**](HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest.md)> | The body of the request. |  |

### Return type

[**models::HelloAssoApiV5CommonModelsFormsFormQuickCreateModel**](HelloAsso.Api.V5.Common.Models.Forms.FormQuickCreateModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_form_type_form_slug_public_get

> models::HelloAssoApiV5CommonModelsFormsFormPublicModel organizations_organization_slug_forms_form_type_form_slug_public_get(organization_slug, form_type, form_slug)
Get detailed public data about a specific form

This api allows you to retrieve all public information of a form whether it is Crowdfunding, Membership, Event, Donation...<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**form_type** | [**HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md) |  | [required] |
**form_slug** | **String** |  | [required] |

### Return type

[**models::HelloAssoApiV5CommonModelsFormsFormPublicModel**](HelloAsso.Api.V5.Common.Models.Forms.FormPublicModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_form_type_form_slug_state_put

> organizations_organization_slug_forms_form_type_form_slug_state_put(organization_slug, form_slug, form_type, hello_asso_api_v5_common_models_forms_form_state_request)
Update a form state

Update form state.<br/><br/><b>Your token must have one of these roles : </b><br/>FormAdmin<br/>OrganizationAdmin<br/><br/>If you are an <b>association</b>, you can obtain these roles with your client.<br/>If you are a <b>partner</b>, you can obtain these roles by the authorize flow.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> FormAdministration<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** |  | [required] |
**form_slug** | **String** |  | [required] |
**form_type** | [**HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md) |  | [required] |
**hello_asso_api_v5_common_models_forms_form_state_request** | Option<[**HelloAssoApiV5CommonModelsFormsFormStateRequest**](HelloAssoApiV5CommonModelsFormsFormStateRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: application/json-patch+json, application/json, text/json, application/*+json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## organizations_organization_slug_forms_get

> models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelFormLightModel organizations_organization_slug_forms_get(organization_slug, states, form_types, page_index, page_size, continuation_token)
Get the forms of a specific organization

List all forms matching the filtered states and types.  If filters are left empty, no filter is applied.  Results are ordered by creation date descending.<br/><br/><b>Your clientId must be allowed all of those privileges : </b> <br/> AccessPublicData<br/><br/>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_slug** | **String** | The organization Slug | [required] |
**states** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormState>**](Models__HelloAssoApiV5CommonModelsEnumsFormState.md)> | States to filter  Available values: * `Public` - The form is publicly visible and findable on search engines * `Private` - The form is visible only with the URL, you cannot find it on search engines * `Draft` - The form is not yet published but visible if you have admin rights * `Disabled` - The form is disabled and can be reenabled by changing state to public or private |  |
**form_types** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormType>**](Models__HelloAssoApiV5CommonModelsEnumsFormType.md)> | Types to filter |  |
**page_index** | Option<**i32**> | The page of results to retrieve |  |[default to 1]
**page_size** | Option<**i32**> | The number of items per page |  |[default to 20]
**continuation_token** | Option<**String**> | Continuation Token from which we wish to retrieve results |  |

### Return type

[**models::HelloAssoApiV5CommonModelsCommonResultsWithPaginationModelFormLightModel**](HelloAsso.Api.V5.Common.Models.Common.ResultsWithPaginationModel_FormLightModel.md)

### Authorization

[User](../README.md#User), [bearer](../README.md#bearer), [Client](../README.md#Client)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json, text/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

