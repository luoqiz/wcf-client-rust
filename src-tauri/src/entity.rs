use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct KCoinfig {
    pub cburl: String,
    pub wsurl: String,
    pub file_dir: String,
}