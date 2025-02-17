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
pub struct ActorListActorsResponse {
    /// A list of actors for the project associated with the token.
    #[serde(rename = "actors")]
    pub actors: Vec<crate::models::ActorActor>,
}

impl ActorListActorsResponse {
    pub fn new(actors: Vec<crate::models::ActorActor>) -> ActorListActorsResponse {
        ActorListActorsResponse {
            actors,
        }
    }
}


