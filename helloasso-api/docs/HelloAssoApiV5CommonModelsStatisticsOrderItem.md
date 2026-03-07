# HelloAssoApiV5CommonModelsStatisticsOrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**payments** | Option<[**Vec<models::HelloAssoApiV5CommonModelsStatisticsSharePayment>**](HelloAssoApiV5CommonModelsStatisticsSharePayment.md)> | Payments linked to this item and each share between the item and the payment | [optional]
**name** | Option<**String**> |  | [optional]
**user** | Option<[**models::HelloAssoApiV5CommonModelsStatisticsUser**](HelloAssoApiV5CommonModelsStatisticsUser.md)> |  | [optional]
**price_category** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPriceCategory**](HelloAssoApiV5CommonModelsEnumsPriceCategory.md)> |  | [optional]
**min_amount** | Option<**i32**> | Minimum amount that was specified on the tier (in cents) | [optional]
**discount** | Option<[**models::HelloAssoApiV5CommonModelsStatisticsItemDiscount**](HelloAssoApiV5CommonModelsStatisticsItemDiscount.md)> |  | [optional]
**custom_fields** | Option<[**Vec<models::HelloAssoApiV5CommonModelsStatisticsItemCustomField>**](HelloAssoApiV5CommonModelsStatisticsItemCustomField.md)> | Custom fields related to this item | [optional]
**options** | Option<[**Vec<models::HelloAssoApiV5CommonModelsStatisticsItemOption>**](HelloAssoApiV5CommonModelsStatisticsItemOption.md)> | Extra options taken with this item | [optional]
**ticket_url** | Option<**String**> | The Ticket Url | [optional]
**qr_code** | Option<**String**> | The item QrCode (for ticket scanning only) | [optional]
**membership_card_url** | Option<**String**> | The Membership Card Url | [optional]
**day_of_levy** | Option<**i32**> | The day of levy for monthly donation only | [optional]
**tier_description** | Option<**String**> | Tier description | [optional]
**tier_id** | Option<**i32**> |  | [optional]
**comment** | Option<**String**> |  | [optional]
**id** | Option<**i32**> | ID of the Item | [optional]
**amount** | Option<**i32**> | Total item Price in cents (after discount without extra options) | [optional]
**r#type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsTierType**](HelloAssoApiV5CommonModelsEnumsTierType.md)> |  | [optional]
**initial_amount** | Option<**i32**> | The raw amount (without reduction) | [optional]
**state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsItemState**](HelloAssoApiV5CommonModelsEnumsItemState.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


