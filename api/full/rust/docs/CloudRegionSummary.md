# CloudRegionSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**region_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**region_name_id** | **String** | A human readable short identifier used to references resources. Different than a `tivet.common#Uuid` because this is intended to be human readable. Different than `tivet.common#DisplayName` because this should not include special characters and be short. | 
**provider** | **String** | The server provider of this region. | 
**universal_region** | [**crate::models::CloudUniversalRegion**](CloudUniversalRegion.md) |  | 
**provider_display_name** | **String** | Represent a resource's readable display name. | 
**region_display_name** | **String** | Represent a resource's readable display name. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


