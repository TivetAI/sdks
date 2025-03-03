/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MatchmakerJoinRegion : A matchmaker lobby region.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MatchmakerJoinRegion {
    /// Represent a resource's readable display name.
    #[serde(rename = "display_name")]
    pub display_name: String,
    /// A human readable short identifier used to references resources. Different than a `uuid` because this is intended to be human readable. Different than `DisplayName` because this should not include special characters and be short.
    #[serde(rename = "region_id")]
    pub region_id: String,
}

impl MatchmakerJoinRegion {
    /// A matchmaker lobby region.
    pub fn new(display_name: String, region_id: String) -> MatchmakerJoinRegion {
        MatchmakerJoinRegion {
            display_name,
            region_id,
        }
    }
}


