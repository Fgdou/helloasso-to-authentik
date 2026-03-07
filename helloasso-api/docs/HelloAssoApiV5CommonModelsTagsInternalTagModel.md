# HelloAssoApiV5CommonModelsTagsInternalTagModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Tag Id | [optional]
**name** | Option<**String**> | Name tag | [optional]
**form_count** | Option<**i32**> | Count of times Tag is used by forms | [optional]
**organization_count** | Option<**i32**> | Count of times Tag is used by Organizations | [optional]
**tag_type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsTagType**](HelloAssoApiV5CommonModelsEnumsTagType.md)> |  | [optional]
**tag_parent** | Option<[**models::HelloAssoApiV5CommonModelsTagsInternalTagModel**](HelloAssoApiV5CommonModelsTagsInternalTagModel.md)> |  | [optional]
**amount_collected** | Option<**i64**> | Amount collected by all forms linked to this tag (in cents) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


