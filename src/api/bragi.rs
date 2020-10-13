use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::error;

#[derive(Debug, Deserialize, Serialize, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteResponseBody {
    /// This is the text of the response (json)
    pub resp: String,
}

impl From<String> for AutocompleteResponseBody {
    fn from(resp: String) -> Self {
        Self { resp }
    }
}

#[derive(Debug, Deserialize, Serialize, GraphQLInputObject)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteRequestBody {
    /// This is the text of the response (json)
    pub parameters: AutocompleteParameters,
}

pub async fn autocomplete(
    request: &AutocompleteRequestBody,
    url: &str,
) -> Result<AutocompleteResponseBody, error::Error> {
    let client = reqwest::Client::new();
    let req = client.get(url);
    let req = req.query(&request.parameters);

    let response = req.send().await.context(error::ReqwestError {
        // FIXME Need more information in the error msg
        details: String::from("Error querying bragi"),
    })?;

    response
        .text()
        .await
        .map(AutocompleteResponseBody::from)
        .context(error::ReqwestError {
            // FIXME Need more information in the error msg
            details: String::from("Error extracting bragi's response"),
        })
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, GraphQLEnum)]
pub enum PoiType {
    #[serde(rename = "city")]
    City,
    #[serde(rename = "house")]
    House,
    #[serde(rename = "poi")]
    Poi,
    #[serde(rename = "public_transport:stop_area")]
    StopArea,
    #[serde(rename = "street")]
    Street,
    #[serde(rename = "zone")]
    Zone,
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLInputObject)]
pub struct AutocompleteParameters {
    #[serde(rename = "q")]
    query: String,
}
