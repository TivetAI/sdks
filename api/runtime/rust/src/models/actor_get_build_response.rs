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
pub struct ActorGetBuildResponse {
    #[serde(rename = "build")]
    pub build: Box<crate::models::ActorBuild>,
}

impl ActorGetBuildResponse {
    pub fn new(build: crate::models::ActorBuild) -> ActorGetBuildResponse {
        ActorGetBuildResponse {
            build: Box::new(build),
        }
    }
}


