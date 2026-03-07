# HelloAssoApiV5CommonModelsFormsFormQuickCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tier_list** | Option<[**Vec<models::HelloAssoApiV5CommonModelsFormsTierLightModel>**](HelloAssoApiV5CommonModelsFormsTierLightModel.md)> |  | [optional]
**banner** | Option<**String**> | The banner of the form | [optional]
**description** | Option<**String**> | The description of form | [optional]
**end_date** | Option<**String**> | The datetime of the activity end | [optional]
**logo** | Option<**String**> | The logo of the form | [optional]
**private_title** | Option<**String**> | Private Title : displayed only in the organization back office | [optional]
**start_date** | Option<**String**> | The datetime of the activity start | [optional]
**title** | **String** | The title of the form. It will be used to generate the url which that can't be changed. | 
**activity_type_id** | Option<**i32**> | Activity type identifier, matching one of the provided type values <a href=\"index#!/Values/Values_Get\"> provided here</a> | [optional]
**place** | Option<[**models::HelloAssoApiV5CommonModelsCommonPlaceModel**](HelloAssoApiV5CommonModelsCommonPlaceModel.md)> |  | [optional]
**sale_end_date** | Option<**String**> | The datetime (Inclusive) at which the sales end.  If null the orders will be available until the end of the campaign. | [optional]
**sale_start_date** | Option<**String**> | The datetime (Inclusive) at which the users can start placing orders.  If null the orders will be available as soon as the campaign is published. | [optional]
**validity_type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsMembershipValidityType**](HelloAssoApiV5CommonModelsEnumsMembershipValidityType.md)> |  | [optional]
**accept_open_donation** | Option<**bool**> | Whether the user will be allowed to make a single open donation with an order. The amount of the donation is open, but 3 presets can be set in OpenDonationPresetAmount | [optional]
**accept_open_monthly_donation** | Option<**bool**> | Whether the user will be allowed to make a monthly open donation for donation forms | [optional]
**allow_comment** | Option<**bool**> | allowComment | [optional]
**amount_visible** | Option<**bool**> | amountVisible | [optional]
**color** | Option<**String**> | The color of the form | [optional]
**widget_button_text** | Option<**String**> | The text displayed in the widget button | [optional]
**contact** | Option<[**models::HelloAssoApiV5CommonModelsCommonContactModel**](HelloAssoApiV5CommonModelsCommonContactModel.md)> |  | [optional]
**display_contributor_name** | Option<**bool**> | Display contributor name for fundraiser and donation forms. | [optional]
**display_participants_count** | Option<**bool**> | Indicates that the members count must be displayed on the form. | [optional]
**display_remaining_entries** | Option<**bool**> | Indicates that the remaining entries must be displayed on the form. | [optional]
**financial_goal** | Option<**i64**> | Indicates the financial goal (amount of money raised) for the whole form. Null means no goal. | [optional]
**generate_membership_cards** | Option<**bool**> | Entrust the issuance of membership cards to HelloAsso (automatically sent by email to participants) | [optional]
**generate_tickets** | Option<**bool**> | Entrust the issuance of tickets to HelloAsso (automatically sent by email to participants) | [optional]
**invert_descriptions** | Option<**bool**> | Allows you to add the long description above the store catalog. | [optional]
**label_conditions_and_terms_file** | Option<**String**> | Label conditions and terms file | [optional]
**long_description** | Option<**String**> | The long description of the form (rich Html) | [optional]
**open_donation_preset_amounts** | Option<**Vec<i32>**> | The preset amounts to be shown to the user. Maximum 3 amounts. | [optional]
**personalized_message** | Option<**String**> | Personalized message for participants | [optional]
**project_beneficiaries** | Option<**String**> | The project beneficiaries of the form (rich Html) | [optional]
**project_expenses_details** | Option<**String**> | Details of the project expenses (rich Html) | [optional]
**project_owners** | Option<**String**> | Description of the project owners (rich Html) | [optional]
**suggest_monthly_donation** | Option<**bool**> | Suggest monthly donation for donation forms. | [optional]
**display_monthly_donations_first** | Option<**bool**> | The monthly donation option should be displayed or selected by default for donation forms. | [optional]
**project_target_country** | Option<**String**> | 3 letter country code | [optional]
**allow_organism_payer** | Option<**bool**> | Whether users are allowed to contribute to this form through an organism (only for donation and crowdfunding). | [optional]
**allow_individual_payer** | Option<**bool**> | Whether user are allowed to personally contribute to this form (only for donation and crowdfunding). | [optional]
**remind_abandoned_cart** | Option<**bool**> | Whether a reminder email should be sent for abandoned carts. | [optional]
**max_entries** | Option<**i32**> | Indicates the maximum available entries for the whole form. Null means unlimited entries. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


