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
pub struct CloudVersionIdentityCustomAvatar {
	#[serde(rename = "upload_id")]
	pub upload_id: uuid::Uuid,
}

impl CloudVersionIdentityCustomAvatar {
	pub fn new(upload_id: uuid::Uuid) -> CloudVersionIdentityCustomAvatar {
		CloudVersionIdentityCustomAvatar { upload_id }
	}
}
