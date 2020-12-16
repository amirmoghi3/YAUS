use bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct URL {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub timestamp: DateTime,
}
