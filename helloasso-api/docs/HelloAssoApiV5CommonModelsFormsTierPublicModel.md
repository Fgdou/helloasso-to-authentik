# HelloAssoApiV5CommonModelsFormsTierPublicModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**custom_fields** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel>**](HelloAssoApiV5CommonModelsFormsCustomFieldPublicModel.md)> | List of custom fields to be filled by the user | [optional]
**extra_options** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsExtraOptionPublicModel>**](HelloAssoApiV5CommonModelsFormsExtraOptionPublicModel.md)> | List of available extra options to buy along the tier | [optional]
**id** | Option<**i32**> | id | [optional]
**label** | Option<**String**> | label | [optional]
**description** | Option<**String**> | description | [optional]
**tier_type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsTierType**](HelloAssoApiV5CommonModelsEnumsTierType.md)> |  | [optional]
**price** | Option<**i32**> | the Price in cents  if price equals 0 then it is free or there is a MinAmount | [optional]
**vat_rate** | Option<**f64**> | Vat rate if applicable  Amount have to be 0.10 for 10% | [optional]
**min_amount** | Option<**i32**> | If set, it means the payment is free to choose, according to the specified minAmount in cents | [optional]
**payment_frequency** | Option<[**models::HelloAssoApiV5CommonModelsEnumsPaymentFrequencyType**](HelloAssoApiV5CommonModelsEnumsPaymentFrequencyType.md)> |  | [optional]
**max_per_user** | Option<**i32**> | Max quantity buyable in this cart | [optional]
**meta** | Option<[**models::HelloAssoApiV5CommonModelsCommonMetaModel**](HelloAssoApiV5CommonModelsCommonMetaModel.md)> |  | [optional]
**sale_start_date** | Option<**String**> | The datetime (Inclusive) at which the users can start buying this tier.  If null the tier will be available at the start of the event. | [optional]
**sale_end_date** | Option<**String**> | The datetime (Inclusive) at which the tier is no longer available.  If null the tier will be available until the end of the event. | [optional]
**is_eligible_tax_receipt** | Option<**bool**> | Whether this is eligible to a deduction | [optional]
**terms** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsTermModel>**](HelloAssoApiV5CommonModelsFormsTermModel.md)> | Terms of tier | [optional]
**picture** | Option<[**models::HelloAssoApiV5CommonModelsCommonDocumentModel**](HelloAssoApiV5CommonModelsCommonDocumentModel.md)> |  | [optional]
**is_excluded_from_form_payment_terms** | Option<**bool**> | True means this tier must be paid in the initial payment, false means it can be paid in payment with installments  Null when the form payment terms are disabled or not compatible with the related form | [optional]
**is_favorite** | Option<**bool**> | Indicates whether this tier is marked as a favorite.  Used only in Donation form type. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


