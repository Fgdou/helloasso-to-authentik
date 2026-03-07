# HelloAssoApiV5CommonModelsOrganizationsOrganizationPublicModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**facebook_page** | Option<**String**> |  | [optional]
**gallery_images** | Option<[**Vec<models::HelloAssoApiV5CommonModelsCommonImageModel>**](HelloAssoApiV5CommonModelsCommonImageModel.md)> |  | [optional]
**long_description** | Option<**String**> |  | [optional]
**twitter_page** | Option<**String**> |  | [optional]
**videos** | Option<[**Vec<models::HelloAssoApiV5CommonModelsCommonVideoModel>**](HelloAssoApiV5CommonModelsCommonVideoModel.md)> |  | [optional]
**web_site** | Option<**String**> |  | [optional]
**is_authenticated** | Option<**bool**> | The organization is authenticated. Property returned only when asked by an organization admin. | [optional]
**display_coordinates** | Option<**bool**> | If the organization chose to display its coordinates. Property returned only when asked by an organization admin. | [optional]
**is_cash_in_compliant** | Option<**bool**> | If transaction can be init on the organization or not. Property returned only when asked by an organization admin. | [optional]
**banner** | Option<**String**> | The organization banner | [optional]
**fiscal_receipt_eligibility** | Option<**bool**> | The organism can issue fiscal receipts (type ok and has not deactivated it)  Must configure it and be authenticated to become enabled | [optional]
**fiscal_receipt_issuance_enabled** | Option<**bool**> | The organism is eligible, has set up his options, and is authenticated. | [optional]
**r#type** | Option<[**models::HelloAssoApiV5CommonModelsEnumsOrganizationType**](HelloAssoApiV5CommonModelsEnumsOrganizationType.md)> |  | [optional]
**category** | Option<**String**> | Organization category label | [optional]
**address** | Option<**String**> | Organization Address (for authorized applications or if authorized by the organization) | [optional]
**geolocation** | Option<[**models::HelloAssoModelsSharedGeoLocation**](HelloAssoModelsSharedGeoLocation.md)> |  | [optional]
**rna_number** | Option<**String**> | Unique identifier assigned when creating the association | [optional]
**logo** | Option<**String**> | Logo of organization | [optional]
**name** | Option<**String**> | Name of organization | [optional]
**role** | Option<[**models::HelloAssoModelsEnumsGlobalRole**](HelloAssoModelsEnumsGlobalRole.md)> |  | [optional]
**city** | Option<**String**> | Organization city | [optional]
**zip_code** | Option<**String**> | Organization zip code | [optional]
**description** | Option<**String**> | Organization description | [optional]
**update_date** | Option<**String**> | Last update date of the organization | [optional]
**category_jo_id** | Option<**i32**> |  | [optional]
**url** | Option<**String**> | The organization url | [optional]
**organization_slug** | Option<**String**> | The organization slug | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


