# HelloAssoApiV5CommonModelsStatisticsItemPayment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cash_out_state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentCashOutState**](HelloAssoApiV5CommonModelsEnumsPaymentCashOutState.md)> |  | [optional]
**share_amount** | Option<**i32**> | Amount of the item and extra options payed on this payment term (in cents) | [optional]
**id** | Option<**i32**> | The ID of the payment | [optional]
**amount** | Option<**i32**> | Total Amount of the payment (in cents) | [optional]
**amount_tip** | Option<**i32**> | Tip Amount of the payment (in cents) | [optional]
**date** | Option<**String**> | Date of the payment | [optional]
**payment_means** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentMeans**](HelloAssoApiV5CommonModelsEnumsPaymentMeans.md)> |  | [optional]
**installment_number** | Option<**i32**> | Indicates the payment number (useful in the case of an order comprising payments with installments) | [optional]
**state** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentState**](HelloAssoApiV5CommonModelsEnumsPaymentState.md)> |  | [optional]
**r#type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentType**](HelloAssoApiV5CommonModelsEnumsPaymentType.md)> |  | [optional]
**meta** | Option<[**models::HelloAssoApiV5CommonModelsCommonMetaModel**](HelloAssoApiV5CommonModelsCommonMetaModel.md)> |  | [optional]
**payment_off_line_mean** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentMeans**](HelloAssoApiV5CommonModelsEnumsPaymentMeans.md)> |  | [optional]
**refund_operations** | Option<[**Vec<models::HelloAssoApiV5CommonModelsStatisticsRefundOperationLightModel>**](HelloAssoApiV5CommonModelsStatisticsRefundOperationLightModel.md)> | The refund operations information for the specific payment. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


