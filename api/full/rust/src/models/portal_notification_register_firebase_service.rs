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
pub struct PortalNotificationRegisterFirebaseService {
	#[serde(rename = "access_key")]
	pub access_key: String,
}

impl PortalNotificationRegisterFirebaseService {
	pub fn new(access_key: String) -> PortalNotificationRegisterFirebaseService {
		PortalNotificationRegisterFirebaseService { access_key }
	}
}
