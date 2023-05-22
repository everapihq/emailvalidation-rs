/// Response of the emailvalidation api
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct DetailsResponse {
    /// Data source
    pub data: String,
    /// Request status
    pub meta: String,
}