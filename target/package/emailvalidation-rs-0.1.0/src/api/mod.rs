//! Module that contains the main [Emailvalidation] struct

use std::sync::Arc;
use reqwest::Client;
use crate::error::EmailvalidationError;
use crate::{error, models, utils};
use crate::utils::baseline::construct_base_url;

/// Settings struct that contains the api key
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: String,
}

/// The main struct of the crate giving access to the emailvalidation..
/// Create a new instance of the struct with your api key as parameter.
#[derive(Debug, Clone)]
pub struct Emailvalidation {
    client: Client,
    settings: Arc<Settings>,
}

impl<'a> Emailvalidation {
    /// Creates a new instance of the Emailvalidation struct by passing your api key as
    /// function parameter.
    pub fn new(api_key: &'a str) -> Result<Self, EmailvalidationError> {
        let settings = std::sync::Arc::new(Settings {
            api_key: String::from(api_key),
        });
        let client = utils::baseline::construct_client(None, &settings)?;
        Ok(Self { client, settings })
    }

    pub async fn status(
        &self,
    ) -> Result<models::DetailsResponse, error::EmailvalidationError> {
        let url = construct_base_url(&self.settings.api_key, Some("status"))?;
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::EmailvalidationError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::EmailvalidationError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::EmailvalidationError::ResponseParsingError { body: res_body })
    }

    pub async fn info(
        &self,
        email: &'a str,
        catch_all: &'a str,
    ) -> Result<models::DetailsResponse, error::EmailvalidationError> {
        let mut url = construct_base_url(&self.settings.api_key, Some("info"))?;
        url.query_pairs_mut()
            .append_pair("email", email)
            .append_pair("catch_all", catch_all);
        let res_body = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|err| error::EmailvalidationError::RequestError { source: err })?
            .text()
            .await
            .map_err(|err| error::EmailvalidationError::RequestError { source: err })?;
        serde_json::from_str::<models::DetailsResponse>(&res_body)
            .map_err(|_| error::EmailvalidationError::ResponseParsingError { body: res_body })
    }
}
