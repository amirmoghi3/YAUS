use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExpireWithClickCreationDTO {
    pub url: String,
    pub click:u64,
}
