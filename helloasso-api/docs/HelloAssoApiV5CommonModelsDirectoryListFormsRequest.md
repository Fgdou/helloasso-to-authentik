# HelloAssoApiV5CommonModelsDirectoryListFormsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**form_name** | Option<**String**> | Textual search for form name | [optional]
**form_description** | Option<**String**> | Textual search for form description | [optional]
**form_zip_codes** | Option<**Vec<String>**> | The zip codes where the forms are located | [optional]
**form_cities** | Option<**Vec<String>**> | The cities where the forms are located | [optional]
**form_regions** | Option<**Vec<String>**> | The regions where the forms are located | [optional]
**form_departments** | Option<**Vec<String>**> | The departments where the forms are located | [optional]
**form_countries** | Option<**Vec<String>**> | The countries where the forms are located | [optional]
**form_types** | Option<[**Vec<models::HelloAssoApiV5CommonModelsEnumsFormType>**](HelloAssoApiV5CommonModelsEnumsFormType.md)> | The form types : CrowdFunding, Membership, Event, Donation, PaymentForm ... | [optional]
**form_activity_type** | Option<**Vec<String>**> | The Activity Type of the form | [optional]
**form_publication_start_date_min** | Option<**String**> | The inclusive minimum publication date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_publication_start_date_max** | Option<**String**> | The exclusive maximum publication date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_start_date_min** | Option<**String**> | The inclusive minimum start date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_start_date_max** | Option<**String**> | The exclusive maximum start date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_end_date_max** | Option<**String**> | The exclusive maximum end date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_end_date_min** | Option<**String**> | The inclusive minimum end date of the forms, format \"yyyy-MM-ddTHH:mm:ss.fffK\" | [optional]
**form_is_free** | Option<**bool**> | Allow only free forms if true | [optional]
**form_has_remaining_entries** | Option<**bool**> | Allow only forms with remaning entries if true | [optional]
**form_internal_tags** | Option<**Vec<String>**> | Allow only forms with internal tags  this filter is for special operations only | [optional]
**form_public_tags** | Option<**Vec<String>**> | Allow only forms with public tags | [optional]
**organization_name** | Option<**String**> | Textual search for organization name | [optional]
**organization_description** | Option<**String**> | Textual search for organization description | [optional]
**organization_categories** | Option<**Vec<String>**> | The categories of the forms | [optional]
**organization_types** | Option<**Vec<String>**> | The organization types | [optional]
**organization_zip_codes** | Option<**Vec<String>**> | The zip codes where the organizations are located | [optional]
**organization_cities** | Option<**Vec<String>**> | The cities where the organizations are located | [optional]
**organization_regions** | Option<**Vec<String>**> | The regions where the organizations are located | [optional]
**organization_departments** | Option<**Vec<String>**> | The departments where the organizations are located | [optional]
**organization_fiscal_receipt_eligibility** | Option<**bool**> | Allow only organization with a fiscal receipt eligibility | [optional]
**organization_linked_partners** | Option<**Vec<String>**> | Organization linked partners | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


