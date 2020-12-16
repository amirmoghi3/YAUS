use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExpireWithTimeCreationDTO {
    pub url: String,
    pub expire_at: i64,
}
