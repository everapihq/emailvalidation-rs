pub mod baseline {
    use crate::api;
    use crate::error::EmailvalidationError;
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use reqwest::{Client, Url};

    const BASE_URL: &str = "https://api.emailvalidation.io/v1/";

    pub fn construct_client(
        user_agent: Option<&str>,
        _settings: &api::Settings,
    ) -> Result<Client, EmailvalidationError> {
        let mut headers = HeaderMap::new();
        let content_type = HeaderValue::from_str("application/json")?;
        headers.insert(CONTENT_TYPE, content_type);
        let agent = user_agent.map_or_else(
            || format!("{}/{}", "", ""),
            String::from,
        );
        let client = Client::builder()
            .user_agent(agent)
            .default_headers(headers)
            .build()
            .map_err(|err| EmailvalidationError::ClientConstruction { source: err })?;
        Ok(client)
    }

    pub fn construct_base_url(
        api_key: &str,
        with_path: Option<&str>,
    ) -> Result<Url, EmailvalidationError> {
        let mut url = Url::parse(BASE_URL).map_err(|_| EmailvalidationError::UrlConstruction)?;
        url.query_pairs_mut().append_pair("apikey", api_key);
        if let Some(path) = with_path {
            url.set_path(path);
        }
        Ok(url)
    }
}

#[cfg(test)]
mod baseline_test {
    use super::baseline::*;

    #[test]
    fn should_create_base_url_with_api_key() {
        let base_url = construct_base_url("123", None).unwrap();
        assert_eq!(base_url.query(), Some("apikey=123"));
    }
}
