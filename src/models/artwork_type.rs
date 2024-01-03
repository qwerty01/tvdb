/*
 * TVDB API V4
 *
 * Documentation of [TheTVDB](https://thetvdb.com/) API V4. All related information is linked from our [Github repo](https://github.com/thetvdb/v4-api). You might also want to use our [Postman collection] (https://www.getpostman.com/collections/7a9397ce69ff246f74d0) ## Authentication 1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call. 2. Executing this call will provide you with a bearer token, which is valid for 1 month. 3. Provide your bearer token for subsequent API calls by clicking Authorize below or including in the header of all direct API calls: `Authorization: Bearer [your-token]`  ## Notes 1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.
 *
 * The version of the OpenAPI document: 4.7.8
 *
 * Generated by: https://openapi-generator.tech
 */

/// ArtworkType : artwork type record

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtworkType {
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "imageFormat", skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "recordType", skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "thumbHeight", skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
    #[serde(rename = "thumbWidth", skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl ArtworkType {
    /// artwork type record
    pub fn new() -> ArtworkType {
        ArtworkType {
            height: None,
            id: None,
            image_format: None,
            name: None,
            record_type: None,
            slug: None,
            thumb_height: None,
            thumb_width: None,
            width: None,
        }
    }
}
