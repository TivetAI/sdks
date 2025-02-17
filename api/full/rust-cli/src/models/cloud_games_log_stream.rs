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
pub enum CloudGamesLogStream {
    #[serde(rename = "std_out")]
    StdOut,
    #[serde(rename = "std_err")]
    StdErr,

}

impl ToString for CloudGamesLogStream {
    fn to_string(&self) -> String {
        match self {
            Self::StdOut => String::from("std_out"),
            Self::StdErr => String::from("std_err"),
        }
    }
}

impl Default for CloudGamesLogStream {
    fn default() -> CloudGamesLogStream {
        Self::StdOut
    }
}




