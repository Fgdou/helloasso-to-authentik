# HelloAssoApiV5CommonModelsPaymentRefundOperationModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The refund operation id | [optional]
**amount** | Option<**i32**> | The amount to refund | [optional]
**cancel_order** | Option<**bool**> | Whether the future payments and linked items of this order must be canceled (possible only if the payment is fully refunded) | [optional]
**creation_date** | Option<**String**> | The refund operation creation date | [optional]
**state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsOperationState**](HelloAssoApiV5CommonModelsEnumsOperationState.md)> |  | [optional]
**send_refund_mail** | Option<**bool**> | Whether a refund mail must be send or not. | [optional]
**payment_id** | Option<**i32**> | The payment id | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


