/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// CloudBootstrapDomains : Domains that host parts of Tivet

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudBootstrapDomains {
	#[serde(rename = "cdn", skip_serializing_if = "Option::is_none")]
	pub cdn: Option<String>,
	#[serde(rename = "job", skip_serializing_if = "Option::is_none")]
	pub job: Option<String>,
	#[serde(rename = "main", skip_serializing_if = "Option::is_none")]
	pub main: Option<String>,
	#[serde(rename = "opengb", skip_serializing_if = "Option::is_none")]
	pub opengb: Option<String>,
}

impl CloudBootstrapDomains {
	/// Domains that host parts of Tivet
	pub fn new() -> CloudBootstrapDomains {
		CloudBootstrapDomains {
			cdn: None,
			job: None,
			main: None,
			opengb: None,
		}
	}
}
