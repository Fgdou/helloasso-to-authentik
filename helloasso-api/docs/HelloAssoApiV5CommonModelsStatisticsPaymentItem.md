# HelloAssoApiV5CommonModelsStatisticsPaymentItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**share_amount** | Option<**i32**> | Amount of the payment assigned to the item and its options (in cents) | [optional]
**share_item_amount** | Option<**i32**> | Amount of the item payed on this payment term (in cents) | [optional]
**share_options_amount** | Option<**i32**> | Amount of all extra options linked to this item and payed on this payment (in cents) | [optional]
**id** | Option<**i32**> | ID of the Item | [optional]
**amount** | Option<**i32**> | Total item Price in cents (after discount without extra options) | [optional]
**r#type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsTierType**](HelloAssoApiV5CommonModelsEnumsTierType.md)> |  | [optional]
**initial_amount** | Option<**i32**> | The raw amount (without reduction) | [optional]
**state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsItemState**](HelloAssoApiV5CommonModelsEnumsItemState.md)> |  | [optional]
**name** | Option<**String**> | Name of the item paid (relevant for checkout forms) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


