/*
 * Oxidrive API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListInfo {
    /// number of items in the current slice of the collection
    #[serde(rename = "count")]
    pub count: u32,
    /// total number of items in the collection
    #[serde(rename = "total")]
    pub total: u32,
    /// Cursor of the next element, to be used as the `after` parameter in paginated operations
    #[serde(rename = "next", deserialize_with = "Option::deserialize")]
    pub next: Option<String>,
}

impl ListInfo {
    pub fn new(count: u32, total: u32, next: Option<String>) -> ListInfo {
        ListInfo { count, total, next }
    }
}