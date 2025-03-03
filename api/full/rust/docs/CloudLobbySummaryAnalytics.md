# CloudLobbySummaryAnalytics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lobby_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**lobby_group_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**lobby_group_name_id** | **String** | A human readable short identifier used to references resources. Different than a `tivet.common#Uuid` because this is intended to be human readable. Different than `tivet.common#DisplayName` because this should not include special characters and be short. | 
**region_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**create_ts** | **String** | RFC3339 timestamp | 
**is_ready** | **bool** | Whether or not this lobby is ready. | 
**is_idle** | **bool** | Whether or not this lobby is idle. | 
**is_closed** | **bool** | Whether or not this lobby is in a closed state. | 
**is_outdated** | **bool** | Whether or not this lobby is outdated. | 
**max_players_normal** | **i32** | Unsigned 32 bit integer. | 
**max_players_direct** | **i32** | Unsigned 32 bit integer. | 
**max_players_party** | **i32** | Unsigned 32 bit integer. | 
**total_player_count** | **i32** | Unsigned 32 bit integer. | 
**registered_player_count** | **i32** | Unsigned 32 bit integer. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


