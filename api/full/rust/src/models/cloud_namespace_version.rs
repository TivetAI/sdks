/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudNamespaceVersion : A previously deployed namespace version.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudNamespaceVersion {
	/// A universally unique identifier.
	#[serde(rename = "namespace_id")]
	pub namespace_id: String,
	/// A universally unique identifier.
	#[serde(rename = "version_id")]
	pub version_id: String,
	/// RFC3339 timestamp
	#[serde(rename = "deploy_ts")]
	pub deploy_ts: String,
}

impl CloudNamespaceVersion {
	/// A previously deployed namespace version.
	pub fn new(
		namespace_id: String,
		version_id: String,
		deploy_ts: String,
	) -> CloudNamespaceVersion {
		CloudNamespaceVersion {
			namespace_id,
			version_id,
			deploy_ts,
		}
	}
}
