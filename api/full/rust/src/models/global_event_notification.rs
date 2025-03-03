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
pub struct GlobalEventNotification {
	#[serde(rename = "title")]
	pub title: String,
	#[serde(rename = "description")]
	pub description: String,
	#[serde(rename = "thumbnail_url")]
	pub thumbnail_url: String,
	#[serde(rename = "url")]
	pub url: String,
}

impl GlobalEventNotification {
	pub fn new(
		title: String,
		description: String,
		thumbnail_url: String,
		url: String,
	) -> GlobalEventNotification {
		GlobalEventNotification {
			title,
			description,
			thumbnail_url,
			url,
		}
	}
}
