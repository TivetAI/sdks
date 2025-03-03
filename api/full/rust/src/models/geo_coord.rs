/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

/// GeoCoord : Geographical coordinates for a location on Planet Earth.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GeoCoord {
	#[serde(rename = "latitude")]
	pub latitude: f64,
	#[serde(rename = "longitude")]
	pub longitude: f64,
}

impl GeoCoord {
	/// Geographical coordinates for a location on Planet Earth.
	pub fn new(latitude: f64, longitude: f64) -> GeoCoord {
		GeoCoord {
			latitude,
			longitude,
		}
	}
}
