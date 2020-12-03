use serde::Deserialize;
#[derive(Deserialize)]
pub struct CreateURLDTO {
    pub url: String,
}
