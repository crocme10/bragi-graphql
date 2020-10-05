// use chrono::prelude::*;
// use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
// use futures::future::TryFutureExt;
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
// use url::Url;

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

fn default_limit() -> i32 {
    10i32
}

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLInputObject)]
pub struct AutocompleteParameters {
    #[serde(rename = "q")]
    query: String,
    // #[serde(default)]
    // pt_dataset: Vec<String>,

    // #[serde(default)]
    // poi_dataset: Vec<String>,

    // #[serde(rename = "_all_data", default)]
    // all_data: bool,

    // //Note: for the moment we can't use an external struct and flatten it (https://github.com/nox/serde_urlencoded/issues/33)
    // #[serde(default = "default_limit")]
    // limit: i32,

    // #[serde(default)]
    // offset: i32,

    // /// timeout in milliseconds
    // timeout: Option<i32>,

    // // Position of the request
    // lat: Option<f64>,
    // lon: Option<f64>,

    // // If specified, override parameters for the normal decay computed by elasticsearch around the
    // // position: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-function-score-query.html#_supported_decay_functions
    // proximity_scale: Option<f64>,
    // proximity_offset: Option<f64>,
    // proximity_decay: Option<f64>,

    // // FIXME Put me back
    // // #[serde(default, rename = "type")]
    // // types: Vec<Type>,

    // // FIXME Put me back
    // // #[serde(default, rename = "zone_type")]
    // // zone_types: Vec<cosmogony::ZoneType>,
    // #[serde(default, rename = "poi_type")]
    // poi_types: Vec<PoiType>,

    // lang: Option<String>,

    // // Forwards a request for explanation to Elastic Search.
    // // This parameter is useful to analyze the order in which search results appear.
    // // It is prefixed by an underscore to indicate its not a public parameter.
    // #[serde(default, rename = "_debug")]
    // debug: Option<bool>,
}
