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
pub struct CloudVersionCdnRoute {
	#[serde(rename = "glob")]
	pub glob: String,
	/// Unsigned 32 bit integer.
	#[serde(rename = "priority")]
	pub priority: i32,
	/// Multiple CDN version middleware.
	#[serde(rename = "middlewares")]
	pub middlewares: Vec<crate::models::CloudVersionCdnMiddleware>,
}

impl CloudVersionCdnRoute {
	pub fn new(
		glob: String,
		priority: i32,
		middlewares: Vec<crate::models::CloudVersionCdnMiddleware>,
	) -> CloudVersionCdnRoute {
		CloudVersionCdnRoute {
			glob,
			priority,
			middlewares,
		}
	}
}
