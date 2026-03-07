# HelloAssoApiV5CommonModelsFormsFormPublicModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**organization_logo** | Option<**String**> | Organization Logo | [optional]
**organization_name** | Option<**String**> | Organization Name | [optional]
**tiers** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsTierPublicModel>**](HelloAssoApiV5CommonModelsFormsTierPublicModel.md)> | Tiers | [optional]
**activity_type** | Option<**String**> | Activity type of the event eg. \"Atelier(s) / Stage(s)\" matching one of the provided type values <a href=\"index#!/Values/Values_Get\"> provided here</a> or a custom value is allowed. | [optional]
**activity_type_id** | Option<**i32**> | Activity type identifier | [optional]
**place** | Option<[**models::HelloAssoApiV5CommonModelsCommonPlaceModel**](HelloAssoApiV5CommonModelsCommonPlaceModel.md)> |  | [optional]
**sale_end_date** | Option<**String**> | The datetime (Inclusive) at which the sales end.  If null the orders will be available until the end of the campaign. | [optional]
**sale_start_date** | Option<**String**> | The datetime (Inclusive) at which the users can start placing orders.  If null the orders will be available as soon as the campaign is published. | [optional]
**validity_type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsMembershipValidityType**](HelloAssoApiV5CommonModelsEnumsMembershipValidityType.md)> |  | [optional]
**personalized_message** | Option<**String**> | A message customized by the organization administrator. | [optional]
**banner** | Option<[**models::HelloAssoApiV5CommonModelsCommonDocumentModel**](HelloAssoApiV5CommonModelsCommonDocumentModel.md)> |  | [optional]
**currency** | Option<**String**> | Currency | [optional]
**description** | Option<**String**> | Short description (one line) | [optional]
**start_date** | Option<**String**> | The datetime of the activity start | [optional]
**end_date** | Option<**String**> | The datetime of the activity end | [optional]
**logo** | Option<[**models::HelloAssoApiV5CommonModelsCommonDocumentModel**](HelloAssoApiV5CommonModelsCommonDocumentModel.md)> |  | [optional]
**meta** | Option<[**models::HelloAssoApiV5CommonModelsCommonMetaModel**](HelloAssoApiV5CommonModelsCommonMetaModel.md)> |  | [optional]
**state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsFormState**](HelloAssoApiV5CommonModelsEnumsFormState.md)> |  | [optional]
**title** | Option<**String**> | Title | [optional]
**private_title** | Option<**String**> | Private Title | [optional]
**widget_button_url** | Option<**String**> | Url of the widget button | [optional]
**widget_full_url** | Option<**String**> | Url of the form widget | [optional]
**widget_vignette_horizontal_url** | Option<**String**> | Url of the horizontal vignette widget | [optional]
**widget_vignette_vertical_url** | Option<**String**> | Url of the vertical vignette widget | [optional]
**widget_counter_url** | Option<**String**> | Url of the counter widget | [optional]
**form_slug** | Option<**String**> | The form slug | [optional]
**form_type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsFormType**](HelloAssoApiV5CommonModelsEnumsFormType.md)> |  | [optional]
**url** | Option<**String**> | The form url | [optional]
**organization_slug** | Option<**String**> | The organization slug | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


