# HelloAssoApiV5CommonModelsFormsExtraOptionPublicModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_fields** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel>**](HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel.md)> | List of custom fields to be filled by the user | [optional]
**id** | Option<**i32**> | Id | [optional]
**price** | Option<**i32**> | Price of the extraOption, can be free | [optional]
**vat_rate** | Option<**f64**> | Vat rate if applicable  Amount have to be 0.10 for 10% | [optional]
**label** | Option<**String**> | The name of the option | [optional]
**description** | Option<**String**> | The description of the option | [optional]
**is_required** | Option<**bool**> | Additional option is required/mandatory | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


