/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ActorRegion {
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "name")]
	pub name: String,
}

impl ActorRegion {
	pub fn new(id: String, name: String) -> ActorRegion {
		ActorRegion { id, name }
	}
}
