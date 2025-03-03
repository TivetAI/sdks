/*
 * Tivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CloudGamesBuildKind {
    #[serde(rename = "docker_image")]
    DockerImage,
    #[serde(rename = "oci_bundle")]
    OciBundle,

}

impl ToString for CloudGamesBuildKind {
    fn to_string(&self) -> String {
        match self {
            Self::DockerImage => String::from("docker_image"),
            Self::OciBundle => String::from("oci_bundle"),
        }
    }
}

impl Default for CloudGamesBuildKind {
    fn default() -> CloudGamesBuildKind {
        Self::DockerImage
    }
}




